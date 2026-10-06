### 0.143 GET https://app.sowork.com/s/v0XvoIt3uK40pdF0mQDB/mistymountainlodge -> 200
RES: <!DOCTYPE html>
<html lang="en" translate="no">

<head>
  <script>
    let loaded = false;
    window.onload = function () { loaded = true; }
    window.onerror = function (message, source, lineno, colno, error) {
      if (!loaded && error instanceof SyntaxError && source.endsWith('.js')) {
        const refreshed = Boolean(sessionStorage.getItem("appRefreshed"));
        if (!refreshed) {
          sessionStorage.setItem("appRefreshed", true);
          window.location.reload();
        }
      }
    }
  </script>
  <script>
      (function () { var w = window; var ic = w.Intercom; if (typeof ic === "function") { ic('reattach_activator'); ic('update', w.intercomSettings); } else { var d = document; var i = function () { i.c(arguments); }; i.q = []; i.c = function (args) { i.q.push(args); }; w.Intercom = i; var l = function () { var s = d.createElement('script'); s.type = 'text/javascript'; s.async = true; s.src = 'https://widget.intercom.io/widget/qwwvp1eu'; var x = d.getElementsByTagName('script')[0]; x.parentNode.insertBefore(s, x); }; if (w.attachEvent) { w.attachEvent('onload', l); } else { w.addEventListener('load', l, false); } } })();
  </script>
  <!-- Meta Pixel Code -->
  <script>
    !function (f, b, e, v, n, t, s) {
      if (f.fbq) return; n = f.fbq = function () {
        n.callMethod ?
          n.callMethod.apply(n, arguments) : n.queue.push(arguments)
      };
      if (!f._fbq) f._fbq = n; n.push = n; n.loaded = !0; n.version = '2.0';
      n.queue = []; t = b.createElement(e); t.async = !0;
      t.src = v; s = b.getElementsByTagName(e)[0];
      s.parentNode.insertBefore(t, s)
    }(window, document, 'script',
      'https://connect.facebook.net/en_US/fbevents.js');
  </script>
  <!-- End Meta Pixel Code -->
  <meta charset="utf-8" />
  <meta name="slack-app-id" content="A01EA3S0HCZ">
  <!-- https://api.slack.com/start/distributing/directory#suggestions__enabling-app-suggestions -->
  <meta name="format-detection" content="telephone=no" />
  <meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1.0, user-scalable=no" />
  <meta name="google-site-verification" content="pdlPLGJx3NRTl46VN1HanrNY5dnj8imog2Omxy2XRLw" />
  <meta name="google" content="notranslate" />
  <meta name="referrer" content="strict-origin-when-cross-origin" />
  <title>Our Office in SoWork</title>
  <meta name="description" content="Let's work together in our SoWork HQ">

  <!-- Facebook Meta Tags -->
  <meta property="og:url" content="">
  <meta property="og:type" content="website">
  <meta property="og:title" content="Our Office in SoWork">
  <meta property="og:description" content="Let's work together in our SoWork HQ">
  <meta property="og:image" content="//app.sowork.com/images/og_office.webp">

  <!-- Twitter Meta Tags -->
  <meta name="twitter:card" content="summary_large_image" />
  <meta property="twitter:domain" content="sophya.world" />
  <meta name="twitter:title" content="Our Office in SoWork" />
  <meta name="twitter:description" content="Let's work together in our SoWork HQ" />
  <!-- twitter:image does not support relative paths -->
  <meta name="twitter:image" content="//app.sowork.com/images/og_office.webp" />

  <link rel="apple-touch-icon" sizes="57x57" href="/apple-icon-57x57.png" />
  <link rel="apple-touch-icon" sizes="60x60" href="/apple-icon-60x60.png" />
  <link rel="apple-touch-icon" sizes="72x72" href="/apple-icon-72x72.png" />
  <link rel="apple-touch-icon" sizes="76x76" href="/apple-icon-76x76.png" />
  <link rel="apple-touch-icon" sizes="114x114" href="/apple-icon-114x114.png" />
  <link rel="apple-touch-icon" sizes="120x120" href="/apple-icon-120x120.png" />
  <link rel="apple-touch-icon" sizes="144x144" href="/apple-icon-144x144.png" />
  <link rel="apple-touch-icon" sizes="152x152" href="/apple-icon-152x152.png" />
  <link rel="apple-touch-icon" sizes="180x180" href="/apple-icon-180x180.png" />
  <link rel="icon" type="image/png" sizes="192x192" href="/android-icon-192x192.png" />
  <link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png" />
  <link rel="icon" type="image/png" sizes="96x96" href="/favicon-96x96.png" />
  <link rel='icon' href='/favicon.ico' />
  <link rel="manifest" href="/manifest.json" />
  <meta name="msapplication-TileColor" content="#ffffff" />
  <meta name="msapplication-TileImage" content="/ms-icon-144x144.png" />
  <meta name="theme-color" content="#5766b4" />
  <meta name="apple-moble-web-app-capable" content="yes" />
  <script>
    /* Theme-aware pre-paint: resolve the user's stored THEME preference (or
       the system query when stored ∉ {'light','dark'} — i.e. 'system',
       unset, or any legacy / malformed value) and stamp
       data-theme="light|dark" on <html> BEFORE the first body paint. The body
       background-color rule below keys off that attribute, so we don't flash
       a light-mode body to dark-mode users (and vice versa) while the React
       bundle and MUI's CssBaseline boot. The React-side ThemeContext writes
       the same 'THEME' localStorage key on every user change, so this stays
       in sync across sessions without any extra bookkeeping. Wrapped in
       try/catch so a hostile localStorage implementation (private mode in
       older browsers, blocked storage) can't break the boot. */
    (function () {
      try {
        var stored = window.localStorage.getItem('THEME');
        var mode = stored;
        if (mode !== 'light' && mode !== 'dark') {
          mode =
            window.matchMedia &&
            window.matchMedia('(prefers-color-scheme: dark)').matches
              ? 'dark'
              : 'light';
        }
        document.documentElement.setAttribute('data-theme', mode);
      } catch (e) {
        document.documentElement.setAttribute('data-theme', 'light');
      }
    })();
  </script>
  <style>
    /* !important beats MUI <CssBaseline>, which renders later and would
       otherwise repaint the body with palette.background.default — lea

### 0.321 GET https://app.sowork.com/manifest.json -> 200
RES: {
  "name": "SoWork",
  "short_name": "SoWork",
  "start_url": "/",
  "icons": [
    {
      "src": "/android-icon-36x36.png",
      "sizes": "36x36",
      "type": "image/png",
      "density": "0.75"
    },
    {
      "src": "/android-icon-48x48.png",
      "sizes": "48x48",
      "type": "image/png",
      "density": "1.0"
    },
    {
      "src": "/android-icon-72x72.png",
      "sizes": "72x72",
      "type": "image/png",
      "density": "1.5"
    },
    {
      "src": "/android-icon-96x96.png",
      "sizes": "96x96",
      "type": "image/png",
      "density": "2.0"
    },
    {
      "src": "/android-icon-144x144.png",
      "sizes": "144x144",
      "type": "image/png",
      "density": "3.0"
    },
    {
      "src": "/android-icon-192x192.png",
      "sizes": "192x192",
      "type": "image/png",
      "density": "4.0"
    }
  ]
}


### 2.782 GET https://api.sowork.com/eyekia/api/v1/assets-version -> 200
RES: {"assets":[{"id":"00XJqiZEwyi6d5OFqA56","v":"1hpjpl91s"},{"id":"01wOnQBcMp9mnF1StinN","v":"1hmmajsu9"},{"id":"02QOeICHQfb73TM1k2Mf","v":"1i9cbct7c"},{"id":"08twuvH2aez3KY3Se4r6","v":"1iphodciv"},{"id":"08uGTz1mIZU2tQTAwwny","v":"1j7mriidj"},{"id":"09tCQAE5cK5jcndbaOtz","v":"1gq9pko7o"},{"id":"0Bk0PjgjHb9pXup34tsq","v":"1j7mriiqo"},{"id":"0C0X2BqpDmwkrUDmO3M4","v":"1hpjplaer"},{"id":"0EMrQiQSAdusl5ErszbH","v":"1jjk37och"},{"id":"0F3UNdF2syOXNKsvPmKe","v":"1hg99kfi4"},{"id":"0Hs0WfFjVy3s3jkFk5HX","v":"1jkl06sh3"},{"id":"0Hv2YVqPK7JPjbOC6yMU","v":"1jjk37p1q"},{"id":"0L88tyzcDWXRO5tOOgYp","v":"1jjk37p87"},{"id":"0Lakiq4fYpC5FPTcpb1f","v":"1hql4ea95"},{"id":"0LuCWQXgInHMzlae1LXw","v":"1hg99kgd3"},{"id":"0MaIQKjuehhQ7GD8F2ch","v":"1jjk37ppe"},{"id":"0O0YxD5uW5gNWR0QWdpP","v":"1hr6lg96s"},{"id":"0OrpUmm5VyfneBfG4atj","v":"1hpjplc3p"},{"id":"0OusslPPD4YZnjn1bdRG","v":"1j7mrildv"},{"id":"0PlxoyY5VEWlYbW3qqHA","v":"1hpjplcij"},{"id":"0WTmcDq1QrIBs0sBpqZ6","v":"1hg16nsq9"},{"id":"0WxQuSzahYQMxx1NUpHw","v":"1io8nj6lt"},{"id":"0Xw236ULN1iJg3ltkH1Y","v":"1i4t39i75"},{"id":"0YLrZzoN6e9yp04ZLv2B","v":"1gu2db168"},{"id":"0ejRSM3tGGlpRuwnlTeX","v":"1jjk37ro9"},{"id":"0ghzveoFRBsmGZKDKBUg","v":"1hg99kibr"},{"id":"0ha10kUCuA6rm6ZvvSg7","v":"1hg99kil3"},{"id":"0iK6vtAVJiXYJH5V3GN6","v":"1hg99kiuj"},{"id":"0jtrdiOaU1LK7NAIDZhy","v":"1iphodgru"},{"id":"0kBhRLHdvhJvFef0OrGw","v":"1j7mrimuc"},{"id":"0khfDsirhgjnOQlpJJ7h","v":"1k0g64d9m"},{"id":"0lXIdpYZOqAnuS5rYhPM","v":"1glg461a3"},{"id":"0lx2CpsNY8te83zDZJhI","v":"1jjk37t5q"},{"id":"0mZICodBbwuzNT1w1cco","v":"1j7mrincb"},{"id":"0niPAnvWteAQYVC5A9zm","v":"1hg16nvgu"},{"id":"0pz5QiXRU4hzi6DSb4aH","v":"1itv6c8uo"},{"id":"0rZ3g18hV6bPgf2ipLYy","v":"1jjk37tph"},{"id":"0trRlvdyUIKg3LfSrby8","v":"1jaj8caa4"},{"id":"0u0L6pjDwrJC4szbh4Dp","v":"1icoahrsl"},{"id":"0u0zC11BC9LwGhsG3Sn2","v":"1frtvccdv"},{"id":"0uDErNZUlNK2NEru65GS","v":"1im2ehcsn"},{"id":"0v4LpQ3zFN2U4EH9EIw8","v":"1i4t39k62"},{"id":"0wnEJ9xKIhbbHj2bZibg","v":"1hpjplekv"},{"id":"0zIG6SLCAsc1ELzJtnCK","v":"1hg16o0op"},{"id":"0zLqvBmNsgPxGEGey6lQ","v":"1ibkcckut"},{"id":"113LfdBtbhg0xVgVmLeQ","v":"1hpjples8"},{"id":"114FRGQhpq3hFBOwbEJr","v":"1guvi40kr"},{"id":"11t9poblx8tgRTsJVYle","v":"1iphodiga"},{"id":"128osBDlw4X3DIvUYSmq","v":"1hpjplf50"},{"id":"12xUBE1tZGzmBlZWbCIc","v":"1iphodim8"},{"id":"13KKcw1tT6vtYmGmnDTo","v":"1ilp0bf26"},{"id":"13nrjcGbAWRywqtfB4K0","v":"1hmmak42s"},{"id":"17rXYuWSLSM3te2J2nqE","v":"1jjk3804i"},{"id":"18DhyVimc81XQl24LNof","v":"1glg462ed"},{"id":"1CVpTAHK5m6KqltanIMO","v":"1fttaqt5d"},{"id":"1EtkRThDZxuJA872PovL","v":"1hg99km6h"},{"id":"1I9TgxZqNUQbtUHUw03p","v":"1io8nj9iv"},{"id":"1JFGbWzgGvVNfSln9aPU","v":"1hp1o0c4k"},{"id":"1Ld8d07nxRkXdb8qCCh4","v":"1gu8oq7r1"},{"id":"1MAizr1VCC4v6Q7ZXc2f","v":"1jjk38133"},{"id":"1Mt3plNp3zT1FoAPUbcu","v":"1hg99kmrt"},{"id":"1ODoMAnQLRhz5h5ABoJ3","v":"1hpjplg4o"},{"id":"1QC8re1QEmoMtdXQg9nY","v":"1hpjplg97"},{"id":"1QiB9laLWZEndOYqOU6V","v":"1hg99knab"},{"id":"1SsvT9XEtzo8anxOzaWK","v":"1hofpsgkl"},{"id":"1VEQ9nWBEWbsFOF9678C","v":"1hr9l99p9"},{"id":"1VobWpOKIVK2fKWgYCNq","v":"1icoahtjr"},{"id":"1W0cx9o1GIeLOJID4Fqb","v":"1jaj8cbld"},{"id":"1WSCk2DRBUrXvyU9P0Nc","v":"1hql4eeu7"},{"id":"1Z4TeJsmmioccjbeP9fP","v":"1ibkccms2"},{"id":"1Z7a18kygGriTiwCFxsm","v":"1io8njamd"},{"id":"1bG4d7RM4NqVaNk3XpTB","v":"1h19kukt7"},{"id":"1bHzwOMbJBiJ7G1kkzBI","v":"1ge304o8n"},{"id":"1dyuFQNLSfFX31aYI7rJ","v":"1hg99koch"},{"id":"1gkib5M4a8UAVRRjnJ1A","v":"1jaj8cbuh"},{"id":"1i28selpjEYCd1HjvXUG","v":"1hpjplhbs"},{"id":"1lQ9TJ9WzgGPLeiXdDrq","v":"1iphodkl9"},{"id":"1m24G06NoTTShTu6ECZP","v":"1hpjplhj5"},{"id":"1nB2nUjru0kRWA0q2IWW","v":"1jjk383nh"},{"id":"1oJS1os1G4J1naTk3gqQ","v":"1jjk383tc"},{"id":"1rIKM93JLD2bm6gxL6AJ","v":"1jjk3843f"},{"id":"1scxwYsIPaQgOKrD1Bml","v":"1jjk38492"},{"id":"1sySw25p8WgIf1Tz2qwh","v":"1j7mrir5o"},{"id":"1t0NLTJRXUetswvT90We","v":"1hulqtr7u"},{"id":"1tOeMoyb9kQGJjTF7JN9","v":"1i4t39nrn"},{"id":"1xLq0vjJ5f1IHJ1C9HeG","v":"1glg464nu"},{"id":"1yGIh996dmwV3W4sWrV6","v":"1icoahuvg"},{"id":"20DVBx1kjWkkhrmHkW01","v":"1jaj8ccm7"},{"id":"22sZfE51esnw62FnPeUy","v":"1jajhimts"},{"id":"23S0pfltatlb2iChCaWW","v":"1j7mrirqc"},{"id":"23q112oYoLv8NrXOmgR9","v":"1hmmak64i"},{"id":"2Cbckt7yMw2GNlZL8Fhu","v":"1hg99kppf"},{"id":"2DMgVMR5o28tpxFBmzUx","v":"1i0tgt2l6"},{"id":"2EbEBbCrwF8Uwym7Z0Qq","v":"1jaj8cd41"},{"id":"2FRy6d5FPYaa9K4Df4zi","v":"1hpjpliob"},{"id":"2I7Qmc87tFyePzMQWSqy","v":"1j7mrisba"},{"id":"2J1n8EKM8iXr4FYxjGw7","v":"1iphodmcj"},{"id":"2LdZKJQ8LVgjrWzeD2Qx","v":"1jjk386pc"},{"id":"2Max0C24CH4nIPbaj0L8","v":"1frtvce8r"},{"id":"2MoIhyWNjCmaOTe7dIMf","v":"1hntjuerc"},{"id":"2OGIJBBXk9UGs8aIhjk0","v":"1iphodml3"},{"id":"2OKF2fJN75tHXVRUq9uQ","v":"1j4sj7f7d"},{"id":"2SET0ZP3aKf0gM7MhnFh","v":"1hpjplj9k"},{"id":"2Sa8R3CtCVKp3dbfpRB1","v":"1iphodmqp"},{"id":"2TD4HVsDvJWurM3xrDPN","v":"1hmmak72q"},{"id":"2VWW3jIhyhkRRAVXwHrt","v":"1jjk387t0"},{"id":"2WhhUJw1jOZVpyqGusjU","v":"1iphodn3o"},{"id":"2XP7B8CwGpdIP38LiWkU","v":"1hpjpljoc"},{"id":"2YKecfHjwAidxLcCypA9","v":"1i9u4mk9h"},{"id":"2da1DTeae0lSoAyzzDdJ","v":"1hr9l9e0f"},{"id":"2kWVIk0f4F8spe7NCwF0","v":"1hpjplk0o"},{"id":"2kaqFa0Y9WirUvhYxwQE","v":"1h5nit7er"},{"id":"2lll4c69rVnOGrHjqylJ","v":"1j4sj7gmg"},{"id":"2m0cGHqNz0aYfx863UbR","v":"1io8njdpj"},{"id":"2oQjqpy0QJaUUmjZDIyW","v":"1io8njdst"},{"id":"2qucd0YAFEj6kiJVXKLI","v":"1j4sj7h4k"},{"id":"2roE3M3ULyGPN2FXtNWL","v":"1jjk389km"},{"id":"2t6EJ2JnKnwQMTCwVZz1","v":"1hpjplkbd"},{"id":"2tGkINbf5qxtDjJNk9fN","v":"1hp1o0hn2"},{"id":"2xjbaqdDkQF8SRwCZ748","v":"1j4sj7hkt"},{"id":"2zcMOx46DO0sRwzHJYGT","v":"1i0tgt4te"},{"id":"34ym2xgRGeArHhyiSBpW","v":"1hg99ks1j"},{"id":"387zDIXuZbKwdPsSMUgP","v":"1hql4ehfo"},{"id":"39AemWJFq2s7p5mZrr1X","v":"1hpjplkrj"},{"id":"3DuszbShagv5bry0UC0Z","v":"1jjk38asp"},{"id":"3H8XO2XJz7OBWEtxtzhX","v":"1hg99ksqs"},{"id":"3IUc22yqOG58FDAy0XaJ","v":"1hpjpll6f"},{"id":"3JffWXUY7XxXTzzfbZW0","v":"1hq2c9cun"},{"id":"3Ke82jpx1XREz7FrZlSj","v":"1if0guvdf"},{"id":"3KpFrsopNFuknFtXBOdF","v":"1i3nsunii"},{"id":"3L

### 3.170 OPTIONS https://firebaseinstallations.googleapis.com/v1/projects/sage-learning-prod/installations -> 200
RES: 

### 3.170 POST https://firebaseinstallations.googleapis.com/v1/projects/sage-learning-prod/installations -> 200
REQ: {"fid":"dHOTawcZ4HT2y4QaKEhr3E","authVersion":"FIS_v2","appId":"1:69834214729:web:c3e2769cb392ca9203de98","sdkVersion":"w:0.6.4"}
RES: {
  "name": "projects/69834214729/installations/dHOTawcZ4HT2y4QaKEhr3E",
  "fid": "dHOTawcZ4HT2y4QaKEhr3E",
  "refreshToken": "3_AS3qfwLfz4tV2OCddARykp3n3A0vvVHI0w8cvRtOBRV9lyDz1IbjiLOyXvJJJ8yOctpXxFDI0UPQ3sFpk2uPbbo09IIrPFU9pLmg9-YUAuQprAM",
  "authToken": {
    "token": "<JWT>",
    "expiresIn": "604800s"
  }
}


### 3.224 GET https://app.sowork.com/characters/v5/customizer_config.json -> 200
RES: {"README":"THIS IS AN AUTOGENERATED FILE BY CI, DO NOT MODIFY","bodyTypes":{"characterA":{"slots":{"backAccessory":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"category":{"id":"backAccessories","order":0},"items":{"bday":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"disabled":false,"accessKey":"PREMIUM","publishedAt":"2024-03-08T16:08:53.946Z","variations":{"basecolor":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"single","primary":"#d5b276ff"},"disabled":true},"blue":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":164,"saturationMultiplier":1.2,"valueMultiplier":1}}},"yellow":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":1.8,"valueMultiplier":0.9}}},"purple":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-120,"saturationMultiplier":1,"valueMultiplier":0.9}}},"red":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-44,"saturationMultiplier":2,"valueMultiplier":0.7}}},"green":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":81,"saturationMultiplier":1.2,"valueMultiplier":1}}}},"baseVariation":"basecolor"},"twentyfiveballoon":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"disabled":false,"accessKey":"PREMIUM","publishedAt":"2024-12-24T11:42:38.240Z","variations":{"basecolor":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"single","primary":"#d5b276ff"},"disabled":true},"blue":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":164,"saturationMultiplier":1.2,"valueMultiplier":1}}},"black":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":0,"valueMultiplier":0.35}}},"yellow":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":1.8,"valueMultiplier":0.9}}},"brown":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-20,"saturationMultiplier":1.5,"valueMultiplier":0.55}}},"orange":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-18,"saturationMultiplier":2,"valueMultiplier":0.85}}},"purple":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-120,"saturationMultiplier":1,"valueMultiplier":0.9}}},"red":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-44,"saturationMultiplier":2,"valueMultiplier":0.7}}},"turquoise":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":119,"saturationMultiplier":1,"valueMultiplier":1}}},"grey":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":0,"valueMultiplier":1}}},"white":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":0,"valueMultiplier":1.3}}},"green":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":81,"saturationMultiplier":1.2,"valueMultiplier":1}}}},"baseVariation":"basecolor"},"chineseLuck":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"disabled":false,"accessKey":"PREMIUM","publishedAt":"2025-01-23T20:36:35.648Z","variations":{"basecolor":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"single","primary":"#d5b276ff"},"disabled":true},"yellow":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":1,"valueMultiplier":1}}}},"baseVariation":"basecolor"},"chooseyou":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"disabled":false,"accessKey":"PREMIUM","publishedAt":"2025-02-07T16:24:23.811Z","variations":{"basecolor":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"single","primary":"#d5b276ff"},"disabled":true},"blue":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":1,"valueMultiplier":1}}}},"baseVariation":"basecolor"},"guitar":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"disabled":false,"publishedAt":"2021-11-08T23:00:00.000Z","variations":{"brown":{"disabled":false,"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-20,"saturationMultiplier":1.5,"valueMultiplier":0.55}}},"basecolor":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"single","primary":"#d5b276ff"},"disabled":true},"blue":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":164,"saturationMultiplier":1.2,"valueMultiplier":1}}},"black":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":0,"valueMultiplier":0.35}}},"yellow":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":1.8,"valueMultiplier":0.9}}},"orange":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-18,"saturationMultiplier":2,"valueMultiplier":0.85}}},"purple":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-120,"saturationMultiplier":1,"valueMultiplier":0.9}}},"red":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-44,"saturationMultiplier":2,"valueMultiplier":0.7}}},"turquoise":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transf

### 3.224 GET https://app.sowork.com/characters/v5/character_art_database.json -> 200
RES: {"README":"THIS IS A FILE THAT'S AUTOGENERATED BY THE CI / CD SYSTEM. DO NOT TOUCH!","lastUpdatedCommit":"79530061912db310dc167b166caaa5de72f22e29","assets":{"assistants_body_belfort+basecolor":{"preview":{"ver":1,"type":"atlas"},"layers":{"body":{"idle":{"ver":1,"type":"atlas"},"jump1":{"ver":1,"type":"atlas"},"sit":{"ver":1,"type":"atlas"},"walk":{"ver":1,"type":"atlas"},"work1":{"ver":1,"type":"atlas"},"work2":{"ver":1,"type":"atlas"},"work3":{"ver":1,"type":"atlas"},"work4":{"ver":1,"type":"atlas"},"work5":{"ver":1,"type":"atlas"},"work6":{"ver":1,"type":"atlas"},"work7":{"ver":1,"type":"atlas"},"working":{"ver":1,"type":"atlas"}}}},"assistants_body_gf+basecolor":{"preview":{"ver":1,"type":"atlas"},"layers":{"body":{"idle":{"ver":1,"type":"atlas"},"jump1":{"ver":1,"type":"atlas"},"sit":{"ver":1,"type":"atlas"},"walk":{"ver":1,"type":"atlas"},"work1":{"ver":1,"type":"atlas"},"work2":{"ver":1,"type":"atlas"},"work3":{"ver":1,"type":"atlas"},"work4":{"ver":1,"type":"atlas"},"work5":{"ver":1,"type":"atlas"},"work6":{"ver":1,"type":"atlas"},"work7":{"ver":1,"type":"atlas"},"working":{"ver":1,"type":"atlas"}}}},"assistants_body_mross+basecolor":{"preview":{"ver":1,"type":"atlas"},"layers":{"body":{"idle":{"ver":2,"type":"atlas"},"jump1":{"ver":1,"type":"atlas"},"sit":{"ver":1,"type":"atlas"},"walk":{"ver":1,"type":"atlas"},"work1":{"ver":1,"type":"atlas"},"work2":{"ver":2,"type":"atlas"},"work3":{"ver":2,"type":"atlas"},"work4":{"ver":2,"type":"atlas"},"work5":{"ver":1,"type":"atlas"},"work6":{"ver":1,"type":"atlas"},"work7":{"ver":1,"type":"atlas"},"working":{"ver":2,"type":"atlas"}}}},"cats_body_catTypeA+typeA":{"preview":{"ver":0,"type":"atlas"},"layers":{"body":{"idle":{"ver":2,"type":"atlas"},"sit":{"ver":2,"type":"atlas"},"walk":{"ver":2,"type":"atlas"}}}},"cats_body_catTypeA+typeB":{"preview":{"ver":0,"type":"atlas"},"layers":{"body":{"idle":{"ver":2,"type":"atlas"},"sit":{"ver":2,"type":"atlas"},"walk":{"ver":2,"type":"atlas"}}}},"characterA_avatarExtra_avatarExtra+basecolor":{"preview":{"ver":0,"type":"atlas"},"layers":{"avatarExtra":{"angryEmojiMeeting":{"ver":1,"type":"atlas"},"beerEmojiMeeting":{"ver":1,"type":"atlas"},"bowheEmojiMeeting":{"ver":1,"type":"atlas"},"bowsheEmojiMeeting":{"ver":1,"type":"atlas"},"cashEmojiMeeting":{"ver":1,"type":"atlas"},"champagneEmojiMeeting":{"ver":1,"type":"atlas"},"clapEmojiMeeting":{"ver":1,"type":"atlas"},"clockEmojiMeeting":{"ver":1,"type":"atlas"},"cocktailEmojiMeeting":{"ver":1,"type":"atlas"},"crazyEmojiMeeting":{"ver":1,"type":"atlas"},"cryEmojiMeeting":{"ver":1,"type":"atlas"},"eggplantEmojiMeeting":{"ver":1,"type":"atlas"},"fightEmojiMeeting":{"ver":1,"type":"atlas"},"fireEmojiMeeting":{"ver":1,"type":"atlas"},"flowersEmojiMeeting":{"ver":1,"type":"atlas"},"frozenEmojiMeeting":{"ver":1,"type":"atlas"},"glassEmojiMeeting":{"ver":1,"type":"atlas"},"handEmojiMeeting":{"ver":1,"type":"atlas"},"happyEmojiMeeting":{"ver":1,"type":"atlas"},"hilariousEmojiMeeting":{"ver":1,"type":"atlas"},"hugEmojiMeeting":{"ver":1,"type":"atlas"},"inloveEmojiMeeting":{"ver":1,"type":"atlas"},"kissEmojiMeeting":{"ver":1,"type":"atlas"},"likeEmojiMeeting":{"ver":1,"type":"atlas"},"lolEmojiMeeting":{"ver":1,"type":"atlas"},"lollypopEmojiMeeting":{"ver":1,"type":"atlas"},"loveEmojiMeeting":{"ver":1,"type":"atlas"},"normalEmojiMeeting":{"ver":1,"type":"atlas"},"oleEmojiMeeting":{"ver":1,"type":"atlas"},"partyEmojiMeeting":{"ver":1,"type":"atlas"},"peaceEmojiMeeting":{"ver":1,"type":"atlas"},"prayEmojiMeeting":{"ver":1,"type":"atlas"},"rainbowEmojiMeeting":{"ver":1,"type":"atlas"},"rocketEmojiMeeting":{"ver":1,"type":"atlas"},"sadEmojiMeeting":{"ver":1,"type":"atlas"},"seedEmojiMeeting":{"ver":1,"type":"atlas"},"silenceEmojiMeeting":{"ver":1,"type":"atlas"},"sleepEmojiMeeting":{"ver":1,"type":"atlas"},"snowEmojiMeeting":{"ver":1,"type":"atlas"},"sockyEmojiMeeting":{"ver":1,"type":"atlas"},"teethEmojiMeeting":{"ver":1,"type":"atlas"},"thinkEmojiMeeting":{"ver":1,"type":"atlas"},"valentinEmojiMeeting":{"ver":1,"type":"atlas"},"winkEmojiMeeting":{"ver":1,"type":"atlas"}}}},"characterA_avatarExtra_avatarExtra+guest":{"preview":{"ver":0,"type":"atlas"},"layers":{"avatarExtra":{"angryEmojiMeeting":{"ver":1,"type":"atlas"},"beerEmojiMeeting":{"ver":1,"type":"atlas"},"bowheEmojiMeeting":{"ver":1,"type":"atlas"},"bowsheEmojiMeeting":{"ver":1,"type":"atlas"},"cashEmojiMeeting":{"ver":2,"type":"atlas"},"champagneEmojiMeeting":{"ver":1,"type":"atlas"},"clapEmojiMeeting":{"ver":1,"type":"atlas"},"clockEmojiMeeting":{"ver":1,"type":"atlas"},"cocktailEmojiMeeting":{"ver":1,"type":"atlas"},"crazyEmojiMeeting":{"ver":1,"type":"atlas"},"cryEmojiMeeting":{"ver":1,"type":"atlas"},"eggplantEmojiMeeting":{"ver":1,"type":"atlas"},"fightEmojiMeeting":{"ver":1,"type":"atlas"},"fireEmojiMeeting":{"ver":1,"type":"atlas"},"flowersEmojiMeeting":{"ver":1,"type":"atlas"},"frozenEmojiMeeting":{"ver":1,"type":"atlas"},"glassEmojiMeeting":{"ver":1,"type":"atlas"},"handEmojiMeeting":{"ver":1,"type":"atlas"},"happyEmojiMeeting":{"ver":1,"type":"atlas"},"hilariousEmojiMeeting":{"ver":1,"type":"atlas"},"hugEmojiMeeting":{"ver":1,"type":"atlas"},"inloveEmojiMeeting":{"ver":1,"type":"atlas"},"kissEmojiMeeting":{"ver":1,"type":"atlas"},"likeEmojiMeeting":{"ver":1,"type":"atlas"},"lolEmojiMeeting":{"ver":1,"type":"atlas"},"lollypopEmojiMeeting":{"ver":1,"type":"atlas"},"loveEmojiMeeting":{"ver":1,"type":"atlas"},"negationEmojiMeeting":{"ver":1,"type":"atlas"},"nodEmojiMeeting":{"ver":1,"type":"atlas"},"normalEmojiMeeting":{"ver":1,"type":"atlas"},"oleEmojiMeeting":{"ver":1,"type":"atlas"},"partyEmojiMeeting":{"ver":1,"type":"atlas"},"peaceEmojiMeeting":{"ver":1,"type":"atlas"},"prayEmojiMeeting":{"ver":1,"type":"atlas"},"rainbowEmojiMeeting":{"ver":1,"type":"atlas"},"rocketEmojiMeeting":{"ver":1,"type":"atlas"},"sadEmojiMeeting":{"ver":1,"type":"atlas"},"seedEmojiMeeting":{"ver":1,"type":"atlas"},"silenceEmojiMeeting":{"ver":1,"type":"atlas"},"sleepEmojiMeeting":{"ver":1,"type":"atlas"},"snowEmojiMeeting

### 3.224 GET https://app.sowork.com/characters/v5/customizer_product_prices.json -> 200
RES: {"README":"THIS IS AN AUTOGENERATED FILE BY CI, DO NOT MODIFY","products":{"wings":{"name":{"en":"Wings"},"description":{"en":"TODO"},"currency":"usd","unitAmount":149},"moon":{"name":{"en":"The Deadline Moon"},"description":{"en":"TODO"},"currency":"usd","unitAmount":149},"vgicon":{"name":{"en":"Press start"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"grank":{"name":{"en":"Into the Technodrome"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"emojiMask":{"name":{"en":"Emoji Mask"},"description":{"en":"TODO"},"currency":"usd","unitAmount":99},"emojiMask2":{"name":{"en":"Emoji Mask set 2"},"description":{"en":"TODO"},"currency":"usd","unitAmount":99},"teamDog":{"name":{"en":"Dog mask"},"description":{"en":"TODO"},"currency":"usd","unitAmount":149},"teamCat":{"name":{"en":"Cat mask"},"description":{"en":"TODO"},"currency":"usd","unitAmount":149},"brazSoccer":{"name":{"en":"La Canarinha"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"spaSoccer":{"name":{"en":"La Roja"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"itaSoccer":{"name":{"en":"Gli Azzurri"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"argenSoccer":{"name":{"en":"La Albiceleste"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"gerSoccer":{"name":{"en":"Die Mannschaft"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"brazilSoccer":{"name":{"en":"Soccer Outfit"},"description":{"en":"TODO"},"currency":"usd","unitAmount":999},"corinthians":{"name":{"en":"corinthians"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"cruzeiro":{"name":{"en":"cruzeiro"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"flamingo":{"name":{"en":"flamingo"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"gremio":{"name":{"en":"gremio"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"palmeiras":{"name":{"en":"palmeiras"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"leonal":{"name":{"en":"Katana Turtle"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"donal":{"name":{"en":"Bo Staff Turtle"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"mikelal":{"name":{"en":"Nunchaku Turtle"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"rafal":{"name":{"en":"Nunchaku Turtle"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"dancingBanana":{"name":{"en":"Peanut Butter Jelly Time!"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"lsn":{"name":{"en":"Sugar, spice and everything nice"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"chocob":{"name":{"en":"Kue!!!"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"ashGb":{"name":{"en":"Become a Master"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"pachiko":{"name":{"en":"I choose you"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"saidack":{"name":{"en":"I choose you"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"charman":{"name":{"en":"I choose you"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"squilo":{"name":{"en":"I choose you"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"bilbosur":{"name":{"en":"I choose you"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"mecha":{"name":{"en":"One with the machine"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"sailorS":{"name":{"en":"In the name of the moon"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"afromonster":{"name":{"en":"afromonster"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"mistery":{"name":{"en":"FX specialist"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"octoOct":{"name":{"en":"Dr LongLegs"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"sopiper":{"name":{"en":"sopiper"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"vespa":{"name":{"en":"Vespa"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"fone":{"name":{"en":"Ka-Chow"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"motogp":{"name":{"en":"Pole!"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"sk8ter":{"name":{"en":"Tony Pigeon Amateur Skater"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"zombieDlc":{"name":{"en":"Braaaaaiiiinssss"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"futuHead":{"name":{"en":"Head in Jar!"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"spidman":{"name":{"en":"Your friendly teammate "},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"lowclo":{"name":{"en":"ex-SOLDIER First Class"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"lowsep":{"name":{"en":"One-Winged Angel"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"bugbuster":{"name":{"en":"Who ya gonna code?"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"sadako":{"name":{"en":"Seven days..."},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"colormap":{"name":{"en":"The prey"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"flyingWiz":{"name":{"en":"Magic sports"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"hoverskater":{"name":{"en":"Marty McHover"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"skaterPack":{"name":{"en":"M&T Skater Pack"},"description":{"en":"TODO"},"currency":"usd","unitAmount":399},"futuShip":{"name":{"en":"Happy 2999"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"dancingDragon":{"name":{"en":"Dancing Dragon"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"lamia":{"name":{"en":"Sliding body"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"tank":{"name":{"en":"Like a Tank"},"description":{"en":"TODO"},"cu

### 3.260 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=96293&CVER=22&X-HTTP-Session-Id=gsessionid&zx=oa1hj0au8xsm&t=1 -> 200
REQ: headers=X-Goog-Api-Client%3Agl-js%2F%20fire%2F9.23.0%0D%0AContent-Type%3Atext%2Fplain%0D%0AX-Firebase-GMPID%3A1%3A69834214729%3Aweb%3Ac3e2769cb392ca9203de98%0D%0A&count=1&ofs=0&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2FgroupPublic%2Fv0XvoIt3uK40pdF0mQDB%22%5D%7D%2C%22targetId%22%3A2%7D%7D
RES: 51
[[0,["c","Y4POaW6z8X_DYiCXA1_Edw","",8,15,30000]]]


### 3.614 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=1lRvV3lIPH3c3aCDvk0UGP8vtFjf5Dah4DZlS8lOpZz-3M9iZn_bFw&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=Y4POaW6z8X_DYiCXA1_Edw&AID=0&CI=0&TYPE=xmlhttp&zx=li8br13mcb6l&t=1 -> 200
RES: 

### 3.850 POST https://firebaseremoteconfig.googleapis.com/v1/projects/sage-learning-prod/namespaces/firebase:fetch?key=<REDACTED> -> 200
REQ: {"sdk_version":"9.23.0","app_instance_id":"dHOTawcZ4HT2y4QaKEhr3E","app_instance_id_token":"<JWT>","app_id":"1:69834214729:web:c3e2769cb392ca9203de98","language_code":"en-US"}
RES: {
  "entries": {
    "feat_0_fps": "all",
    "feat_action_items_to_zapier": "all",
    "feat_active_app_analytics": "all",
    "feat_active_app_presence": "all",
    "feat_affiliate_banner": "all",
    "feat_affiliate_creators": "all",
    "feat_affiliate_notifications": "all",
    "feat_affiliate_program": "all",
    "feat_ai_bot_animation_dynamic": "group_feature",
    "feat_analytics": "all",
    "feat_analytics_meeting_lists": "all",
    "feat_api_release_banner": "none",
    "feat_apps_framework": "group_feature",
    "feat_apps_framework_custom": "group_feature",
    "feat_apps_framework_promo_banner": "none",
    "feat_apps_framework_sowork_hosted": "group_feature",
    "feat_ask_to_mute": "all",
    "feat_asset_download_indicator": "all",
    "feat_asset_quality_setting": "all",
    "feat_asset_throttling": "all",
    "feat_audio_only": "all",
    "feat_auto_sit": "all",
    "feat_availability": "all",
    "feat_awards": "all",
    "feat_away_on_idle": "none",
    "feat_away_on_system_idle": "all",
    "feat_belfort": "all",
    "feat_bevy": "none",
    "feat_billing_plans": "all",
    "feat_block_on_away": "all",
    "feat_blur_background": "all",
    "feat_browser_extension": "all",
    "feat_calendar_details": "all",
    "feat_calendar_link_with_write": "all",
    "feat_calendar_upgrade": "all",
    "feat_cap_fps_workaround": "none",
    "feat_chat_channel_rewatch": "all",
    "feat_chat_group_filter_by_id": "all",
    "feat_chat_meeting_rewatch": "all",
    "feat_chat_message_links": "all",
    "feat_chat_message_links_click_throttling": "none",
    "feat_chat_notifications_prompt": "all",
    "feat_chat_pinned_channels": "all",
    "feat_chat_pinned_messages": "all",
    "feat_chat_read_receipt": "all",
    "feat_chat_save_for_later": "all",
    "feat_chat_search_channels": "all",
    "feat_chat_search_messages": "all",
    "feat_chat_search_messages_v2": "all",
    "feat_chat_slash_commands": "all",
    "feat_chat_threads": "all",
    "feat_chat_v2_tab": "all",
    "feat_chatbot": "only_sophyan",
    "feat_chime_downlink_policy": "none",
    "feat_chime_metrics": "only_sophyan",
    "feat_chime_screenshare_priority": "group_feature",
    "feat_chime_simulcast": "none",
    "feat_chime_svc": "group_feature",
    "feat_commands_upgrade_tooltip": "all",
    "feat_community_maps_submit": "all",
    "feat_confine_camera": "all",
    "feat_connecting_tiles": "all",
    "feat_connection_test": "all",
    "feat_create_event_button": "all",
    "feat_crm_chat_byline": "only_sophyan",
    "feat_crm_comments": "all",
    "feat_crm_tracking": "all",
    "feat_crm_update_contact_date": "all",
    "feat_deeplink_desktop": "all",
    "feat_desks": "all",
    "feat_dial_in": "all",
    "feat_digest_combined": "all",
    "feat_digest_search": "all",
    "feat_digest_templates": "all",
    "feat_disconnect_on_suspend": "all",
    "feat_dlc_assets": "all",
    "feat_door_knock": "all",
    "feat_downgrade_fps_on_meeting": "all",
    "feat_downgrade_fps_when_inactive": "all",
    "feat_dynamic_framerate": "all",
    "feat_electron_auto_start": "all",
    "feat_electron_gate_av_check": "all",
    "feat_employee_analytics": "all",
    "feat_enable_3ds_support": "all",
    "feat_end_for_all_admin_only": "all",
    "feat_event_flow": "all",
    "feat_external_meetings": "all",
    "feat_fleet_section": "only_sophyan",
    "feat_floors_and_walls": "all",
    "feat_follow_person": "all",
    "feat_freeze_subscription": "all",
    "feat_ftux_corner_card": "all",
    "feat_ftux_new_checklists": "all",
    "feat_ftux_v1": "all",
    "feat_full_suspend": "all",
    "feat_gallery_mode": "all",
    "feat_gamepad": "all",
    "feat_gesture_panning": "all",
    "feat_ghostbuster": "all",
    "feat_ghostbuster_reconciliation": "all",
    "feat_gong_interaction": "all",
    "feat_google_drive_integration": "all",
    "feat_guest_always_focus_mode": "none",
    "feat_guest_confine": "all",
    "feat_guest_entry": "all",
    "feat_guest_focus_mode": "all",
    "feat_guest_force_knock": "all",
    "feat_halloween_2025_banner": "none",
    "feat_heart_projectile": "none",
    "feat_hide_guest_office_invite": "none",
    "feat_home_dashboard_expand": "all",
    "feat_hoverable_zones": "all",
    "feat_idle_desktop": "none",
    "feat_idle_on_screen_lock": "none",
    "feat_image_background": "all",
    "feat_in_a_meeting": "all",
    "feat_in_app_feedback": "all",
    "feat_input_video_quality_360p": "all",
    "feat_invite_system": "all",
    "feat_jump": "all",
    "feat_lite_mode": "all",
    "feat_lite_mode_toggle": "all",
    "feat_livekit_logs": "group_feature",
    "feat_local_notifications": "all",
    "feat_low_disk_space_banner": "all",
    "feat_manage_auth_page": "all",
    "feat_map_filter_tags": "all",
    "feat_map_objects_integrity": "all",
    "feat_map_panning": "all",
    "feat_map_performance_indicator": "all",
    "feat_mark_notifications": "all",
    "feat_meeting_new_video_tile": "all",
    "feat_meeting_recording": "all",
    "feat_meeting_tile_sorting": "all",
    "feat_meeting_transcription": "all",
    "feat_meeting_zone_lock": "only_sophyan",
    "feat_meeting_zones": "all",
    "feat_message_board": "only_sophyan",
    "feat_microphone_analyzer": "all",
    "feat_migrate_calendar_events": "all",
    "feat_miniMode": "only_sophyan",
    "feat_minimap": "all",
    "feat_missed_connection_notifications": "all",
    "feat_mobile_beta_prompt": "only_sophyan",
    "feat_mobile_calendar_team_blocks": "all",
    "feat_mobile_calendar_timeline": "all",
    "feat_mobile_chat_mentions": "user_feature",
    "feat_mobile_chat_push_settings": "all",
    "feat_mobile_chat_split_pane": "all",
    "feat_mobile_chat_summary": "all",
    "feat_mobile_chat_thread_list": "only_sophyan",
    "feat_mobile_connectivity_checklist": "only_sophyan",
    "feat_mobile_crm_activity_chart": "all",
    "feat_mobile_crm_feature_adoption": "only_sophyan",
    "feat_mobile_floors_view": "all",
    "feat_mobile_guests":

### 3.850 OPTIONS https://firebaseremoteconfig.googleapis.com/v1/projects/sage-learning-prod/namespaces/firebase:fetch?key=<REDACTED> -> 200
RES: 

### 4.323 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=1lRvV3lIPH3c3aCDvk0UGP8vtFjf5Dah4DZlS8lOpZz-3M9iZn_bFw&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=Y4POaW6z8X_DYiCXA1_Edw&AID=0&CI=1&TYPE=xmlhttp&zx=vlgende7f61w&t=1 -> 200
RES: 1205
[[1,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      2
    ]
  }
}
]],[2,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/groupPublic/v0XvoIt3uK40pdF0mQDB",
      "fields": {
        "allowGuest": {
          "booleanValue": true
        },
        "name": {
          "stringValue": "20DASH"
        },
        "disableAnonOwner": {
          "booleanValue": true
        },
        "authLogoUrl": {
          "stringValue": "https://sowork-space.s3.us-east-2.amazonaws.com/v0XvoIt3uK40pdF0mQDB/world_logo/hGSo1hNMUkfKCo0jx6qoU?v=1786124943496"
        },
        "allowAnonGuest": {
          "booleanValue": false
        }
      },
      "createTime": "2026-08-07T17:29:21.810933Z",
      "updateTime": "2026-08-07T17:49:04.781068Z"
    },
    "targetIds": [
      2
    ]
  }
}
]],[3,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      2
    ],
    "resumeToken": "CgkIvN3q27allwM=",
    "readTime": "2026-10-06T12:48:48.443068Z"
  }
}
]],[4,[{
  "targetChange": {
    "resumeToken": "CgkIvN3q27allwM=",
    "readTime": "2026-10-06T12:48:48.443068Z"
  }
}
]],[5,["noop"]]]

### 4.466 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=1lRvV3lIPH3c3aCDvk0UGP8vtFjf5Dah4DZlS8lOpZz-3M9iZn_bFw&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=Y4POaW6z8X_DYiCXA1_Edw&AID=5&CI=1&TYPE=xmlhttp&zx=6iq4blst9buu&t=1 -> 200
RES: 

### 4.471 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=1lRvV3lIPH3c3aCDvk0UGP8vtFjf5Dah4DZlS8lOpZz-3M9iZn_bFw&SID=Y4POaW6z8X_DYiCXA1_Edw&RID=96294&AID=5&zx=nvo408v8uys5&t=1 -> 200
REQ: count=1&ofs=1&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A2%7D
RES: 8
[1,6,7]


### 4.617 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=1lRvV3lIPH3c3aCDvk0UGP8vtFjf5Dah4DZlS8lOpZz-3M9iZn_bFw&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=Y4POaW6z8X_DYiCXA1_Edw&AID=7&CI=1&TYPE=xmlhttp&zx=4ltd3wxxp3dc&t=1 -> 200
RES: 

### 25.758 GET https://app.sowork.com/login?returnTo=%2Fs%2Fv0XvoIt3uK40pdF0mQDB%2Fmistymountainlodge -> 200
RES: <!DOCTYPE html>
<html lang="en" translate="no">

<head>
  <script>
    let loaded = false;
    window.onload = function () { loaded = true; }
    window.onerror = function (message, source, lineno, colno, error) {
      if (!loaded && error instanceof SyntaxError && source.endsWith('.js')) {
        const refreshed = Boolean(sessionStorage.getItem("appRefreshed"));
        if (!refreshed) {
          sessionStorage.setItem("appRefreshed", true);
          window.location.reload();
        }
      }
    }
  </script>
  <script>
      (function () { var w = window; var ic = w.Intercom; if (typeof ic === "function") { ic('reattach_activator'); ic('update', w.intercomSettings); } else { var d = document; var i = function () { i.c(arguments); }; i.q = []; i.c = function (args) { i.q.push(args); }; w.Intercom = i; var l = function () { var s = d.createElement('script'); s.type = 'text/javascript'; s.async = true; s.src = 'https://widget.intercom.io/widget/qwwvp1eu'; var x = d.getElementsByTagName('script')[0]; x.parentNode.insertBefore(s, x); }; if (w.attachEvent) { w.attachEvent('onload', l); } else { w.addEventListener('load', l, false); } } })();
  </script>
  <!-- Meta Pixel Code -->
  <script>
    !function (f, b, e, v, n, t, s) {
      if (f.fbq) return; n = f.fbq = function () {
        n.callMethod ?
          n.callMethod.apply(n, arguments) : n.queue.push(arguments)
      };
      if (!f._fbq) f._fbq = n; n.push = n; n.loaded = !0; n.version = '2.0';
      n.queue = []; t = b.createElement(e); t.async = !0;
      t.src = v; s = b.getElementsByTagName(e)[0];
      s.parentNode.insertBefore(t, s)
    }(window, document, 'script',
      'https://connect.facebook.net/en_US/fbevents.js');
  </script>
  <!-- End Meta Pixel Code -->
  <meta charset="utf-8" />
  <meta name="slack-app-id" content="A01EA3S0HCZ">
  <!-- https://api.slack.com/start/distributing/directory#suggestions__enabling-app-suggestions -->
  <meta name="format-detection" content="telephone=no" />
  <meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1.0, user-scalable=no" />
  <meta name="google-site-verification" content="pdlPLGJx3NRTl46VN1HanrNY5dnj8imog2Omxy2XRLw" />
  <meta name="google" content="notranslate" />
  <meta name="referrer" content="strict-origin-when-cross-origin" />
  <title>Our Office in SoWork</title>
  <meta name="description" content="Let's work together in our SoWork HQ">

  <!-- Facebook Meta Tags -->
  <meta property="og:url" content="">
  <meta property="og:type" content="website">
  <meta property="og:title" content="Our Office in SoWork">
  <meta property="og:description" content="Let's work together in our SoWork HQ">
  <meta property="og:image" content="//app.sowork.com/images/og_office.webp">

  <!-- Twitter Meta Tags -->
  <meta name="twitter:card" content="summary_large_image" />
  <meta property="twitter:domain" content="sophya.world" />
  <meta name="twitter:title" content="Our Office in SoWork" />
  <meta name="twitter:description" content="Let's work together in our SoWork HQ" />
  <!-- twitter:image does not support relative paths -->
  <meta name="twitter:image" content="//app.sowork.com/images/og_office.webp" />

  <link rel="apple-touch-icon" sizes="57x57" href="/apple-icon-57x57.png" />
  <link rel="apple-touch-icon" sizes="60x60" href="/apple-icon-60x60.png" />
  <link rel="apple-touch-icon" sizes="72x72" href="/apple-icon-72x72.png" />
  <link rel="apple-touch-icon" sizes="76x76" href="/apple-icon-76x76.png" />
  <link rel="apple-touch-icon" sizes="114x114" href="/apple-icon-114x114.png" />
  <link rel="apple-touch-icon" sizes="120x120" href="/apple-icon-120x120.png" />
  <link rel="apple-touch-icon" sizes="144x144" href="/apple-icon-144x144.png" />
  <link rel="apple-touch-icon" sizes="152x152" href="/apple-icon-152x152.png" />
  <link rel="apple-touch-icon" sizes="180x180" href="/apple-icon-180x180.png" />
  <link rel="icon" type="image/png" sizes="192x192" href="/android-icon-192x192.png" />
  <link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png" />
  <link rel="icon" type="image/png" sizes="96x96" href="/favicon-96x96.png" />
  <link rel='icon' href='/favicon.ico' />
  <link rel="manifest" href="/manifest.json" />
  <meta name="msapplication-TileColor" content="#ffffff" />
  <meta name="msapplication-TileImage" content="/ms-icon-144x144.png" />
  <meta name="theme-color" content="#5766b4" />
  <meta name="apple-moble-web-app-capable" content="yes" />
  <script>
    /* Theme-aware pre-paint: resolve the user's stored THEME preference (or
       the system query when stored ∉ {'light','dark'} — i.e. 'system',
       unset, or any legacy / malformed value) and stamp
       data-theme="light|dark" on <html> BEFORE the first body paint. The body
       background-color rule below keys off that attribute, so we don't flash
       a light-mode body to dark-mode users (and vice versa) while the React
       bundle and MUI's CssBaseline boot. The React-side ThemeContext writes
       the same 'THEME' localStorage key on every user change, so this stays
       in sync across sessions without any extra bookkeeping. Wrapped in
       try/catch so a hostile localStorage implementation (private mode in
       older browsers, blocked storage) can't break the boot. */
    (function () {
      try {
        var stored = window.localStorage.getItem('THEME');
        var mode = stored;
        if (mode !== 'light' && mode !== 'dark') {
          mode =
            window.matchMedia &&
            window.matchMedia('(prefers-color-scheme: dark)').matches
              ? 'dark'
              : 'light';
        }
        document.documentElement.setAttribute('data-theme', mode);
      } catch (e) {
        document.documentElement.setAttribute('data-theme', 'light');
      }
    })();
  </script>
  <style>
    /* !important beats MUI <CssBaseline>, which renders later and would
       otherwise repaint the body with palette.background.default — lea

### 25.845 GET https://app.sowork.com/manifest.json -> 200
RES: {
  "name": "SoWork",
  "short_name": "SoWork",
  "start_url": "/",
  "icons": [
    {
      "src": "/android-icon-36x36.png",
      "sizes": "36x36",
      "type": "image/png",
      "density": "0.75"
    },
    {
      "src": "/android-icon-48x48.png",
      "sizes": "48x48",
      "type": "image/png",
      "density": "1.0"
    },
    {
      "src": "/android-icon-72x72.png",
      "sizes": "72x72",
      "type": "image/png",
      "density": "1.5"
    },
    {
      "src": "/android-icon-96x96.png",
      "sizes": "96x96",
      "type": "image/png",
      "density": "2.0"
    },
    {
      "src": "/android-icon-144x144.png",
      "sizes": "144x144",
      "type": "image/png",
      "density": "3.0"
    },
    {
      "src": "/android-icon-192x192.png",
      "sizes": "192x192",
      "type": "image/png",
      "density": "4.0"
    }
  ]
}


### 27.112 GET https://api.sowork.com/eyekia/api/v1/assets-version -> 200
RES: {"assets":[{"id":"00XJqiZEwyi6d5OFqA56","v":"1hpjpl91s"},{"id":"01wOnQBcMp9mnF1StinN","v":"1hmmajsu9"},{"id":"02QOeICHQfb73TM1k2Mf","v":"1i9cbct7c"},{"id":"08twuvH2aez3KY3Se4r6","v":"1iphodciv"},{"id":"08uGTz1mIZU2tQTAwwny","v":"1j7mriidj"},{"id":"09tCQAE5cK5jcndbaOtz","v":"1gq9pko7o"},{"id":"0Bk0PjgjHb9pXup34tsq","v":"1j7mriiqo"},{"id":"0C0X2BqpDmwkrUDmO3M4","v":"1hpjplaer"},{"id":"0EMrQiQSAdusl5ErszbH","v":"1jjk37och"},{"id":"0F3UNdF2syOXNKsvPmKe","v":"1hg99kfi4"},{"id":"0Hs0WfFjVy3s3jkFk5HX","v":"1jkl06sh3"},{"id":"0Hv2YVqPK7JPjbOC6yMU","v":"1jjk37p1q"},{"id":"0L88tyzcDWXRO5tOOgYp","v":"1jjk37p87"},{"id":"0Lakiq4fYpC5FPTcpb1f","v":"1hql4ea95"},{"id":"0LuCWQXgInHMzlae1LXw","v":"1hg99kgd3"},{"id":"0MaIQKjuehhQ7GD8F2ch","v":"1jjk37ppe"},{"id":"0O0YxD5uW5gNWR0QWdpP","v":"1hr6lg96s"},{"id":"0OrpUmm5VyfneBfG4atj","v":"1hpjplc3p"},{"id":"0OusslPPD4YZnjn1bdRG","v":"1j7mrildv"},{"id":"0PlxoyY5VEWlYbW3qqHA","v":"1hpjplcij"},{"id":"0WTmcDq1QrIBs0sBpqZ6","v":"1hg16nsq9"},{"id":"0WxQuSzahYQMxx1NUpHw","v":"1io8nj6lt"},{"id":"0Xw236ULN1iJg3ltkH1Y","v":"1i4t39i75"},{"id":"0YLrZzoN6e9yp04ZLv2B","v":"1gu2db168"},{"id":"0ejRSM3tGGlpRuwnlTeX","v":"1jjk37ro9"},{"id":"0ghzveoFRBsmGZKDKBUg","v":"1hg99kibr"},{"id":"0ha10kUCuA6rm6ZvvSg7","v":"1hg99kil3"},{"id":"0iK6vtAVJiXYJH5V3GN6","v":"1hg99kiuj"},{"id":"0jtrdiOaU1LK7NAIDZhy","v":"1iphodgru"},{"id":"0kBhRLHdvhJvFef0OrGw","v":"1j7mrimuc"},{"id":"0khfDsirhgjnOQlpJJ7h","v":"1k0g64d9m"},{"id":"0lXIdpYZOqAnuS5rYhPM","v":"1glg461a3"},{"id":"0lx2CpsNY8te83zDZJhI","v":"1jjk37t5q"},{"id":"0mZICodBbwuzNT1w1cco","v":"1j7mrincb"},{"id":"0niPAnvWteAQYVC5A9zm","v":"1hg16nvgu"},{"id":"0pz5QiXRU4hzi6DSb4aH","v":"1itv6c8uo"},{"id":"0rZ3g18hV6bPgf2ipLYy","v":"1jjk37tph"},{"id":"0trRlvdyUIKg3LfSrby8","v":"1jaj8caa4"},{"id":"0u0L6pjDwrJC4szbh4Dp","v":"1icoahrsl"},{"id":"0u0zC11BC9LwGhsG3Sn2","v":"1frtvccdv"},{"id":"0uDErNZUlNK2NEru65GS","v":"1im2ehcsn"},{"id":"0v4LpQ3zFN2U4EH9EIw8","v":"1i4t39k62"},{"id":"0wnEJ9xKIhbbHj2bZibg","v":"1hpjplekv"},{"id":"0zIG6SLCAsc1ELzJtnCK","v":"1hg16o0op"},{"id":"0zLqvBmNsgPxGEGey6lQ","v":"1ibkcckut"},{"id":"113LfdBtbhg0xVgVmLeQ","v":"1hpjples8"},{"id":"114FRGQhpq3hFBOwbEJr","v":"1guvi40kr"},{"id":"11t9poblx8tgRTsJVYle","v":"1iphodiga"},{"id":"128osBDlw4X3DIvUYSmq","v":"1hpjplf50"},{"id":"12xUBE1tZGzmBlZWbCIc","v":"1iphodim8"},{"id":"13KKcw1tT6vtYmGmnDTo","v":"1ilp0bf26"},{"id":"13nrjcGbAWRywqtfB4K0","v":"1hmmak42s"},{"id":"17rXYuWSLSM3te2J2nqE","v":"1jjk3804i"},{"id":"18DhyVimc81XQl24LNof","v":"1glg462ed"},{"id":"1CVpTAHK5m6KqltanIMO","v":"1fttaqt5d"},{"id":"1EtkRThDZxuJA872PovL","v":"1hg99km6h"},{"id":"1I9TgxZqNUQbtUHUw03p","v":"1io8nj9iv"},{"id":"1JFGbWzgGvVNfSln9aPU","v":"1hp1o0c4k"},{"id":"1Ld8d07nxRkXdb8qCCh4","v":"1gu8oq7r1"},{"id":"1MAizr1VCC4v6Q7ZXc2f","v":"1jjk38133"},{"id":"1Mt3plNp3zT1FoAPUbcu","v":"1hg99kmrt"},{"id":"1ODoMAnQLRhz5h5ABoJ3","v":"1hpjplg4o"},{"id":"1QC8re1QEmoMtdXQg9nY","v":"1hpjplg97"},{"id":"1QiB9laLWZEndOYqOU6V","v":"1hg99knab"},{"id":"1SsvT9XEtzo8anxOzaWK","v":"1hofpsgkl"},{"id":"1VEQ9nWBEWbsFOF9678C","v":"1hr9l99p9"},{"id":"1VobWpOKIVK2fKWgYCNq","v":"1icoahtjr"},{"id":"1W0cx9o1GIeLOJID4Fqb","v":"1jaj8cbld"},{"id":"1WSCk2DRBUrXvyU9P0Nc","v":"1hql4eeu7"},{"id":"1Z4TeJsmmioccjbeP9fP","v":"1ibkccms2"},{"id":"1Z7a18kygGriTiwCFxsm","v":"1io8njamd"},{"id":"1bG4d7RM4NqVaNk3XpTB","v":"1h19kukt7"},{"id":"1bHzwOMbJBiJ7G1kkzBI","v":"1ge304o8n"},{"id":"1dyuFQNLSfFX31aYI7rJ","v":"1hg99koch"},{"id":"1gkib5M4a8UAVRRjnJ1A","v":"1jaj8cbuh"},{"id":"1i28selpjEYCd1HjvXUG","v":"1hpjplhbs"},{"id":"1lQ9TJ9WzgGPLeiXdDrq","v":"1iphodkl9"},{"id":"1m24G06NoTTShTu6ECZP","v":"1hpjplhj5"},{"id":"1nB2nUjru0kRWA0q2IWW","v":"1jjk383nh"},{"id":"1oJS1os1G4J1naTk3gqQ","v":"1jjk383tc"},{"id":"1rIKM93JLD2bm6gxL6AJ","v":"1jjk3843f"},{"id":"1scxwYsIPaQgOKrD1Bml","v":"1jjk38492"},{"id":"1sySw25p8WgIf1Tz2qwh","v":"1j7mrir5o"},{"id":"1t0NLTJRXUetswvT90We","v":"1hulqtr7u"},{"id":"1tOeMoyb9kQGJjTF7JN9","v":"1i4t39nrn"},{"id":"1xLq0vjJ5f1IHJ1C9HeG","v":"1glg464nu"},{"id":"1yGIh996dmwV3W4sWrV6","v":"1icoahuvg"},{"id":"20DVBx1kjWkkhrmHkW01","v":"1jaj8ccm7"},{"id":"22sZfE51esnw62FnPeUy","v":"1jajhimts"},{"id":"23S0pfltatlb2iChCaWW","v":"1j7mrirqc"},{"id":"23q112oYoLv8NrXOmgR9","v":"1hmmak64i"},{"id":"2Cbckt7yMw2GNlZL8Fhu","v":"1hg99kppf"},{"id":"2DMgVMR5o28tpxFBmzUx","v":"1i0tgt2l6"},{"id":"2EbEBbCrwF8Uwym7Z0Qq","v":"1jaj8cd41"},{"id":"2FRy6d5FPYaa9K4Df4zi","v":"1hpjpliob"},{"id":"2I7Qmc87tFyePzMQWSqy","v":"1j7mrisba"},{"id":"2J1n8EKM8iXr4FYxjGw7","v":"1iphodmcj"},{"id":"2LdZKJQ8LVgjrWzeD2Qx","v":"1jjk386pc"},{"id":"2Max0C24CH4nIPbaj0L8","v":"1frtvce8r"},{"id":"2MoIhyWNjCmaOTe7dIMf","v":"1hntjuerc"},{"id":"2OGIJBBXk9UGs8aIhjk0","v":"1iphodml3"},{"id":"2OKF2fJN75tHXVRUq9uQ","v":"1j4sj7f7d"},{"id":"2SET0ZP3aKf0gM7MhnFh","v":"1hpjplj9k"},{"id":"2Sa8R3CtCVKp3dbfpRB1","v":"1iphodmqp"},{"id":"2TD4HVsDvJWurM3xrDPN","v":"1hmmak72q"},{"id":"2VWW3jIhyhkRRAVXwHrt","v":"1jjk387t0"},{"id":"2WhhUJw1jOZVpyqGusjU","v":"1iphodn3o"},{"id":"2XP7B8CwGpdIP38LiWkU","v":"1hpjpljoc"},{"id":"2YKecfHjwAidxLcCypA9","v":"1i9u4mk9h"},{"id":"2da1DTeae0lSoAyzzDdJ","v":"1hr9l9e0f"},{"id":"2kWVIk0f4F8spe7NCwF0","v":"1hpjplk0o"},{"id":"2kaqFa0Y9WirUvhYxwQE","v":"1h5nit7er"},{"id":"2lll4c69rVnOGrHjqylJ","v":"1j4sj7gmg"},{"id":"2m0cGHqNz0aYfx863UbR","v":"1io8njdpj"},{"id":"2oQjqpy0QJaUUmjZDIyW","v":"1io8njdst"},{"id":"2qucd0YAFEj6kiJVXKLI","v":"1j4sj7h4k"},{"id":"2roE3M3ULyGPN2FXtNWL","v":"1jjk389km"},{"id":"2t6EJ2JnKnwQMTCwVZz1","v":"1hpjplkbd"},{"id":"2tGkINbf5qxtDjJNk9fN","v":"1hp1o0hn2"},{"id":"2xjbaqdDkQF8SRwCZ748","v":"1j4sj7hkt"},{"id":"2zcMOx46DO0sRwzHJYGT","v":"1i0tgt4te"},{"id":"34ym2xgRGeArHhyiSBpW","v":"1hg99ks1j"},{"id":"387zDIXuZbKwdPsSMUgP","v":"1hql4ehfo"},{"id":"39AemWJFq2s7p5mZrr1X","v":"1hpjplkrj"},{"id":"3DuszbShagv5bry0UC0Z","v":"1jjk38asp"},{"id":"3H8XO2XJz7OBWEtxtzhX","v":"1hg99ksqs"},{"id":"3IUc22yqOG58FDAy0XaJ","v":"1hpjpll6f"},{"id":"3JffWXUY7XxXTzzfbZW0","v":"1hq2c9cun"},{"id":"3Ke82jpx1XREz7FrZlSj","v":"1if0guvdf"},{"id":"3KpFrsopNFuknFtXBOdF","v":"1i3nsunii"},{"id":"3L

### 27.225 GET https://firebase.googleapis.com/v1alpha/projects/-/apps/1:69834214729:web:c3e2769cb392ca9203de98/webConfig -> 200
RES: {
  "projectId": "sage-learning-prod",
  "appId": "1:69834214729:web:c3e2769cb392ca9203de98",
  "databaseURL": "https://sage-learning-prod.firebaseio.com",
  "storageBucket": "sage-learning-prod.appspot.com",
  "locationId": "us-central",
  "authDomain": "sage-learning-prod.firebaseapp.com",
  "messagingSenderId": "69834214729",
  "measurementId": "G-MXHRMNM5C0"
}


### 27.231 OPTIONS https://firebase.googleapis.com/v1alpha/projects/-/apps/1:69834214729:web:c3e2769cb392ca9203de98/webConfig -> 200
RES: 

### 27.547 GET https://app.sowork.com/characters/v5/customizer_config.json -> 200
RES: {"README":"THIS IS AN AUTOGENERATED FILE BY CI, DO NOT MODIFY","bodyTypes":{"characterA":{"slots":{"backAccessory":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"category":{"id":"backAccessories","order":0},"items":{"bday":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"disabled":false,"accessKey":"PREMIUM","publishedAt":"2024-03-08T16:08:53.946Z","variations":{"basecolor":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"single","primary":"#d5b276ff"},"disabled":true},"blue":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":164,"saturationMultiplier":1.2,"valueMultiplier":1}}},"yellow":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":1.8,"valueMultiplier":0.9}}},"purple":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-120,"saturationMultiplier":1,"valueMultiplier":0.9}}},"red":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-44,"saturationMultiplier":2,"valueMultiplier":0.7}}},"green":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":81,"saturationMultiplier":1.2,"valueMultiplier":1}}}},"baseVariation":"basecolor"},"twentyfiveballoon":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"disabled":false,"accessKey":"PREMIUM","publishedAt":"2024-12-24T11:42:38.240Z","variations":{"basecolor":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"single","primary":"#d5b276ff"},"disabled":true},"blue":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":164,"saturationMultiplier":1.2,"valueMultiplier":1}}},"black":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":0,"valueMultiplier":0.35}}},"yellow":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":1.8,"valueMultiplier":0.9}}},"brown":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-20,"saturationMultiplier":1.5,"valueMultiplier":0.55}}},"orange":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-18,"saturationMultiplier":2,"valueMultiplier":0.85}}},"purple":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-120,"saturationMultiplier":1,"valueMultiplier":0.9}}},"red":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-44,"saturationMultiplier":2,"valueMultiplier":0.7}}},"turquoise":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":119,"saturationMultiplier":1,"valueMultiplier":1}}},"grey":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":0,"valueMultiplier":1}}},"white":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":0,"valueMultiplier":1.3}}},"green":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":81,"saturationMultiplier":1.2,"valueMultiplier":1}}}},"baseVariation":"basecolor"},"chineseLuck":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"disabled":false,"accessKey":"PREMIUM","publishedAt":"2025-01-23T20:36:35.648Z","variations":{"basecolor":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"single","primary":"#d5b276ff"},"disabled":true},"yellow":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":1,"valueMultiplier":1}}}},"baseVariation":"basecolor"},"chooseyou":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"disabled":false,"accessKey":"PREMIUM","publishedAt":"2025-02-07T16:24:23.811Z","variations":{"basecolor":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"single","primary":"#d5b276ff"},"disabled":true},"blue":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":1,"valueMultiplier":1}}}},"baseVariation":"basecolor"},"guitar":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"disabled":false,"publishedAt":"2021-11-08T23:00:00.000Z","variations":{"brown":{"disabled":false,"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-20,"saturationMultiplier":1.5,"valueMultiplier":0.55}}},"basecolor":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"single","primary":"#d5b276ff"},"disabled":true},"blue":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":164,"saturationMultiplier":1.2,"valueMultiplier":1}}},"black":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":0,"valueMultiplier":0.35}}},"yellow":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":1.8,"valueMultiplier":0.9}}},"orange":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-18,"saturationMultiplier":2,"valueMultiplier":0.85}}},"purple":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-120,"saturationMultiplier":1,"valueMultiplier":0.9}}},"red":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-44,"saturationMultiplier":2,"valueMultiplier":0.7}}},"turquoise":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transf

### 27.547 GET https://app.sowork.com/characters/v5/character_art_database.json -> 200
RES: {"README":"THIS IS A FILE THAT'S AUTOGENERATED BY THE CI / CD SYSTEM. DO NOT TOUCH!","lastUpdatedCommit":"79530061912db310dc167b166caaa5de72f22e29","assets":{"assistants_body_belfort+basecolor":{"preview":{"ver":1,"type":"atlas"},"layers":{"body":{"idle":{"ver":1,"type":"atlas"},"jump1":{"ver":1,"type":"atlas"},"sit":{"ver":1,"type":"atlas"},"walk":{"ver":1,"type":"atlas"},"work1":{"ver":1,"type":"atlas"},"work2":{"ver":1,"type":"atlas"},"work3":{"ver":1,"type":"atlas"},"work4":{"ver":1,"type":"atlas"},"work5":{"ver":1,"type":"atlas"},"work6":{"ver":1,"type":"atlas"},"work7":{"ver":1,"type":"atlas"},"working":{"ver":1,"type":"atlas"}}}},"assistants_body_gf+basecolor":{"preview":{"ver":1,"type":"atlas"},"layers":{"body":{"idle":{"ver":1,"type":"atlas"},"jump1":{"ver":1,"type":"atlas"},"sit":{"ver":1,"type":"atlas"},"walk":{"ver":1,"type":"atlas"},"work1":{"ver":1,"type":"atlas"},"work2":{"ver":1,"type":"atlas"},"work3":{"ver":1,"type":"atlas"},"work4":{"ver":1,"type":"atlas"},"work5":{"ver":1,"type":"atlas"},"work6":{"ver":1,"type":"atlas"},"work7":{"ver":1,"type":"atlas"},"working":{"ver":1,"type":"atlas"}}}},"assistants_body_mross+basecolor":{"preview":{"ver":1,"type":"atlas"},"layers":{"body":{"idle":{"ver":2,"type":"atlas"},"jump1":{"ver":1,"type":"atlas"},"sit":{"ver":1,"type":"atlas"},"walk":{"ver":1,"type":"atlas"},"work1":{"ver":1,"type":"atlas"},"work2":{"ver":2,"type":"atlas"},"work3":{"ver":2,"type":"atlas"},"work4":{"ver":2,"type":"atlas"},"work5":{"ver":1,"type":"atlas"},"work6":{"ver":1,"type":"atlas"},"work7":{"ver":1,"type":"atlas"},"working":{"ver":2,"type":"atlas"}}}},"cats_body_catTypeA+typeA":{"preview":{"ver":0,"type":"atlas"},"layers":{"body":{"idle":{"ver":2,"type":"atlas"},"sit":{"ver":2,"type":"atlas"},"walk":{"ver":2,"type":"atlas"}}}},"cats_body_catTypeA+typeB":{"preview":{"ver":0,"type":"atlas"},"layers":{"body":{"idle":{"ver":2,"type":"atlas"},"sit":{"ver":2,"type":"atlas"},"walk":{"ver":2,"type":"atlas"}}}},"characterA_avatarExtra_avatarExtra+basecolor":{"preview":{"ver":0,"type":"atlas"},"layers":{"avatarExtra":{"angryEmojiMeeting":{"ver":1,"type":"atlas"},"beerEmojiMeeting":{"ver":1,"type":"atlas"},"bowheEmojiMeeting":{"ver":1,"type":"atlas"},"bowsheEmojiMeeting":{"ver":1,"type":"atlas"},"cashEmojiMeeting":{"ver":1,"type":"atlas"},"champagneEmojiMeeting":{"ver":1,"type":"atlas"},"clapEmojiMeeting":{"ver":1,"type":"atlas"},"clockEmojiMeeting":{"ver":1,"type":"atlas"},"cocktailEmojiMeeting":{"ver":1,"type":"atlas"},"crazyEmojiMeeting":{"ver":1,"type":"atlas"},"cryEmojiMeeting":{"ver":1,"type":"atlas"},"eggplantEmojiMeeting":{"ver":1,"type":"atlas"},"fightEmojiMeeting":{"ver":1,"type":"atlas"},"fireEmojiMeeting":{"ver":1,"type":"atlas"},"flowersEmojiMeeting":{"ver":1,"type":"atlas"},"frozenEmojiMeeting":{"ver":1,"type":"atlas"},"glassEmojiMeeting":{"ver":1,"type":"atlas"},"handEmojiMeeting":{"ver":1,"type":"atlas"},"happyEmojiMeeting":{"ver":1,"type":"atlas"},"hilariousEmojiMeeting":{"ver":1,"type":"atlas"},"hugEmojiMeeting":{"ver":1,"type":"atlas"},"inloveEmojiMeeting":{"ver":1,"type":"atlas"},"kissEmojiMeeting":{"ver":1,"type":"atlas"},"likeEmojiMeeting":{"ver":1,"type":"atlas"},"lolEmojiMeeting":{"ver":1,"type":"atlas"},"lollypopEmojiMeeting":{"ver":1,"type":"atlas"},"loveEmojiMeeting":{"ver":1,"type":"atlas"},"normalEmojiMeeting":{"ver":1,"type":"atlas"},"oleEmojiMeeting":{"ver":1,"type":"atlas"},"partyEmojiMeeting":{"ver":1,"type":"atlas"},"peaceEmojiMeeting":{"ver":1,"type":"atlas"},"prayEmojiMeeting":{"ver":1,"type":"atlas"},"rainbowEmojiMeeting":{"ver":1,"type":"atlas"},"rocketEmojiMeeting":{"ver":1,"type":"atlas"},"sadEmojiMeeting":{"ver":1,"type":"atlas"},"seedEmojiMeeting":{"ver":1,"type":"atlas"},"silenceEmojiMeeting":{"ver":1,"type":"atlas"},"sleepEmojiMeeting":{"ver":1,"type":"atlas"},"snowEmojiMeeting":{"ver":1,"type":"atlas"},"sockyEmojiMeeting":{"ver":1,"type":"atlas"},"teethEmojiMeeting":{"ver":1,"type":"atlas"},"thinkEmojiMeeting":{"ver":1,"type":"atlas"},"valentinEmojiMeeting":{"ver":1,"type":"atlas"},"winkEmojiMeeting":{"ver":1,"type":"atlas"}}}},"characterA_avatarExtra_avatarExtra+guest":{"preview":{"ver":0,"type":"atlas"},"layers":{"avatarExtra":{"angryEmojiMeeting":{"ver":1,"type":"atlas"},"beerEmojiMeeting":{"ver":1,"type":"atlas"},"bowheEmojiMeeting":{"ver":1,"type":"atlas"},"bowsheEmojiMeeting":{"ver":1,"type":"atlas"},"cashEmojiMeeting":{"ver":2,"type":"atlas"},"champagneEmojiMeeting":{"ver":1,"type":"atlas"},"clapEmojiMeeting":{"ver":1,"type":"atlas"},"clockEmojiMeeting":{"ver":1,"type":"atlas"},"cocktailEmojiMeeting":{"ver":1,"type":"atlas"},"crazyEmojiMeeting":{"ver":1,"type":"atlas"},"cryEmojiMeeting":{"ver":1,"type":"atlas"},"eggplantEmojiMeeting":{"ver":1,"type":"atlas"},"fightEmojiMeeting":{"ver":1,"type":"atlas"},"fireEmojiMeeting":{"ver":1,"type":"atlas"},"flowersEmojiMeeting":{"ver":1,"type":"atlas"},"frozenEmojiMeeting":{"ver":1,"type":"atlas"},"glassEmojiMeeting":{"ver":1,"type":"atlas"},"handEmojiMeeting":{"ver":1,"type":"atlas"},"happyEmojiMeeting":{"ver":1,"type":"atlas"},"hilariousEmojiMeeting":{"ver":1,"type":"atlas"},"hugEmojiMeeting":{"ver":1,"type":"atlas"},"inloveEmojiMeeting":{"ver":1,"type":"atlas"},"kissEmojiMeeting":{"ver":1,"type":"atlas"},"likeEmojiMeeting":{"ver":1,"type":"atlas"},"lolEmojiMeeting":{"ver":1,"type":"atlas"},"lollypopEmojiMeeting":{"ver":1,"type":"atlas"},"loveEmojiMeeting":{"ver":1,"type":"atlas"},"negationEmojiMeeting":{"ver":1,"type":"atlas"},"nodEmojiMeeting":{"ver":1,"type":"atlas"},"normalEmojiMeeting":{"ver":1,"type":"atlas"},"oleEmojiMeeting":{"ver":1,"type":"atlas"},"partyEmojiMeeting":{"ver":1,"type":"atlas"},"peaceEmojiMeeting":{"ver":1,"type":"atlas"},"prayEmojiMeeting":{"ver":1,"type":"atlas"},"rainbowEmojiMeeting":{"ver":1,"type":"atlas"},"rocketEmojiMeeting":{"ver":1,"type":"atlas"},"sadEmojiMeeting":{"ver":1,"type":"atlas"},"seedEmojiMeeting":{"ver":1,"type":"atlas"},"silenceEmojiMeeting":{"ver":1,"type":"atlas"},"sleepEmojiMeeting":{"ver":1,"type":"atlas"},"snowEmojiMeeting

### 27.547 GET https://app.sowork.com/characters/v5/customizer_product_prices.json -> 200
RES: {"README":"THIS IS AN AUTOGENERATED FILE BY CI, DO NOT MODIFY","products":{"wings":{"name":{"en":"Wings"},"description":{"en":"TODO"},"currency":"usd","unitAmount":149},"moon":{"name":{"en":"The Deadline Moon"},"description":{"en":"TODO"},"currency":"usd","unitAmount":149},"vgicon":{"name":{"en":"Press start"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"grank":{"name":{"en":"Into the Technodrome"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"emojiMask":{"name":{"en":"Emoji Mask"},"description":{"en":"TODO"},"currency":"usd","unitAmount":99},"emojiMask2":{"name":{"en":"Emoji Mask set 2"},"description":{"en":"TODO"},"currency":"usd","unitAmount":99},"teamDog":{"name":{"en":"Dog mask"},"description":{"en":"TODO"},"currency":"usd","unitAmount":149},"teamCat":{"name":{"en":"Cat mask"},"description":{"en":"TODO"},"currency":"usd","unitAmount":149},"brazSoccer":{"name":{"en":"La Canarinha"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"spaSoccer":{"name":{"en":"La Roja"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"itaSoccer":{"name":{"en":"Gli Azzurri"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"argenSoccer":{"name":{"en":"La Albiceleste"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"gerSoccer":{"name":{"en":"Die Mannschaft"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"brazilSoccer":{"name":{"en":"Soccer Outfit"},"description":{"en":"TODO"},"currency":"usd","unitAmount":999},"corinthians":{"name":{"en":"corinthians"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"cruzeiro":{"name":{"en":"cruzeiro"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"flamingo":{"name":{"en":"flamingo"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"gremio":{"name":{"en":"gremio"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"palmeiras":{"name":{"en":"palmeiras"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"leonal":{"name":{"en":"Katana Turtle"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"donal":{"name":{"en":"Bo Staff Turtle"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"mikelal":{"name":{"en":"Nunchaku Turtle"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"rafal":{"name":{"en":"Nunchaku Turtle"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"dancingBanana":{"name":{"en":"Peanut Butter Jelly Time!"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"lsn":{"name":{"en":"Sugar, spice and everything nice"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"chocob":{"name":{"en":"Kue!!!"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"ashGb":{"name":{"en":"Become a Master"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"pachiko":{"name":{"en":"I choose you"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"saidack":{"name":{"en":"I choose you"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"charman":{"name":{"en":"I choose you"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"squilo":{"name":{"en":"I choose you"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"bilbosur":{"name":{"en":"I choose you"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"mecha":{"name":{"en":"One with the machine"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"sailorS":{"name":{"en":"In the name of the moon"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"afromonster":{"name":{"en":"afromonster"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"mistery":{"name":{"en":"FX specialist"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"octoOct":{"name":{"en":"Dr LongLegs"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"sopiper":{"name":{"en":"sopiper"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"vespa":{"name":{"en":"Vespa"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"fone":{"name":{"en":"Ka-Chow"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"motogp":{"name":{"en":"Pole!"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"sk8ter":{"name":{"en":"Tony Pigeon Amateur Skater"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"zombieDlc":{"name":{"en":"Braaaaaiiiinssss"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"futuHead":{"name":{"en":"Head in Jar!"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"spidman":{"name":{"en":"Your friendly teammate "},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"lowclo":{"name":{"en":"ex-SOLDIER First Class"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"lowsep":{"name":{"en":"One-Winged Angel"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"bugbuster":{"name":{"en":"Who ya gonna code?"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"sadako":{"name":{"en":"Seven days..."},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"colormap":{"name":{"en":"The prey"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"flyingWiz":{"name":{"en":"Magic sports"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"hoverskater":{"name":{"en":"Marty McHover"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"skaterPack":{"name":{"en":"M&T Skater Pack"},"description":{"en":"TODO"},"currency":"usd","unitAmount":399},"futuShip":{"name":{"en":"Happy 2999"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"dancingDragon":{"name":{"en":"Dancing Dragon"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"lamia":{"name":{"en":"Sliding body"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"tank":{"name":{"en":"Like a Tank"},"description":{"en":"TODO"},"cu

### 27.567 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=55150&CVER=22&X-HTTP-Session-Id=gsessionid&zx=ovigi1lwcc3d&t=1 -> 200
REQ: headers=X-Goog-Api-Client%3Agl-js%2F%20fire%2F9.23.0%0D%0AContent-Type%3Atext%2Fplain%0D%0AX-Firebase-GMPID%3A1%3A69834214729%3Aweb%3Ac3e2769cb392ca9203de98%0D%0A&count=1&ofs=0&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2FgroupPublic%2Fv0XvoIt3uK40pdF0mQDB%22%5D%7D%2C%22targetId%22%3A2%7D%7D
RES: 51
[[0,["c","RCmCBFUE9wekp6KlJRj1qQ","",8,15,30000]]]


### 27.777 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=OmFQ1NYpk97tqxboPYiQGSGo122ZM4rV2hzEMNy4e4J-65LhntFdKg&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=RCmCBFUE9wekp6KlJRj1qQ&AID=0&CI=0&TYPE=xmlhttp&zx=1rqxlmkcblck&t=1 -> 200
RES: 

### 28.172 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=OmFQ1NYpk97tqxboPYiQGSGo122ZM4rV2hzEMNy4e4J-65LhntFdKg&SID=RCmCBFUE9wekp6KlJRj1qQ&RID=55151&AID=4&zx=lts3feezplgq&t=1 -> 200
REQ: count=1&ofs=1&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A2%7D
RES: 8
[1,4,7]


### 39.330 POST https://api.sowork.com/api/v1/auth-providers/getByEmail -> 200
REQ: {"email":"tito@20dash.com"}
RES: {"exists":true,"providers":[{"name":"google.com","provider_id":"google.com","custom_properties":{}}]}

### 43.000 GET https://app.sowork.com/__/auth/iframe?apiKey=<REDACTED>&appName=%5BDEFAULT%5D&v=9.23.0&eid=p&usegapi=1&jsh=m%3B%2F_%2Fscs%2Fabc-static%2F_%2Fjs%2Fk%3Dgapi.lb.en.gh7qIZtzO5w.O%2Fd%3D1%2Frs%3DAHpOoo84YKT1RVy0T6hcXi5rH3LooB1WCw%2Fm%3D__features__ -> 200
RES: <!DOCTYPE html>
<html>
<head>
<meta name=viewport content="width=device-width, initial-scale=1">
<meta http-equiv="Content-Type" content="text/html; charset=utf-8">
<script type="text/javascript" src="iframe.js"></script>
<script type="text/javascript" nonce="firebase-auth-helper">
if (window.fireauth && fireauth.iframe && fireauth.iframe.AuthRelay) {
  fireauth.iframe.AuthRelay.initialize();
}
</script>
</head>
<body>
</body>
</html>


### 43.051 GET https://identitytoolkit.googleapis.com/v1/projects?key=<REDACTED> -> 200
RES: {
  "projectId": "69834214729",
  "authorizedDomains": [
    "localhost",
    "sage-learning-prod.firebaseapp.com",
    "app.sagelearning.ai",
    "app.sophya.ai",
    "app.sophya.world",
    "sophya-assets.web.app",
    "app.sowork.com",
    "assets.sowork.com"
  ]
}


### 43.053 OPTIONS https://identitytoolkit.googleapis.com/v1/projects?key=<REDACTED> -> 200
RES: 

### 43.847 GET https://www.googleapis.com/identitytoolkit/v3/relyingparty/getProjectConfig?key=<REDACTED>&cb=1791290968826 -> 200
RES: {
  "projectId": "69834214729",
  "authorizedDomains": [
    "localhost",
    "sage-learning-prod.firebaseapp.com",
    "app.sagelearning.ai",
    "app.sophya.ai",
    "app.sophya.world",
    "sophya-assets.web.app",
    "app.sowork.com",
    "assets.sowork.com"
  ]
}


### 43.847 OPTIONS https://www.googleapis.com/identitytoolkit/v3/relyingparty/getProjectConfig?key=<REDACTED>&cb=1791290968826 -> 200
RES: 

### 43.851 GET https://www.googleapis.com/identitytoolkit/v3/relyingparty/getProjectConfig?key=<REDACTED>&cb=1791290968831 -> 200
RES: {
  "projectId": "69834214729",
  "authorizedDomains": [
    "localhost",
    "sage-learning-prod.firebaseapp.com",
    "app.sagelearning.ai",
    "app.sophya.ai",
    "app.sophya.world",
    "sophya-assets.web.app",
    "app.sowork.com",
    "assets.sowork.com"
  ]
}


### 43.851 OPTIONS https://www.googleapis.com/identitytoolkit/v3/relyingparty/getProjectConfig?key=<REDACTED>&cb=1791290968831 -> 200
RES: 

### 44.127 GET https://app.sowork.com/__/firebase/init.json -> 200
RES: {
  "apiKey": "AIzaSyA3OhZgExc-GvwlFal2R7rjFH7UqAp-qjI",
  "appId": "1:69834214729:web:c3e2769cb392ca9203de98",
  "authDomain": "sage-learning-prod.firebaseapp.com",
  "databaseURL": "https://sage-learning-prod.firebaseio.com",
  "measurementId": "G-MXHRMNM5C0",
  "messagingSenderId": "69834214729",
  "projectId": "sage-learning-prod",
  "storageBucket": "sage-learning-prod.appspot.com"
}

### 44.298 OPTIONS https://www.googleapis.com/identitytoolkit/v3/relyingparty/createAuthUri?key=<REDACTED> -> 200
RES: 

### 44.298 POST https://www.googleapis.com/identitytoolkit/v3/relyingparty/createAuthUri?key=<REDACTED> -> 200
REQ: {"providerId":"google.com","continueUri":"https://app.sowork.com/__/auth/handler","customParameter":{"display":"popup","prompt":"consent","login_hint":"tito@20dash.com"},"oauthScope":"{\"google.com\":\"profile\"}"}
RES: 

### 71.309 POST https://identitytoolkit.googleapis.com/v1/accounts:signInWithIdp?key=<REDACTED> -> 200
REQ: {"requestUri":"https://app.sowork.com/__/auth/handler?state=<REDACTED>&iss=https%3A%2F%2Faccounts.google.com&code=<REDACTED>&scope=email%20profile%20https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fuserinfo.profile%20https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fuserinfo.email%20openid&authuser=0&hd=20dash.com&prompt=consent","sessionId":"e8L67J8ImO-RuguMCJdg6cgo5Jg","returnSecureToken":true,"returnIdpCredential":true}
RES: {
  "federatedId": "https://accounts.google.com/111790800936946378724",
  "providerId": "google.com",
  "email": "tito@20dash.com",
  "emailVerified": true,
  "firstName": "Tito ",
  "fullName": "Tito Guidotti",
  "lastName": "Guidotti",
  "photoUrl": "https://lh3.googleusercontent.com/a/ACg8ocKVvafbCtCGUmXh2WFuh7TSsQQGC0VkCWLYn0fXgV_2NgQwyw=s96-c",
  "localId": "RMUYRzBXvUOCqtJdxSAiMdiEZM82",
  "displayName": "Tito Guidotti",
  "idToken": "<JWT>",
  "context": "",
  "oauthAccessToken": "ya29.a0AX07Cmt0pjKGCS2uQ5U9nE6U51aNmQxQkQriD2VA7OSA2h9zV_KSkNbp5-0zIGw-DgPfhpwUK2mbsitbm1LKYGlBQAcyp7VOSmEKOhc-3Oo1aQfR3dSFwyHWOIOiP2SLXg6OKVteZaAw3K6wB3mml62Rpjqniz8_UdM9TZxkciDd15RVhhv8l_XntxlK3PYbtGYT7esaCgYKAVcSARcSFQHGX2MirDIBu-mHGkzrxoaG8Apulg0206",
  "oauthExpireIn": 3599,
  "refreshToken": "AMf-vBxtVkQPtdEQzqiW0cx2w4Kwo3zFj1umpfbgxT3falzRIE9rD27G_xBmuJ_tUNlmWV2Av_Pk3MkJ4jueD2UO3If0DkFm-sufm8cwJVfs8F4o1l1iDg13XBVBW0KurtC4AlZBCNlupNOwgIbuFnvQXY-DW1bDC9Oful6eEgPk3p4QtGnoBu3uO3eDD24j-wEnnAFAd6PSRE4Ue9xHfywmdvKC5KsPyI06D4HrYgr_c-YqkDTIYmEGeE2T5HW_oOp_2lpNyRX7fY92d2n6VTpNtQBOpdS7Pue4PHCRm6jBwThBwbq8NAg0h5kY5xtOrTZrHGOq1BSnpbCCZEs4jM-1U9s8hO4ZoyIkn26PzgS3r4iGTWj6C42UMnjol9A_i0MG-1McLO9WNnB1O7_B0PNonctA_bB-ZjiJP3av3Q_KnMmQho1nY9at4lHhJwXeJTT6OOnWPl_w",
  "expiresIn": "3600",
  "oauthIdToken": "<JWT>",
  "rawUserInfo": "{\"name\":\"Tito Guidotti\",\"granted_scopes\":\"https://www.googleapis.com/auth/userinfo.profile openid https://www.googleapis.com/auth/userinfo.email\",\"id\":\"111790800936946378724\",\"verified_email\":true,\"given_name\":\"Tito \",\"hd\":\"20dash.com\",\"family_name\":\"Guidotti\",\"email\":\"tito@20dash.com\",\"picture\":\"https://lh3.googleusercontent.com/a/ACg8ocKVvafbCtCGUmXh2WFuh7TSsQQGC0VkCWLYn0fXgV_2NgQwyw=s96-c\"}",
  "kind": "identitytoolkit#VerifyAssertionResponse"
}


### 71.309 OPTIONS https://identitytoolkit.googleapis.com/v1/accounts:signInWithIdp?key=<REDACTED> -> 200
RES: 

### 71.844 POST https://identitytoolkit.googleapis.com/v1/accounts:lookup?key=<REDACTED> -> 200
REQ: {"idToken":"<JWT>"}
RES: {
  "kind": "identitytoolkit#GetAccountInfoResponse",
  "users": [
    {
      "localId": "RMUYRzBXvUOCqtJdxSAiMdiEZM82",
      "email": "tito@20dash.com",
      "displayName": "Tito Guidotti",
      "photoUrl": "https://lh3.googleusercontent.com/a/ACg8ocKVvafbCtCGUmXh2WFuh7TSsQQGC0VkCWLYn0fXgV_2NgQwyw=s96-c",
      "emailVerified": true,
      "providerUserInfo": [
        {
          "providerId": "google.com",
          "displayName": "Tito Guidotti",
          "photoUrl": "https://lh3.googleusercontent.com/a/ACg8ocKVvafbCtCGUmXh2WFuh7TSsQQGC0VkCWLYn0fXgV_2NgQwyw=s96-c",
          "federatedId": "111790800936946378724",
          "email": "tito@20dash.com",
          "rawId": "111790800936946378724"
        }
      ],
      "validSince": "1786365155",
      "lastLoginAt": "1791290996735",
      "createdAt": "1786365155887",
      "lastRefreshAt": "2026-10-06T12:49:56.735Z"
    }
  ]
}


### 71.844 OPTIONS https://identitytoolkit.googleapis.com/v1/accounts:lookup?key=<REDACTED> -> 200
RES: 

### 72.135 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=OmFQ1NYpk97tqxboPYiQGSGo122ZM4rV2hzEMNy4e4J-65LhntFdKg&SID=RCmCBFUE9wekp6KlJRj1qQ&RID=55152&TYPE=terminate&zx=9zd30e865qx6 -> 200
RES: 

### 72.141 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=62231&CVER=22&X-HTTP-Session-Id=gsessionid&zx=t2u3ixvq0811&t=1 -> 200
REQ: headers=X-Goog-Api-Client%3Agl-js%2F%20fire%2F9.23.0%0D%0AContent-Type%3Atext%2Fplain%0D%0AX-Firebase-GMPID%3A1%3A69834214729%3Aweb%3Ac3e2769cb392ca9203de98%0D%0AAuthorization%3ABearer%20<JWT>%0D%0A&count=2&ofs=0&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FRMUYRzBXvUOCqtJdxSAiMdiEZM82%22%5D%7D%2C%22targetId%22%3A4%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers_sys%2FRMUYRzBXvUOCqtJdxSAiMdiEZM82%22%5D%7D%2C%22targetId%22%3A6%7D%7D
RES: 51
[[0,["c","QvpyrketJ3lO0IfaEgpvRQ","",8,15,30000]]]


### 72.338 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=jInTxxe_-dgqgWEFEnBA84caz3-k6l5LnnwlLjI0rF8h9WK3c-As-A&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=QvpyrketJ3lO0IfaEgpvRQ&AID=0&CI=0&TYPE=xmlhttp&zx=jevy4asmn93m&t=1 -> 200
RES: 

### 72.522 GET https://api.sowork.com/api/v1/calendar/token -> 200
RES: {"token":"<JWT>"}

### 72.522 POST https://api.sowork.com/api/v1/user/getIntercomUserHash -> 200
REQ: {"platform":"web"}
RES: {"userHash":"91fc88470dffc7b765b6a7013f3a1f5b2059d00d0c6ec5a08c146820c70b707b"}

### 72.522 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jInTxxe_-dgqgWEFEnBA84caz3-k6l5LnnwlLjI0rF8h9WK3c-As-A&SID=QvpyrketJ3lO0IfaEgpvRQ&RID=62232&AID=7&zx=4kz0oh61v8fp&t=1 -> 200
REQ: count=1&ofs=2&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2FgroupPublic%2Fv0XvoIt3uK40pdF0mQDB%22%5D%7D%2C%22targetId%22%3A8%7D%7D
RES: 8
[1,7,7]


### 72.774 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jInTxxe_-dgqgWEFEnBA84caz3-k6l5LnnwlLjI0rF8h9WK3c-As-A&SID=QvpyrketJ3lO0IfaEgpvRQ&RID=62233&AID=11&zx=idobjddjpl7w&t=1 -> 200
REQ: count=1&ofs=3&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A8%7D
RES: 9
[1,11,7]


### 72.912 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jInTxxe_-dgqgWEFEnBA84caz3-k6l5LnnwlLjI0rF8h9WK3c-As-A&SID=QvpyrketJ3lO0IfaEgpvRQ&RID=62234&AID=11&zx=ga108mm20twx&t=1 -> 200
REQ: count=5&ofs=4&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22groupJoinRequests%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22groupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22requestorUserId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A10%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22groupJoinRequests%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22pending%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22requestorUserId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A12%7D%7D&req2___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22groupJoinRequests%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22seenReqAnswer%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22booleanValue%22%3Afalse%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22requestorUserId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A14%7D%7D&req3___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22groupJoinRequests%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22groupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22pending%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A16%7D%7D&req4___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22external_group_access%22%7D%5D%2C%22where%22%3A%7B%22compositeFi
RES: 9
[1,12,7]


### 73.077 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jInTxxe_-dgqgWEFEnBA84caz3-k6l5LnnwlLjI0rF8h9WK3c-As-A&SID=QvpyrketJ3lO0IfaEgpvRQ&RID=62235&AID=23&zx=y6ncbi22maor&t=1 -> 200
REQ: count=1&ofs=9&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fgroups%2Fv0XvoIt3uK40pdF0mQDB%22%5D%7D%2C%22targetId%22%3A20%7D%7D
RES: 9
[1,23,7]


### 73.226 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jInTxxe_-dgqgWEFEnBA84caz3-k6l5LnnwlLjI0rF8h9WK3c-As-A&SID=QvpyrketJ3lO0IfaEgpvRQ&RID=62236&AID=26&zx=k5crcijz89z0&t=1 -> 200
REQ: count=6&ofs=10&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2FgroupPublic%2Fv0XvoIt3uK40pdF0mQDB%22%5D%7D%2C%22targetId%22%3A22%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2FgroupMembers_sys%2FRMUYRzBXvUOCqtJdxSAiMdiEZM82_v0XvoIt3uK40pdF0mQDB%22%5D%7D%2C%22targetId%22%3A24%7D%7D&req2___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fgroup_membermaps%2Fv0XvoIt3uK40pdF0mQDB_0%22%5D%7D%2C%22targetId%22%3A26%7D%7D&req3___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fgroup_membermaps%2Fv0XvoIt3uK40pdF0mQDB_1%22%5D%7D%2C%22targetId%22%3A28%7D%7D&req4___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fgroup_membermaps%2Fv0XvoIt3uK40pdF0mQDB_2%22%5D%7D%2C%22targetId%22%3A30%7D%7D&req5___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fgroup_membermaps%2Fv0XvoIt3uK40pdF0mQDB_3%22%5D%7D%2C%22targetId%22%3A32%7D%7D
RES: 9
[1,27,7]


### 73.334 POST https://api.sowork.com/api/v1/reports/getWeeklyReportsMetadata -> 200
REQ: {"groupId":"v0XvoIt3uK40pdF0mQDB"}
RES: {"count":0,"hasReports":false}

### 73.633 GET https://nearest-media-region.l.chime.aws/ -> 200
RES: {"region": "eu-central-1"}

### 73.633 GET https://uy7tlymu.apicdn.sanity.io/v1/data/query/production?query=*%5B_type%20%3D%3D%20%27releases%27%5D%20%7C%20order(version%20desc)%5B0%5D%20%7B%0A%20%20%20%20version%2C%0A%20%20%20%20bannerLabel%2C%0A%20%20%20%20weekOfTitle%0A%20%20%7D -> 200
RES: {"query":"*[_type == 'releases'] | order(version desc)[0] {\n    version,\n    bannerLabel,\n    weekOfTitle\n  }","result":{"version":198,"bannerLabel":"NEW: A Smarter Command Palette, Seamless Screenshare Switching, Better Analytics, and Our 200th Web Release 🎉","weekOfTitle":"2026-10-04"},"ms":2}

### 73.633 GET https://api.sowork.com/eyekia/api/v1/category/ -> 200
RES: {"categories":[{"id":"Bl4KXcUjIeJHdoNHREcO","data":{"name":"desks","rank":0,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/jPi1QFQUlSda7bznZlNtc","defaultInteraction":"desk","admin":false}},{"id":"AqbUhOZx_FwtXREtAxyd","data":{"name":"smartObjects","rank":1,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/htlhqBzQN2tdd_Ol4xcOt","defaultInteraction":null,"admin":null}},{"id":"XlY1RnbeV-jsblgs6iJV","data":{"name":"seating","rank":2,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/WUZLdYym0OxbRc0ftPGCu","defaultInteraction":null,"admin":null}},{"id":"vSqvdbaroakKNLDVXkfd","data":{"name":"officeTools","rank":3,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/Rz9t6yLsub7H7TEP36Bgu","defaultInteraction":null,"admin":null}},{"id":"H3ywIaJ4kzUsAi9peXLO","data":{"name":"surfaces","rank":4,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/zB-hdKs8tKX8FZZxT2bVi","defaultInteraction":null,"admin":null}},{"id":"iw8UQhE0Ggcbuv__ch7k","data":{"name":"dividers","rank":5,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/j1EIYtpCpa8LdmEeeHd-o","defaultInteraction":null,"admin":null}},{"id":"L3RHfHPdA3wt3dQNVtjo","data":{"name":"decor","rank":6,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/Yih10JNxYZnGbNreVPEhk","defaultInteraction":null,"admin":null}},{"id":"Dzxt39P_NqVVFgjOWxPL","data":{"name":"nature","rank":7,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/vNlhbMK6LqhtOBAOqr4kS","defaultInteraction":null,"admin":null}},{"id":"fNy6bF2-J_r6WqpbUs4T","data":{"name":"rugs","rank":8,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/FB4irJzpv0mV2cFXF-Bh9","defaultInteraction":null,"admin":null}},{"id":"swnHXPv5zWiqZgGNyguj","data":{"name":"lighting","rank":9,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/QUWCKjmF3cfefc4O2g-vz","defaultInteraction":null,"admin":null}},{"id":"TwPOubhIoMnUIREt9EpB","data":{"name":"seasonal","rank":10,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/FP0qUfO3Oh5KeWgJqCNId","defaultInteraction":null,"admin":null}},{"id":"FDmZ5Rw8tXfB6Cf5phLx","data":{"name":"adminTools","rank":11,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/Pe1iP8-uK7IbFstOrSGLD","defaultInteraction":null,"admin":true}},{"id":"mOcxd-GpPKWuLU_y3o75","data":{"name":"games","rank":12,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/z8kNRKBBkkGx_yaAjvkto","defaultInteraction":null,"admin":null}},{"id":"M-NCxaZuNulcwHH1U4eu","data":{"name":"quickSets","rank":13,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/FCDyWDckEH2BwOGxmkaE-","defaultInteraction":null,"admin":null}},{"id":"y8rThFmt-Xj1idpZb7Ib","data":{"name":"custom","rank":14,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/36RvsnctcazQuF3MRCxAd","defaultInteraction":null,"admin":null}}]}

### 73.633 GET https://api.sowork.com/api/v1/chat-v2/?groupId=v0XvoIt3uK40pdF0mQDB -> 200
RES: {"channels":[{"id":"019ff5fb-36d3-7e72-86fb-d921516249aa","group_id":"v0XvoIt3uK40pdF0mQDB","type":"direct","name":null,"description":null,"created_by":"admin","member_key":"RMUYRzBXvUOCqtJdxSAiMdiEZM82:Rxp3M2MJzra9wQJm0VkDfRSgJgt1","app_id":null,"conversation_id":null,"lifecycle":null,"meeting_started_at":null,"meeting_ended_at":null,"metadata":{"legacy":{"streamId":"!members-7FxDVbT0mo4NNzNo9B3xvoCMLI2R1jHnuqzuXiZVtY4","streamCid":"messaging:!members-7FxDVbT0mo4NNzNo9B3xvoCMLI2R1jHnuqzuXiZVtY4","streamName":"Kury","streamType":"messaging"}},"is_default":false,"is_locked":false,"auto_translate_enabled":false,"default_designated_at":null,"stale_at":null,"archived_at":null,"archived_by":null,"support_outreach_pending_at":null,"hidden":false,"last_message_at":"2026-08-12T12:38:38.550Z","created_at":"2026-08-12T12:38:29.331Z","updated_at":"2026-08-30T05:52:43.505Z","deleted_at":null,"membership":{"pinned_at":null,"unread_count":0,"unread_mention_count":0,"unread_broadcast_count":0,"notification_level":null,"muted_until":null,"role":"member","joined_at":"2026-08-12T12:38:29.338Z","via_team":false},"dm_member_user_ids":["Rxp3M2MJzra9wQJm0VkDfRSgJgt1"],"last_message":{"id":"019ff5fb-5ad6-7f25-98e1-7c3043eb9047","user_id":"Rxp3M2MJzra9wQJm0VkDfRSgJgt1","app_id":null,"type":"regular","parent_id":null,"text":"devo entrar?","created_at":"2026-08-12T12:38:38.550Z"}},{"id":"019fdd45-cafa-7ad9-be02-2fa77379428e","group_id":"v0XvoIt3uK40pdF0mQDB","type":"general","name":"general","description":null,"created_by":"zE7SCh3eZDggMBNhjB5xkpeglNn2","member_key":null,"app_id":null,"conversation_id":null,"lifecycle":null,"meeting_started_at":null,"meeting_ended_at":null,"metadata":null,"is_default":true,"is_locked":true,"auto_translate_enabled":false,"default_designated_at":"2026-08-07T17:29:26.522Z","stale_at":null,"archived_at":null,"archived_by":null,"support_outreach_pending_at":null,"hidden":false,"last_message_at":null,"created_at":"2026-08-07T17:29:26.522Z","updated_at":"2026-08-07T17:29:26.522Z","deleted_at":null,"membership":{"pinned_at":null,"unread_count":0,"unread_mention_count":0,"unread_broadcast_count":0,"notification_level":null,"muted_until":null,"role":"member","joined_at":"2026-08-10T12:32:38.504Z","via_team":false},"last_message":null}],"current_user_group_role":"contributor","has_more":false,"users":{"Rxp3M2MJzra9wQJm0VkDfRSgJgt1":{"name":"Kury","photo_url":"https://app.sowork.com/portrait/v1/avatar!SE/characterA!backAccessory:~fishingBackPack+brown!beard:~turk+black!body:~v1+typeA!bodyAccessory:~nateDrakeHolster+blonde!eyebrows:~eyebrows1+brown!eyes:~eyes1+grey!hair:~barret+black!mouth:~mouth1+typeA!pants:~bermudaLong+black!shirt:~alanShirt+blonde!shoes:~stepLuck+green?background=transparent"}},"scope_group_ids":["v0XvoIt3uK40pdF0mQDB","__global__"]}

### 73.633 GET https://api.sowork.com/api/v1/chat-v2/?groupId=__global__&actingGroupId=v0XvoIt3uK40pdF0mQDB -> 200
RES: {"channels":[],"current_user_group_role":null,"has_more":false}

### 73.633 GET https://api.sowork.com/api/v1/chat-v2/me/saved-message-ids?groupId=v0XvoIt3uK40pdF0mQDB -> 200
RES: {"ids":[]}

### 73.633 GET https://api.sowork.com/api/v1/chat-v2/me/saved-messages?groupId=v0XvoIt3uK40pdF0mQDB&limit=50 -> 200
RES: {"server_time":"2026-10-06T12:49:59.066Z","items":[],"users":{},"apps":{},"channels":{},"has_more":false,"next_cursor":null,"scope_group_ids":["v0XvoIt3uK40pdF0mQDB","__global__"]}

### 73.633 GET https://api.sowork.com/api/v1/chat-v2/support/me -> 200
RES: {"is_support_staff":false}

### 73.633 POST https://api.sowork.com/api/v1/conversation/searchDigests -> 200
REQ: {"groupId":"v0XvoIt3uK40pdF0mQDB","section":{"kind":"all"},"page":1,"pageSize":25,"readFilter":"all","grouped":true}
RES: {"apiVersion":5,"results":[],"pagination":{"page":1,"size":25,"totalCount":0}}

### 73.633 POST https://api.sowork.com/api/v1/gaia/find -> 200
REQ: {"type":"group","groupId":"v0XvoIt3uK40pdF0mQDB","alias":"mistymountainlodge","redirect":true}
RES: {"room":{"id":"YmAfBb3h2eY5r9FsAIfV","name":"Escritório principal","index":1,"local":false,"serverId":"g5","server":{"url":"https://api.sowork.com/gaia/g5"},"userIds":["zdXi8bJLG6cNbd4P87bqTAzTdTq1","JLDRGMskJRXiYcHccNVaLuwr3XJ3","RMUYRzBXvUOCqtJdxSAiMdiEZM82","rjOwBlMG93QD2sjutWoF2fc8Mv63","fCfzuTuBkBSLyF1tKdMJOZSOnUu1","G7EWVnLMQngko5v7KcgNnH06Fhg1","Rxp3M2MJzra9wQJm0VkDfRSgJgt1","blKsp8Fxi6VGJfXfizN0iu248IB2"],"userCount":8,"roomDefinitionId":"yPsvwRdkb5LnGHfXFgfO","roomDefinition":{"ownerKey":"group_v0XvoIt3uK40pdF0mQDB","name":"Misty Mountain Lodge","groupId":"v0XvoIt3uK40pdF0mQDB","zones":null,"projection":null},"isMeetingSpace":false,"createdAt":{"seconds":1786123764,"nanoseconds":340000000},"updatedAt":{"seconds":1786123764,"nanoseconds":340000000},"roomGroupId":null},"roomDefinition":{"id":"yPsvwRdkb5LnGHfXFgfO","groupId":"v0XvoIt3uK40pdF0mQDB","name":"Escritório principal","alias":"mistymountainlodge","mapDefinitionId":"V1vAAHj80gty4MwBqEFp","ownerKey":"group_v0XvoIt3uK40pdF0mQDB","primary":true,"isMeetingSpace":false,"zones":null,"createdAt":{"seconds":1786123761,"nanoseconds":850000000},"updatedAt":{"seconds":1786123761,"nanoseconds":850000000},"roomObjects":{},"roomGroupId":null,"pricingType":"premium"},"roomGroupId":null}

### 73.634 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jInTxxe_-dgqgWEFEnBA84caz3-k6l5LnnwlLjI0rF8h9WK3c-As-A&SID=QvpyrketJ3lO0IfaEgpvRQ&RID=62237&AID=46&zx=6rbt5ji60qnt&t=1 -> 200
REQ: count=1&ofs=16&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2FgroupMembers_integrations%2FRMUYRzBXvUOCqtJdxSAiMdiEZM82_v0XvoIt3uK40pdF0mQDB%22%5D%7D%2C%22targetId%22%3A34%7D%7D
RES: 9
[1,46,7]


### 73.754 POST https://api.sowork.com/api/v1/gaia/rooms -> 200
REQ: {"groupId":"v0XvoIt3uK40pdF0mQDB"}
RES: {"roomDefinitionMap":{"yPsvwRdkb5LnGHfXFgfO":{"roomDefinition":{"id":"yPsvwRdkb5LnGHfXFgfO","groupId":"v0XvoIt3uK40pdF0mQDB","name":"Escritório principal","alias":"mistymountainlodge","mapDefinitionId":"V1vAAHj80gty4MwBqEFp","ownerKey":"group_v0XvoIt3uK40pdF0mQDB","primary":true,"isMeetingSpace":false,"zones":null,"createdAt":{"seconds":1786123761,"nanoseconds":850000000},"updatedAt":{"seconds":1786123761,"nanoseconds":850000000},"roomObjects":{},"roomGroupId":null,"pricingType":"premium"},"rooms":[{"id":"YmAfBb3h2eY5r9FsAIfV","name":"Escritório principal","index":1,"local":false,"serverId":"g5","server":{"url":"https://api.sowork.com/gaia/g5"},"userIds":["zdXi8bJLG6cNbd4P87bqTAzTdTq1","JLDRGMskJRXiYcHccNVaLuwr3XJ3","RMUYRzBXvUOCqtJdxSAiMdiEZM82","rjOwBlMG93QD2sjutWoF2fc8Mv63","fCfzuTuBkBSLyF1tKdMJOZSOnUu1","G7EWVnLMQngko5v7KcgNnH06Fhg1","Rxp3M2MJzra9wQJm0VkDfRSgJgt1","blKsp8Fxi6VGJfXfizN0iu248IB2"],"userCount":8,"roomDefinitionId":"yPsvwRdkb5LnGHfXFgfO","roomDefinition":{"ownerKey":"group_v0XvoIt3uK40pdF0mQDB","name":"Misty Mountain Lodge","groupId":"v0XvoIt3uK40pdF0mQDB","zones":null,"projection":null},"isMeetingSpace":false,"createdAt":{"seconds":1786123764,"nanoseconds":340000000},"updatedAt":{"seconds":1786123764,"nanoseconds":340000000},"roomGroupId":null}]}}}

### 73.812 GET https://api.sowork.com/eyekia/api/v1/asset/?categoryId=XlY1RnbeV-jsblgs6iJV&pathOnlyUrls=true -> 200
RES: {"assets":[{"id":"AAOgxlkS2YCBmOIXCmuH","data":{"name":"* 50s Chair","description":"","userHint":"","categoryId":"XlY1RnbeV-jsblgs6iJV","thumbnail":"/AAOgxlkS2YCBmOIXCmuH/t7pvqo4bFv_zn_02Ok15d","accessKey":"PREMIUM","defaultInteractionType":null,"colors":{"baseColor":"#48ab85ff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":197.03030303030303,"saturationMultiplier":1.62,"valueMultiplier":1}],"presetColorsDictionary":{"ZEgWNNav1lsUyLEi07IN":{"hueRotationDegrees":197.03030303030303,"saturationMultiplier":1.62,"valueMultiplier":1}}},"projections":{"isometric":true,"orthographic":false},"published":true,"modifiedSinceLastPublish":false,"orientations":{"N":{"goToPivot":{"x":0,"y":0},"images":{"JIEWyAHz8zQcjxsWvju-":{"url":"/JIEWyAHz8zQcjxsWvju-/g24cgU3MxZy0FD6wprwYw","transform":{"position":{"x":0,"y":0},"scale":{"x":1,"y":1},"rotationRad":0,"frontPoint":{"x":0,"y":-116.5},"depthBand":0},"type":"image","primitives":{"BAnygGjNDaCybx1xDdxy":{"height":159,"shapeIntersectionType":"selector","shape":{"rotationRad":0,"position":{"x":99.22570991516113,"y":93.07648658752441},"shape":"rect","scale":{"y":100.5590934753418,"x":103.91320419311523}},"type":"shape"},"CNPikoGzgp4VwBHdmqB8":{"type":"seat","transform":{"frontPoint":{"x":0,"y":0},"scale":{"y":1,"x":1},"depthBand":0,"position":{"x":61,"y":-41}},"facingDir":"N","shape":{"position":{"y":5.869720458984375,"x":34.37971305847168},"shape":"rect","scale":{"x":172.11334609985352,"y":117.32969665527344},"rotationRad":0},"shapeIntersectionType":"sensor"},"4HONNstsfOab7SCUeZ-Z":{"type":"shadow","shadowType":"rect","shape":{"scale":{"x":137.45425987243652,"y":138.57228469848633},"position":{"x":99.22579669952393,"y":93.07657432556152},"rotationRad":0,"shape":"rect"}}},"blendMode":null}}},"E":{"goToPivot":{"x":0,"y":0},"images":{"EAyP_aViWNPEXesHwc1Y":{"url":"/EAyP_aViWNPEXesHwc1Y/HzQMGpGD5uvB3SwePxJvd","transform":{"position":{"x":0,"y":0},"scale":{"x":1,"y":1},"rotationRad":0,"frontPoint":{"x":0,"y":-116.5},"depthBand":0},"type":"image","primitives":{"gww_hBULRsxDJ7yh5GR9":{"type":"shape","height":163,"shape":{"rotationRad":0,"shape":"rect","position":{"x":128.57413864135742,"y":121.86593246459961},"scale":{"x":103.35419464111328,"y":105.59024810791016}},"shapeIntersectionType":"selector"},"UwZyvR5y8Tc5QarMwFs5":{"transform":{"position":{"x":80,"y":-127},"frontPoint":{"y":0,"x":0},"scale":{"y":1,"x":1},"depthBand":0},"shapeIntersectionType":"sensor","type":"seat","facingDir":"E","shape":{"scale":{"x":105.59023666381836,"y":177.1443862915039},"position":{"x":10.062410354614258,"y":26.832881927490234},"shape":"rect","rotationRad":0}},"HCezKpgXdCgyjXBOfKdm":{"shape":{"scale":{"y":143.60348510742188,"x":145.83956146240234},"rotationRad":0,"position":{"y":118.51179504394531,"x":127.45613479614258},"shape":"rect"},"type":"shadow","shadowType":"rect"}},"blendMode":null}}},"S":{"goToPivot":{"x":0,"y":0},"images":{"f0oncLsegpXMbHrpvU5z":{"url":"/f0oncLsegpXMbHrpvU5z/-y0mzVLwE1D229BPJFjvm","transform":{"position":{"x":0,"y":0},"scale":{"x":1,"y":1},"rotationRad":0,"frontPoint":{"x":0,"y":-134},"depthBand":0},"type":"image","primitives":{"mFmvxI4W8kzrKGWAJ9wm":{"shadowType":"rect","shape":{"shape":"rect","rotationRad":0,"position":{"x":120.74788475036621,"y":131.92825317382812},"scale":{"y":148.07562255859375,"x":145.83955764770508}},"type":"shadow"},"ggivqcoKvA60d6a8wObS":{"type":"seat","transform":{"depthBand":0,"scale":{"y":1,"x":1},"position":{"x":-69,"y":-121},"frontPoint":{"x":0,"y":0}},"shape":{"shape":"rect","position":{"y":22.640243530273438,"x":20.963191986083984},"scale":{"x":184.41168975830078,"y":116.21163940429688},"rotationRad":0},"facingDir":"S","shapeIntersectionType":"sensor"},"s2KKWVqh7-2LRIP1J800":{"shapeIntersectionType":"selector","type":"shape","shape":{"scale":{"x":94.40982818603516,"y":96.6458969116211},"position":{"y":124.10199356079102,"x":117.39385604858398},"shape":"rect","rotationRad":0},"height":195}},"blendMode":null}}},"W":{"goToPivot":{"x":0,"y":0},"images":{"SWZ5vxNL9aBD7K2K4xDf":{"url":"/SWZ5vxNL9aBD7K2K4xDf/65u9U7KC6-h0yPubQMSfJ","transform":{"position":{"x":0,"y":0},"scale":{"x":1,"y":1},"rotationRad":0,"frontPoint":{"x":0,"y":-134},"depthBand":0},"type":"image","primitives":{"JZlyPxQ-Jh33zXz4GLbG":{"shapeIntersectionType":"selector","type":"shape","shape":{"scale":{"y":91.61474609375,"x":94.9688491821289},"position":{"x":94.19456100463867,"y":100.34378051757812},"shape":"rect","rotationRad":0},"height":160},"upcNeVnkWiS6Pxat00zs":{"shapeIntersectionType":"sensor","transform":{"frontPoint":{"y":0,"x":0},"position":{"y":-25,"x":-73},"depthBand":0,"scale":{"y":1,"x":1}},"facingDir":"W","type":"seat","shape":{"scale":{"x":89.93785572052002,"y":179.3805389404297},"position":{"x":35.21812963485718,"y":30.745994567871094},"rotationRad":0,"shape":"rect"}},"gC1448JY836T9fgcgBuv":{"shape":{"shape":"rect","rotationRad":0,"scale":{"y":144.16244888305664,"x":151.98867797851562},"position":{"x":91.39950561523438,"y":99.78472709655762}},"shadowType":"rect","type":"shadow"}},"blendMode":null}}}},"config":{"frames":{}}}},{"id":"P8H50lH2QwM6eXwUMwQP","data":{"name":"* 50s Chair Copy","description":"","userHint":"","categoryId":"XlY1RnbeV-jsblgs6iJV","thumbnail":"/AAOgxlkS2YCBmOIXCmuH/t7pvqo4bFv_zn_02Ok15d","accessKey":"PREMIUM","defaultInteractionType":null,"colors":{"baseColor":"#48ab85ff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":197.03030303030303,"saturationMultiplier":1.62,"valueMultiplier":1}],"presetColorsDictionary":{"ZEgWNNav1lsUyLEi07IN":{"hueRotationDegrees":197.03030303030303,"saturationMultiplier":1.62,"valueMultiplier":1}}},"projections":{"isometric":true,"orthographic":false},"published":true,"modifiedSinceLastPublish":true,"orientations":{"N":{"goToPivot":null,"images":{"JIEWyAHz8zQcjxsWvju-":{"url":"/JIEWyAHz8zQcjxsWvju-/g24cgU3MxZy0FD6wprwYw","transform":{"position":{"x":0,"y":0},"scale":{"x":1,"y":1},"rotationRad":0,"frontPoint":{"x":0,"y":-116.5},"depthBand":0},"type":"image","primitives":{"B

### 73.831 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jInTxxe_-dgqgWEFEnBA84caz3-k6l5LnnwlLjI0rF8h9WK3c-As-A&SID=QvpyrketJ3lO0IfaEgpvRQ&RID=62238&AID=47&zx=wqx7is6btbmi&t=1 -> 200
REQ: count=15&ofs=17&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fgroup_desks%2Fv0XvoIt3uK40pdF0mQDB%22%5D%7D%2C%22targetId%22%3A36%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22digestTemplates%22%7D%5D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fgroups%2Fv0XvoIt3uK40pdF0mQDB%22%7D%2C%22targetId%22%3A38%7D%7D&req2___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FRMUYRzBXvUOCqtJdxSAiMdiEZM82%2Fprivate%2FdigestReadStates%22%5D%7D%2C%22targetId%22%3A40%7D%7D&req3___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22notifications%22%7D%5D%2C%22where%22%3A%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22removed%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22booleanValue%22%3Afalse%7D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22createdAt%22%7D%2C%22direction%22%3A%22DESCENDING%22%7D%2C%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22DESCENDING%22%7D%5D%2C%22limit%22%3A50%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers_sys%2FRMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%2C%22targetId%22%3A42%7D%7D&req4___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22strong_connections%22%7D%5D%2C%22where%22%3A%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22userId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%2C%22limit%22%3A1%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A44%7D%7D&req5___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fgroup_rooms_state%2Fv0XvoIt3uK40pdF0mQDB%22%5D%7D%2C%22targetId%22%3A46%7D%7D&req6___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22group_announce%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22sc%22%7D%2C%22op%22%3A%22ARRAY_CONTAINS_ANY%22%2C%22value%22%3A%7B%22arrayValue%22%3A%7B%22values%22%3A%5B%7B%22stringValue%22%3A%22user%3ARMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%2C%7B%22stringValue%22%3A%22group%3Av0XvoIt3uK40pdF0mQDB%3Acontributor%22%7D%5D%7D%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22targetGroupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22expiresAt%22%7D%2C%22op%22%3A%22GREATER_THAN%22%2C%22value%22%3A%7B%22timestampValue%22%3A%222026-10-06T12%3A49%3A58.428000000Z%22%7D%7D%7D%5D
RES: 9
[1,50,7]


### 73.856 POST https://identitytoolkit.googleapis.com/v1/accounts:signInWithCustomToken?key=<REDACTED> -> 200
REQ: {"token":"<JWT>","returnSecureToken":true}
RES: {
  "kind": "identitytoolkit#VerifyCustomTokenResponse",
  "idToken": "<JWT>",
  "refreshToken": "AMf-vBwbbgsmcIA_8n4FQ7hl1s_7no08rNDENP0xPaZ4Aeene_kC-U1Ziqn5P6QDhTDfRyeUrQ_ZkczRgt3KuplGhDCqkBbp47ilXMTNzjOIyCodcJNz8yb2WCuaDn7HWCnjRGY4-tuJ_c0371F9TGBPyqhYL3JklLUdS3UH6fwLCvohEfIDKFkTk9V5zE1REYYfkBucba-TcOcjRLLaqNP9Ei-TlMWGcIWzNx-REGNZWxBMu3KEuAQ",
  "expiresIn": "3600",
  "isNewUser": false
}


### 73.857 OPTIONS https://identitytoolkit.googleapis.com/v1/accounts:signInWithCustomToken?key=<REDACTED> -> 200
RES: 

### 74.518 POST https://api.sowork.com/api/v1/user/updateUiSettings -> 200
REQ: {"userId":"RMUYRzBXvUOCqtJdxSAiMdiEZM82","data":{"mutedSounds":{"go":false,"lose":false,"memo":true,"doorForceOpen":false,"chatNotification":false,"jump":false,"duck_hitter":false,"countdown":false,"cat_hitter":false,"rotateAsset":false,"coworkingEnd1":true,"removeNotification":false,"applause":false,"shoot_2":false,"speechHigh":false,"win":false,"deleteAsset":false,"battleMusic":false,"speechMid":false,"participantJoined":false,"receiveWave":false,"codeSuccess":false,"requestToJoinPm":false,"sendWave":false,"joinCall":true,"coworkingStart2":true,"pencilLine":false,"notification":false,"ring":false,"hit":false,"coworkingEnd2":true,"polaroid":false,"disableVideo":false,"coworkingStart1":true,"calendarNotification":true,"gong":false,"doorKnock":false,"unmuteMic":false,"coworkingEnd3":true,"shoot_1":false,"equip":false,"shark_hitter":false,"productUnlocked":false,"muteMic":false,"raiseHand":false,"characterPicker":false,"placeAsset":false,"privateMeeting":false,"speechLow":false,"editAsset":false,"mapMakerMusic":true,"enableVideo":false,"selectAsset":false,"textBroadcast":false,"coworkingStart3":true,"leaveCall":true}}}
RES: {"success":true}

### 74.848 POST https://identitytoolkit.googleapis.com/v1/accounts:lookup?key=<REDACTED> -> 200
REQ: {"idToken":"<JWT>"}
RES: {
  "kind": "identitytoolkit#GetAccountInfoResponse",
  "users": [
    {
      "localId": "RMUYRzBXvUOCqtJdxSAiMdiEZM82",
      "validSince": "1786365157",
      "lastLoginAt": "1791290999067",
      "createdAt": "1786365157281",
      "customAuth": true,
      "lastRefreshAt": "2026-10-06T12:49:59.067Z"
    }
  ]
}


### 74.848 OPTIONS https://identitytoolkit.googleapis.com/v1/accounts:lookup?key=<REDACTED> -> 200
RES: 

### 75.099 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jInTxxe_-dgqgWEFEnBA84caz3-k6l5LnnwlLjI0rF8h9WK3c-As-A&SID=QvpyrketJ3lO0IfaEgpvRQ&RID=62239&AID=100&zx=iji2ow9v6ad9&t=1 -> 200
REQ: count=1&ofs=32&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FrjOwBlMG93QD2sjutWoF2fc8Mv63%22%5D%7D%2C%22targetId%22%3A66%7D%7D
RES: 10
[1,100,7]


### 75.248 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsophya-calendar-production%2Fdatabases%2F(default)&RID=65088&CVER=22&X-HTTP-Session-Id=gsessionid&zx=g5yalyjqwfe0&t=1 -> 200
REQ: headers=X-Goog-Api-Client%3Agl-js%2F%20fire%2F9.23.0%0D%0AContent-Type%3Atext%2Fplain%0D%0AX-Firebase-GMPID%3A1%3A537328756772%3Aweb%3Ad487ca7b2e66572b9510b1%0D%0AAuthorization%3ABearer%20<JWT>%0D%0A&count=2&ofs=0&req0___data__=%7B%22database%22%3A%22projects%2Fsophya-calendar-production%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22events%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22userId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22start%22%7D%2C%22op%22%3A%22GREATER_THAN_OR_EQUAL%22%2C%22value%22%3A%7B%22timestampValue%22%3A%222026-09-29T12%3A50%3A00.208000000Z%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22start%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%2C%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsophya-calendar-production%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A2%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsophya-calendar-production%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22accounts%22%7D%5D%2C%22where%22%3A%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22userId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsophya-calendar-production%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A4%7D%7D
RES: 51
[[0,["c","M7w4eKqOZEDny8WbcZBKXw","",8,15,30000]]]


### 75.259 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jInTxxe_-dgqgWEFEnBA84caz3-k6l5LnnwlLjI0rF8h9WK3c-As-A&SID=QvpyrketJ3lO0IfaEgpvRQ&RID=62240&AID=103&zx=moookablddil&t=1 -> 200
REQ: count=10&ofs=33&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FJLDRGMskJRXiYcHccNVaLuwr3XJ3%22%5D%7D%2C%22targetId%22%3A68%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FG7EWVnLMQngko5v7KcgNnH06Fhg1%22%5D%7D%2C%22targetId%22%3A70%7D%7D&req2___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FblKsp8Fxi6VGJfXfizN0iu248IB2%22%5D%7D%2C%22targetId%22%3A72%7D%7D&req3___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FRxp3M2MJzra9wQJm0VkDfRSgJgt1%22%5D%7D%2C%22targetId%22%3A74%7D%7D&req4___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FzdXi8bJLG6cNbd4P87bqTAzTdTq1%22%5D%7D%2C%22targetId%22%3A76%7D%7D&req5___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FFLBKjFg1PYe5bD6cE0bdR7BhE7r1%22%5D%7D%2C%22targetId%22%3A78%7D%7D&req6___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FEsFlL0qd1kZNo0IsotREJWQWNeC3%22%5D%7D%2C%22targetId%22%3A80%7D%7D&req7___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22conversations%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22ownerGroupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomDefinitionId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22yPsvwRdkb5LnGHfXFgfO%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22active%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22op%22%3A%22GREATER_THAN%22%2C%22value%22%3A%7B%22integerValue%22%3A%220%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%2C%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A82%7D%7D&req8___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22conversations%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22ownerGroupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF
RES: 10
[1,104,7]


### 75.390 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=b5x-EdtPARVaSdrpwaTJ4AfmUkKTC2qxFBc4ZNwkGx-0wFVlFrg8Pw&VER=8&database=projects%2Fsophya-calendar-production%2Fdatabases%2F(default)&RID=rpc&SID=M7w4eKqOZEDny8WbcZBKXw&AID=0&CI=0&TYPE=xmlhttp&zx=kzqop1uj55m&t=1 -> 200
RES: 

### 75.489 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jInTxxe_-dgqgWEFEnBA84caz3-k6l5LnnwlLjI0rF8h9WK3c-As-A&SID=QvpyrketJ3lO0IfaEgpvRQ&RID=62241&AID=135&zx=fausrnf73r4&t=1 -> 200
REQ: count=1&ofs=43&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A80%7D
RES: 10
[1,135,7]


### 75.675 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=b5x-EdtPARVaSdrpwaTJ4AfmUkKTC2qxFBc4ZNwkGx-0wFVlFrg8Pw&VER=8&database=projects%2Fsophya-calendar-production%2Fdatabases%2F(default)&RID=rpc&SID=M7w4eKqOZEDny8WbcZBKXw&AID=0&CI=1&TYPE=xmlhttp&zx=xmwyuzt71x0a&t=1 -> 200
RES: 

### 76.216 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=b5x-EdtPARVaSdrpwaTJ4AfmUkKTC2qxFBc4ZNwkGx-0wFVlFrg8Pw&VER=8&database=projects%2Fsophya-calendar-production%2Fdatabases%2F(default)&RID=rpc&SID=M7w4eKqOZEDny8WbcZBKXw&AID=6&CI=1&TYPE=xmlhttp&zx=hxricsd7w2i3&t=1 -> 200
RES: 

### 78.239 POST https://api.sowork.com/api/v1/gaia/listRoomByRoomDefinition -> 200
REQ: {"roomDefinitionId":"yPsvwRdkb5LnGHfXFgfO"}
RES: {"rooms":[{"id":"YmAfBb3h2eY5r9FsAIfV","name":"Escritório principal","index":1,"local":false,"serverId":"g5","server":{"url":"https://api.sowork.com/gaia/g5"},"userIds":["zdXi8bJLG6cNbd4P87bqTAzTdTq1","JLDRGMskJRXiYcHccNVaLuwr3XJ3","RMUYRzBXvUOCqtJdxSAiMdiEZM82","rjOwBlMG93QD2sjutWoF2fc8Mv63","fCfzuTuBkBSLyF1tKdMJOZSOnUu1","G7EWVnLMQngko5v7KcgNnH06Fhg1","Rxp3M2MJzra9wQJm0VkDfRSgJgt1","blKsp8Fxi6VGJfXfizN0iu248IB2"],"userCount":8,"roomDefinitionId":"yPsvwRdkb5LnGHfXFgfO","roomDefinition":{"ownerKey":"group_v0XvoIt3uK40pdF0mQDB","name":"Misty Mountain Lodge","groupId":"v0XvoIt3uK40pdF0mQDB","zones":null,"projection":null},"isMeetingSpace":false,"createdAt":{"seconds":1786123764,"nanoseconds":340000000},"updatedAt":{"seconds":1786123764,"nanoseconds":340000000},"roomGroupId":null}]}

### 78.239 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jInTxxe_-dgqgWEFEnBA84caz3-k6l5LnnwlLjI0rF8h9WK3c-As-A&SID=QvpyrketJ3lO0IfaEgpvRQ&RID=62242&AID=136&zx=v9v431u0emum&t=1 -> 200
REQ: count=1&ofs=44&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22broadcasts%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22started%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22groupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22unaryFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomDefinitionId%22%7D%2C%22op%22%3A%22IS_NULL%22%7D%7D%2C%7B%22unaryFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomId%22%7D%2C%22op%22%3A%22IS_NULL%22%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%2C%22limit%22%3A1%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A88%7D%7D
RES: 10
[1,151,7]


### 78.377 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jInTxxe_-dgqgWEFEnBA84caz3-k6l5LnnwlLjI0rF8h9WK3c-As-A&SID=QvpyrketJ3lO0IfaEgpvRQ&RID=62243&AID=152&zx=6aokn2wtwyfy&t=1 -> 200
REQ: count=5&ofs=45&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22broadcasts%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22started%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22groupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomDefinitionId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22yPsvwRdkb5LnGHfXFgfO%22%7D%7D%7D%2C%7B%22unaryFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomId%22%7D%2C%22op%22%3A%22IS_NULL%22%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%2C%22limit%22%3A1%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A90%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22broadcasts%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22started%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22groupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomDefinitionId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22yPsvwRdkb5LnGHfXFgfO%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22YmAfBb3h2eY5r9FsAIfV%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%2C%22limit%22%3A1%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A92%7D%7D&req2___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22doorbells%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22groupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22requestorUserId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A94%7D%7D&req3___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22doorbells%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22groupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3
RES: 10
[1,154,7]


### 78.579 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jInTxxe_-dgqgWEFEnBA84caz3-k6l5LnnwlLjI0rF8h9WK3c-As-A&SID=QvpyrketJ3lO0IfaEgpvRQ&RID=62244&AID=163&zx=2498xc53fvpo&t=1 -> 200
REQ: count=1&ofs=50&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A64%7D
RES: 10
[1,165,7]


### 79.270 GET https://app.sowork.com/eyekia/api/v1/floor/E5xP37vJSqXDYogbZtF2?pathOnlyUrls=true&v=1gq9po2s3 -> 200
RES: {"orientations":{"N":{"url":"/floor/E5xP37vJSqXDYogbZtF2/5Rf9WTghHwr3FP3XGfn9v","pivotPosition":{"x":110,"y":67}},"E":{"url":"/floor/E5xP37vJSqXDYogbZtF2/em2Itlqzq1GQG0RpIwomy","pivotPosition":{"x":110,"y":67}}},"type":"floor","height":"","pixelDensity":211,"isCorner":false,"colors":{"baseColor":"#9ba27eff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"","name":"Moss","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.270 GET https://app.sowork.com/eyekia/api/v1/floor/QWaiSda3qH5Rf1OHQlQG?pathOnlyUrls=true&v=1gq9po48n -> 200
RES: {"orientations":{"N":{"url":"/floor/QWaiSda3qH5Rf1OHQlQG/JvA6ecfq7_W8q2BOcTDJm","pivotPosition":{"x":110,"y":67}},"E":{"url":"/floor/QWaiSda3qH5Rf1OHQlQG/rIH4zNHUNtdSWR9yHAGc6","pivotPosition":{"x":110,"y":67}},"S":{"url":"/floor/QWaiSda3qH5Rf1OHQlQG/sbGhUCZKgLZ2vaUszo12y","pivotPosition":{"x":110,"y":67}},"W":{"url":"/floor/QWaiSda3qH5Rf1OHQlQG/t41ps6h8YvGk48DmkspOo","pivotPosition":{"x":110,"y":67}}},"type":"floor","height":"","pixelDensity":211,"isCorner":false,"colors":{"baseColor":"#979176ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"","name":"Moss to Dirt Straight","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.270 GET https://app.sowork.com/eyekia/api/v1/floor/ivRu7ugLZtuj99UGC0ZS?pathOnlyUrls=true&v=1gq9po6ji -> 200
RES: {"orientations":{"N":{"url":"/floor/ivRu7ugLZtuj99UGC0ZS/Wj8RnNShh-bdvtHOqS7hr","pivotPosition":{"x":110,"y":67}},"E":{"url":"/floor/ivRu7ugLZtuj99UGC0ZS/qPiKqHhdJLrTcSYlPjMY2","pivotPosition":{"x":110,"y":67}},"S":{"url":"/floor/ivRu7ugLZtuj99UGC0ZS/qP_BZy_BxZIW826qNriHE","pivotPosition":{"x":110,"y":67}},"W":{"url":"/floor/ivRu7ugLZtuj99UGC0ZS/sOfnrckCyWBaePBOuEdnD","pivotPosition":{"x":110,"y":67}}},"type":"floor","height":"","pixelDensity":211,"isCorner":false,"colors":{"baseColor":"#9a9d7bff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"","name":"Moss to Dirt Inside Corner","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.270 GET https://app.sowork.com/eyekia/api/v1/floor/fO3Bcpb1hQA3xqaqAu3w?pathOnlyUrls=true -> 404
RES: {"statusCode":404,"code":"not-found","error":"Not Found","message":"floor fO3Bcpb1hQA3xqaqAu3w could not be found."}

### 79.270 GET https://app.sowork.com/eyekia/api/v1/floor/V3CVBBA0KMmSmxpdClc2?pathOnlyUrls=true&v=1gq9po4qn -> 200
RES: {"orientations":{"N":{"url":"/floor/V3CVBBA0KMmSmxpdClc2/t_5uNwucQSlvWpdw53t_h","pivotPosition":{"x":110,"y":67}},"E":{"url":"/floor/V3CVBBA0KMmSmxpdClc2/xQbh6MQyqF9jYF2VWZtST","pivotPosition":{"x":110,"y":67}},"S":{"url":"/floor/V3CVBBA0KMmSmxpdClc2/c9WzbPYyllPnX31jzArO1","pivotPosition":{"x":110,"y":67}},"W":{"url":"/floor/V3CVBBA0KMmSmxpdClc2/vqO2n0OpDt8nAVjt8QA9v","pivotPosition":{"x":110,"y":67}}},"type":"floor","height":"","pixelDensity":211,"isCorner":false,"colors":{"baseColor":"#9ca17eff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"","name":"Moss Rounded Corder","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.270 GET https://app.sowork.com/eyekia/api/v1/floor/GTZ5vz4MCBe2Fi7dhxZN?pathOnlyUrls=true&v=1im2eo0aj -> 200
RES: {"orientations":{"N":{"url":"/floor/GTZ5vz4MCBe2Fi7dhxZN/1iKVhgx48n3iIujn_W3MX","pivotPosition":{"x":220,"y":134}},"S":{"url":"/floor/GTZ5vz4MCBe2Fi7dhxZN/QJWQ86Cafk7OidMLFHpIH","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#bb8e53ff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":-1.03846153846154,"saturationMultiplier":0.5,"valueMultiplier":1.1},{"hueRotationDegrees":0,"saturationMultiplier":1.2,"valueMultiplier":0.8},{"hueRotationDegrees":-6.03846153846154,"saturationMultiplier":0.86,"valueMultiplier":1}],"presetColorsDictionary":{"qhka-msVVkfbC907_eS8":{"hueRotationDegrees":0,"saturationMultiplier":1.2,"valueMultiplier":0.8},"yf8jBuzfbt47SD8WakwE":{"hueRotationDegrees":-6.03846153846154,"saturationMultiplier":0.86,"valueMultiplier":1},"kA-rWLO5lWnUTR1vruaQ":{"hueRotationDegrees":-1.03846153846154,"saturationMultiplier":0.5,"valueMultiplier":1.1}}},"accessKey":"","name":"Wood Floor06","description":"SM_2m_Floor_Wood_06a","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.270 GET https://app.sowork.com/eyekia/api/v1/wall/3ccOJmgDjXL7px0PJii5?pathOnlyUrls=true&v=1iicb87nl -> 200
RES: {"orientations":{"N":{"url":"/wall/3ccOJmgDjXL7px0PJii5/KPTLLOR1X6tcmsNxuYKVI","pivotPosition":{"x":256,"y":134}},"E":{"url":"/wall/3ccOJmgDjXL7px0PJii5/-mTzMWVtXWDk8HsoY2FvW","pivotPosition":{"x":256,"y":134}},"S":{"url":"/wall/3ccOJmgDjXL7px0PJii5/YH1f30bAZ0cOJCH_XNmPs","pivotPosition":{"x":14,"y":134}},"W":{"url":"/wall/3ccOJmgDjXL7px0PJii5/5nIe7NfR9obl98kPFf3jD","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"1M","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#815221ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Cabin Wood Wall012 Tiny","description":"Cabin Wood 01 Wall 1m","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.270 GET https://app.sowork.com/eyekia/api/v1/wall/QxUlJJdYyM4obrTSLWVc?pathOnlyUrls=true&v=1iicb8hgp -> 200
RES: {"orientations":{"N":{"url":"/wall/QxUlJJdYyM4obrTSLWVc/8NUgvPi1_QLSIDPRc8QRD","pivotPosition":{"x":256,"y":134}},"W":{"url":"/wall/QxUlJJdYyM4obrTSLWVc/iqJaJWHMv5jg5OhWUgZF5","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"6m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#77828eff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Cabin Glass Wall015 Tall A Far Left","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.271 GET https://app.sowork.com/eyekia/api/v1/wall/NZoKxve6xpCxDlSrwo2G?pathOnlyUrls=true&v=1iicb8fe1 -> 200
RES: {"orientations":{"N":{"url":"/wall/NZoKxve6xpCxDlSrwo2G/XztMOagUjW3lEpAUoEOMt","pivotPosition":{"x":256,"y":134}},"W":{"url":"/wall/NZoKxve6xpCxDlSrwo2G/tWESSx3ydyfbu4T8WXi1m","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"6m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#798490ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Cabin Glass Wall015 Tall A Middle Left","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.271 GET https://app.sowork.com/eyekia/api/v1/wall/KV9ygsHV6rAMFPEZjON6?pathOnlyUrls=true&v=1iicb8drh -> 200
RES: {"orientations":{"N":{"url":"/wall/KV9ygsHV6rAMFPEZjON6/UDxysVm3ZmNJoI-x_S0cJ","pivotPosition":{"x":256,"y":134}},"W":{"url":"/wall/KV9ygsHV6rAMFPEZjON6/-hvSkoZSVEPtxwcLfUtVS","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"6m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#808d99ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Cabin Glass Wall015 Tall A Center","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.271 GET https://app.sowork.com/eyekia/api/v1/wall/svaiH50qMbUeFZcNAhGW?pathOnlyUrls=true&v=1iicb8t1r -> 200
RES: {"orientations":{"N":{"url":"/wall/svaiH50qMbUeFZcNAhGW/KKvmODBTnGXLTws0PZGau","pivotPosition":{"x":256,"y":134}},"W":{"url":"/wall/svaiH50qMbUeFZcNAhGW/3Ifis8wLABGsuLvbCcwce","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"6m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#798692ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Cabin Glass Wall015 Tall A Middle Right","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.271 GET https://app.sowork.com/eyekia/api/v1/wall/SyPKTxVfB6hexSmSuQQK?pathOnlyUrls=true&v=1iicb8j1d -> 200
RES: {"orientations":{"N":{"url":"/wall/SyPKTxVfB6hexSmSuQQK/GpPh9-UJKx8fCy_jnUpUC","pivotPosition":{"x":256,"y":134}},"W":{"url":"/wall/SyPKTxVfB6hexSmSuQQK/I1vqLP2RUqQCwTtmNsGt3","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"6m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#778490ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Cabin Glass Wall015 Tall A Far Right","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.271 GET https://app.sowork.com/eyekia/api/v1/wall/13AVKZ5OqX8hlpq5PEhZ?pathOnlyUrls=true&v=1iicb85u1 -> 200
RES: {"orientations":{"N":{"url":"/wall/13AVKZ5OqX8hlpq5PEhZ/bzTM2xn_44OfoNVmIKwRZ","pivotPosition":{"x":256,"y":134}},"E":{"url":"/wall/13AVKZ5OqX8hlpq5PEhZ/nZLRrUgDfvy5xH7J4F9dD","pivotPosition":{"x":256,"y":134}},"S":{"url":"/wall/13AVKZ5OqX8hlpq5PEhZ/ak6KtutT9280Hf9XrsLRl","pivotPosition":{"x":14,"y":134}},"W":{"url":"/wall/13AVKZ5OqX8hlpq5PEhZ/Bf6TnIwkhi4g-XIBSEd9r","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"6m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#8e581eff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Cabin Wood Wall015 Tall A","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.271 GET https://app.sowork.com/eyekia/api/v1/wall/nud1cv7lst8S5QjRM4jS?pathOnlyUrls=true&v=1ins2on0i -> 200
RES: {"orientations":{"N":{"url":"/wall/nud1cv7lst8S5QjRM4jS/CodsBPGcQBuXvd-fN18M8","pivotPosition":{"x":256,"y":134}},"E":{"url":"/wall/nud1cv7lst8S5QjRM4jS/HE5u3eqlmYotdQ8HjMGlI","pivotPosition":{"x":256,"y":134}},"S":{"url":"/wall/nud1cv7lst8S5QjRM4jS/zrdbylp02CxGFuWa2cS9p","pivotPosition":{"x":14,"y":134}},"W":{"url":"/wall/nud1cv7lst8S5QjRM4jS/aA2SrZaN_8BY1u19Tu1Ys","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"0.15m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#FFFFFFFF","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":137,"saturationMultiplier":2.5,"valueMultiplier":4}],"presetColorsDictionary":{"0LwYFoXUKIUtCxDC_1II":{"hueRotationDegrees":137,"saturationMultiplier":2.5,"valueMultiplier":4}}},"accessKey":"","name":"Glass Wall011 Border","description":"Glass 01 Wall Border","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.271 GET https://app.sowork.com/eyekia/api/v1/wall/qLEkw2xDZXDrzL5lAAzB?pathOnlyUrls=true&v=1iicb8rk2 -> 200
RES: {"orientations":{"N":{"url":"/wall/qLEkw2xDZXDrzL5lAAzB/GRBCLtuqkYesa_laGQXsW","pivotPosition":{"x":256,"y":134}},"E":{"url":"/wall/qLEkw2xDZXDrzL5lAAzB/lLPuqAiaFqxo7GrWrMyfC","pivotPosition":{"x":256,"y":134}},"S":{"url":"/wall/qLEkw2xDZXDrzL5lAAzB/MDkPSiRgdEJf4fqV6DU0S","pivotPosition":{"x":14,"y":134}},"W":{"url":"/wall/qLEkw2xDZXDrzL5lAAzB/-lSxLv4pFNKOdasefIeIv","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"6m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#826340ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Cabin Wood Wall015 Tall Window A","description":"Cabin Wood 01 Wall Window A 6m","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.271 GET https://app.sowork.com/eyekia/api/v1/wall/rjPVjv9zS5DPkJ1nDGpw?pathOnlyUrls=true&v=1iicb8s89 -> 200
RES: {"orientations":{"N":{"url":"/wall/rjPVjv9zS5DPkJ1nDGpw/tPWWkcA4uVQMk-R2XWSoY","pivotPosition":{"x":256,"y":134}},"E":{"url":"/wall/rjPVjv9zS5DPkJ1nDGpw/gqs_5yXSrkHfQkkAOakAL","pivotPosition":{"x":256,"y":134}},"S":{"url":"/wall/rjPVjv9zS5DPkJ1nDGpw/lFVdnFeItI-sNBTnylCfP","pivotPosition":{"x":14,"y":134}},"W":{"url":"/wall/rjPVjv9zS5DPkJ1nDGpw/EGYKJJIH9VtWKDI65q-OB","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"6m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#7a8692ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Cabin Glass Wall015 Tall A","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.271 GET https://app.sowork.com/eyekia/api/v1/wall/gYc0iQJU4fsSOzoaslpS?pathOnlyUrls=true&v=1hr6lkpsv -> 200
RES: {"orientations":{"N":{"url":"/wall/gYc0iQJU4fsSOzoaslpS/6HUqdsBhk2n-nx-sC8ZDN","pivotPosition":{"x":220,"y":134}},"E":{"url":"/wall/gYc0iQJU4fsSOzoaslpS/KyJ4GVqdJXimL0Capoqm9","pivotPosition":{"x":220,"y":134}},"S":{"url":"/wall/gYc0iQJU4fsSOzoaslpS/8CC9V1cJ6sUJszU2dpPyD","pivotPosition":{"x":220,"y":134}},"W":{"url":"/wall/gYc0iQJU4fsSOzoaslpS/ACb33ZMz7gKmSXA4hDS37","pivotPosition":{"x":220,"y":134}}},"type":"wall","height":"0m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#e4d46aff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":107.8688524590164,"saturationMultiplier":0.88,"valueMultiplier":0.1},{"hueRotationDegrees":161.8688524590164,"saturationMultiplier":0.48,"valueMultiplier":0.28},{"hueRotationDegrees":24.868852459016395,"saturationMultiplier":0,"valueMultiplier":0},{"hueRotationDegrees":-52.131147540983605,"saturationMultiplier":0,"valueMultiplier":1.25},{"hueRotationDegrees":181.8688524590164,"saturationMultiplier":0.52,"valueMultiplier":0.15},{"hueRotationDegrees":-33.131147540983605,"saturationMultiplier":0.66,"valueMultiplier":0.38}],"presetColorsDictionary":{"L5vSSoXWCeUJ7WkQ_sWE":{"hueRotationDegrees":-52.131147540983605,"saturationMultiplier":0,"valueMultiplier":1.25},"LrGyks9sRDTblR65Agbb":{"hueRotationDegrees":-33.131147540983605,"saturationMultiplier":0.66,"valueMultiplier":0.38},"BL3hzlyUEpDUyxZBIHGw":{"hueRotationDegrees":161.8688524590164,"saturationMultiplier":0.48,"valueMultiplier":0.28},"eEXJq3eGv-plKEzesQbU":{"hueRotationDegrees":181.8688524590164,"saturationMultiplier":0.52,"valueMultiplier":0.15},"0wZ-vkaguwIN2luDP219":{"hueRotationDegrees":107.8688524590164,"saturationMultiplier":0.88,"valueMultiplier":0.1},"D2EduBBq_wYB2QUyt4xf":{"hueRotationDegrees":24.868852459016395,"saturationMultiplier":0,"valueMultiplier":0}}},"accessKey":"","name":"Plaster Wall011 Border NoCollider","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.271 GET https://app.sowork.com/eyekia/api/v1/floor/8P0BHQn4nWoOB0bnHH2w?pathOnlyUrls=true&v=1han79cve -> 200
RES: {"orientations":{"N":{"url":"/floor/8P0BHQn4nWoOB0bnHH2w/01xUn7ESvKbepxZPGq68b","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#6fbe48ff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":-48.16949152542374,"saturationMultiplier":1,"valueMultiplier":1}],"presetColorsDictionary":{"FDu5VKJW1n8mexU_rvU4":{"hueRotationDegrees":-48.16949152542374,"saturationMultiplier":1,"valueMultiplier":1}}},"accessKey":"","name":"GrassStone Terrain01","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.271 GET https://app.sowork.com/eyekia/api/v1/floor/WDknDl3TmJ34rR0vIL7R?pathOnlyUrls=true&v=1im5bcnms -> 200
RES: {"orientations":{"N":{"url":"/floor/WDknDl3TmJ34rR0vIL7R/iV2MB8_iVig6hD-TNdXAZ","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#FFFFFFFF","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"","name":"Matte Black FLoor","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.271 GET https://app.sowork.com/eyekia/api/v1/floor/Ic8sFy8QQeSvZAcr9xkW?pathOnlyUrls=true&v=1gq9po3f2 -> 200
RES: {"orientations":{"N":{"url":"/floor/Ic8sFy8QQeSvZAcr9xkW/K1YiqlPz1NULZNKxj379_","pivotPosition":{"x":110,"y":67}},"E":{"url":"/floor/Ic8sFy8QQeSvZAcr9xkW/UHn2AlR8505se-l2DYMm2","pivotPosition":{"x":110,"y":67}},"S":{"url":"/floor/Ic8sFy8QQeSvZAcr9xkW/YnO_9HzzQVx6CAaJfsL0y","pivotPosition":{"x":110,"y":67}},"W":{"url":"/floor/Ic8sFy8QQeSvZAcr9xkW/rj7UrKHEpSMpAiGquTH3u","pivotPosition":{"x":110,"y":67}}},"type":"floor","height":"","pixelDensity":211,"isCorner":false,"colors":{"baseColor":"#FFFFFFFF","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"","name":"Moss Inner Rounded Corder","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.271 GET https://app.sowork.com/eyekia/api/v1/floor/ndqwVBMJKxvBj7jmtSaR?pathOnlyUrls=true&v=1ilp0id0h -> 200
RES: {"orientations":{"N":{"url":"/floor/ndqwVBMJKxvBj7jmtSaR/geffPtNgBAVHQCcj11C5j","pivotPosition":{"x":220,"y":134}},"E":{"url":"/floor/ndqwVBMJKxvBj7jmtSaR/hn8-n2cgBwSoQfVKOmMHT","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#422015ff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":0,"saturationMultiplier":0.5,"valueMultiplier":1},{"hueRotationDegrees":2.333333333333332,"saturationMultiplier":1,"valueMultiplier":1.3},{"hueRotationDegrees":0,"saturationMultiplier":0.3,"valueMultiplier":0.78}],"presetColorsDictionary":{"ICXDb4GVtDJolpLul37r":{"hueRotationDegrees":2.333333333333332,"saturationMultiplier":1,"valueMultiplier":1.3},"9eJ2CC9PSh9J4zFApX4A":{"hueRotationDegrees":0,"saturationMultiplier":0.5,"valueMultiplier":1},"ZzlLOnVLx6MJQHUan9xM":{"hueRotationDegrees":0,"saturationMultiplier":0.3,"valueMultiplier":0.78}}},"accessKey":"PREMIUM","name":"Retro Wood Floor15","description":"SM_2m_Floor_Wood_15a_N","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.271 GET https://app.sowork.com/eyekia/api/v1/floor/9QKOMlrx1jITZMdAaNvW?pathOnlyUrls=true&v=1j4sjef4i -> 200
RES: {"orientations":{"N":{"url":"/floor/9QKOMlrx1jITZMdAaNvW/LH7xY2K4-PqExNp0gSD8N","pivotPosition":{"x":220,"y":134}},"E":{"url":"/floor/9QKOMlrx1jITZMdAaNvW/pBoT2jwgYMTtKt30qjAnH","pivotPosition":{"x":220,"y":134}},"S":{"url":"/floor/9QKOMlrx1jITZMdAaNvW/17vaRK1B7c7kKXYbXZWGZ","pivotPosition":{"x":220,"y":134}},"W":{"url":"/floor/9QKOMlrx1jITZMdAaNvW/N0KoqInje8hsBGAfetYjW","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#cec6b3ff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":-42.22222222222222,"saturationMultiplier":0.76,"valueMultiplier":0.27},{"hueRotationDegrees":0.6713947990543758,"saturationMultiplier":2,"valueMultiplier":0.9215686274509803}],"presetColorsDictionary":{"ZDOWIWjbGrf49f0tkk2x":{"hueRotationDegrees":0.6713947990543758,"saturationMultiplier":2,"valueMultiplier":0.9215686274509803},"GybInlbb5eFxS3WMvbTU":{"hueRotationDegrees":-42.22222222222222,"saturationMultiplier":0.76,"valueMultiplier":0.27}}},"accessKey":"PREMIUM","name":"Retro Wood Floor14","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.271 GET https://app.sowork.com/eyekia/api/v1/wall/3JwFmhsELEM7lpDnGiHX?pathOnlyUrls=true&v=1hd72n554 -> 200
RES: {"orientations":{"N":{"url":"/wall/3JwFmhsELEM7lpDnGiHX/CQUsYqnCzD0C3KA6S785s","pivotPosition":{"x":256,"y":134}},"E":{"url":"/wall/3JwFmhsELEM7lpDnGiHX/bVA5_0FQ1N5z-mMHb1GXi","pivotPosition":{"x":256,"y":134}},"S":{"url":"/wall/3JwFmhsELEM7lpDnGiHX/i6EEEQLM11N-x4roHiNrO","pivotPosition":{"x":14,"y":134}},"W":{"url":"/wall/3JwFmhsELEM7lpDnGiHX/EEzsi9cE7wSxyuJofwESi","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"2m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#152734ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Industrial Glass Wall013 Short","description":"Glass 02 Wall 2m","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.271 GET https://app.sowork.com/eyekia/api/v1/floor/hZBdo3zDFHe1raotIOy6?pathOnlyUrls=true&v=1hjvon2pj -> 200
RES: {"orientations":{"N":{"url":"/floor/hZBdo3zDFHe1raotIOy6/4ekFmWmnCIg3xRSFrIOTT","pivotPosition":{"x":220,"y":134}},"E":{"url":"/floor/hZBdo3zDFHe1raotIOy6/9yEXAOicrpP-Wt4yfHkad","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#21201fff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Marble Floor Luxury 02","description":"SM_2m_Floor_Ceramic_03a","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.271 GET https://app.sowork.com/eyekia/api/v1/wall/hdOqw0EotFirBMmkqjny?pathOnlyUrls=true&v=1hebbl5vh -> 200
RES: {"orientations":{"N":{"url":"/wall/hdOqw0EotFirBMmkqjny/ul6-I6r2jDLRNJO0f51Rl","pivotPosition":{"x":256,"y":134}},"E":{"url":"/wall/hdOqw0EotFirBMmkqjny/987DCw_pGAfq8Iwtamwr3","pivotPosition":{"x":256,"y":134}},"S":{"url":"/wall/hdOqw0EotFirBMmkqjny/o9y9gHB5NE_hPyfoH2sAT","pivotPosition":{"x":14,"y":134}},"W":{"url":"/wall/hdOqw0EotFirBMmkqjny/WkMab5lYK_r9KG2SB7K8R","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"2m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#33677bff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Retro Wall033 Short","description":"Glass 03 Wall 2m","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.322 GET https://app.sowork.com/eyekia/api/v1/floor/YfEQLH4oDULWLv0ihdmd?pathOnlyUrls=true&v=1k32hgp2e -> 200
RES: {"orientations":{"N":{"url":"/floor/YfEQLH4oDULWLv0ihdmd/dfiEkckIBbcpo2Eh_Uklc","pivotPosition":{"x":220,"y":134}},"E":{"url":"/floor/YfEQLH4oDULWLv0ihdmd/wII8zBYgZ5QRA7pipMysp","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#763e23ff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":-19.518072289156628,"saturationMultiplier":0.1,"valueMultiplier":1.2},{"hueRotationDegrees":173.48192771084337,"saturationMultiplier":0.48,"valueMultiplier":0.85},{"hueRotationDegrees":-19.518072289156628,"saturationMultiplier":0.1,"valueMultiplier":2.2},{"hueRotationDegrees":-17.11807228915663,"saturationMultiplier":1.1627906976744187,"valueMultiplier":0.5058823529411764}],"presetColorsDictionary":{"BKfH82n9HfOHx38LGETc":{"hueRotationDegrees":-19.518072289156628,"saturationMultiplier":0.1,"valueMultiplier":1.2},"a14VP0uqYQb-M1Qudc1u":{"hueRotationDegrees":-17.11807228915663,"saturationMultiplier":1.1627906976744187,"valueMultiplier":0.5058823529411764},"yjYSXxT3N1nIIOVKKx7C":{"hueRotationDegrees":-19.518072289156628,"saturationMultiplier":0.1,"valueMultiplier":2.2},"MPVgpDCoQLQyXp7mmbEJ":{"hueRotationDegrees":173.48192771084337,"saturationMultiplier":0.48,"valueMultiplier":0.85}}},"accessKey":"","name":"Stone Floor09","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.379 GET https://app.sowork.com/eyekia/api/v1/asset/1W0cx9o1GIeLOJID4Fqb?pathOnlyUrls=true&v=1jaj8cbld -> 200
RES: {"name":"The Desk That Isn’t","description":"For people who refuse to desk. Looks like nothing, acts like a desk. Trust us, it works.","userHint":"","categoryId":"Bl4KXcUjIeJHdoNHREcO","thumbnail":"/kOjGlbkmAMxdhmbcZf0b/3LUAnrqTGJ0EEENdohN4O","accessKey":"","defaultInteractionType":"desk","colors":{"baseColor":"#e8cf00ff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":151.4655172413793,"saturationMultiplier":1,"valueMultiplier":1},{"hueRotationDegrees":291.4655172413793,"saturationMultiplier":0.2,"valueMultiplier":1.4},{"hueRotationDegrees":62.044464609800364,"saturationMultiplier":0.9090909090909091,"valueMultiplier":0.8196078431372549},{"hueRotationDegrees":208.4655172413793,"saturationMultiplier":1,"valueMultiplier":1},{"hueRotationDegrees":-53.53448275862069,"saturationMultiplier":1.450199203187251,"valueMultiplier":0.984313725490196},{"hueRotationDegrees":165.9777123633305,"saturationMultiplier":1.0081967213114753,"valueMultiplier":0.9568627450980393},{"hueRotationDegrees":-15.673520191775772,"saturationMultiplier":1.4841269841269842,"valueMultiplier":0.9882352941176471},{"hueRotationDegrees":-53.53448275862069,"saturationMultiplier":0,"valueMultiplier":0.6627450980392157},{"hueRotationDegrees":109.15782493368701,"saturationMultiplier":0.5279187817258884,"valueMultiplier":0.7725490196078432},{"hueRotationDegrees":0,"saturationMultiplier":0,"valueMultiplier":0.3}],"presetColorsDictionary":{"7f56dtvmOyesfEIhCZPb":{"hueRotationDegrees":62.044464609800364,"saturationMultiplier":0.9090909090909091,"valueMultiplier":0.8196078431372549},"fCrmnj31doijJbDdLdxw":{"hueRotationDegrees":-53.53448275862069,"saturationMultiplier":0,"valueMultiplier":0.6627450980392157},"eGd02gT53Ga_UU3vcjlD":{"hueRotationDegrees":165.9777123633305,"saturationMultiplier":1.0081967213114753,"valueMultiplier":0.9568627450980393},"euJ5ySihIxBrw92nDxwD":{"hueRotationDegrees":-15.673520191775772,"saturationMultiplier":1.4841269841269842,"valueMultiplier":0.9882352941176471},"rGunPAhXwfj5T4Y0CkpC":{"hueRotationDegrees":0,"saturationMultiplier":0,"valueMultiplier":0.3},"7UOELZ3loEyy4r1PbxjD":{"hueRotationDegrees":291.4655172413793,"saturationMultiplier":0.2,"valueMultiplier":1.4},"qSaMu4LkYPW_4BXEldqc":{"hueRotationDegrees":109.15782493368701,"saturationMultiplier":0.5279187817258884,"valueMultiplier":0.7725490196078432},"Q0VsqnfZrZoHzBdvgPQb":{"hueRotationDegrees":-53.53448275862069,"saturationMultiplier":1.450199203187251,"valueMultiplier":0.984313725490196},"3UYK2Xi__VyeQ0MXpkIr":{"hueRotationDegrees":151.4655172413793,"saturationMultiplier":1,"valueMultiplier":1},"L8RYIgk__v5arCHc86wT":{"hueRotationDegrees":208.4655172413793,"saturationMultiplier":1,"valueMultiplier":1}}},"projections":{"isometric":true,"orthographic":false},"published":true,"modifiedSinceLastPublish":false,"orientations":{"N":{"goToPivot":{"x":0,"y":0},"images":{"xSNMvoWQPwLyQmYN6xe9":{"url":"/xSNMvoWQPwLyQmYN6xe9/oHsMrzRYXj5drcfu9lIzS","transform":{"position":{"x":0,"y":0},"scale":{"x":1,"y":1},"rotationRad":0,"frontPoint":{"x":0,"y":0},"depthBand":0},"type":"image","primitives":{"p6Oi6dtBp-eSwAy6_gGv":{"shape":{"rotationRad":0,"shape":"rect","scale":{"y":124.00440216064453,"x":127.35382843017578},"position":{"y":6.978008270263672,"x":5.303295135498047}},"type":"shape","shapeIntersectionType":"selector","height":31.095836764966634}},"blendMode":null},"89aUzN0VYIgMASVTHcRG":{"url":"/kOjGlbkmAMxdhmbcZf0b/8IKFnRqPtzqooyxE5S5La","transform":{"position":{"x":0,"y":0},"scale":{"x":1,"y":1},"rotationRad":0,"frontPoint":{"x":-0.333038330078125,"y":3.264129638671875},"depthBand":-1},"type":"image","primitives":{},"blendMode":null}}}},"config":{"frames":{}}}

### 79.454 GET https://app.sowork.com/eyekia/api/v1/floor/8IJBGHtZYWUImQdhKXbK?pathOnlyUrls=true&v=1gu2devkd -> 200
RES: {"orientations":{"N":{"url":"/floor/8IJBGHtZYWUImQdhKXbK/eZpmXi6gDZczWpXbcRlF1","pivotPosition":{"x":220,"y":134}},"E":{"url":"/floor/8IJBGHtZYWUImQdhKXbK/AWLlKjpTt4CP6fcNJLFV1","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#536678ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"","name":"Stone Floor10","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.454 GET https://app.sowork.com/eyekia/api/v1/floor/pKTtn39tMSLg7owFUwjY?pathOnlyUrls=true&v=1gu2df62d -> 200
RES: {"orientations":{"N":{"url":"/floor/pKTtn39tMSLg7owFUwjY/ZX_3vbWyNuSftYmJWgCYd","pivotPosition":{"x":220,"y":134}},"E":{"url":"/floor/pKTtn39tMSLg7owFUwjY/_ECFevFfV32UhJeSqkWiD","pivotPosition":{"x":220,"y":134}},"S":{"url":"/floor/pKTtn39tMSLg7owFUwjY/NbwiHCUQWWV11o3xdFx5q","pivotPosition":{"x":220,"y":134}},"W":{"url":"/floor/pKTtn39tMSLg7owFUwjY/U_IxtF0xtnIEec1YIRlwU","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#4dcdb9ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"","name":"Ocean to Grass Inside CornerB","description":"SM_2m_Floor_OceanToGrass_InsideCorner_01b","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.454 GET https://app.sowork.com/eyekia/api/v1/floor/0ZjuujEA8UsgGztK23XU?pathOnlyUrls=true&v=1i9cbjrdv -> 200
RES: {"orientations":{"N":{"url":"/floor/0ZjuujEA8UsgGztK23XU/Jp6Sed-y_olJHH4vqknEN","pivotPosition":{"x":108,"y":67}}},"type":"floor","height":"","pixelDensity":211,"isCorner":false,"colors":{"baseColor":"#3cd1d6ff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":18.051948051948045,"saturationMultiplier":1.4,"valueMultiplier":0.95},{"hueRotationDegrees":-7.948051948051955,"saturationMultiplier":1,"valueMultiplier":1},{"hueRotationDegrees":19.051948051948045,"saturationMultiplier":0.7,"valueMultiplier":1}],"presetColorsDictionary":{"2zdNor2jI6Wg4EseLLxc":{"hueRotationDegrees":18.051948051948045,"saturationMultiplier":1.4,"valueMultiplier":0.95},"ZQ9fCGXxDoUc41t7sBsM":{"hueRotationDegrees":19.051948051948045,"saturationMultiplier":0.7,"valueMultiplier":1},"wahJYF2fc6bleF4bgqsM":{"hueRotationDegrees":-7.948051948051955,"saturationMultiplier":1,"valueMultiplier":1}}},"accessKey":"","name":"Ocean","description":"SM_2m_Floor_Ocean_01a","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.454 GET https://app.sowork.com/eyekia/api/v1/floor/0r7rfxu7Xfp8ByLhYYJz?pathOnlyUrls=true&v=1gu2deun8 -> 200
RES: {"orientations":{"N":{"url":"/floor/0r7rfxu7Xfp8ByLhYYJz/Ofw3aVMWaauvH-PvEW-18","pivotPosition":{"x":220,"y":134}},"E":{"url":"/floor/0r7rfxu7Xfp8ByLhYYJz/j-aFYTwBSSI1elWiiMiXO","pivotPosition":{"x":220,"y":134}},"S":{"url":"/floor/0r7rfxu7Xfp8ByLhYYJz/G0lA0rB_sJSJIJbetjahZ","pivotPosition":{"x":220,"y":134}},"W":{"url":"/floor/0r7rfxu7Xfp8ByLhYYJz/XoxzvnkiHcDy9Wz1_EUVo","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#58caa5ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"","name":"Ocean to Grass StraightB","description":"SM_2m_Floor_OceanToGrass_Straight_01b","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.454 GET https://app.sowork.com/eyekia/api/v1/floor/AJmshYdHAb79CVWSv90t?pathOnlyUrls=true&v=1han79dff -> 200
RES: {"orientations":{"N":{"url":"/floor/AJmshYdHAb79CVWSv90t/R7BKbBTNIqsw7AOZ-0Qpr","pivotPosition":{"x":110,"y":67}},"E":{"url":"/floor/AJmshYdHAb79CVWSv90t/zFSUBL6TLgO24h0KTdJvd","pivotPosition":{"x":110,"y":67}},"S":{"url":"/floor/AJmshYdHAb79CVWSv90t/83B1cR9s44U0H3dfOvfXm","pivotPosition":{"x":110,"y":67}},"W":{"url":"/floor/AJmshYdHAb79CVWSv90t/nyKwsGEtGob9YRzuF9nvK","pivotPosition":{"x":110,"y":67}}},"type":"floor","height":"","pixelDensity":211,"isCorner":false,"colors":{"baseColor":"#a0cd5bff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":35.315789473684205,"saturationMultiplier":1.4,"valueMultiplier":0.8},{"hueRotationDegrees":-14.684210526315795,"saturationMultiplier":1.2,"valueMultiplier":1},{"hueRotationDegrees":81.3157894736842,"saturationMultiplier":1.35,"valueMultiplier":0.9},{"hueRotationDegrees":-39.684210526315795,"saturationMultiplier":1,"valueMultiplier":1}],"presetColorsDictionary":{"h_D8d0UEr9Jsu27XGG7k":{"hueRotationDegrees":81.3157894736842,"saturationMultiplier":1.35,"valueMultiplier":0.9},"RBVzvxnVunOme1Bqd1zL":{"hueRotationDegrees":-39.684210526315795,"saturationMultiplier":1,"valueMultiplier":1},"6PIljT3qXbpDcqJVkjb1":{"hueRotationDegrees":35.315789473684205,"saturationMultiplier":1.4,"valueMultiplier":0.8},"NuzQh2JreF4tNs-XO1ZM":{"hueRotationDegrees":-14.684210526315795,"saturationMultiplier":1.2,"valueMultiplier":1}}},"accessKey":"","name":"Grass Terrain01","description":"SM_2m_Floor_Grass_01a","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.454 GET https://app.sowork.com/eyekia/api/v1/floor/qXWWOC3HpRZ8W3NAMjO2?pathOnlyUrls=true&v=1hebbkkfm -> 200
RES: {"orientations":{"N":{"url":"/floor/qXWWOC3HpRZ8W3NAMjO2/QgHtN9OorNIz31bcYUdFn","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#b2a48aff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":-39,"saturationMultiplier":0.34,"valueMultiplier":1.1},{"hueRotationDegrees":-39,"saturationMultiplier":0,"valueMultiplier":0.34},{"hueRotationDegrees":286,"saturationMultiplier":1.3,"valueMultiplier":1},{"hueRotationDegrees":-39,"saturationMultiplier":0,"valueMultiplier":0.74},{"hueRotationDegrees":129,"saturationMultiplier":1.86,"valueMultiplier":0.75},{"hueRotationDegrees":105,"saturationMultiplier":1,"valueMultiplier":0.96}],"presetColorsDictionary":{"xhUVK32NWQpqgYp_2O99":{"hueRotationDegrees":129,"saturationMultiplier":1.86,"valueMultiplier":0.75},"FIMk3dEKeBgbamwtEA4x":{"hueRotationDegrees":286,"saturationMultiplier":1.3,"valueMultiplier":1},"K6jvYb_PjtmEF_-8eDCS":{"hueRotationDegrees":-39,"saturationMultiplier":0,"valueMultiplier":0.74},"7b3zo9jv6PUh_gMaKo3h":{"hueRotationDegrees":-39,"saturationMultiplier":0.34,"valueMultiplier":1.1},"y9tDxZlXgyGcjJHQ6vUZ":{"hueRotationDegrees":105,"saturationMultiplier":1,"valueMultiplier":0.96},"CQAko1r2JIwQszzp6VtL":{"hueRotationDegrees":-39,"saturationMultiplier":0,"valueMultiplier":0.34}}},"accessKey":"PREMIUM","name":"Retro Fabric Floor04","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.454 GET https://app.sowork.com/eyekia/api/v1/wall/saW1jZlEj3islAtt0Ubo?pathOnlyUrls=true&v=1gu2dfj97 -> 200
RES: {"orientations":{"N":{"url":"/wall/saW1jZlEj3islAtt0Ubo/QJa8jkZ32Pggm72MDp5Ax","pivotPosition":{"x":128,"y":67}},"E":{"url":"/wall/saW1jZlEj3islAtt0Ubo/MXzHAl00wlmRflOMYTdwX","pivotPosition":{"x":128,"y":67}},"S":{"url":"/wall/saW1jZlEj3islAtt0Ubo/J3clIL5N3i2TWriSsf0FY","pivotPosition":{"x":7,"y":67}},"W":{"url":"/wall/saW1jZlEj3islAtt0Ubo/1X-89bTYIEpN1CUbLfoMG","pivotPosition":{"x":7,"y":67}}},"type":"wall","height":"2m","pixelDensity":211,"isCorner":false,"colors":{"baseColor":"#cc8d3fff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"","name":"Wood Wall013 Short","description":"Wood 01 Wall 2m","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.454 GET https://app.sowork.com/eyekia/api/v1/floor/Zp7ldmHdLT4hTzkjMQr7?pathOnlyUrls=true&v=1gn7nvqpe -> 200
RES: {"orientations":{"N":{"url":"/floor/Zp7ldmHdLT4hTzkjMQr7/DMGyjutvluPsxRAHKSLrP","pivotPosition":{"x":108,"y":67}},"E":{"url":"/floor/Zp7ldmHdLT4hTzkjMQr7/hFHd5bRyd8yKRmFS20Wcg","pivotPosition":{"x":108,"y":67}},"W":{"url":"/floor/Zp7ldmHdLT4hTzkjMQr7/BmA_TrLLvoQEfEFUkyOJM","pivotPosition":{"x":108,"y":67}}},"type":"floor","height":"","pixelDensity":211,"isCorner":false,"colors":{"baseColor":"#76b68bff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"","name":"Wood Floor08b","description":"SM_2m_Floor_Wood_08b","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.454 GET https://app.sowork.com/eyekia/api/v1/floor/PhOwmTdkRzZRShXHFveA?pathOnlyUrls=true&v=1i36238nc -> 200
RES: {"orientations":{"N":{"url":"/floor/PhOwmTdkRzZRShXHFveA/zHB9B8dbbGU2-9AYAl-vQ","pivotPosition":{"x":220,"y":134}},"E":{"url":"/floor/PhOwmTdkRzZRShXHFveA/lY0Kl3SNwc-QXOe_ML6Bm","pivotPosition":{"x":220,"y":134}},"S":{"url":"/floor/PhOwmTdkRzZRShXHFveA/9-9T3kys9MLEphhOh6s1p","pivotPosition":{"x":220,"y":134}},"W":{"url":"/floor/PhOwmTdkRzZRShXHFveA/ewqvkvV19F-AUKNcQ12xx","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#49cdbeff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Pool Round Corner ","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.454 GET https://app.sowork.com/eyekia/api/v1/floor/ktXDgTLcLLfnIi9n64cK?pathOnlyUrls=true&v=1i3623fvf -> 200
RES: {"orientations":{"N":{"url":"/floor/ktXDgTLcLLfnIi9n64cK/fsQ43sSHFuGYnfB82H17a","pivotPosition":{"x":220,"y":134}},"E":{"url":"/floor/ktXDgTLcLLfnIi9n64cK/4iaBR1XM5r8AK1afkbCCY","pivotPosition":{"x":220,"y":134}},"S":{"url":"/floor/ktXDgTLcLLfnIi9n64cK/BbH_Yra91FarEq97rL4bi","pivotPosition":{"x":220,"y":134}},"W":{"url":"/floor/ktXDgTLcLLfnIi9n64cK/zMzz6WgwInaUJwKoSRYTh","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#3cd0d6ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Pool Straight ","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.454 GET https://app.sowork.com/eyekia/api/v1/floor/x9WjkhBPKwPeVBJhXvaZ?pathOnlyUrls=true&v=1hp1o8n98 -> 200
RES: {"orientations":{"N":{"url":"/floor/x9WjkhBPKwPeVBJhXvaZ/i-3lyTwUVsCZveWBrXRTV","pivotPosition":{"x":220,"y":134}},"E":{"url":"/floor/x9WjkhBPKwPeVBJhXvaZ/FplDERMLonunYfl1_a5nE","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#446a94ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Floor Pixelbit 01","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.454 GET https://app.sowork.com/eyekia/api/v1/floor/jg7BWx7uVJiVsFj86jxg?pathOnlyUrls=true&v=1j5e8lou7 -> 200
RES: {"orientations":{"N":{"url":"/floor/jg7BWx7uVJiVsFj86jxg/C0ZVhnlXRLRWF9coyZ7p4","pivotPosition":{"x":220,"y":134}},"E":{"url":"/floor/jg7BWx7uVJiVsFj86jxg/v_jZqzClO5ud708a8YaNo","pivotPosition":{"x":220,"y":134}},"S":{"url":"/floor/jg7BWx7uVJiVsFj86jxg/QSwBdP9RRnpIBiP8oQHPf","pivotPosition":{"x":220,"y":134}},"W":{"url":"/floor/jg7BWx7uVJiVsFj86jxg/cqUG0C20xo00I5o1CegY7","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#a4c24dff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"BLOCKS Floor02","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.455 GET https://app.sowork.com/eyekia/api/v1/wall/VOxXVCZKvjX9Q1qWmwQp?pathOnlyUrls=true&v=1hgjgdvba -> 200
RES: {"orientations":{"N":{"url":"/wall/VOxXVCZKvjX9Q1qWmwQp/uzSqInAYMwnsXVTat4238","pivotPosition":{"x":256,"y":134}},"E":{"url":"/wall/VOxXVCZKvjX9Q1qWmwQp/hd20zCyb6vs92Sf0kLqL2","pivotPosition":{"x":256,"y":134}},"S":{"url":"/wall/VOxXVCZKvjX9Q1qWmwQp/oEOg0HLl8sQmuAIUGXTI9","pivotPosition":{"x":14,"y":134}},"W":{"url":"/wall/VOxXVCZKvjX9Q1qWmwQp/0OHVrAGB0AV4NJgLUdaWC","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"6m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#858c39ff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":141.93975903614458,"saturationMultiplier":1,"valueMultiplier":1},{"hueRotationDegrees":234.93975903614458,"saturationMultiplier":1,"valueMultiplier":1},{"hueRotationDegrees":-57.06024096385542,"saturationMultiplier":1,"valueMultiplier":1}],"presetColorsDictionary":{"zgQ4UnyeuoW0WOF4-YLZ":{"hueRotationDegrees":-57.06024096385542,"saturationMultiplier":1,"valueMultiplier":1},"T3XPqm96d3tz9D0ds9fC":{"hueRotationDegrees":234.93975903614458,"saturationMultiplier":1,"valueMultiplier":1},"HEcqMUNxlg9wTnW64G4u":{"hueRotationDegrees":141.93975903614458,"saturationMultiplier":1,"valueMultiplier":1}}},"accessKey":"PREMIUM","name":"SoBLOCKS Wall015 Tall A","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.455 GET https://app.sowork.com/eyekia/api/v1/floor/klB54rvXfHrAOms5qPNY?pathOnlyUrls=true&v=1hp1o8ikc -> 200
RES: {"orientations":{"N":{"url":"/floor/klB54rvXfHrAOms5qPNY/bWsw9IrhPBHK_wXa79SWU","pivotPosition":{"x":220,"y":134}},"E":{"url":"/floor/klB54rvXfHrAOms5qPNY/3zgJjbbf2hXmbd1FdMrRv","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#486d95ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Floor Pixelbit 02","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.455 GET https://app.sowork.com/eyekia/api/v1/floor/vIMm3Ipv2zBMGnPrBklM?pathOnlyUrls=true&v=1jck91h3u -> 200
RES: {"orientations":{"N":{"url":"/floor/vIMm3Ipv2zBMGnPrBklM/1VJZwCTwZzUW7-oaZX-xe","pivotPosition":{"x":108,"y":67}},"E":{"url":"/floor/vIMm3Ipv2zBMGnPrBklM/64Y459wuE9ihq3Qqw4Ev5","pivotPosition":{"x":108,"y":67}},"S":{"url":"/floor/vIMm3Ipv2zBMGnPrBklM/HcP7PdKS8LVXM4eX6y__g","pivotPosition":{"x":108,"y":67}},"W":{"url":"/floor/vIMm3Ipv2zBMGnPrBklM/ZHNpmYJJsSow5QUiJ5Cnv","pivotPosition":{"x":108,"y":67}}},"type":"floor","height":"","pixelDensity":211,"isCorner":false,"colors":{"baseColor":"#52c4c3ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"","name":"Ocean to WetSand Straight","description":"SM_2m_Floor_OceanToSand_Straight_01a","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 79.472 GET https://app.sowork.com/characters/v5/character_art_database.json -> 200
RES: {"README":"THIS IS A FILE THAT'S AUTOGENERATED BY THE CI / CD SYSTEM. DO NOT TOUCH!","lastUpdatedCommit":"79530061912db310dc167b166caaa5de72f22e29","assets":{"assistants_body_belfort+basecolor":{"preview":{"ver":1,"type":"atlas"},"layers":{"body":{"idle":{"ver":1,"type":"atlas"},"jump1":{"ver":1,"type":"atlas"},"sit":{"ver":1,"type":"atlas"},"walk":{"ver":1,"type":"atlas"},"work1":{"ver":1,"type":"atlas"},"work2":{"ver":1,"type":"atlas"},"work3":{"ver":1,"type":"atlas"},"work4":{"ver":1,"type":"atlas"},"work5":{"ver":1,"type":"atlas"},"work6":{"ver":1,"type":"atlas"},"work7":{"ver":1,"type":"atlas"},"working":{"ver":1,"type":"atlas"}}}},"assistants_body_gf+basecolor":{"preview":{"ver":1,"type":"atlas"},"layers":{"body":{"idle":{"ver":1,"type":"atlas"},"jump1":{"ver":1,"type":"atlas"},"sit":{"ver":1,"type":"atlas"},"walk":{"ver":1,"type":"atlas"},"work1":{"ver":1,"type":"atlas"},"work2":{"ver":1,"type":"atlas"},"work3":{"ver":1,"type":"atlas"},"work4":{"ver":1,"type":"atlas"},"work5":{"ver":1,"type":"atlas"},"work6":{"ver":1,"type":"atlas"},"work7":{"ver":1,"type":"atlas"},"working":{"ver":1,"type":"atlas"}}}},"assistants_body_mross+basecolor":{"preview":{"ver":1,"type":"atlas"},"layers":{"body":{"idle":{"ver":2,"type":"atlas"},"jump1":{"ver":1,"type":"atlas"},"sit":{"ver":1,"type":"atlas"},"walk":{"ver":1,"type":"atlas"},"work1":{"ver":1,"type":"atlas"},"work2":{"ver":2,"type":"atlas"},"work3":{"ver":2,"type":"atlas"},"work4":{"ver":2,"type":"atlas"},"work5":{"ver":1,"type":"atlas"},"work6":{"ver":1,"type":"atlas"},"work7":{"ver":1,"type":"atlas"},"working":{"ver":2,"type":"atlas"}}}},"cats_body_catTypeA+typeA":{"preview":{"ver":0,"type":"atlas"},"layers":{"body":{"idle":{"ver":2,"type":"atlas"},"sit":{"ver":2,"type":"atlas"},"walk":{"ver":2,"type":"atlas"}}}},"cats_body_catTypeA+typeB":{"preview":{"ver":0,"type":"atlas"},"layers":{"body":{"idle":{"ver":2,"type":"atlas"},"sit":{"ver":2,"type":"atlas"},"walk":{"ver":2,"type":"atlas"}}}},"characterA_avatarExtra_avatarExtra+basecolor":{"preview":{"ver":0,"type":"atlas"},"layers":{"avatarExtra":{"angryEmojiMeeting":{"ver":1,"type":"atlas"},"beerEmojiMeeting":{"ver":1,"type":"atlas"},"bowheEmojiMeeting":{"ver":1,"type":"atlas"},"bowsheEmojiMeeting":{"ver":1,"type":"atlas"},"cashEmojiMeeting":{"ver":1,"type":"atlas"},"champagneEmojiMeeting":{"ver":1,"type":"atlas"},"clapEmojiMeeting":{"ver":1,"type":"atlas"},"clockEmojiMeeting":{"ver":1,"type":"atlas"},"cocktailEmojiMeeting":{"ver":1,"type":"atlas"},"crazyEmojiMeeting":{"ver":1,"type":"atlas"},"cryEmojiMeeting":{"ver":1,"type":"atlas"},"eggplantEmojiMeeting":{"ver":1,"type":"atlas"},"fightEmojiMeeting":{"ver":1,"type":"atlas"},"fireEmojiMeeting":{"ver":1,"type":"atlas"},"flowersEmojiMeeting":{"ver":1,"type":"atlas"},"frozenEmojiMeeting":{"ver":1,"type":"atlas"},"glassEmojiMeeting":{"ver":1,"type":"atlas"},"handEmojiMeeting":{"ver":1,"type":"atlas"},"happyEmojiMeeting":{"ver":1,"type":"atlas"},"hilariousEmojiMeeting":{"ver":1,"type":"atlas"},"hugEmojiMeeting":{"ver":1,"type":"atlas"},"inloveEmojiMeeting":{"ver":1,"type":"atlas"},"kissEmojiMeeting":{"ver":1,"type":"atlas"},"likeEmojiMeeting":{"ver":1,"type":"atlas"},"lolEmojiMeeting":{"ver":1,"type":"atlas"},"lollypopEmojiMeeting":{"ver":1,"type":"atlas"},"loveEmojiMeeting":{"ver":1,"type":"atlas"},"normalEmojiMeeting":{"ver":1,"type":"atlas"},"oleEmojiMeeting":{"ver":1,"type":"atlas"},"partyEmojiMeeting":{"ver":1,"type":"atlas"},"peaceEmojiMeeting":{"ver":1,"type":"atlas"},"prayEmojiMeeting":{"ver":1,"type":"atlas"},"rainbowEmojiMeeting":{"ver":1,"type":"atlas"},"rocketEmojiMeeting":{"ver":1,"type":"atlas"},"sadEmojiMeeting":{"ver":1,"type":"atlas"},"seedEmojiMeeting":{"ver":1,"type":"atlas"},"silenceEmojiMeeting":{"ver":1,"type":"atlas"},"sleepEmojiMeeting":{"ver":1,"type":"atlas"},"snowEmojiMeeting":{"ver":1,"type":"atlas"},"sockyEmojiMeeting":{"ver":1,"type":"atlas"},"teethEmojiMeeting":{"ver":1,"type":"atlas"},"thinkEmojiMeeting":{"ver":1,"type":"atlas"},"valentinEmojiMeeting":{"ver":1,"type":"atlas"},"winkEmojiMeeting":{"ver":1,"type":"atlas"}}}},"characterA_avatarExtra_avatarExtra+guest":{"preview":{"ver":0,"type":"atlas"},"layers":{"avatarExtra":{"angryEmojiMeeting":{"ver":1,"type":"atlas"},"beerEmojiMeeting":{"ver":1,"type":"atlas"},"bowheEmojiMeeting":{"ver":1,"type":"atlas"},"bowsheEmojiMeeting":{"ver":1,"type":"atlas"},"cashEmojiMeeting":{"ver":2,"type":"atlas"},"champagneEmojiMeeting":{"ver":1,"type":"atlas"},"clapEmojiMeeting":{"ver":1,"type":"atlas"},"clockEmojiMeeting":{"ver":1,"type":"atlas"},"cocktailEmojiMeeting":{"ver":1,"type":"atlas"},"crazyEmojiMeeting":{"ver":1,"type":"atlas"},"cryEmojiMeeting":{"ver":1,"type":"atlas"},"eggplantEmojiMeeting":{"ver":1,"type":"atlas"},"fightEmojiMeeting":{"ver":1,"type":"atlas"},"fireEmojiMeeting":{"ver":1,"type":"atlas"},"flowersEmojiMeeting":{"ver":1,"type":"atlas"},"frozenEmojiMeeting":{"ver":1,"type":"atlas"},"glassEmojiMeeting":{"ver":1,"type":"atlas"},"handEmojiMeeting":{"ver":1,"type":"atlas"},"happyEmojiMeeting":{"ver":1,"type":"atlas"},"hilariousEmojiMeeting":{"ver":1,"type":"atlas"},"hugEmojiMeeting":{"ver":1,"type":"atlas"},"inloveEmojiMeeting":{"ver":1,"type":"atlas"},"kissEmojiMeeting":{"ver":1,"type":"atlas"},"likeEmojiMeeting":{"ver":1,"type":"atlas"},"lolEmojiMeeting":{"ver":1,"type":"atlas"},"lollypopEmojiMeeting":{"ver":1,"type":"atlas"},"loveEmojiMeeting":{"ver":1,"type":"atlas"},"negationEmojiMeeting":{"ver":1,"type":"atlas"},"nodEmojiMeeting":{"ver":1,"type":"atlas"},"normalEmojiMeeting":{"ver":1,"type":"atlas"},"oleEmojiMeeting":{"ver":1,"type":"atlas"},"partyEmojiMeeting":{"ver":1,"type":"atlas"},"peaceEmojiMeeting":{"ver":1,"type":"atlas"},"prayEmojiMeeting":{"ver":1,"type":"atlas"},"rainbowEmojiMeeting":{"ver":1,"type":"atlas"},"rocketEmojiMeeting":{"ver":1,"type":"atlas"},"sadEmojiMeeting":{"ver":1,"type":"atlas"},"seedEmojiMeeting":{"ver":1,"type":"atlas"},"silenceEmojiMeeting":{"ver":1,"type":"atlas"},"sleepEmojiMeeting":{"ver":1,"type":"atlas"},"snowEmojiMeeting

### 79.623 GET https://app.sowork.com/characters/v5/customizer_config.json -> 200
RES: {"README":"THIS IS AN AUTOGENERATED FILE BY CI, DO NOT MODIFY","bodyTypes":{"characterA":{"slots":{"backAccessory":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"category":{"id":"backAccessories","order":0},"items":{"bday":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"disabled":false,"accessKey":"PREMIUM","publishedAt":"2024-03-08T16:08:53.946Z","variations":{"basecolor":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"single","primary":"#d5b276ff"},"disabled":true},"blue":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":164,"saturationMultiplier":1.2,"valueMultiplier":1}}},"yellow":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":1.8,"valueMultiplier":0.9}}},"purple":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-120,"saturationMultiplier":1,"valueMultiplier":0.9}}},"red":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-44,"saturationMultiplier":2,"valueMultiplier":0.7}}},"green":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":81,"saturationMultiplier":1.2,"valueMultiplier":1}}}},"baseVariation":"basecolor"},"twentyfiveballoon":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"disabled":false,"accessKey":"PREMIUM","publishedAt":"2024-12-24T11:42:38.240Z","variations":{"basecolor":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"single","primary":"#d5b276ff"},"disabled":true},"blue":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":164,"saturationMultiplier":1.2,"valueMultiplier":1}}},"black":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":0,"valueMultiplier":0.35}}},"yellow":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":1.8,"valueMultiplier":0.9}}},"brown":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-20,"saturationMultiplier":1.5,"valueMultiplier":0.55}}},"orange":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-18,"saturationMultiplier":2,"valueMultiplier":0.85}}},"purple":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-120,"saturationMultiplier":1,"valueMultiplier":0.9}}},"red":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-44,"saturationMultiplier":2,"valueMultiplier":0.7}}},"turquoise":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":119,"saturationMultiplier":1,"valueMultiplier":1}}},"grey":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":0,"valueMultiplier":1}}},"white":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":0,"valueMultiplier":1.3}}},"green":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":81,"saturationMultiplier":1.2,"valueMultiplier":1}}}},"baseVariation":"basecolor"},"chineseLuck":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"disabled":false,"accessKey":"PREMIUM","publishedAt":"2025-01-23T20:36:35.648Z","variations":{"basecolor":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"single","primary":"#d5b276ff"},"disabled":true},"yellow":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":1,"valueMultiplier":1}}}},"baseVariation":"basecolor"},"chooseyou":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"disabled":false,"accessKey":"PREMIUM","publishedAt":"2025-02-07T16:24:23.811Z","variations":{"basecolor":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"single","primary":"#d5b276ff"},"disabled":true},"blue":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":1,"valueMultiplier":1}}}},"baseVariation":"basecolor"},"guitar":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"disabled":false,"publishedAt":"2021-11-08T23:00:00.000Z","variations":{"brown":{"disabled":false,"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-20,"saturationMultiplier":1.5,"valueMultiplier":0.55}}},"basecolor":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"single","primary":"#d5b276ff"},"disabled":true},"blue":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":164,"saturationMultiplier":1.2,"valueMultiplier":1}}},"black":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":0,"valueMultiplier":0.35}}},"yellow":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":1.8,"valueMultiplier":0.9}}},"orange":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-18,"saturationMultiplier":2,"valueMultiplier":0.85}}},"purple":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-120,"saturationMultiplier":1,"valueMultiplier":0.9}}},"red":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-44,"saturationMultiplier":2,"valueMultiplier":0.7}}},"turquoise":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transf

### 79.860 POST https://api.sowork.com/api/v1/user/recordLogin -> 200
REQ: {}
RES: {"success":true}

### 79.860 POST https://api.sowork.com/api/v1/userSession/upsert -> 200
REQ: {"sessionId":"tab_1791290998367_kunqg2mjy"}
RES: {"userId":"RMUYRzBXvUOCqtJdxSAiMdiEZM82","sessionId":"tab_1791290998367_kunqg2mjy"}

### 80.297 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jInTxxe_-dgqgWEFEnBA84caz3-k6l5LnnwlLjI0rF8h9WK3c-As-A&SID=QvpyrketJ3lO0IfaEgpvRQ&RID=62245&AID=179&zx=2s4oic9iv885&t=1 -> 200
REQ: count=1&ofs=51&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fuser_sessions%2FRMUYRzBXvUOCqtJdxSAiMdiEZM82%22%5D%7D%2C%22targetId%22%3A100%7D%7D
RES: 10
[1,181,7]


### 84.637 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jInTxxe_-dgqgWEFEnBA84caz3-k6l5LnnwlLjI0rF8h9WK3c-As-A&SID=QvpyrketJ3lO0IfaEgpvRQ&RID=62246&AID=185&zx=bkxlggda7fd&t=1 -> 200
REQ: count=1&ofs=52&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22npcs%22%7D%5D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fgroups%2Fv0XvoIt3uK40pdF0mQDB%22%7D%2C%22targetId%22%3A102%7D%7D
RES: 10
[1,185,7]


### 85.343 POST https://api.sowork.com/api/v1/user/updateUiSettings -> 200
REQ: {"userId":"RMUYRzBXvUOCqtJdxSAiMdiEZM82","data":{"mutedSounds":{"go":false,"lose":false,"memo":false,"doorForceOpen":false,"chatNotification":false,"jump":false,"duck_hitter":false,"countdown":false,"cat_hitter":false,"rotateAsset":false,"coworkingEnd1":true,"removeNotification":false,"applause":false,"shoot_2":false,"speechHigh":false,"win":false,"deleteAsset":false,"battleMusic":false,"speechMid":false,"participantJoined":false,"receiveWave":false,"codeSuccess":false,"requestToJoinPm":false,"sendWave":false,"joinCall":true,"coworkingStart2":true,"pencilLine":false,"notification":false,"ring":false,"hit":false,"coworkingEnd2":true,"polaroid":false,"disableVideo":false,"coworkingStart1":true,"calendarNotification":true,"gong":false,"doorKnock":false,"unmuteMic":false,"coworkingEnd3":true,"shoot_1":false,"equip":false,"shark_hitter":false,"productUnlocked":false,"muteMic":false,"raiseHand":false,"characterPicker":false,"placeAsset":false,"privateMeeting":false,"speechLow":false,"editAsset":false,"mapMakerMusic":true,"enableVideo":false,"selectAsset":false,"textBroadcast":false,"coworkingStart3":true,"leaveCall":true}}}
RES: {"success":true}

### 89.589 POST https://api.sowork.com/api/v1/crm/track -> 200
REQ: {"userId":"RMUYRzBXvUOCqtJdxSAiMdiEZM82","groupId":"v0XvoIt3uK40pdF0mQDB","events":[{"eventName":"entered_office","timestamp":1791291009.569}]}
RES: {"success":true}

### 105.433 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=b5x-EdtPARVaSdrpwaTJ4AfmUkKTC2qxFBc4ZNwkGx-0wFVlFrg8Pw&VER=8&database=projects%2Fsophya-calendar-production%2Fdatabases%2F(default)&RID=rpc&SID=M7w4eKqOZEDny8WbcZBKXw&AID=8&CI=1&TYPE=xmlhttp&zx=ifu1krk94zhu&t=1 -> 200
RES: 

### 111.626 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jInTxxe_-dgqgWEFEnBA84caz3-k6l5LnnwlLjI0rF8h9WK3c-As-A&SID=QvpyrketJ3lO0IfaEgpvRQ&RID=62247&AID=193&zx=pfceh8a75ahh&t=1 -> 200
REQ: count=1&ofs=53&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FEsFlL0qd1kZNo0IsotREJWQWNeC3%22%5D%7D%2C%22targetId%22%3A104%7D%7D
RES: 10
[1,193,7]


### 111.999 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jInTxxe_-dgqgWEFEnBA84caz3-k6l5LnnwlLjI0rF8h9WK3c-As-A&SID=QvpyrketJ3lO0IfaEgpvRQ&RID=62248&AID=197&zx=t3eq8chncq7a&t=1 -> 200
REQ: count=1&ofs=54&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FIEwNgnRoYUcFOwc4KSrz7MyHNvG2%22%5D%7D%2C%22targetId%22%3A106%7D%7D
RES: 10
[1,197,7]


### 120.485 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=b5x-EdtPARVaSdrpwaTJ4AfmUkKTC2qxFBc4ZNwkGx-0wFVlFrg8Pw&VER=8&database=projects%2Fsophya-calendar-production%2Fdatabases%2F(default)&RID=rpc&SID=M7w4eKqOZEDny8WbcZBKXw&AID=10&CI=1&TYPE=xmlhttp&zx=qi4086f3y3us&t=1 -> 200
RES: 

### 129.975 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jInTxxe_-dgqgWEFEnBA84caz3-k6l5LnnwlLjI0rF8h9WK3c-As-A&SID=QvpyrketJ3lO0IfaEgpvRQ&RID=62249&TYPE=terminate&zx=td7aljiktrg5 -> 200
RES: 

### 129.975 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsophya-calendar-production%2Fdatabases%2F(default)&gsessionid=b5x-EdtPARVaSdrpwaTJ4AfmUkKTC2qxFBc4ZNwkGx-0wFVlFrg8Pw&SID=M7w4eKqOZEDny8WbcZBKXw&RID=65089&TYPE=terminate&zx=85m8ukfggg42 -> 200
RES: 

### 129.980 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=36663&CVER=22&X-HTTP-Session-Id=gsessionid&zx=yk24k6zgaqf&t=1 -> 200
REQ: headers=X-Goog-Api-Client%3Agl-js%2F%20fire%2F9.23.0%0D%0AContent-Type%3Atext%2Fplain%0D%0AX-Firebase-GMPID%3A1%3A69834214729%3Aweb%3Ac3e2769cb392ca9203de98%0D%0AAuthorization%3ABearer%20<JWT>%0D%0A&count=49&ofs=0&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FRMUYRzBXvUOCqtJdxSAiMdiEZM82%22%5D%7D%2C%22targetId%22%3A4%2C%22resumeToken%22%3A%22CgkIvOLPj7ellwM%3D%22%2C%22expectedCount%22%3A1%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers_sys%2FRMUYRzBXvUOCqtJdxSAiMdiEZM82%22%5D%7D%2C%22targetId%22%3A6%2C%22resumeToken%22%3A%22CgkIvOLPj7ellwM%3D%22%2C%22expectedCount%22%3A1%7D%7D&req2___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22groupJoinRequests%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22groupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22requestorUserId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A10%2C%22resumeToken%22%3A%22CgkIvOLPj7ellwM%3D%22%2C%22expectedCount%22%3A0%7D%7D&req3___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22groupJoinRequests%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22pending%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22requestorUserId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A12%2C%22resumeToken%22%3A%22CgkIvOLPj7ellwM%3D%22%2C%22expectedCount%22%3A0%7D%7D&req4___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22groupJoinRequests%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22seenReqAnswer%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22booleanValue%22%3Afalse%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22requestorUserId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A14%2C%22resumeToken%22%3A%22CgkIvOLPj7ellwM%3D%22%2C%22expectedCount%22%3A0%7D%7D&req5___
RES: 51
[[0,["c","d2f7aCXto4Rc7OWBmbmjbQ","",8,15,30000]]]


### 129.981 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsophya-calendar-production%2Fdatabases%2F(default)&RID=80799&CVER=22&X-HTTP-Session-Id=gsessionid&zx=i5zsied7z8k8&t=1 -> 200
REQ: headers=X-Goog-Api-Client%3Agl-js%2F%20fire%2F9.23.0%0D%0AContent-Type%3Atext%2Fplain%0D%0AX-Firebase-GMPID%3A1%3A537328756772%3Aweb%3Ad487ca7b2e66572b9510b1%0D%0AAuthorization%3ABearer%20<JWT>%0D%0A&count=2&ofs=0&req0___data__=%7B%22database%22%3A%22projects%2Fsophya-calendar-production%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22events%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22userId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22start%22%7D%2C%22op%22%3A%22GREATER_THAN_OR_EQUAL%22%2C%22value%22%3A%7B%22timestampValue%22%3A%222026-09-29T12%3A50%3A00.208000000Z%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22start%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%2C%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsophya-calendar-production%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A2%2C%22resumeToken%22%3A%22CgkI9MfKk7ellwM%3D%22%2C%22expectedCount%22%3A0%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsophya-calendar-production%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22accounts%22%7D%5D%2C%22where%22%3A%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22userId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsophya-calendar-production%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A4%2C%22resumeToken%22%3A%22CgkI9MfKk7ellwM%3D%22%2C%22expectedCount%22%3A0%7D%7D
RES: 51
[[0,["c","lPm8XchvMkEw2Ub9kAeTUg","",8,15,30000]]]


### 130.069 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=90669&CVER=22&X-HTTP-Session-Id=gsessionid&zx=298yyt6tr9ua&t=1 -> 200
REQ: headers=X-Goog-Api-Client%3Agl-js%2F%20fire%2F9.23.0%0D%0AContent-Type%3Atext%2Fplain%0D%0AX-Firebase-GMPID%3A1%3A69834214729%3Aweb%3Ac3e2769cb392ca9203de98%0D%0AAuthorization%3ABearer%20<JWT>%0D%0A&count=49&ofs=0&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FRMUYRzBXvUOCqtJdxSAiMdiEZM82%22%5D%7D%2C%22targetId%22%3A4%2C%22resumeToken%22%3A%22CgkIvOLPj7ellwM%3D%22%2C%22expectedCount%22%3A1%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers_sys%2FRMUYRzBXvUOCqtJdxSAiMdiEZM82%22%5D%7D%2C%22targetId%22%3A6%2C%22resumeToken%22%3A%22CgkIvOLPj7ellwM%3D%22%2C%22expectedCount%22%3A1%7D%7D&req2___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22groupJoinRequests%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22groupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22requestorUserId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A10%2C%22resumeToken%22%3A%22CgkIvOLPj7ellwM%3D%22%2C%22expectedCount%22%3A0%7D%7D&req3___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22groupJoinRequests%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22pending%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22requestorUserId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A12%2C%22resumeToken%22%3A%22CgkIvOLPj7ellwM%3D%22%2C%22expectedCount%22%3A0%7D%7D&req4___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22groupJoinRequests%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22seenReqAnswer%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22booleanValue%22%3Afalse%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22requestorUserId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A14%2C%22resumeToken%22%3A%22CgkIvOLPj7ellwM%3D%22%2C%22expectedCount%22%3A0%7D%7D&req5___
RES: 51
[[0,["c","zSmTPqk8vKgDS_kVLidJFQ","",8,15,30000]]]


### 130.070 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsophya-calendar-production%2Fdatabases%2F(default)&RID=3305&CVER=22&X-HTTP-Session-Id=gsessionid&zx=noiey9isfox3&t=1 -> 200
REQ: headers=X-Goog-Api-Client%3Agl-js%2F%20fire%2F9.23.0%0D%0AContent-Type%3Atext%2Fplain%0D%0AX-Firebase-GMPID%3A1%3A537328756772%3Aweb%3Ad487ca7b2e66572b9510b1%0D%0AAuthorization%3ABearer%20<JWT>%0D%0A&count=2&ofs=0&req0___data__=%7B%22database%22%3A%22projects%2Fsophya-calendar-production%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22events%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22userId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22start%22%7D%2C%22op%22%3A%22GREATER_THAN_OR_EQUAL%22%2C%22value%22%3A%7B%22timestampValue%22%3A%222026-09-29T12%3A50%3A00.208000000Z%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22start%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%2C%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsophya-calendar-production%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A2%2C%22resumeToken%22%3A%22CgkI9MfKk7ellwM%3D%22%2C%22expectedCount%22%3A0%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsophya-calendar-production%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22accounts%22%7D%5D%2C%22where%22%3A%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22userId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsophya-calendar-production%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A4%2C%22resumeToken%22%3A%22CgkI9MfKk7ellwM%3D%22%2C%22expectedCount%22%3A0%7D%7D
RES: 51
[[0,["c","8Cc0kyHmeeDTvyh6WcFcpQ","",8,15,30000]]]


### 131.204 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=CUjfeQtaoeYnxV3TjFeZLvDsgCLNm87uyuNR0pnsFSL1XsKWdfw2BQ&VER=8&database=projects%2Fsophya-calendar-production%2Fdatabases%2F(default)&RID=rpc&SID=8Cc0kyHmeeDTvyh6WcFcpQ&AID=0&CI=0&TYPE=xmlhttp&zx=ar9ttjyax36e&t=1 -> 200
RES: 

### 131.267 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=jVGIefZ9h_uVSODsFtlR3Iv2EoHNjQMGzdg0mI1YWxY-Ud6yRHA_cA&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=zSmTPqk8vKgDS_kVLidJFQ&AID=0&CI=0&TYPE=xmlhttp&zx=v72ekp9d281b&t=1 -> 200
RES: 

### 153.124 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 153.152 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jVGIefZ9h_uVSODsFtlR3Iv2EoHNjQMGzdg0mI1YWxY-Ud6yRHA_cA&SID=zSmTPqk8vKgDS_kVLidJFQ&RID=90670&AID=261&zx=oas57s3q5w&t=1 -> 200
REQ: count=1&ofs=49&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fconversations%2F912l38MU0BfjJ8fyWwkR%22%5D%7D%2C%22targetId%22%3A108%7D%7D
RES: 10
[1,261,7]


### 154.914 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 156.785 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jVGIefZ9h_uVSODsFtlR3Iv2EoHNjQMGzdg0mI1YWxY-Ud6yRHA_cA&SID=zSmTPqk8vKgDS_kVLidJFQ&RID=90671&AID=280&zx=gq63ras6guqe&t=1 -> 200
REQ: count=1&ofs=50&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A104%7D
RES: 10
[1,280,7]


### 159.966 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 165.017 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 170.060 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 175.094 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 180.137 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 180.185 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=jVGIefZ9h_uVSODsFtlR3Iv2EoHNjQMGzdg0mI1YWxY-Ud6yRHA_cA&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=zSmTPqk8vKgDS_kVLidJFQ&AID=282&CI=0&TYPE=xmlhttp&zx=75ht8xfusew1&t=1 -> 200
RES: 

### 180.268 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jVGIefZ9h_uVSODsFtlR3Iv2EoHNjQMGzdg0mI1YWxY-Ud6yRHA_cA&SID=zSmTPqk8vKgDS_kVLidJFQ&RID=90672&AID=282&zx=iicr4pu3ctrb&t=1 -> 200
REQ: count=1&ofs=51&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A108%7D
RES: 10
[1,297,7]


### 185.175 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 185.197 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jVGIefZ9h_uVSODsFtlR3Iv2EoHNjQMGzdg0mI1YWxY-Ud6yRHA_cA&SID=zSmTPqk8vKgDS_kVLidJFQ&RID=90673&AID=302&zx=b2auunc2l5ab&t=1 -> 200
REQ: count=1&ofs=52&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fconversations%2F912l38MU0BfjJ8fyWwkR%22%5D%7D%2C%22targetId%22%3A1%7D%2C%22labels%22%3A%7B%22goog-listen-tags%22%3A%22limbo-document%22%7D%7D
RES: 10
[1,302,7]


### 185.378 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jVGIefZ9h_uVSODsFtlR3Iv2EoHNjQMGzdg0mI1YWxY-Ud6yRHA_cA&SID=zSmTPqk8vKgDS_kVLidJFQ&RID=90674&AID=306&zx=z8ciivcreukk&t=1 -> 200
REQ: count=1&ofs=53&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A1%7D
RES: 10
[1,306,7]


### 185.520 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jVGIefZ9h_uVSODsFtlR3Iv2EoHNjQMGzdg0mI1YWxY-Ud6yRHA_cA&SID=zSmTPqk8vKgDS_kVLidJFQ&RID=90675&AID=307&zx=p19gg3aqq9fx&t=1 -> 200
REQ: count=1&ofs=54&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FEsFlL0qd1kZNo0IsotREJWQWNeC3%22%5D%7D%2C%22targetId%22%3A110%7D%7D
RES: 10
[1,307,7]


### 188.023 HEAD https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud/ -> 200
RES: 

### 188.026 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 188.040 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jVGIefZ9h_uVSODsFtlR3Iv2EoHNjQMGzdg0mI1YWxY-Ud6yRHA_cA&SID=zSmTPqk8vKgDS_kVLidJFQ&RID=90676&AID=324&zx=v4jwanxfyxku&t=1 -> 200
REQ: count=1&ofs=55&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fconversations%2Fp2M1I3W0c2ndeitB5Baa%22%5D%7D%2C%22targetId%22%3A112%7D%7D
RES: 10
[1,324,7]


### 191.883 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jVGIefZ9h_uVSODsFtlR3Iv2EoHNjQMGzdg0mI1YWxY-Ud6yRHA_cA&SID=zSmTPqk8vKgDS_kVLidJFQ&RID=90677&AID=345&zx=q7lsc1lvcrmr&t=1 -> 200
REQ: count=1&ofs=56&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A110%7D
RES: 10
[1,345,7]


### 193.321 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 198.356 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 204.107 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jVGIefZ9h_uVSODsFtlR3Iv2EoHNjQMGzdg0mI1YWxY-Ud6yRHA_cA&SID=zSmTPqk8vKgDS_kVLidJFQ&RID=90678&AID=349&zx=x0kzfyv3gdi9&t=1 -> 200
REQ: count=1&ofs=57&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A62%7D
RES: 10
[1,349,7]


### 204.318 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jVGIefZ9h_uVSODsFtlR3Iv2EoHNjQMGzdg0mI1YWxY-Ud6yRHA_cA&SID=zSmTPqk8vKgDS_kVLidJFQ&RID=90679&AID=349&zx=8ne9rrrdnecx&t=1 -> 200
REQ: count=9&ofs=58&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A60%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A78%7D&req2___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A76%7D&req3___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A74%7D&req4___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A72%7D&req5___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A70%7D&req6___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A68%7D&req7___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A66%7D&req8___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A106%7D
RES: 10
[1,350,7]


### 205.977 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jVGIefZ9h_uVSODsFtlR3Iv2EoHNjQMGzdg0mI1YWxY-Ud6yRHA_cA&SID=zSmTPqk8vKgDS_kVLidJFQ&RID=90680&AID=359&zx=8fnwxo2s5pm&t=1 -> 200
REQ: count=1&ofs=67&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A82%7D
RES: 10
[1,359,7]


### 206.123 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jVGIefZ9h_uVSODsFtlR3Iv2EoHNjQMGzdg0mI1YWxY-Ud6yRHA_cA&SID=zSmTPqk8vKgDS_kVLidJFQ&RID=90681&AID=360&zx=kltsgefckodi&t=1 -> 200
REQ: count=2&ofs=68&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A86%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A84%7D
RES: 10
[1,360,7]


### 206.279 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 206.301 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=CUjfeQtaoeYnxV3TjFeZLvDsgCLNm87uyuNR0pnsFSL1XsKWdfw2BQ&VER=8&database=projects%2Fsophya-calendar-production%2Fdatabases%2F(default)&RID=rpc&SID=8Cc0kyHmeeDTvyh6WcFcpQ&AID=12&CI=0&TYPE=xmlhttp&zx=wavdah7jg2k2&t=1 -> 200
RES: 

### 212.349 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 213.624 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jVGIefZ9h_uVSODsFtlR3Iv2EoHNjQMGzdg0mI1YWxY-Ud6yRHA_cA&SID=zSmTPqk8vKgDS_kVLidJFQ&RID=90682&AID=362&zx=vfa8apc6umre&t=1 -> 200
REQ: count=1&ofs=70&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22conversations%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22ownerGroupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomDefinitionId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22yPsvwRdkb5LnGHfXFgfO%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22active%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22op%22%3A%22GREATER_THAN%22%2C%22value%22%3A%7B%22integerValue%22%3A%220%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%2C%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A114%7D%7D
RES: 10
[1,362,7]


### 213.700 POST https://api.sowork.com/api/v1/gaia/rooms -> 200
REQ: {"groupId":"v0XvoIt3uK40pdF0mQDB"}
RES: {"roomDefinitionMap":{"yPsvwRdkb5LnGHfXFgfO":{"roomDefinition":{"id":"yPsvwRdkb5LnGHfXFgfO","groupId":"v0XvoIt3uK40pdF0mQDB","name":"Escritório principal","alias":"mistymountainlodge","mapDefinitionId":"V1vAAHj80gty4MwBqEFp","ownerKey":"group_v0XvoIt3uK40pdF0mQDB","primary":true,"isMeetingSpace":false,"zones":null,"createdAt":{"seconds":1786123761,"nanoseconds":850000000},"updatedAt":{"seconds":1786123761,"nanoseconds":850000000},"roomObjects":{},"roomGroupId":null,"pricingType":"premium"},"rooms":[{"id":"YmAfBb3h2eY5r9FsAIfV","name":"Escritório principal","index":1,"local":false,"serverId":"g5","server":{"url":"https://api.sowork.com/gaia/g5"},"userIds":["JLDRGMskJRXiYcHccNVaLuwr3XJ3","RMUYRzBXvUOCqtJdxSAiMdiEZM82","rjOwBlMG93QD2sjutWoF2fc8Mv63","fCfzuTuBkBSLyF1tKdMJOZSOnUu1","G7EWVnLMQngko5v7KcgNnH06Fhg1","Rxp3M2MJzra9wQJm0VkDfRSgJgt1","blKsp8Fxi6VGJfXfizN0iu248IB2"],"userCount":7,"roomDefinitionId":"yPsvwRdkb5LnGHfXFgfO","roomDefinition":{"ownerKey":"group_v0XvoIt3uK40pdF0mQDB","name":"Misty Mountain Lodge","groupId":"v0XvoIt3uK40pdF0mQDB","zones":null,"projection":null},"isMeetingSpace":false,"createdAt":{"seconds":1786123764,"nanoseconds":340000000},"updatedAt":{"seconds":1786123764,"nanoseconds":340000000},"roomGroupId":null}]}}}

### 214.267 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jVGIefZ9h_uVSODsFtlR3Iv2EoHNjQMGzdg0mI1YWxY-Ud6yRHA_cA&SID=zSmTPqk8vKgDS_kVLidJFQ&RID=90683&AID=367&zx=3xxbv0mfapkj&t=1 -> 200
REQ: count=13&ofs=71&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22conversations%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22ownerGroupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22unaryFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomDefinitionId%22%7D%2C%22op%22%3A%22IS_NULL%22%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22active%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22op%22%3A%22GREATER_THAN%22%2C%22value%22%3A%7B%22integerValue%22%3A%220%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%2C%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A116%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22conversations%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22ownerGroupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomDefinitionId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22yPsvwRdkb5LnGHfXFgfO%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22YmAfBb3h2eY5r9FsAIfV%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22active%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22op%22%3A%22GREATER_THAN%22%2C%22value%22%3A%7B%22integerValue%22%3A%220%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%2C%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A118%7D%7D&req2___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FEsFlL0qd1kZNo0IsotREJWQWNeC3%22%5D%7D%2C%22targetId%22%3A120%7D%7D&req3___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FIEwNgnRoYUcFOwc4KSrz7MyHNvG2%22%5D%7D%2C%22targetId%22%3A122%7D%7D&req4___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FzdXi8bJLG6cNbd4P87bqTAzTdTq1%22%5D%7D%2C%22targetId%22%3A124%7D%7D&req5___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases
RES: 10
[1,367,7]


### 214.931 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jVGIefZ9h_uVSODsFtlR3Iv2EoHNjQMGzdg0mI1YWxY-Ud6yRHA_cA&SID=zSmTPqk8vKgDS_kVLidJFQ&RID=90684&AID=421&zx=uha34xspkx76&t=1 -> 200
REQ: count=1&ofs=84&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A112%7D
RES: 10
[1,423,7]


### 216.006 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jVGIefZ9h_uVSODsFtlR3Iv2EoHNjQMGzdg0mI1YWxY-Ud6yRHA_cA&SID=zSmTPqk8vKgDS_kVLidJFQ&RID=90685&AID=428&zx=kbqw58v6jmli&t=1 -> 200
REQ: count=1&ofs=85&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fconversations%2Fp2M1I3W0c2ndeitB5Baa%22%5D%7D%2C%22targetId%22%3A3%7D%2C%22labels%22%3A%7B%22goog-listen-tags%22%3A%22limbo-document%22%7D%7D
RES: 10
[1,428,7]


### 216.165 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=jVGIefZ9h_uVSODsFtlR3Iv2EoHNjQMGzdg0mI1YWxY-Ud6yRHA_cA&SID=zSmTPqk8vKgDS_kVLidJFQ&RID=90686&AID=432&zx=2yu5ttdhzso6&t=1 -> 200
REQ: count=1&ofs=86&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A3%7D
RES: 10
[1,432,7]


### 217.385 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 222.431 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 227.505 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 232.309 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=jVGIefZ9h_uVSODsFtlR3Iv2EoHNjQMGzdg0mI1YWxY-Ud6yRHA_cA&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=zSmTPqk8vKgDS_kVLidJFQ&AID=454&CI=0&TYPE=xmlhttp&zx=24z6mmo1csku&t=1 -> 200
RES: 

### 232.543 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 237.581 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 242.619 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}
