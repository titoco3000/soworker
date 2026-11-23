package dev.soworker.app

import android.Manifest
import android.annotation.SuppressLint
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.media.AudioAttributes
import android.media.AudioDeviceCallback
import android.media.AudioDeviceInfo
import android.media.AudioManager
import android.media.MediaPlayer
import android.os.Build
import android.os.Handler
import android.os.Looper
import androidx.core.content.ContextCompat
import java.io.File

/**
 * The bridge between Kotlin and the Rust library (src/android.rs). Kotlin to Rust: the `external` functions, which
 * only report what happened. Rust to Kotlin: the other static methods, called from the Rust threads; they hop to the
 * main thread where Android needs it. Every decision stays in Rust.
 */
@SuppressLint("StaticFieldLeak")
object Native {
    /** The activity, while it exists (set before the native library starts). */
    @Volatile var activity: MainActivity? = null
    @Volatile var appContext: Context? = null
    private val main = Handler(Looper.getMainLooper())

    // ---- Kotlin to Rust

    /** WebRTC's Android context: from the main thread, where its Java classes can be found. */
    @JvmStatic external fun initContext(context: Context)
    @JvmStatic external fun onBack()
    /** 0 play/pause (toggle the mic), 1 play (unmute), 2 pause (mute), 3 leave the call. */
    @JvmStatic external fun onMediaButton(kind: Int)
    @JvmStatic external fun onScreen(locked: Boolean)
    /** Window pixels: system bars and cutout, and the keyboard's height from the bottom. */
    @JvmStatic external fun onInsets(top: Int, bottom: Int, left: Int, right: Int, ime: Int)
    @JvmStatic external fun onPermission(name: String, granted: Boolean)
    @JvmStatic external fun onLoginResult(refreshToken: String, apiKey: String, group: String, alias: String)
    @JvmStatic external fun onQr(text: String)
    /** The picked photo's bytes, empty when cancelled. */
    @JvmStatic external fun onPhoto(bytes: ByteArray)

    // ---- Rust to Kotlin

    private fun ctx(): Context? = appContext ?: activity?.applicationContext

    @JvmStatic fun startCall(title: String, mic: Boolean, camera: Boolean) {
        val c = ctx() ?: return
        main.post { CallService.start(c, title, mic, camera) }
    }

    @JvmStatic fun stopCall() {
        val c = ctx() ?: return
        main.post { CallService.stop(c) }
    }

    @JvmStatic fun setMicState(on: Boolean) {
        main.post { CallService.instance?.setMic(on) }
    }

    @JvmStatic fun moveTaskToBack() {
        main.post { activity?.moveTaskToBack(true) }
    }

    @JvmStatic fun hasPermission(name: String): Boolean {
        val c = ctx() ?: return false
        if (name == Manifest.permission.POST_NOTIFICATIONS && Build.VERSION.SDK_INT < 33) return true
        return ContextCompat.checkSelfPermission(c, name) == PackageManager.PERMISSION_GRANTED
    }

    /** Comma-separated permission names; each answer comes back through onPermission. */
    @JvmStatic fun requestPermissions(names: String) {
        main.post { activity?.askPermissions(names.split(',').filter { it.isNotEmpty() }) }
    }

    @JvmStatic fun openLogin() {
        main.post { activity?.let { it.startActivity(Intent(it, LoginActivity::class.java)) } }
    }

    @JvmStatic fun pickPhoto() {
        main.post { activity?.pickPhoto() ?: onPhoto(ByteArray(0)) }
    }

    /** Display.getRotation of the activity's screen (0 to 3), for turning camera pictures upright. */
    @JvmStatic fun displayRotation(): Int {
        val a = activity ?: return 0
        return if (Build.VERSION.SDK_INT >= 30) a.display?.rotation ?: 0 else @Suppress("DEPRECATION") a.windowManager.defaultDisplay.rotation
    }

    @JvmStatic fun scanQr() {
        // the QR scanner is drawn by Rust with the camera; nothing to open here
    }

    /** A UI sound: notification usage, so it does not duck the call. */
    @JvmStatic fun playSound(wav: ByteArray) {
        val c = ctx() ?: return
        main.post {
            try {
                val f = File(c.cacheDir, "snd-${wav.contentHashCode()}.wav")
                if (!f.exists()) f.writeBytes(wav)
                MediaPlayer().apply {
                    setAudioAttributes(AudioAttributes.Builder().setUsage(AudioAttributes.USAGE_ASSISTANCE_SONIFICATION).setContentType(AudioAttributes.CONTENT_TYPE_SONIFICATION).build())
                    setDataSource(f.path)
                    setOnCompletionListener { it.release() }
                    setOnErrorListener { mp, _, _ -> mp.release(); true }
                    prepare()
                    start()
                }
            } catch (e: Exception) {
                android.util.Log.w("soworker", "sound: $e")
            }
        }
    }

