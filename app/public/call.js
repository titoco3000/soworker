// Calls: LiveKit room joined from the token in the `cluster` event (zone or proximity meetings).
// Mic and camera start OFF; we join muted so walking past someone never opens your microphone.
const bar = { info: document.getElementById('callinfo'), mic: document.getElementById('mic'), cam: document.getElementById('cam'), away: document.getElementById('away'), snd: document.getElementById('snd') };
const tiles = document.getElementById('tiles'), mediaBox = document.getElementById('media');
let audioBlocked = false, lk = null, lkRoom = null, lkRoomId = null, cluster = null, micOn = false, camOn = false, available = true, joining = false;

function loadLivekit() {
  if (lk) return Promise.resolve(lk);
  return new Promise((resolve, reject) => {
    const s = document.createElement('script');
    s.src = '/vendor/livekit-client.umd.js';
    s.onload = () => resolve((lk = window.LivekitClient));
    s.onerror = () => reject(new Error('could not load livekit-client'));
    document.head.appendChild(s);
  });
}
const nameOf = (id) => users.get(id)?.name ?? cluster?.users.find((u) => u.id === id)?.name ?? id.slice(0, 6);

function renderBar() {
  const n = cluster ? cluster.users.filter((u) => u.id !== meId).length : 0;
  bar.info.textContent = lkRoom ? `in call (${n} other${n === 1 ? '' : 's'})` : joining ? 'joining…' : cluster?.unsupported ? `call provider ${cluster.unsupported} not supported` : '';
  bar.mic.textContent = micOn ? 'mic on' : 'mic off'; bar.mic.className = micOn ? 'on' : 'off';
  bar.cam.textContent = camOn ? 'cam on' : 'cam off'; bar.cam.className = camOn ? 'on' : '';
  bar.away.textContent = available ? 'available' : 'away'; bar.away.className = available ? 'on' : 'off';
  bar.mic.disabled = bar.cam.disabled = !lkRoom;
  bar.snd.hidden = !(lkRoom && audioBlocked);
  dirty = true;
}

function addTile(id, track, local) {
  removeTile(id);
  const el = track.attach();
  el.muted = !!local; el.playsInline = true;
  if (local) el.style.transform = 'scaleX(-1)';
  const t = document.createElement('div');
  t.className = 'tile'; t.dataset.id = id;
  const label = document.createElement('span'); label.textContent = local ? 'you' : nameOf(id);
  t.append(el, label); tiles.appendChild(t);
}
const removeTile = (id) => tiles.querySelectorAll(`.tile[data-id="${id}"]`).forEach((t) => t.remove());

async function leave() {
  const r = lkRoom; lkRoom = null; lkRoomId = null; micOn = camOn = false;
  tiles.replaceChildren(); mediaBox.replaceChildren();
  if (r) { try { await r.disconnect(); } catch {} act({ ev: 'camera', on: false }); act({ ev: 'speech', on: false }); }
  renderBar();
}

async function join(c) {
  joining = true; renderBar();
  try {
    const { Room, RoomEvent, Track } = await loadLivekit();
    const room = new Room({ adaptiveStream: true, dynacast: true, videoCaptureDefaults: { resolution: { width: 640, height: 360, frameRate: 15 } } });
    room
      .on(RoomEvent.TrackSubscribed, (track, pub, p) => {
        if (track.kind === Track.Kind.Audio) mediaBox.appendChild(track.attach());
        else addTile(p.identity, track, false);
      })
      .on(RoomEvent.TrackUnsubscribed, (track, pub, p) => { track.detach().forEach((el) => el.remove()); if (track.kind === Track.Kind.Video) removeTile(p.identity); })
      .on(RoomEvent.ParticipantDisconnected, (p) => removeTile(p.identity))
      .on(RoomEvent.LocalTrackPublished, (pub) => { if (pub.track?.kind === Track.Kind.Video) addTile('local', pub.track, true); })
      .on(RoomEvent.LocalTrackUnpublished, (pub) => { if (pub.kind === Track.Kind.Video) removeTile('local'); })
      .on(RoomEvent.ActiveSpeakersChanged, (speakers) => {
        const on = new Set(speakers.map((s) => s.identity));
        for (const t of tiles.querySelectorAll('.tile')) t.classList.toggle('speaking', on.has(t.dataset.id) || (t.dataset.id === 'local' && on.has(meId)));
        const me = users.get(meId), mine = on.has(meId);
        if (me && !!me.speaking !== mine) { me.speaking = mine; act({ ev: 'speech', on: mine }); }
        dirty = true;
      })
      .on(RoomEvent.AudioPlaybackStatusChanged, () => { audioBlocked = !room.canPlaybackAudio; renderBar(); })
      .on(RoomEvent.Disconnected, () => { if (lkRoom === room) leave(); });
    await room.connect(`wss://${c.token.apiHost}`, c.token.token);
    if (cluster?.token?.roomId !== c.token.roomId) { room.disconnect(); return; }   // cluster changed while connecting
    lkRoom = room; lkRoomId = c.token.roomId;
    audioBlocked = !room.canPlaybackAudio;
    room.startAudio().catch(() => {});          // works when the page has had a click or key press
  } catch (e) {
    toast = `call failed: ${e.message}`; toastUntil = performance.now() + 5000; setTimeout(() => (dirty = true), 5100);
    console.error(e);
  } finally { joining = false; renderBar(); }
}

// called by client.js with the latest `cluster` (null when we are not in a conversation)
function onCluster(c) {
  cluster = c;
  if (!c || !c.token) { if (lkRoom || lkRoomId) leave(); else renderBar(); return; }
  if (lkRoomId === c.token.roomId || joining) { renderBar(); return; }
  if (lkRoom) leave().then(() => join(c)); else join(c);
}

async function setMic(on) {
  if (!lkRoom) return;
  try { await lkRoom.localParticipant.setMicrophoneEnabled(on); micOn = on; }
  catch (e) { toast = `microphone: ${e.message}`; toastUntil = performance.now() + 5000; setTimeout(() => (dirty = true), 5100); }
  renderBar();
}
async function setCam(on) {
  if (!lkRoom) return;
  try { await lkRoom.localParticipant.setCameraEnabled(on); camOn = on; act({ ev: 'camera', on }); }
  catch (e) { toast = `camera: ${e.message}`; toastUntil = performance.now() + 5000; setTimeout(() => (dirty = true), 5100); }
  renderBar();
}
bar.snd.onclick = () => lkRoom?.startAudio().then(() => { audioBlocked = !lkRoom.canPlaybackAudio; renderBar(); });
for (const ev of ['pointerdown', 'keydown']) addEventListener(ev, () => { if (lkRoom && audioBlocked) lkRoom.startAudio().then(() => { audioBlocked = !lkRoom?.canPlaybackAudio; renderBar(); }).catch(() => {}); });
bar.mic.onclick = () => setMic(!micOn);
bar.cam.onclick = () => setCam(!camOn);
bar.away.onclick = () => { available = !available; act({ ev: 'mode', mode: available ? 'cluster' : 'none-away' }); renderBar(); };
addEventListener('keydown', (e) => {
  if (e.ctrlKey || e.metaKey || e.altKey) return;
  if (e.key.toLowerCase() === 'm') setMic(!micOn);
  else if (e.key.toLowerCase() === 'c') setCam(!camOn);
});
renderBar();
