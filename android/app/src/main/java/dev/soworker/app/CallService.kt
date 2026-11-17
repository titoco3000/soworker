package dev.soworker.app

import android.Manifest
import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.app.Service
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.content.pm.ServiceInfo
import android.media.session.MediaSession
import android.media.session.PlaybackState
import android.net.wifi.WifiManager
import android.os.Build
import android.os.IBinder
import android.os.PowerManager
import android.view.KeyEvent
import androidx.core.content.ContextCompat

/**
 * Keeps a call running with the screen locked: a foreground service (microphone type, camera too while it is on)
 * with the ongoing-call notification, a media session so play/pause mutes and unmutes, and the wake and Wi-Fi locks.
 * Started on join while the app is in front (Android 12+ requires that for a microphone service), stopped on leave.
 */
class CallService : Service() {
    companion object {
        @Volatile var instance: CallService? = null
        private const val CHANNEL = "call"
        private const val ID = 1

        fun start(c: Context, title: String, mic: Boolean, camera: Boolean) {
            val i = Intent(c, CallService::class.java).setAction("start")
                .putExtra("title", title).putExtra("mic", mic).putExtra("camera", camera)
            ContextCompat.startForegroundService(c, i)
        }

        fun stop(c: Context) {
            instance?.let {
                it.stopForeground(STOP_FOREGROUND_REMOVE)
                it.stopSelf()
            }
        }
    }

    private var title = "Meeting"
    private var mic = false
    private var camera = false
    private var session: MediaSession? = null
    private var wake: PowerManager.WakeLock? = null
    private var wifi: WifiManager.WifiLock? = null

    override fun onBind(intent: Intent?): IBinder? = null

    override fun onCreate() {
        super.onCreate()
        instance = this
        val nm = getSystemService(NotificationManager::class.java)
        nm.createNotificationChannel(NotificationChannel(CHANNEL, "Ongoing call", NotificationManager.IMPORTANCE_LOW).apply {
            description = "Shown while you are in a meeting"
            setShowBadge(false)
        })
        session = MediaSession(this, "soworker").apply {
            setCallback(object : MediaSession.Callback() {
                override fun onPlay() = Native.onMediaButton(1)
                override fun onPause() = Native.onMediaButton(2)
                override fun onStop() = Native.onMediaButton(2)
                override fun onMediaButtonEvent(i: Intent): Boolean {
                    @Suppress("DEPRECATION")
                    val k = i.getParcelableExtra<KeyEvent>(Intent.EXTRA_KEY_EVENT) ?: return super.onMediaButtonEvent(i)
                    if (k.action != KeyEvent.ACTION_DOWN || k.repeatCount > 0) return true
                    when (k.keyCode) {
                        KeyEvent.KEYCODE_HEADSETHOOK, KeyEvent.KEYCODE_MEDIA_PLAY_PAUSE -> Native.onMediaButton(0)
                        KeyEvent.KEYCODE_MEDIA_PLAY -> Native.onMediaButton(1)
                        KeyEvent.KEYCODE_MEDIA_PAUSE, KeyEvent.KEYCODE_MEDIA_STOP -> Native.onMediaButton(2)
                        else -> return super.onMediaButtonEvent(i)
                    }
                    return true
                }
            })
            isActive = true
        }
        val pm = getSystemService(PowerManager::class.java)
        wake = pm.newWakeLock(PowerManager.PARTIAL_WAKE_LOCK, "soworker:call").apply { setReferenceCounted(false); acquire() }
        val wm = applicationContext.getSystemService(WifiManager::class.java)
        @Suppress("DEPRECATION")
        val mode = if (Build.VERSION.SDK_INT >= 29) WifiManager.WIFI_MODE_FULL_LOW_LATENCY else WifiManager.WIFI_MODE_FULL_HIGH_PERF
        wifi = wm?.createWifiLock(mode, "soworker:call")?.apply { setReferenceCounted(false); acquire() }
        Native.beginRouting(this)
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        when (intent?.action) {
            "start" -> {
                title = intent.getStringExtra("title") ?: title
                mic = intent.getBooleanExtra("mic", mic)
                camera = intent.getBooleanExtra("camera", camera)
                show()
            }
            "mute" -> Native.onMediaButton(2)
            "unmute" -> Native.onMediaButton(1)
            "leave" -> Native.onMediaButton(3)
            else -> show()
        }
        return START_NOT_STICKY
    }