    // ---- where the call's sound goes

    /** "auto" (Bluetooth, else a wired headset, else the speaker), "speaker", "earpiece", "headset", "bluetooth". */
    @Volatile private var route = "auto"
    private var routing = false
    private val deviceCallback = object : AudioDeviceCallback() {
        override fun onAudioDevicesAdded(added: Array<out AudioDeviceInfo>) = applyRoute()
        override fun onAudioDevicesRemoved(removed: Array<out AudioDeviceInfo>) = applyRoute()
    }

    @JvmStatic fun setAudioRoute(r: String) {
        route = r
        main.post { if (routing) applyRoute() }
    }

    /** The outputs there are now, as setAudioRoute names them. */
    @JvmStatic fun audioRoutes(): String {
        val am = ctx()?.getSystemService(AudioManager::class.java) ?: return ""
        val types = am.getDevices(AudioManager.GET_DEVICES_OUTPUTS).map { it.type }.toSet()
        val out = mutableListOf("speaker")
        if (AudioDeviceInfo.TYPE_BUILTIN_EARPIECE in types) out += "earpiece"
        if (types.any { it in WIRED }) out += "headset"
        if (types.any { it in BLUETOOTH }) out += "bluetooth"
        return out.joinToString(",")
    }

    private val WIRED = setOf(AudioDeviceInfo.TYPE_WIRED_HEADSET, AudioDeviceInfo.TYPE_WIRED_HEADPHONES, AudioDeviceInfo.TYPE_USB_HEADSET)
    private val BLUETOOTH = buildSet {
        add(AudioDeviceInfo.TYPE_BLUETOOTH_SCO)
        if (Build.VERSION.SDK_INT >= 31) add(AudioDeviceInfo.TYPE_BLE_HEADSET)
    }

    /** The call started: follow the route, and re-route when a headset comes or goes. */
    fun beginRouting(c: Context) {
        if (routing) return
        routing = true
        c.getSystemService(AudioManager::class.java).registerAudioDeviceCallback(deviceCallback, main)
        applyRoute()
        main.postDelayed({ if (routing) applyRoute() }, 1500) // WebRTC switches the audio mode as the call connects
    }

    fun endRouting(c: Context) {
        if (!routing) return
        routing = false
        val am = c.getSystemService(AudioManager::class.java)
        am.unregisterAudioDeviceCallback(deviceCallback)
        if (Build.VERSION.SDK_INT >= 31) {
            am.clearCommunicationDevice()
        } else {
            @Suppress("DEPRECATION")
            am.isSpeakerphoneOn = false
            @Suppress("DEPRECATION")
            am.stopBluetoothSco()
        }
    }

    fun applyRoute() {
        val am = ctx()?.getSystemService(AudioManager::class.java) ?: return
        if (Build.VERSION.SDK_INT >= 31) {
            val devs = am.availableCommunicationDevices
            fun of(types: Set<Int>) = devs.firstOrNull { it.type in types }
            val bt = of(BLUETOOTH)
            val wired = of(WIRED)
            val speaker = of(setOf(AudioDeviceInfo.TYPE_BUILTIN_SPEAKER))
            val ear = of(setOf(AudioDeviceInfo.TYPE_BUILTIN_EARPIECE))
            val dev = when (route) {
                "speaker" -> speaker
                "earpiece" -> ear ?: speaker
                "headset" -> wired ?: bt ?: speaker
                "bluetooth" -> bt ?: wired ?: speaker
                else -> bt ?: wired ?: speaker
            }
            if (dev != null && am.communicationDevice?.id != dev.id) am.setCommunicationDevice(dev)
        } else {
            val types = am.getDevices(AudioManager.GET_DEVICES_OUTPUTS).map { it.type }.toSet()
            val hasBt = types.any { it in BLUETOOTH }
            val hasWired = types.any { it in WIRED }
            val want = when (route) {
                "auto" -> if (hasBt) "bluetooth" else if (hasWired) "headset" else "speaker"
                else -> route
            }
            @Suppress("DEPRECATION")
            when (want) {
                "bluetooth" -> { am.isSpeakerphoneOn = false; am.startBluetoothSco(); am.isBluetoothScoOn = true }
                "speaker" -> { am.stopBluetoothSco(); am.isSpeakerphoneOn = true }
                else -> { am.stopBluetoothSco(); am.isSpeakerphoneOn = false }
            }
        }
    }
}
