//! The threads behind a signed-in session (gaia socket, chat, calls) and how a frontend talks to them: it hands every
//! event to one `Emit` closure and drains the `State` outboxes into `Backend::flush`. No drawing, no window.
use crate::app::{LoginAct, State};
use crate::calls::{MediaCmd, MediaEvent, MediaTx};
use crate::chat::ChatCmd;
use crate::login;
use crate::net;
use crate::proto::{Cmd, Net};
use crate::session::{self, Auth, SessionFile};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;

/// Everything the worker threads report back.
pub enum Event {
    Net(Net),
    Media(MediaEvent),
    /// The browser sign-in finished: a ready session, or why it did not work.
    Login(Result<Auth, String>),
    /// Push to talk from outside the terminal or window (desktop shortcut or portal).
    Talk(crate::talk::TalkAct),
}

pub type Emit = Arc<dyn Fn(Event) + Send + Sync>;

pub struct Backend {
    cmds: Sender<Cmd>,
    media: Option<MediaTx>,
    chat: Option<Sender<ChatCmd>>,
    /// Loads avatar pictures; only the graphical client wants them.
    avatars: Option<Sender<(String, String)>>,
    emit: Emit,
    dry: bool,
    pub session_path: String,
    login_cancel: Option<Arc<AtomicBool>>,
    logout_started: Option<std::time::Instant>,
    pub overlay: Option<crate::overlay::Handle>,
}

impl Backend {
    pub fn new(emit: Emit, dry: bool, session_path: String) -> Self {
        Backend { cmds: channel().0, media: None, chat: None, avatars: None, emit, dry, session_path, login_cancel: None, logout_started: None, overlay: None }
    }

    /// Connect everything for a signed-in user: gaia socket, chat, calls (and avatar pictures if `pictures`).
    pub fn start(&mut self, auth: Auth, pictures: bool) {
        self.session_path = auth.path.clone();
        let (tx, rx) = channel();
        self.cmds = tx;
        let (emit, dry) = (self.emit.clone(), self.dry);
        let net_emit: crate::chat::Emit = {
            let e = emit.clone();
            Arc::new(move |n| e(Event::Net(n)))
        };
        if !dry {
            let (ctx, crx) = channel();
            self.chat = Some(ctx);
            crate::chat::spawn(auth.clone(), net_emit.clone(), crx);
        }
        std::thread::spawn(move || net::run(auth, dry, move |e| net_emit(e), rx));
        #[cfg(feature = "calls")]
        if !dry {
            let e = emit.clone();
            self.media = Some(crate::media::spawn(move |m| e(Event::Media(m))));
        }
        #[cfg(feature = "gui")]
        if pictures && self.avatars.is_none() {
            let e = emit;
            self.avatars = Some(crate::avatars::spawn(move |n| e(Event::Net(n))));
        }
        let _ = pictures;
        self.logout_started = None;
        self.login_cancel = None;
    }

    /// Open the browser sign-in on its own thread; the outcome comes back as `Event::Login`.
    pub fn begin_login(&mut self, st: &mut State) {
        let cancel = Arc::new(AtomicBool::new(false));
        self.login_cancel = Some(cancel.clone());
        st.login = Some(crate::app::LoginPhase::Waiting);
        let (path, emit) = (self.session_path.clone(), self.emit.clone());
        std::thread::spawn(move || {
            let known = session::load_space(&path);
            let opts = login::Options { start_url: login::login_url(&known), origin: "https://app.sowork.com".into(), known_space: known, headless: false };
            let res = login::capture(&opts, &cancel).and_then(|c| {
                let find = serde_json::json!({"type": "group", "groupId": c.space.0, "alias": c.space.1, "redirect": true});
                session::save_session(&path, &SessionFile { api_key: c.api_key, refresh_token: c.refresh_token, find })?;
                session::save_space(&path, &c.space.0, &c.space.1);
                Auth::load(&path)
            });
            emit(Event::Login(res));
        });
    }

    /// Hand everything the state queued to the threads that act on it.
    pub fn flush(&mut self, st: &mut State, start: std::time::Instant) {
        match st.login_act.take() {
            Some(LoginAct::Start) => self.begin_login(st),
            Some(LoginAct::Retry) => {
                let (path, emit) = (self.session_path.clone(), self.emit.clone());
                std::thread::spawn(move || emit(Event::Login(Auth::load(&path))));
            }
            Some(LoginAct::Cancel) => {
                if let Some(c) = &self.login_cancel {
                    c.store(true, Ordering::Relaxed);
                }
            }
            None => {}
        }
        if st.logout_requested && self.logout_started.is_none() {
            // leave the call, close the socket cleanly, then forget the session (see finish_logout)
            self.logout_started = Some(std::time::Instant::now());
            if let Some(tx) = &self.media {
                let _ = tx.send(MediaCmd::Leave);
            }
            let _ = self.cmds.send(Cmd::Logout);
        }
        for c in st.outbox.drain(..) {
            let _ = self.cmds.send(c);
        }
        if let Some(o) = st.overlay_out.take() {
            self.overlay.get_or_insert_with(crate::overlay::Handle::new).show(o);
        }
        if std::mem::take(&mut st.hint_out) {
            crate::notify::talk_hint(&st.talk_key);
        }
        for snd in st.sounds.drain(..) {
            crate::sound::play(snd);
        }
        for c in st.chat_out.drain(..) {
            if let Some(tx) = &self.chat {
                let _ = tx.send(c);
            }
        }
        for req in st.avatar_req.drain(..) {
            if let Some(tx) = &self.avatars {
                let _ = tx.send(req);
            }
        }
        for m in st.media_out.drain(..) {
            eprintln!("[{:6.1}] media command: {}", start.elapsed().as_secs_f64(), match &m {
                MediaCmd::Join { url, room_id, .. } => format!("join {url} room {room_id}{}", if self.media.is_none() { "  (media disabled: dry run / demo)" } else { "" }),
                other => format!("{other:?}"),
            });
            if let Some(tx) = &self.media {
                let _ = tx.send(m);
            }
        }
    }

    pub fn logging_out(&self) -> Option<std::time::Duration> {
        self.logout_started.map(|t| t.elapsed())
    }

    /// Forget the saved sign-in. Called when the socket thread confirms it closed, or after a timeout.
    pub fn finish_logout(&mut self) {
        if self.media.is_some() {
            std::thread::sleep(std::time::Duration::from_millis(300)); // let the LiveKit leave message go out
        }
        let p = &self.session_path;
        match std::fs::remove_file(p) {
            Ok(()) => println!("logged out: removed {p}"),
            Err(e) => eprintln!("logged out, but could not remove {p}: {e}"),
        }
        self.media = None; // ends the media thread
        self.chat = None; // ends the chat threads
        self.cmds = channel().0;
        self.logout_started = None;
    }
}
