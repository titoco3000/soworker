# Push to talk with the window hidden

Calls start muted. Three ways to talk without clicking the mic button (all end in `State::talk`, `talk.rs`):

| Way | Behaviour |
|---|---|
| Key in the window | the **talk key** (default `Super+'`; `talkKey` in `~/.config/soworker/settings.json`, written like `F1`, `Pause` or `Ctrl+Alt+Space`) opens the mic while held, if it was muted. Modifiers must be down when the key goes down; on release only the key counts |
| `soworker --talk [toggle\|down\|up]` | talks to the running app over `$XDG_RUNTIME_DIR/soworker/ctl.sock` (mode 600). Bind `soworker --talk` to a key in the desktop's keyboard settings (COSMIC: Settings > Keyboard > Custom shortcuts, or the one-time terminal command in `README.md`). Desktop shortcuts only report a press, so the default `toggle` flips the mic |
| GlobalShortcuts portal | registers "Push to talk" (preferred trigger = the talk key) and gets press and release events, so the mic is open while the key is held. **COSMIC has no such portal yet** ([xdg-desktop-portal-cosmic#4](https://github.com/pop-os/xdg-desktop-portal-cosmic/issues/4)); KDE and GNOME 48+ do. Untested on a real portal; without one the app logs a line and carries on. Build without it: `--no-default-features --features calls` |

**Reminder:** when the window loses focus during a meeting while the mic is muted (at most once a minute), or when a
meeting starts while the window is unfocused, a desktop notification "Press Super+' to talk" appears (`notify-send`, replaces the
previous one, expires after 5 s).

The key is only a default: the global binding is whatever you set in the desktop's shortcut settings (COSMIC: Settings >
Keyboard > Custom shortcuts, command `soworker --talk`); `talkKey` just has to name the same keys so the hint and the in-window
key agree. Avoid keys other apps need: plain F1 is Help, Ctrl+1 is a browser tab.

## "You are live" overlay

While the mic is open in a meeting and the window is not focused, a small pill ("● Mic on · 2 listening") shows in the
top-right corner of the screen, above all windows, and says "Muted" for a moment when the mic closes. A normal window
cannot do this on Wayland (no always-on-top, no placement), so `overlay.rs` uses the compositor's **wlr layer-shell** protocol
(overlay layer, top-right, 16 px margin, no keyboard focus, clicks pass through). COSMIC, sway, Hyprland and KDE have
it; GNOME does not, where the same text is sent as a short notification instead. Focusing the window hides the pill.
`soworker --overlay-test` shows it for a few seconds. Build without it: drop the `overlay` feature.
