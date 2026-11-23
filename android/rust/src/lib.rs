//! The Android app's native library: `android_main` (started by GameActivity) and the JNI functions `Native.kt`
//! calls. Everything else lives in the soworker library (`soworker::android`, `soworker::gui`).
#![cfg(target_os = "android")]

use jni::objects::{JByteArray, JClass, JObject, JString};
use jni::sys::{jboolean, jint, JNI_VERSION_1_6};
use jni::{JNIEnv, JavaVM};
use soworker::android::{self as a, Msg};

#[no_mangle]
fn android_main(app: soworker::android::AndroidApp) {
    a::main(app);
}

#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _reserved: *mut std::ffi::c_void) -> jint {
    a::on_load(&vm);
    JNI_VERSION_1_6
}

#[no_mangle]
pub extern "system" fn Java_dev_soworker_app_Native_initContext(mut env: JNIEnv, _class: JClass, ctx: JObject) {
    a::init_context(&mut env, &ctx);
}

fn string(env: &mut JNIEnv, s: &JString) -> String {
    env.get_string(s).map(Into::into).unwrap_or_default()
}

#[no_mangle]
pub extern "system" fn Java_dev_soworker_app_Native_onBack(_env: JNIEnv, _class: JClass) {
    a::post(Msg::Back);
}

#[no_mangle]
pub extern "system" fn Java_dev_soworker_app_Native_onMediaButton(_env: JNIEnv, _class: JClass, kind: jint) {
    a::post(Msg::Media(kind));
}

#[no_mangle]
pub extern "system" fn Java_dev_soworker_app_Native_onScreen(_env: JNIEnv, _class: JClass, locked: jboolean) {
    a::post(Msg::Screen { locked: locked != 0 });
}

#[no_mangle]
pub extern "system" fn Java_dev_soworker_app_Native_onInsets(_env: JNIEnv, _class: JClass, top: jint, bottom: jint, left: jint, right: jint, ime: jint) {
    a::set_insets([top, bottom, left, right, ime].map(|v| v.max(0) as f32));
}

#[no_mangle]
pub extern "system" fn Java_dev_soworker_app_Native_onPermission(mut env: JNIEnv, _class: JClass, name: JString, granted: jboolean) {
    let name = string(&mut env, &name);
    a::post(Msg::Permission { name, granted: granted != 0 });
}

#[no_mangle]
pub extern "system" fn Java_dev_soworker_app_Native_onLoginResult(mut env: JNIEnv, _class: JClass, refresh: JString, key: JString, group: JString, alias: JString) {
    let (refresh_token, api_key) = (string(&mut env, &refresh), string(&mut env, &key));
    let (group, alias) = (string(&mut env, &group), string(&mut env, &alias));
    a::post(Msg::Login { refresh_token, api_key, group, alias });
}

#[no_mangle]
pub extern "system" fn Java_dev_soworker_app_Native_onQr(mut env: JNIEnv, _class: JClass, text: JString) {
    let text = string(&mut env, &text);
    a::post(Msg::Qr(text));
}

#[no_mangle]
pub extern "system" fn Java_dev_soworker_app_Native_onPhoto(env: JNIEnv, _class: JClass, bytes: JByteArray) {
    let data = env.convert_byte_array(&bytes).unwrap_or_default();
    a::photo_picked(data);
}
