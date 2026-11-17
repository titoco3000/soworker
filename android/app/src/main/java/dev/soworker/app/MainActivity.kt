package dev.soworker.app

import android.Manifest
import android.app.KeyguardManager
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.content.pm.PackageManager
import android.os.Build
import android.os.Bundle
import androidx.activity.OnBackPressedCallback
import androidx.activity.result.PickVisualMediaRequest
import androidx.activity.result.contract.ActivityResultContracts
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import androidx.core.view.ViewCompat
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import com.google.androidgamesdk.GameActivity

/**
 * The window: a GameActivity whose native thread runs the Rust client (android_main in android/rust). Here only
 * what needs the activity: back, insets, the screen lock, permission dialogs, the photo picker.
 */
class MainActivity : GameActivity() {
    companion object {
        init {
            System.loadLibrary("soworker_android")
        }
    }

    private val screen = object : BroadcastReceiver() {
        override fun onReceive(c: Context, i: Intent) = reportLock()
    }

    private val photo = registerForActivityResult(ActivityResultContracts.PickVisualMedia()) { uri ->
        val bytes = uri?.let { runCatching { contentResolver.openInputStream(it)?.use { s -> s.readBytes() } }.getOrNull() }
        Native.onPhoto(bytes ?: ByteArray(0))
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        Native.activity = this
        Native.appContext = applicationContext
        Native.initContext(applicationContext)
        super.onCreate(savedInstanceState)
        // edge to edge (forced by targetSdk 36 anyway): Rust keeps its controls out of the bars with the insets
        WindowCompat.setDecorFitsSystemWindows(window, false)
        ViewCompat.setOnApplyWindowInsetsListener(window.decorView) { v, insets ->
            val bars = insets.getInsets(WindowInsetsCompat.Type.systemBars() or WindowInsetsCompat.Type.displayCutout())
            val ime = insets.getInsets(WindowInsetsCompat.Type.ime()).bottom
            Native.onInsets(bars.top, bars.bottom, bars.left, bars.right, ime)
            ViewCompat.onApplyWindowInsets(v, insets)
        }
        // always enabled: Rust decides what back closes, and sends the app to the background when nothing is open
        onBackPressedDispatcher.addCallback(this, object : OnBackPressedCallback(true) {
            override fun handleOnBackPressed() = Native.onBack()
        })
        val f = IntentFilter().apply {
            addAction(Intent.ACTION_SCREEN_OFF)
            addAction(Intent.ACTION_SCREEN_ON)
            addAction(Intent.ACTION_USER_PRESENT)
        }
        ContextCompat.registerReceiver(this, screen, f, ContextCompat.RECEIVER_NOT_EXPORTED)
    }

    override fun onResume() {
        super.onResume()
        reportLock()
        ViewCompat.requestApplyInsets(window.decorView)
    }

    override fun onDestroy() {
        runCatching { unregisterReceiver(screen) }
        if (Native.activity === this) Native.activity = null
        super.onDestroy()
    }

    private fun reportLock() {
        val km = getSystemService(KeyguardManager::class.java)
        Native.onScreen(km?.isKeyguardLocked ?: false)
    }

    fun askPermissions(names: List<String>) {
        val want = names.filter { !(it == Manifest.permission.POST_NOTIFICATIONS && Build.VERSION.SDK_INT < 33) }
        val missing = want.filter { ContextCompat.checkSelfPermission(this, it) != PackageManager.PERMISSION_GRANTED }
        for (n in want - missing.toSet()) Native.onPermission(n, true)
        if (missing.isNotEmpty()) ActivityCompat.requestPermissions(this, missing.toTypedArray(), 1)
    }

    @Deprecated("the activity result API has no plain permission callback with these semantics")
    override fun onRequestPermissionsResult(requestCode: Int, permissions: Array<out String>, grantResults: IntArray) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults)
        permissions.forEachIndexed { i, p -> Native.onPermission(p, grantResults.getOrNull(i) == PackageManager.PERMISSION_GRANTED) }
    }

    fun pickPhoto() {
        photo.launch(PickVisualMediaRequest(ActivityResultContracts.PickVisualMedia.ImageOnly))
    }
}
