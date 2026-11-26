//! SoWorker as a library: the desktop binary (`main.rs`) and the Android app (`android/rust`) are both built on it.
pub mod app;
#[cfg(all(target_os = "android", feature = "gui"))]
pub mod android;
#[cfg(all(target_os = "android", feature = "calls"))]
pub mod android_camera;
#[cfg(all(target_os = "android", feature = "gui", feature = "calls"))]
pub mod android_scan;
pub mod backend;
pub mod calls;
pub mod chat;
pub mod chatstate;
pub mod desks;
pub mod desktop;
pub mod bench;
pub mod devices;
pub mod directory;
pub mod login;
pub mod mapobj;
pub mod meeting;
pub mod mode;
pub mod net;
pub mod notify;
pub mod pairing;
pub mod overlay;
pub mod profile;
pub mod proto;
pub mod roster;
pub mod session;
pub mod sound;
pub mod talk;
pub mod transcribe;
pub mod waves;
pub mod world;
#[cfg(feature = "gui")]
pub mod avatars;
#[cfg(feature = "gui")]
pub mod gui;
#[cfg(feature = "gui")]
pub mod photo;
#[cfg(feature = "calls")]
pub mod media;
#[cfg(feature = "calls")]
pub mod mic;
#[cfg(feature = "calls")]
pub mod record;
#[cfg(feature = "mc")]
pub mod mc;
#[cfg(feature = "tui")]
pub mod tui;
#[cfg(feature = "tui")]
pub mod videowin;

#[cfg(not(any(feature = "tui", feature = "gui", feature = "mc")))]
compile_error!("enable a frontend: feature `tui`, `gui` and/or `mc`");
