package dev.soworker.app

import android.annotation.SuppressLint
import android.app.Activity
import android.os.Bundle
import android.widget.FrameLayout
import android.webkit.CookieManager
import android.webkit.WebStorage
import android.webkit.WebView
import android.webkit.WebViewClient
import androidx.core.view.ViewCompat
import androidx.core.view.WindowInsetsCompat
import org.json.JSONObject

/**
 * Sign-in for accounts that work inside a WebView (email and password, non-Google SSO): SoWork's own page signs in,
 * then the Firebase refresh token is read from its IndexedDB, the same lookup as the desktop's login.rs. Google
 * refuses its sign-in in WebViews; those accounts pair from the desktop instead (a QR code, see pairing.rs).
 * The WebView's cookies and storage are wiped afterwards, like the desktop's throwaway browser profile.
 */
class LoginActivity : Activity() {
    private lateinit var web: WebView
    private var done = false

    private val capture = """
        (async () => {
          const m = location.pathname.match(/^\/s\/([^\/]+)\/([^\/?#]+)/);
          if (!m) return null;
          const db = await new Promise((ok, no) => { const r = indexedDB.open('firebaseLocalStorageDb'); r.onsuccess = () => ok(r.result); r.onerror = no; });
          const rows = await new Promise((ok, no) => { const r = db.transaction('firebaseLocalStorage').objectStore('firebaseLocalStorage').getAll(); r.onsuccess = () => ok(r.result); r.onerror = no; });
          // the default Firebase app's login (key firebase:authUser:<apiKey>:[DEFAULT]) comes first, as in login.rs
          rows.sort((a, b) => ((b.fbase_key || '').endsWith(':[DEFAULT]') ? 1 : 0) - ((a.fbase_key || '').endsWith(':[DEFAULT]') ? 1 : 0));
          for (const row of rows) {
            const v = row && row.value;
            const t = v && v.stsTokenManager && v.stsTokenManager.refreshToken;
            if (t) return JSON.stringify({ refreshToken: t, apiKey: v.apiKey || (row.fbase_key || '').split(':')[2] || '', group: m[1], alias: m[2] });
          }
          return null;
        })()
    """.trimIndent()

    @SuppressLint("SetJavaScriptEnabled")
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        web = WebView(this)
        web.settings.javaScriptEnabled = true
        web.settings.domStorageEnabled = true
        web.webViewClient = object : WebViewClient() {
            override fun onPageFinished(view: WebView, url: String) = poll()
        }
        // edge to edge (targetSdk 36): a padded frame keeps the page out from under the system bars and the keyboard
        val root = FrameLayout(this)
        root.addView(web, FrameLayout.LayoutParams(FrameLayout.LayoutParams.MATCH_PARENT, FrameLayout.LayoutParams.MATCH_PARENT))
        setContentView(root)
        ViewCompat.setOnApplyWindowInsetsListener(root) { v, insets ->
            val b = insets.getInsets(WindowInsetsCompat.Type.systemBars() or WindowInsetsCompat.Type.displayCutout() or WindowInsetsCompat.Type.ime())
            v.setPadding(b.left, b.top, b.right, b.bottom)
            WindowInsetsCompat.CONSUMED
        }
        ViewCompat.requestApplyInsets(root)
        web.loadUrl("https://app.sowork.com/login")
    }

    /** The office page loads, then Firebase writes its session: look a few times. */
    private fun poll(tries: Int = 0) {
        if (done || tries > 20) return
        if (!(web.url ?: "").contains("/s/")) return
        // evaluateJavascript does not await promises: keep the answer on window and read it back
        web.evaluateJavascript("$capture.then(v => window.__sw = v); 0", null)
        web.postDelayed({
            web.evaluateJavascript("window.__sw || null") { res ->
                val s = res?.takeIf { it != "null" }?.let { JSONObject("{\"v\":$it}").optString("v") }
                if (s.isNullOrEmpty()) {
                    poll(tries + 1)
                } else {
                    val o = JSONObject(s)
                    done = true
                    Native.onLoginResult(o.optString("refreshToken"), o.optString("apiKey"), o.optString("group"), o.optString("alias"))
                    finish()
                }
            }
        }, 700)
    }

    override fun onDestroy() {
        if (!done) Native.onLoginResult("", "", "", "") // closed without signing in
        CookieManager.getInstance().removeAllCookies(null)
        WebStorage.getInstance().deleteAllData()
        web.destroy()
        super.onDestroy()
    }
}
