//! Keep and export libwebrtc's JNI functions (`Java_livekit_org_webrtc_*`) from our library.
//!
//! WebRTC's Java half (libwebrtc.jar) calls them as `native` methods, and ART looks them up in the loaded libraries'
//! dynamic symbols. webrtc-sys's build script asks for this (`--undefined` per symbol plus a version script), but
//! Cargo only applies `rustc-link-arg` to the package that printed it, never to a dependent cdylib like this one, so
//! without this the linker drops them and starting a call aborts with an UnsatisfiedLinkError for
//! `SoftwareVideoEncoderFactory.nativeCreateFactory`.

use std::{env, fs, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=LK_CUSTOM_WEBRTC");
    println!("cargo:rerun-if-env-changed=ANDROID_NDK_HOME");
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("android") {
        return;
    }
    let webrtc = PathBuf::from(env::var("LK_CUSTOM_WEBRTC").expect("LK_CUSTOM_WEBRTC must point at the Android libwebrtc (use make android-lib)"));
    let lib = webrtc.join("lib/libwebrtc.a");
    println!("cargo:rerun-if-changed={}", lib.display());
    let ndk = PathBuf::from(env::var("ANDROID_NDK_HOME").expect("ANDROID_NDK_HOME must be set (use make android-lib)"));
    let nm = ndk.join("toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-nm");
    let out = Command::new(&nm)
        .args(["--defined-only", "--format=just-symbols"])
        .arg(&lib)
        .output()
        .unwrap_or_else(|e| panic!("running {}: {e}", nm.display()));
    let text = String::from_utf8_lossy(&out.stdout);
    let mut symbols: Vec<&str> = text.lines().map(str::trim).filter(|s| s.starts_with("Java_livekit_org_webrtc_")).collect();
    symbols.sort_unstable();
    symbols.dedup();
    assert!(!symbols.is_empty(), "no JNI symbols found in {}", lib.display());

    for s in &symbols {
        println!("cargo:rustc-link-arg-cdylib=-Wl,--undefined={s}");
    }
    let script = PathBuf::from(env::var("OUT_DIR").unwrap()).join("webrtc_jni.map");
    fs::write(&script, format!("JNI_WEBRTC {{\n\tglobal: {};\n}};\n", symbols.join("; "))).unwrap();
    println!("cargo:rustc-link-arg-cdylib=-Wl,--version-script={}", script.display());
}