    /** The app was swiped away from the recents: the call ends with it. */
    override fun onTaskRemoved(rootIntent: Intent?) {
        Native.onMediaButton(3)
        stopForeground(STOP_FOREGROUND_REMOVE)
        stopSelf()
    }

    override fun onDestroy() {
        Native.endRouting(this)
        session?.run { isActive = false; release() }
        session = null
        wake?.takeIf { it.isHeld }?.release()
        wifi?.takeIf { it.isHeld }?.release()
        if (instance === this) instance = null
        super.onDestroy()
    }

    fun setMic(on: Boolean) {
        mic = on
        show()
    }

    private fun granted(p: String) = ContextCompat.checkSelfPermission(this, p) == PackageManager.PERMISSION_GRANTED

    private fun action(icon: Int, label: String, act: String, code: Int): Notification.Action {
        val pi = PendingIntent.getService(this, code, Intent(this, CallService::class.java).setAction(act), PendingIntent.FLAG_IMMUTABLE or PendingIntent.FLAG_UPDATE_CURRENT)
        return Notification.Action.Builder(android.graphics.drawable.Icon.createWithResource(this, icon), label, pi).build()
    }

    private fun show() {
        val s = session ?: return
        val state = if (mic) PlaybackState.STATE_PLAYING else PlaybackState.STATE_PAUSED
        s.setPlaybackState(PlaybackState.Builder()
            .setActions(PlaybackState.ACTION_PLAY or PlaybackState.ACTION_PAUSE or PlaybackState.ACTION_PLAY_PAUSE or PlaybackState.ACTION_STOP)
            .setState(state, PlaybackState.PLAYBACK_POSITION_UNKNOWN, 1f)
            .build())
        val open = PendingIntent.getActivity(this, 0, Intent(this, MainActivity::class.java).addFlags(Intent.FLAG_ACTIVITY_SINGLE_TOP), PendingIntent.FLAG_IMMUTABLE)
        val n = Notification.Builder(this, CHANNEL)
            .setSmallIcon(android.R.drawable.ic_btn_speak_now)
            .setContentTitle(title)
            .setContentText(if (mic) "In a meeting · microphone on" else "In a meeting · muted")
            .setContentIntent(open)
            .setOngoing(true)
            .setCategory(Notification.CATEGORY_CALL)
            .setVisibility(Notification.VISIBILITY_PUBLIC)
            .addAction(if (mic) action(android.R.drawable.ic_lock_silent_mode, "Mute", "mute", 1) else action(android.R.drawable.ic_btn_speak_now, "Unmute", "unmute", 2))
            .addAction(action(android.R.drawable.ic_menu_close_clear_cancel, "Leave", "leave", 3))
            .setStyle(Notification.MediaStyle().setMediaSession(s.sessionToken).setShowActionsInCompactView(0, 1))
            .build()
        // the microphone type needs the permission granted; listening only is playback
        var type = if (granted(Manifest.permission.RECORD_AUDIO)) ServiceInfo.FOREGROUND_SERVICE_TYPE_MICROPHONE else ServiceInfo.FOREGROUND_SERVICE_TYPE_MEDIA_PLAYBACK
        if (camera && granted(Manifest.permission.CAMERA)) type = type or ServiceInfo.FOREGROUND_SERVICE_TYPE_CAMERA
        try {
            if (Build.VERSION.SDK_INT >= 29) startForeground(ID, n, type) else startForeground(ID, n)
        } catch (e: Exception) {
            android.util.Log.w("soworker", "call service: $e")
        }
    }
}
