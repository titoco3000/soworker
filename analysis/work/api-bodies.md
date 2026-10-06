### 0.157 GET https://app.sowork.com/s/v0XvoIt3uK40pdF0mQDB/mistymountainlodge -> 200
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

### 0.903 GET https://app.sowork.com/manifest.json -> 200
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


### 19.272 GET https://api.sowork.com/eyekia/api/v1/assets-version -> 200
RES: 

### 19.274 GET https://firebase.googleapis.com/v1alpha/projects/-/apps/1:69834214729:web:c3e2769cb392ca9203de98/webConfig -> 200
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


### 19.274 OPTIONS https://firebase.googleapis.com/v1alpha/projects/-/apps/1:69834214729:web:c3e2769cb392ca9203de98/webConfig -> 200
RES: 

### 19.316 POST https://securetoken.googleapis.com/v1/token?key=<REDACTED> -> 200
REQ: grant_type=refresh_token&refresh_token=<REDACTED>
RES: {
  "access_token": "<JWT>",
  "expires_in": "3600",
  "token_type": "Bearer",
  "refresh_token": "AMf-vBxtVkQPtdEQzqiW0cx2w4Kwo3zFj1umpfbgxT3falzRIE9rD27G_xBmuJ_tUNlmWV2Av_Pk3MkJ4jueD2UO3If0DkFm-sufm8cwJVfs8F4o1l1iDg13XBVBW0KurtC4AlZBCNlupNOwgIbuFnvQXY-DW1bDC9Oful6eEgPk3p4QtGnoBu3uO3eDD24j-wEnnAFAd6PSRE4Ue9xHfywmdvKC5KsPyI06D4HrYgr_c-YqkDTIYmEGeE2T5HW_oOp_2lpNyRX7fY92d2n6VTpNtQBOpdS7Pue4PHCRm6jBwThBwbq8NAg0h5kY5xtOrTZrHGOq1BSnpbCCZEs4jM-1U9s8hO4ZoyIkn26PzgS3r4iGTWj6C42UMnjol9A_i0MG-1McLO9WNnB1O7_B0PNonctA_bB-ZjiJP3av3Q_KnMmQho1nY9at4lHhJwXeJTT6OOnWPl_w",
  "id_token": "<JWT>",
  "user_id": "RMUYRzBXvUOCqtJdxSAiMdiEZM82",
  "project_id": "69834214729"
}


### 19.316 OPTIONS https://securetoken.googleapis.com/v1/token?key=<REDACTED> -> 200
RES: 

### 36.088 POST https://identitytoolkit.googleapis.com/v1/accounts:lookup?key=<REDACTED> -> 200
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
      "lastLoginAt": "1791307420776",
      "createdAt": "1786365155887",
      "lastRefreshAt": "2026-10-06T17:31:05.039830Z"
    }
  ]
}


### 36.088 OPTIONS https://identitytoolkit.googleapis.com/v1/accounts:lookup?key=<REDACTED> -> 200
RES: 

### 37.072 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=65305&CVER=22&X-HTTP-Session-Id=gsessionid&zx=o66qv9j29umz&t=1 -> 200
REQ: headers=X-Goog-Api-Client%3Agl-js%2F%20fire%2F9.23.0%0D%0AContent-Type%3Atext%2Fplain%0D%0AX-Firebase-GMPID%3A1%3A69834214729%3Aweb%3Ac3e2769cb392ca9203de98%0D%0AAuthorization%3ABearer%20<JWT>%0D%0A&count=2&ofs=0&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FRMUYRzBXvUOCqtJdxSAiMdiEZM82%22%5D%7D%2C%22targetId%22%3A2%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers_sys%2FRMUYRzBXvUOCqtJdxSAiMdiEZM82%22%5D%7D%2C%22targetId%22%3A4%7D%7D
RES: 51
[[0,["c","iboNN4rB6cuI2nnXp8nuig","",8,15,30000]]]


### 38.273 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=0&CI=0&TYPE=xmlhttp&zx=62it4rqv673l&t=1 -> 200
RES: 

### 40.678 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=0&CI=1&TYPE=xmlhttp&zx=rauzw0kzhttr&t=1 -> 200
RES: 

### 41.334 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=8&CI=1&TYPE=xmlhttp&zx=uyqzrey8bx87&t=1 -> 200
RES: 

### 41.375 GET https://app.sowork.com/characters/v5/customizer_config.json -> 200
RES: {"README":"THIS IS AN AUTOGENERATED FILE BY CI, DO NOT MODIFY","bodyTypes":{"characterA":{"slots":{"backAccessory":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"category":{"id":"backAccessories","order":0},"items":{"bday":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"disabled":false,"accessKey":"PREMIUM","publishedAt":"2024-03-08T16:08:53.946Z","variations":{"basecolor":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"single","primary":"#d5b276ff"},"disabled":true},"blue":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":164,"saturationMultiplier":1.2,"valueMultiplier":1}}},"yellow":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":1.8,"valueMultiplier":0.9}}},"purple":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-120,"saturationMultiplier":1,"valueMultiplier":0.9}}},"red":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-44,"saturationMultiplier":2,"valueMultiplier":0.7}}},"green":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":81,"saturationMultiplier":1.2,"valueMultiplier":1}}}},"baseVariation":"basecolor"},"twentyfiveballoon":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"disabled":false,"accessKey":"PREMIUM","publishedAt":"2024-12-24T11:42:38.240Z","variations":{"basecolor":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"single","primary":"#d5b276ff"},"disabled":true},"blue":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":164,"saturationMultiplier":1.2,"valueMultiplier":1}}},"black":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":0,"valueMultiplier":0.35}}},"yellow":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":1.8,"valueMultiplier":0.9}}},"brown":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-20,"saturationMultiplier":1.5,"valueMultiplier":0.55}}},"orange":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-18,"saturationMultiplier":2,"valueMultiplier":0.85}}},"purple":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-120,"saturationMultiplier":1,"valueMultiplier":0.9}}},"red":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-44,"saturationMultiplier":2,"valueMultiplier":0.7}}},"turquoise":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":119,"saturationMultiplier":1,"valueMultiplier":1}}},"grey":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":0,"valueMultiplier":1}}},"white":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":0,"valueMultiplier":1.3}}},"green":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":81,"saturationMultiplier":1.2,"valueMultiplier":1}}}},"baseVariation":"basecolor"},"chineseLuck":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"disabled":false,"accessKey":"PREMIUM","publishedAt":"2025-01-23T20:36:35.648Z","variations":{"basecolor":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"single","primary":"#d5b276ff"},"disabled":true},"yellow":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":1,"valueMultiplier":1}}}},"baseVariation":"basecolor"},"chooseyou":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"disabled":false,"accessKey":"PREMIUM","publishedAt":"2025-02-07T16:24:23.811Z","variations":{"basecolor":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"single","primary":"#d5b276ff"},"disabled":true},"blue":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":1,"valueMultiplier":1}}}},"baseVariation":"basecolor"},"guitar":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"disabled":false,"publishedAt":"2021-11-08T23:00:00.000Z","variations":{"brown":{"disabled":false,"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-20,"saturationMultiplier":1.5,"valueMultiplier":0.55}}},"basecolor":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"single","primary":"#d5b276ff"},"disabled":true},"blue":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":164,"saturationMultiplier":1.2,"valueMultiplier":1}}},"black":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":0,"valueMultiplier":0.35}}},"yellow":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":1.8,"valueMultiplier":0.9}}},"orange":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-18,"saturationMultiplier":2,"valueMultiplier":0.85}}},"purple":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-120,"saturationMultiplier":1,"valueMultiplier":0.9}}},"red":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-44,"saturationMultiplier":2,"valueMultiplier":0.7}}},"turquoise":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transf

### 41.375 GET https://app.sowork.com/characters/v5/character_art_database.json -> 200
RES: {"README":"THIS IS A FILE THAT'S AUTOGENERATED BY THE CI / CD SYSTEM. DO NOT TOUCH!","lastUpdatedCommit":"79530061912db310dc167b166caaa5de72f22e29","assets":{"assistants_body_belfort+basecolor":{"preview":{"ver":1,"type":"atlas"},"layers":{"body":{"idle":{"ver":1,"type":"atlas"},"jump1":{"ver":1,"type":"atlas"},"sit":{"ver":1,"type":"atlas"},"walk":{"ver":1,"type":"atlas"},"work1":{"ver":1,"type":"atlas"},"work2":{"ver":1,"type":"atlas"},"work3":{"ver":1,"type":"atlas"},"work4":{"ver":1,"type":"atlas"},"work5":{"ver":1,"type":"atlas"},"work6":{"ver":1,"type":"atlas"},"work7":{"ver":1,"type":"atlas"},"working":{"ver":1,"type":"atlas"}}}},"assistants_body_gf+basecolor":{"preview":{"ver":1,"type":"atlas"},"layers":{"body":{"idle":{"ver":1,"type":"atlas"},"jump1":{"ver":1,"type":"atlas"},"sit":{"ver":1,"type":"atlas"},"walk":{"ver":1,"type":"atlas"},"work1":{"ver":1,"type":"atlas"},"work2":{"ver":1,"type":"atlas"},"work3":{"ver":1,"type":"atlas"},"work4":{"ver":1,"type":"atlas"},"work5":{"ver":1,"type":"atlas"},"work6":{"ver":1,"type":"atlas"},"work7":{"ver":1,"type":"atlas"},"working":{"ver":1,"type":"atlas"}}}},"assistants_body_mross+basecolor":{"preview":{"ver":1,"type":"atlas"},"layers":{"body":{"idle":{"ver":2,"type":"atlas"},"jump1":{"ver":1,"type":"atlas"},"sit":{"ver":1,"type":"atlas"},"walk":{"ver":1,"type":"atlas"},"work1":{"ver":1,"type":"atlas"},"work2":{"ver":2,"type":"atlas"},"work3":{"ver":2,"type":"atlas"},"work4":{"ver":2,"type":"atlas"},"work5":{"ver":1,"type":"atlas"},"work6":{"ver":1,"type":"atlas"},"work7":{"ver":1,"type":"atlas"},"working":{"ver":2,"type":"atlas"}}}},"cats_body_catTypeA+typeA":{"preview":{"ver":0,"type":"atlas"},"layers":{"body":{"idle":{"ver":2,"type":"atlas"},"sit":{"ver":2,"type":"atlas"},"walk":{"ver":2,"type":"atlas"}}}},"cats_body_catTypeA+typeB":{"preview":{"ver":0,"type":"atlas"},"layers":{"body":{"idle":{"ver":2,"type":"atlas"},"sit":{"ver":2,"type":"atlas"},"walk":{"ver":2,"type":"atlas"}}}},"characterA_avatarExtra_avatarExtra+basecolor":{"preview":{"ver":0,"type":"atlas"},"layers":{"avatarExtra":{"angryEmojiMeeting":{"ver":1,"type":"atlas"},"beerEmojiMeeting":{"ver":1,"type":"atlas"},"bowheEmojiMeeting":{"ver":1,"type":"atlas"},"bowsheEmojiMeeting":{"ver":1,"type":"atlas"},"cashEmojiMeeting":{"ver":1,"type":"atlas"},"champagneEmojiMeeting":{"ver":1,"type":"atlas"},"clapEmojiMeeting":{"ver":1,"type":"atlas"},"clockEmojiMeeting":{"ver":1,"type":"atlas"},"cocktailEmojiMeeting":{"ver":1,"type":"atlas"},"crazyEmojiMeeting":{"ver":1,"type":"atlas"},"cryEmojiMeeting":{"ver":1,"type":"atlas"},"eggplantEmojiMeeting":{"ver":1,"type":"atlas"},"fightEmojiMeeting":{"ver":1,"type":"atlas"},"fireEmojiMeeting":{"ver":1,"type":"atlas"},"flowersEmojiMeeting":{"ver":1,"type":"atlas"},"frozenEmojiMeeting":{"ver":1,"type":"atlas"},"glassEmojiMeeting":{"ver":1,"type":"atlas"},"handEmojiMeeting":{"ver":1,"type":"atlas"},"happyEmojiMeeting":{"ver":1,"type":"atlas"},"hilariousEmojiMeeting":{"ver":1,"type":"atlas"},"hugEmojiMeeting":{"ver":1,"type":"atlas"},"inloveEmojiMeeting":{"ver":1,"type":"atlas"},"kissEmojiMeeting":{"ver":1,"type":"atlas"},"likeEmojiMeeting":{"ver":1,"type":"atlas"},"lolEmojiMeeting":{"ver":1,"type":"atlas"},"lollypopEmojiMeeting":{"ver":1,"type":"atlas"},"loveEmojiMeeting":{"ver":1,"type":"atlas"},"normalEmojiMeeting":{"ver":1,"type":"atlas"},"oleEmojiMeeting":{"ver":1,"type":"atlas"},"partyEmojiMeeting":{"ver":1,"type":"atlas"},"peaceEmojiMeeting":{"ver":1,"type":"atlas"},"prayEmojiMeeting":{"ver":1,"type":"atlas"},"rainbowEmojiMeeting":{"ver":1,"type":"atlas"},"rocketEmojiMeeting":{"ver":1,"type":"atlas"},"sadEmojiMeeting":{"ver":1,"type":"atlas"},"seedEmojiMeeting":{"ver":1,"type":"atlas"},"silenceEmojiMeeting":{"ver":1,"type":"atlas"},"sleepEmojiMeeting":{"ver":1,"type":"atlas"},"snowEmojiMeeting":{"ver":1,"type":"atlas"},"sockyEmojiMeeting":{"ver":1,"type":"atlas"},"teethEmojiMeeting":{"ver":1,"type":"atlas"},"thinkEmojiMeeting":{"ver":1,"type":"atlas"},"valentinEmojiMeeting":{"ver":1,"type":"atlas"},"winkEmojiMeeting":{"ver":1,"type":"atlas"}}}},"characterA_avatarExtra_avatarExtra+guest":{"preview":{"ver":0,"type":"atlas"},"layers":{"avatarExtra":{"angryEmojiMeeting":{"ver":1,"type":"atlas"},"beerEmojiMeeting":{"ver":1,"type":"atlas"},"bowheEmojiMeeting":{"ver":1,"type":"atlas"},"bowsheEmojiMeeting":{"ver":1,"type":"atlas"},"cashEmojiMeeting":{"ver":2,"type":"atlas"},"champagneEmojiMeeting":{"ver":1,"type":"atlas"},"clapEmojiMeeting":{"ver":1,"type":"atlas"},"clockEmojiMeeting":{"ver":1,"type":"atlas"},"cocktailEmojiMeeting":{"ver":1,"type":"atlas"},"crazyEmojiMeeting":{"ver":1,"type":"atlas"},"cryEmojiMeeting":{"ver":1,"type":"atlas"},"eggplantEmojiMeeting":{"ver":1,"type":"atlas"},"fightEmojiMeeting":{"ver":1,"type":"atlas"},"fireEmojiMeeting":{"ver":1,"type":"atlas"},"flowersEmojiMeeting":{"ver":1,"type":"atlas"},"frozenEmojiMeeting":{"ver":1,"type":"atlas"},"glassEmojiMeeting":{"ver":1,"type":"atlas"},"handEmojiMeeting":{"ver":1,"type":"atlas"},"happyEmojiMeeting":{"ver":1,"type":"atlas"},"hilariousEmojiMeeting":{"ver":1,"type":"atlas"},"hugEmojiMeeting":{"ver":1,"type":"atlas"},"inloveEmojiMeeting":{"ver":1,"type":"atlas"},"kissEmojiMeeting":{"ver":1,"type":"atlas"},"likeEmojiMeeting":{"ver":1,"type":"atlas"},"lolEmojiMeeting":{"ver":1,"type":"atlas"},"lollypopEmojiMeeting":{"ver":1,"type":"atlas"},"loveEmojiMeeting":{"ver":1,"type":"atlas"},"negationEmojiMeeting":{"ver":1,"type":"atlas"},"nodEmojiMeeting":{"ver":1,"type":"atlas"},"normalEmojiMeeting":{"ver":1,"type":"atlas"},"oleEmojiMeeting":{"ver":1,"type":"atlas"},"partyEmojiMeeting":{"ver":1,"type":"atlas"},"peaceEmojiMeeting":{"ver":1,"type":"atlas"},"prayEmojiMeeting":{"ver":1,"type":"atlas"},"rainbowEmojiMeeting":{"ver":1,"type":"atlas"},"rocketEmojiMeeting":{"ver":1,"type":"atlas"},"sadEmojiMeeting":{"ver":1,"type":"atlas"},"seedEmojiMeeting":{"ver":1,"type":"atlas"},"silenceEmojiMeeting":{"ver":1,"type":"atlas"},"sleepEmojiMeeting":{"ver":1,"type":"atlas"},"snowEmojiMeeting

### 41.376 GET https://app.sowork.com/characters/v5/customizer_product_prices.json -> 200
RES: {"README":"THIS IS AN AUTOGENERATED FILE BY CI, DO NOT MODIFY","products":{"wings":{"name":{"en":"Wings"},"description":{"en":"TODO"},"currency":"usd","unitAmount":149},"moon":{"name":{"en":"The Deadline Moon"},"description":{"en":"TODO"},"currency":"usd","unitAmount":149},"vgicon":{"name":{"en":"Press start"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"grank":{"name":{"en":"Into the Technodrome"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"emojiMask":{"name":{"en":"Emoji Mask"},"description":{"en":"TODO"},"currency":"usd","unitAmount":99},"emojiMask2":{"name":{"en":"Emoji Mask set 2"},"description":{"en":"TODO"},"currency":"usd","unitAmount":99},"teamDog":{"name":{"en":"Dog mask"},"description":{"en":"TODO"},"currency":"usd","unitAmount":149},"teamCat":{"name":{"en":"Cat mask"},"description":{"en":"TODO"},"currency":"usd","unitAmount":149},"brazSoccer":{"name":{"en":"La Canarinha"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"spaSoccer":{"name":{"en":"La Roja"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"itaSoccer":{"name":{"en":"Gli Azzurri"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"argenSoccer":{"name":{"en":"La Albiceleste"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"gerSoccer":{"name":{"en":"Die Mannschaft"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"brazilSoccer":{"name":{"en":"Soccer Outfit"},"description":{"en":"TODO"},"currency":"usd","unitAmount":999},"corinthians":{"name":{"en":"corinthians"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"cruzeiro":{"name":{"en":"cruzeiro"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"flamingo":{"name":{"en":"flamingo"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"gremio":{"name":{"en":"gremio"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"palmeiras":{"name":{"en":"palmeiras"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"leonal":{"name":{"en":"Katana Turtle"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"donal":{"name":{"en":"Bo Staff Turtle"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"mikelal":{"name":{"en":"Nunchaku Turtle"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"rafal":{"name":{"en":"Nunchaku Turtle"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"dancingBanana":{"name":{"en":"Peanut Butter Jelly Time!"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"lsn":{"name":{"en":"Sugar, spice and everything nice"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"chocob":{"name":{"en":"Kue!!!"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"ashGb":{"name":{"en":"Become a Master"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"pachiko":{"name":{"en":"I choose you"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"saidack":{"name":{"en":"I choose you"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"charman":{"name":{"en":"I choose you"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"squilo":{"name":{"en":"I choose you"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"bilbosur":{"name":{"en":"I choose you"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"mecha":{"name":{"en":"One with the machine"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"sailorS":{"name":{"en":"In the name of the moon"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"afromonster":{"name":{"en":"afromonster"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"mistery":{"name":{"en":"FX specialist"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"octoOct":{"name":{"en":"Dr LongLegs"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"sopiper":{"name":{"en":"sopiper"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"vespa":{"name":{"en":"Vespa"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"fone":{"name":{"en":"Ka-Chow"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"motogp":{"name":{"en":"Pole!"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"sk8ter":{"name":{"en":"Tony Pigeon Amateur Skater"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"zombieDlc":{"name":{"en":"Braaaaaiiiinssss"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"futuHead":{"name":{"en":"Head in Jar!"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"spidman":{"name":{"en":"Your friendly teammate "},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"lowclo":{"name":{"en":"ex-SOLDIER First Class"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"lowsep":{"name":{"en":"One-Winged Angel"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"bugbuster":{"name":{"en":"Who ya gonna code?"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"sadako":{"name":{"en":"Seven days..."},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"colormap":{"name":{"en":"The prey"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"flyingWiz":{"name":{"en":"Magic sports"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"hoverskater":{"name":{"en":"Marty McHover"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"skaterPack":{"name":{"en":"M&T Skater Pack"},"description":{"en":"TODO"},"currency":"usd","unitAmount":399},"futuShip":{"name":{"en":"Happy 2999"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"dancingDragon":{"name":{"en":"Dancing Dragon"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"lamia":{"name":{"en":"Sliding body"},"description":{"en":"TODO"},"currency":"usd","unitAmount":299},"tank":{"name":{"en":"Like a Tank"},"description":{"en":"TODO"},"cu

### 41.376 POST https://api.sowork.com/api/v1/user/getIntercomUserHash -> 200
REQ: {"platform":"web"}
RES: {"userHash":"91fc88470dffc7b765b6a7013f3a1f5b2059d00d0c6ec5a08c146820c70b707b"}

### 41.376 GET https://api.sowork.com/api/v1/calendar/token -> 200
RES: {"token":"<JWT>"}

### 41.397 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65306&AID=8&zx=4medsjwi5tew&t=1 -> 200
REQ: count=1&ofs=2&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2FgroupPublic%2Fv0XvoIt3uK40pdF0mQDB%22%5D%7D%2C%22targetId%22%3A6%7D%7D
RES: 8
[1,9,7]


### 41.401 POST https://firebaseremoteconfig.googleapis.com/v1/projects/sage-learning-prod/namespaces/firebase:fetch?key=<REDACTED> -> 200
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

### 41.401 OPTIONS https://firebaseremoteconfig.googleapis.com/v1/projects/sage-learning-prod/namespaces/firebase:fetch?key=<REDACTED> -> 200
RES: 

### 60.084 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=10&CI=1&TYPE=xmlhttp&zx=8ulcip85e7kb&t=1 -> 200
RES: 

### 105.085 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=10&CI=1&TYPE=xmlhttp&zx=6bpqo4jo3w67&t=2 -> 200
RES: 

### 128.535 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=16&CI=1&TYPE=xmlhttp&zx=m40w9pxpn9hf&t=1 -> 200
RES: 

### 128.544 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65307&AID=16&zx=uhf5a58pv2ie&t=1 -> 200
REQ: count=1&ofs=3&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A6%7D
RES: 9
[1,17,7]


### 128.683 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65308&AID=17&zx=fa6e1e70bc5u&t=1 -> 200
REQ: count=5&ofs=4&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22groupJoinRequests%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22groupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22requestorUserId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A8%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22groupJoinRequests%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22pending%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22requestorUserId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A10%7D%7D&req2___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22groupJoinRequests%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22seenReqAnswer%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22booleanValue%22%3Afalse%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22requestorUserId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A12%7D%7D&req3___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22groupJoinRequests%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22groupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22pending%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A14%7D%7D&req4___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22external_group_access%22%7D%5D%2C%22where%22%3A%7B%22compositeFil
RES: 9
[0,18,7]


### 128.684 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=18&CI=1&TYPE=xmlhttp&zx=8gmc5k9eze8e&t=1 -> 200
RES: 505
[[19,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      8
    ]
  }
}
]],[20,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      10
    ]
  }
}
]],[21,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      12
    ]
  }
}
]],[22,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      14
    ]
  }
}
]],[23,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      16
    ]
  }
}
]]]

### 128.846 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=23&CI=1&TYPE=xmlhttp&zx=6zrrt7oe0g87&t=1 -> 200
RES: 1076
[[24,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      12
    ],
    "resumeToken": "CgkIh6+SmfallwM=",
    "readTime": "2026-10-06T17:32:48.452487Z"
  }
}
]],[25,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      14
    ],
    "resumeToken": "CgkIh6+SmfallwM=",
    "readTime": "2026-10-06T17:32:48.452487Z"
  }
}
]],[26,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      10
    ],
    "resumeToken": "CgkIh6+SmfallwM=",
    "readTime": "2026-10-06T17:32:48.452487Z"
  }
}
]],[27,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      16
    ],
    "resumeToken": "CgkIh6+SmfallwM=",
    "readTime": "2026-10-06T17:32:48.452487Z"
  }
}
]],[28,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      8
    ],
    "resumeToken": "CgkIh6+SmfallwM=",
    "readTime": "2026-10-06T17:32:48.452487Z"
  }
}
]],[29,[{
  "targetChange": {
    "resumeToken": "CgkIh6+SmfallwM=",
    "readTime": "2026-10-06T17:32:48.452487Z"
  }
}
]]]

### 128.983 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=29&CI=1&TYPE=xmlhttp&zx=dy7vcoo5sbz1&t=1 -> 200
RES: 102
[[30,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      18
    ]
  }
}
]]]

### 128.994 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65309&AID=29&zx=zcl33vuhkn0q&t=1 -> 200
REQ: count=1&ofs=9&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fgroups%2Fv0XvoIt3uK40pdF0mQDB%22%5D%7D%2C%22targetId%22%3A18%7D%7D
RES: 9
[0,29,7]


### 129.244 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65310&AID=29&zx=thm4pgayknyw&t=1 -> 200
REQ: count=6&ofs=10&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2FgroupPublic%2Fv0XvoIt3uK40pdF0mQDB%22%5D%7D%2C%22targetId%22%3A20%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2FgroupMembers_sys%2FRMUYRzBXvUOCqtJdxSAiMdiEZM82_v0XvoIt3uK40pdF0mQDB%22%5D%7D%2C%22targetId%22%3A22%7D%7D&req2___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fgroup_membermaps%2Fv0XvoIt3uK40pdF0mQDB_0%22%5D%7D%2C%22targetId%22%3A24%7D%7D&req3___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fgroup_membermaps%2Fv0XvoIt3uK40pdF0mQDB_1%22%5D%7D%2C%22targetId%22%3A26%7D%7D&req4___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fgroup_membermaps%2Fv0XvoIt3uK40pdF0mQDB_2%22%5D%7D%2C%22targetId%22%3A28%7D%7D&req5___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fgroup_membermaps%2Fv0XvoIt3uK40pdF0mQDB_3%22%5D%7D%2C%22targetId%22%3A30%7D%7D
RES: 9
[0,30,7]


### 129.245 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=30&CI=1&TYPE=xmlhttp&zx=ettw33t5nn0y&t=1 -> 200
RES: 

### 129.323 POST https://api.sowork.com/api/v1/reports/getWeeklyReportsMetadata -> 200
REQ: {"groupId":"v0XvoIt3uK40pdF0mQDB"}
RES: {"count":0,"hasReports":false}

### 129.962 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=49&CI=1&TYPE=xmlhttp&zx=u9elcut6b972&t=1 -> 200
RES: 3462
[[50,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/groupMembers_sys/RMUYRzBXvUOCqtJdxSAiMdiEZM82_v0XvoIt3uK40pdF0mQDB",
      "fields": {
        "userName": {
          "stringValue": "Tito Guidotti"
        },
        "role": {
          "stringValue": "contributor"
        },
        "lastOnline": {
          "timestampValue": "2026-08-10T12:32:37.974Z"
        },
        "groupId": {
          "stringValue": "v0XvoIt3uK40pdF0mQDB"
        },
        "deletedAt": {
          "nullValue": null
        },
        "user": {
          "mapValue": {
            "fields": {
              "email": {
                "stringValue": "tito@20dash.com"
              },
              "name": {
                "stringValue": "Tito Guidotti"
              }
            }
          }
        },
        "group": {
          "mapValue": {
            "fields": {
              "description": {
                "stringValue": ""
              },
              "privacy": {
                "stringValue": "private"
              },
              "name": {
                "stringValue": "20DASH"
              },
              "type": {
                "stringValue": "space"
              },
              "planType": {
                "stringValue": "basic"
              },
              "status": {
                "stringValue": "activated"
              }
            }
          }
        },
        "memberAt": {
          "timestampValue": "2026-08-10T12:32:37.974Z"
        },
        "membershipExpiryDate": {
          "nullValue": null
        },
        "deleted": {
          "booleanValue": false
        },
        "userId": {
          "stringValue": "RMUYRzBXvUOCqtJdxSAiMdiEZM82"
        },
        "knownAs": {
          "mapValue": {
            "fields": {
              "email": {
                "stringValue": "tito@20dash.com"
              },
              "name": {
                "stringValue": "tito"
              }
            }
          }
        },
        "lastUpdatedAt": {
          "timestampValue": "2026-08-10T12:32:37.974Z"
        },
        "createdAt": {
          "timestampValue": "2026-08-10T12:32:37.974Z"
        },
        "invite": {
          "mapValue": {
            "fields": {
              "autoFriend": {
                "booleanValue": false
              },
              "invitedBy": {
                "stringValue": "zE7SCh3eZDggMBNhjB5xkpeglNn2"
              },
              "invitedEmail": {
                "stringValue": "tito@20dash.com"
              },
              "inviteId": {
                "stringValue": "INV:SmpyD6NWoMv53K8yQKp3_v0XvoIt3uK40pdF0mQDB"
              },
              "expiresAt": {
                "integerValue": "1802262585697"
              },
              "type": {
                "stringValue": "email"
              }
            }
          }
        },
        "upgradeError": {
          "nullValue": null
        }
      },
      "createTime": "2026-08-10T12:32:38.094131Z",
      "updateTime": "2026-08-14T03:46:32.102251Z"
    },
    "targetIds": [
      22
    ]
  }
}
]],[51,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      22
    ],
    "resumeToken": "CgkI1s+1mfallwM=",
    "readTime": "2026-10-06T17:32:49.030102Z"
  }
}
]],[52,[{
  "targetChange": {
    "resumeToken": "CgkI1s+1mfallwM=",
    "readTime": "2026-10-06T17:32:49.030102Z"
  }
}
]]]

### 130.195 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=52&CI=1&TYPE=xmlhttp&zx=o7qaflxoldrb&t=1 -> 200
RES: 

### 130.534 GET https://nearest-media-region.l.chime.aws/ -> 200
RES: {"region": "eu-central-1"}

### 130.534 GET https://uy7tlymu.apicdn.sanity.io/v1/data/query/production?query=*%5B_type%20%3D%3D%20%27releases%27%5D%20%7C%20order(version%20desc)%5B0%5D%20%7B%0A%20%20%20%20version%2C%0A%20%20%20%20bannerLabel%2C%0A%20%20%20%20weekOfTitle%0A%20%20%7D -> 200
RES: {"query":"*[_type == 'releases'] | order(version desc)[0] {\n    version,\n    bannerLabel,\n    weekOfTitle\n  }","result":{"version":198,"bannerLabel":"NEW: A Smarter Command Palette, Seamless Screenshare Switching, Better Analytics, and Our 200th Web Release 🎉","weekOfTitle":"2026-10-04"},"ms":2}

### 130.581 GET https://api.sowork.com/eyekia/api/v1/category/ -> 200
RES: {"categories":[{"id":"Bl4KXcUjIeJHdoNHREcO","data":{"name":"desks","rank":0,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/jPi1QFQUlSda7bznZlNtc","defaultInteraction":"desk","admin":false}},{"id":"AqbUhOZx_FwtXREtAxyd","data":{"name":"smartObjects","rank":1,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/htlhqBzQN2tdd_Ol4xcOt","defaultInteraction":null,"admin":null}},{"id":"XlY1RnbeV-jsblgs6iJV","data":{"name":"seating","rank":2,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/WUZLdYym0OxbRc0ftPGCu","defaultInteraction":null,"admin":null}},{"id":"vSqvdbaroakKNLDVXkfd","data":{"name":"officeTools","rank":3,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/Rz9t6yLsub7H7TEP36Bgu","defaultInteraction":null,"admin":null}},{"id":"H3ywIaJ4kzUsAi9peXLO","data":{"name":"surfaces","rank":4,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/zB-hdKs8tKX8FZZxT2bVi","defaultInteraction":null,"admin":null}},{"id":"iw8UQhE0Ggcbuv__ch7k","data":{"name":"dividers","rank":5,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/j1EIYtpCpa8LdmEeeHd-o","defaultInteraction":null,"admin":null}},{"id":"L3RHfHPdA3wt3dQNVtjo","data":{"name":"decor","rank":6,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/Yih10JNxYZnGbNreVPEhk","defaultInteraction":null,"admin":null}},{"id":"Dzxt39P_NqVVFgjOWxPL","data":{"name":"nature","rank":7,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/vNlhbMK6LqhtOBAOqr4kS","defaultInteraction":null,"admin":null}},{"id":"fNy6bF2-J_r6WqpbUs4T","data":{"name":"rugs","rank":8,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/FB4irJzpv0mV2cFXF-Bh9","defaultInteraction":null,"admin":null}},{"id":"swnHXPv5zWiqZgGNyguj","data":{"name":"lighting","rank":9,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/QUWCKjmF3cfefc4O2g-vz","defaultInteraction":null,"admin":null}},{"id":"TwPOubhIoMnUIREt9EpB","data":{"name":"seasonal","rank":10,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/FP0qUfO3Oh5KeWgJqCNId","defaultInteraction":null,"admin":null}},{"id":"FDmZ5Rw8tXfB6Cf5phLx","data":{"name":"adminTools","rank":11,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/Pe1iP8-uK7IbFstOrSGLD","defaultInteraction":null,"admin":true}},{"id":"mOcxd-GpPKWuLU_y3o75","data":{"name":"games","rank":12,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/z8kNRKBBkkGx_yaAjvkto","defaultInteraction":null,"admin":null}},{"id":"M-NCxaZuNulcwHH1U4eu","data":{"name":"quickSets","rank":13,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/FCDyWDckEH2BwOGxmkaE-","defaultInteraction":null,"admin":null}},{"id":"y8rThFmt-Xj1idpZb7Ib","data":{"name":"custom","rank":14,"thumbnail":"https://mm-asset-images-staging.s3.us-east-2.amazonaws.com/categories/36RvsnctcazQuF3MRCxAd","defaultInteraction":null,"admin":null}}]}

### 130.582 GET https://api.sowork.com/api/v1/chat-v2/?groupId=v0XvoIt3uK40pdF0mQDB -> 200
RES: {"channels":[{"id":"019ff5fb-36d3-7e72-86fb-d921516249aa","group_id":"v0XvoIt3uK40pdF0mQDB","type":"direct","name":null,"description":null,"created_by":"admin","member_key":"RMUYRzBXvUOCqtJdxSAiMdiEZM82:Rxp3M2MJzra9wQJm0VkDfRSgJgt1","app_id":null,"conversation_id":null,"lifecycle":null,"meeting_started_at":null,"meeting_ended_at":null,"metadata":{"legacy":{"streamId":"!members-7FxDVbT0mo4NNzNo9B3xvoCMLI2R1jHnuqzuXiZVtY4","streamCid":"messaging:!members-7FxDVbT0mo4NNzNo9B3xvoCMLI2R1jHnuqzuXiZVtY4","streamName":"Kury","streamType":"messaging"}},"is_default":false,"is_locked":false,"auto_translate_enabled":false,"default_designated_at":null,"stale_at":null,"archived_at":null,"archived_by":null,"support_outreach_pending_at":null,"hidden":false,"last_message_at":"2026-08-12T12:38:38.550Z","created_at":"2026-08-12T12:38:29.331Z","updated_at":"2026-08-30T05:52:43.505Z","deleted_at":null,"membership":{"pinned_at":null,"unread_count":0,"unread_mention_count":0,"unread_broadcast_count":0,"notification_level":null,"muted_until":null,"role":"member","joined_at":"2026-08-12T12:38:29.338Z","via_team":false},"dm_member_user_ids":["Rxp3M2MJzra9wQJm0VkDfRSgJgt1"],"last_message":{"id":"019ff5fb-5ad6-7f25-98e1-7c3043eb9047","user_id":"Rxp3M2MJzra9wQJm0VkDfRSgJgt1","app_id":null,"type":"regular","parent_id":null,"text":"devo entrar?","created_at":"2026-08-12T12:38:38.550Z"}},{"id":"019fdd45-cafa-7ad9-be02-2fa77379428e","group_id":"v0XvoIt3uK40pdF0mQDB","type":"general","name":"general","description":null,"created_by":"zE7SCh3eZDggMBNhjB5xkpeglNn2","member_key":null,"app_id":null,"conversation_id":null,"lifecycle":null,"meeting_started_at":null,"meeting_ended_at":null,"metadata":null,"is_default":true,"is_locked":true,"auto_translate_enabled":false,"default_designated_at":"2026-08-07T17:29:26.522Z","stale_at":null,"archived_at":null,"archived_by":null,"support_outreach_pending_at":null,"hidden":false,"last_message_at":null,"created_at":"2026-08-07T17:29:26.522Z","updated_at":"2026-08-07T17:29:26.522Z","deleted_at":null,"membership":{"pinned_at":null,"unread_count":0,"unread_mention_count":0,"unread_broadcast_count":0,"notification_level":null,"muted_until":null,"role":"member","joined_at":"2026-08-10T12:32:38.504Z","via_team":false},"last_message":null}],"current_user_group_role":"contributor","has_more":false,"users":{"Rxp3M2MJzra9wQJm0VkDfRSgJgt1":{"name":"Kury","photo_url":"https://app.sowork.com/portrait/v1/avatar!SE/characterA!backAccessory:~fishingBackPack+brown!beard:~turk+black!body:~v1+typeA!bodyAccessory:~nateDrakeHolster+blonde!eyebrows:~eyebrows1+brown!eyes:~eyes1+grey!hair:~barret+black!mouth:~mouth1+typeA!pants:~bermudaLong+black!shirt:~alanShirt+blonde!shoes:~stepLuck+green?background=transparent"}},"scope_group_ids":["v0XvoIt3uK40pdF0mQDB","__global__"]}

### 130.582 GET https://api.sowork.com/api/v1/chat-v2/?groupId=__global__&actingGroupId=v0XvoIt3uK40pdF0mQDB -> 200
RES: {"channels":[],"current_user_group_role":null,"has_more":false}

### 130.582 GET https://api.sowork.com/api/v1/chat-v2/me/saved-message-ids?groupId=v0XvoIt3uK40pdF0mQDB -> 200
RES: {"ids":[]}

### 130.582 GET https://api.sowork.com/api/v1/chat-v2/me/saved-messages?groupId=v0XvoIt3uK40pdF0mQDB&limit=50 -> 200
RES: {"server_time":"2026-10-06T17:32:50.680Z","items":[],"users":{},"apps":{},"channels":{},"has_more":false,"next_cursor":null,"scope_group_ids":["v0XvoIt3uK40pdF0mQDB","__global__"]}

### 130.582 GET https://api.sowork.com/api/v1/chat-v2/support/me -> 200
RES: {"is_support_staff":false}

### 130.582 POST https://api.sowork.com/api/v1/conversation/searchDigests -> 200
REQ: {"groupId":"v0XvoIt3uK40pdF0mQDB","section":{"kind":"all"},"page":1,"pageSize":25,"readFilter":"all","grouped":true}
RES: {"apiVersion":5,"results":[],"pagination":{"page":1,"size":25,"totalCount":0}}

### 130.582 POST https://api.sowork.com/api/v1/gaia/find -> 200
REQ: {"type":"group","groupId":"v0XvoIt3uK40pdF0mQDB","alias":"mistymountainlodge","redirect":true}
RES: {"room":{"id":"YmAfBb3h2eY5r9FsAIfV","name":"Escritório principal","index":1,"local":false,"serverId":"g5","server":{"url":"https://api.sowork.com/gaia/g5"},"userIds":["JLDRGMskJRXiYcHccNVaLuwr3XJ3","rjOwBlMG93QD2sjutWoF2fc8Mv63","fCfzuTuBkBSLyF1tKdMJOZSOnUu1","G7EWVnLMQngko5v7KcgNnH06Fhg1","Rxp3M2MJzra9wQJm0VkDfRSgJgt1","blKsp8Fxi6VGJfXfizN0iu248IB2","IEwNgnRoYUcFOwc4KSrz7MyHNvG2","zE7SCh3eZDggMBNhjB5xkpeglNn2","FLBKjFg1PYe5bD6cE0bdR7BhE7r1","zdXi8bJLG6cNbd4P87bqTAzTdTq1"],"userCount":10,"roomDefinitionId":"yPsvwRdkb5LnGHfXFgfO","roomDefinition":{"ownerKey":"group_v0XvoIt3uK40pdF0mQDB","name":"Misty Mountain Lodge","groupId":"v0XvoIt3uK40pdF0mQDB","zones":null,"projection":null},"isMeetingSpace":false,"createdAt":{"seconds":1786123764,"nanoseconds":340000000},"updatedAt":{"seconds":1786123764,"nanoseconds":340000000},"roomGroupId":null},"roomDefinition":{"id":"yPsvwRdkb5LnGHfXFgfO","groupId":"v0XvoIt3uK40pdF0mQDB","name":"Escritório principal","alias":"mistymountainlodge","mapDefinitionId":"V1vAAHj80gty4MwBqEFp","ownerKey":"group_v0XvoIt3uK40pdF0mQDB","primary":true,"isMeetingSpace":false,"zones":null,"createdAt":{"seconds":1786123761,"nanoseconds":850000000},"updatedAt":{"seconds":1786123761,"nanoseconds":850000000},"roomObjects":{},"roomGroupId":null,"pricingType":"premium"},"roomGroupId":null}

### 130.582 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65311&AID=52&zx=xfy49i7xw48g&t=1 -> 200
REQ: count=1&ofs=16&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2FgroupMembers_integrations%2FRMUYRzBXvUOCqtJdxSAiMdiEZM82_v0XvoIt3uK40pdF0mQDB%22%5D%7D%2C%22targetId%22%3A32%7D%7D
RES: 9
[1,53,7]


### 130.710 POST https://api.sowork.com/api/v1/gaia/rooms -> 200
REQ: {"groupId":"v0XvoIt3uK40pdF0mQDB"}
RES: {"roomDefinitionMap":{"yPsvwRdkb5LnGHfXFgfO":{"roomDefinition":{"id":"yPsvwRdkb5LnGHfXFgfO","groupId":"v0XvoIt3uK40pdF0mQDB","name":"Escritório principal","alias":"mistymountainlodge","mapDefinitionId":"V1vAAHj80gty4MwBqEFp","ownerKey":"group_v0XvoIt3uK40pdF0mQDB","primary":true,"isMeetingSpace":false,"zones":null,"createdAt":{"seconds":1786123761,"nanoseconds":850000000},"updatedAt":{"seconds":1786123761,"nanoseconds":850000000},"roomObjects":{},"roomGroupId":null,"pricingType":"premium"},"rooms":[{"id":"YmAfBb3h2eY5r9FsAIfV","name":"Escritório principal","index":1,"local":false,"serverId":"g5","server":{"url":"https://api.sowork.com/gaia/g5"},"userIds":["JLDRGMskJRXiYcHccNVaLuwr3XJ3","rjOwBlMG93QD2sjutWoF2fc8Mv63","fCfzuTuBkBSLyF1tKdMJOZSOnUu1","G7EWVnLMQngko5v7KcgNnH06Fhg1","Rxp3M2MJzra9wQJm0VkDfRSgJgt1","blKsp8Fxi6VGJfXfizN0iu248IB2","IEwNgnRoYUcFOwc4KSrz7MyHNvG2","zE7SCh3eZDggMBNhjB5xkpeglNn2","FLBKjFg1PYe5bD6cE0bdR7BhE7r1","zdXi8bJLG6cNbd4P87bqTAzTdTq1"],"userCount":10,"roomDefinitionId":"yPsvwRdkb5LnGHfXFgfO","roomDefinition":{"ownerKey":"group_v0XvoIt3uK40pdF0mQDB","name":"Misty Mountain Lodge","groupId":"v0XvoIt3uK40pdF0mQDB","zones":null,"projection":null},"isMeetingSpace":false,"createdAt":{"seconds":1786123764,"nanoseconds":340000000},"updatedAt":{"seconds":1786123764,"nanoseconds":340000000},"roomGroupId":null}]}}}

### 130.797 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=54&CI=1&TYPE=xmlhttp&zx=bi7us0sogxrm&t=1 -> 200
RES: 592
[[55,[{
  "documentDelete": {
    "document": "projects/sage-learning-prod/databases/(default)/documents/groupMembers_integrations/RMUYRzBXvUOCqtJdxSAiMdiEZM82_v0XvoIt3uK40pdF0mQDB",
    "readTime": "2026-10-06T17:32:50.354600Z",
    "removedTargetIds": [
      32
    ]
  }
}
]],[56,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      32
    ],
    "resumeToken": "CgkIqLuGmvallwM=",
    "readTime": "2026-10-06T17:32:50.354600Z"
  }
}
]],[57,[{
  "targetChange": {
    "resumeToken": "CgkIqLuGmvallwM=",
    "readTime": "2026-10-06T17:32:50.354600Z"
  }
}
]]]

### 130.797 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65312&AID=54&zx=r13fnl3s6bgo&t=1 -> 200
REQ: count=15&ofs=17&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fgroup_desks%2Fv0XvoIt3uK40pdF0mQDB%22%5D%7D%2C%22targetId%22%3A34%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22digestTemplates%22%7D%5D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fgroups%2Fv0XvoIt3uK40pdF0mQDB%22%7D%2C%22targetId%22%3A36%7D%7D&req2___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FRMUYRzBXvUOCqtJdxSAiMdiEZM82%2Fprivate%2FdigestReadStates%22%5D%7D%2C%22targetId%22%3A38%7D%7D&req3___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22notifications%22%7D%5D%2C%22where%22%3A%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22removed%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22booleanValue%22%3Afalse%7D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22createdAt%22%7D%2C%22direction%22%3A%22DESCENDING%22%7D%2C%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22DESCENDING%22%7D%5D%2C%22limit%22%3A50%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers_sys%2FRMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%2C%22targetId%22%3A40%7D%7D&req4___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22strong_connections%22%7D%5D%2C%22where%22%3A%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22userId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%2C%22limit%22%3A1%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A42%7D%7D&req5___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fgroup_rooms_state%2Fv0XvoIt3uK40pdF0mQDB%22%5D%7D%2C%22targetId%22%3A44%7D%7D&req6___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22group_announce%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22sc%22%7D%2C%22op%22%3A%22ARRAY_CONTAINS_ANY%22%2C%22value%22%3A%7B%22arrayValue%22%3A%7B%22values%22%3A%5B%7B%22stringValue%22%3A%22user%3ARMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%2C%7B%22stringValue%22%3A%22group%3Av0XvoIt3uK40pdF0mQDB%3Acontributor%22%7D%5D%7D%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22targetGroupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22expiresAt%22%7D%2C%22op%22%3A%22GREATER_THAN%22%2C%22value%22%3A%7B%22timestampValue%22%3A%222026-10-06T17%3A32%3A49.972000000Z%22%7D%7D%7D%5D
RES: 9
[0,57,7]


### 130.797 GET https://api.sowork.com/eyekia/api/v1/asset/?categoryId=XlY1RnbeV-jsblgs6iJV&pathOnlyUrls=true -> 200
RES: {"assets":[{"id":"AAOgxlkS2YCBmOIXCmuH","data":{"name":"* 50s Chair","description":"","userHint":"","categoryId":"XlY1RnbeV-jsblgs6iJV","thumbnail":"/AAOgxlkS2YCBmOIXCmuH/t7pvqo4bFv_zn_02Ok15d","accessKey":"PREMIUM","defaultInteractionType":null,"colors":{"baseColor":"#48ab85ff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":197.03030303030303,"saturationMultiplier":1.62,"valueMultiplier":1}],"presetColorsDictionary":{"ZEgWNNav1lsUyLEi07IN":{"hueRotationDegrees":197.03030303030303,"saturationMultiplier":1.62,"valueMultiplier":1}}},"projections":{"isometric":true,"orthographic":false},"published":true,"modifiedSinceLastPublish":false,"orientations":{"N":{"goToPivot":{"x":0,"y":0},"images":{"JIEWyAHz8zQcjxsWvju-":{"url":"/JIEWyAHz8zQcjxsWvju-/g24cgU3MxZy0FD6wprwYw","transform":{"position":{"x":0,"y":0},"scale":{"x":1,"y":1},"rotationRad":0,"frontPoint":{"x":0,"y":-116.5},"depthBand":0},"type":"image","primitives":{"4HONNstsfOab7SCUeZ-Z":{"shape":{"position":{"x":99.22579669952393,"y":93.07657432556152},"shape":"rect","scale":{"x":137.45425987243652,"y":138.57228469848633},"rotationRad":0},"type":"shadow","shadowType":"rect"},"CNPikoGzgp4VwBHdmqB8":{"type":"seat","shape":{"shape":"rect","rotationRad":0,"position":{"y":5.869720458984375,"x":34.37971305847168},"scale":{"x":172.11334609985352,"y":117.32969665527344}},"facingDir":"N","shapeIntersectionType":"sensor","transform":{"scale":{"y":1,"x":1},"depthBand":0,"position":{"y":-41,"x":61},"frontPoint":{"x":0,"y":0}}},"BAnygGjNDaCybx1xDdxy":{"shape":{"rotationRad":0,"shape":"rect","scale":{"x":103.91320419311523,"y":100.5590934753418},"position":{"y":93.07648658752441,"x":99.22570991516113}},"height":159,"type":"shape","shapeIntersectionType":"selector"}},"blendMode":null}}},"E":{"goToPivot":{"x":0,"y":0},"images":{"EAyP_aViWNPEXesHwc1Y":{"url":"/EAyP_aViWNPEXesHwc1Y/HzQMGpGD5uvB3SwePxJvd","transform":{"position":{"x":0,"y":0},"scale":{"x":1,"y":1},"rotationRad":0,"frontPoint":{"x":0,"y":-116.5},"depthBand":0},"type":"image","primitives":{"gww_hBULRsxDJ7yh5GR9":{"height":163,"shapeIntersectionType":"selector","shape":{"rotationRad":0,"scale":{"y":105.59024810791016,"x":103.35419464111328},"position":{"x":128.57413864135742,"y":121.86593246459961},"shape":"rect"},"type":"shape"},"HCezKpgXdCgyjXBOfKdm":{"shadowType":"rect","type":"shadow","shape":{"position":{"x":127.45613479614258,"y":118.51179504394531},"shape":"rect","scale":{"y":143.60348510742188,"x":145.83956146240234},"rotationRad":0}},"UwZyvR5y8Tc5QarMwFs5":{"type":"seat","shapeIntersectionType":"sensor","shape":{"shape":"rect","scale":{"x":105.59023666381836,"y":177.1443862915039},"rotationRad":0,"position":{"x":10.062410354614258,"y":26.832881927490234}},"transform":{"scale":{"y":1,"x":1},"position":{"y":-127,"x":80},"frontPoint":{"y":0,"x":0},"depthBand":0},"facingDir":"E"}},"blendMode":null}}},"S":{"goToPivot":{"x":0,"y":0},"images":{"f0oncLsegpXMbHrpvU5z":{"url":"/f0oncLsegpXMbHrpvU5z/-y0mzVLwE1D229BPJFjvm","transform":{"position":{"x":0,"y":0},"scale":{"x":1,"y":1},"rotationRad":0,"frontPoint":{"x":0,"y":-134},"depthBand":0},"type":"image","primitives":{"s2KKWVqh7-2LRIP1J800":{"shape":{"rotationRad":0,"shape":"rect","position":{"x":117.39385604858398,"y":124.10199356079102},"scale":{"x":94.40982818603516,"y":96.6458969116211}},"height":195,"shapeIntersectionType":"selector","type":"shape"},"mFmvxI4W8kzrKGWAJ9wm":{"shape":{"scale":{"x":145.83955764770508,"y":148.07562255859375},"rotationRad":0,"position":{"y":131.92825317382812,"x":120.74788475036621},"shape":"rect"},"type":"shadow","shadowType":"rect"},"ggivqcoKvA60d6a8wObS":{"type":"seat","transform":{"depthBand":0,"scale":{"y":1,"x":1},"frontPoint":{"x":0,"y":0},"position":{"x":-69,"y":-121}},"shape":{"scale":{"y":116.21163940429688,"x":184.41168975830078},"position":{"x":20.963191986083984,"y":22.640243530273438},"rotationRad":0,"shape":"rect"},"shapeIntersectionType":"sensor","facingDir":"S"}},"blendMode":null}}},"W":{"goToPivot":{"x":0,"y":0},"images":{"SWZ5vxNL9aBD7K2K4xDf":{"url":"/SWZ5vxNL9aBD7K2K4xDf/65u9U7KC6-h0yPubQMSfJ","transform":{"position":{"x":0,"y":0},"scale":{"x":1,"y":1},"rotationRad":0,"frontPoint":{"x":0,"y":-134},"depthBand":0},"type":"image","primitives":{"JZlyPxQ-Jh33zXz4GLbG":{"height":160,"type":"shape","shape":{"rotationRad":0,"position":{"x":94.19456100463867,"y":100.34378051757812},"shape":"rect","scale":{"x":94.9688491821289,"y":91.61474609375}},"shapeIntersectionType":"selector"},"upcNeVnkWiS6Pxat00zs":{"shape":{"shape":"rect","position":{"x":35.21812963485718,"y":30.745994567871094},"rotationRad":0,"scale":{"x":89.93785572052002,"y":179.3805389404297}},"shapeIntersectionType":"sensor","transform":{"frontPoint":{"x":0,"y":0},"scale":{"x":1,"y":1},"position":{"y":-25,"x":-73},"depthBand":0},"type":"seat","facingDir":"W"},"gC1448JY836T9fgcgBuv":{"shape":{"scale":{"x":151.98867797851562,"y":144.16244888305664},"position":{"y":99.78472709655762,"x":91.39950561523438},"rotationRad":0,"shape":"rect"},"type":"shadow","shadowType":"rect"}},"blendMode":null}}}},"config":{"frames":{}}}},{"id":"P8H50lH2QwM6eXwUMwQP","data":{"name":"* 50s Chair Copy","description":"","userHint":"","categoryId":"XlY1RnbeV-jsblgs6iJV","thumbnail":"/AAOgxlkS2YCBmOIXCmuH/t7pvqo4bFv_zn_02Ok15d","accessKey":"PREMIUM","defaultInteractionType":null,"colors":{"baseColor":"#48ab85ff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":197.03030303030303,"saturationMultiplier":1.62,"valueMultiplier":1}],"presetColorsDictionary":{"ZEgWNNav1lsUyLEi07IN":{"hueRotationDegrees":197.03030303030303,"saturationMultiplier":1.62,"valueMultiplier":1}}},"projections":{"isometric":true,"orthographic":false},"published":true,"modifiedSinceLastPublish":true,"orientations":{"N":{"goToPivot":null,"images":{"JIEWyAHz8zQcjxsWvju-":{"url":"/JIEWyAHz8zQcjxsWvju-/g24cgU3MxZy0FD6wprwYw","transform":{"position":{"x":0,"y":0},"scale":{"x":1,"y":1},"rotationRad":0,"frontPoint":{"x":0,"y":-116.5},"depthBand":0},"type":"image","primitives":{"4

### 131.941 POST https://api.sowork.com/api/v1/user/updateUiSettings -> 200
REQ: {"userId":"RMUYRzBXvUOCqtJdxSAiMdiEZM82","data":{"mutedSounds":{"speechHigh":false,"shoot_1":false,"selectAsset":false,"shoot_2":false,"codeSuccess":false,"polaroid":false,"go":false,"countdown":false,"gong":false,"notification":false,"doorKnock":false,"speechLow":false,"deleteAsset":false,"editAsset":false,"raiseHand":false,"pencilLine":false,"battleMusic":false,"joinCall":true,"coworkingEnd1":true,"mapMakerMusic":true,"placeAsset":false,"applause":false,"requestToJoinPm":false,"lose":false,"coworkingStart2":true,"win":false,"hit":false,"jump":false,"ring":false,"textBroadcast":false,"receiveWave":false,"coworkingEnd3":true,"coworkingEnd2":true,"coworkingStart3":true,"rotateAsset":false,"shark_hitter":false,"sendWave":false,"disableVideo":false,"speechMid":false,"removeNotification":false,"leaveCall":true,"productUnlocked":false,"muteMic":false,"doorForceOpen":false,"coworkingStart1":true,"chatNotification":false,"unmuteMic":false,"privateMeeting":false,"calendarNotification":true,"enableVideo":false,"characterPicker":false,"cat_hitter":false,"duck_hitter":false,"participantJoined":false,"memo":true,"equip":false}}}
RES: {"success":true}

### 131.962 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=57&CI=1&TYPE=xmlhttp&zx=l0ofjl65svug&t=1 -> 200
RES: 49858
[[58,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      34
    ]
  }
}
]],[59,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      36
    ]
  }
}
]],[60,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      38
    ]
  }
}
]],[61,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      40
    ]
  }
}
]],[62,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      42
    ]
  }
}
]],[63,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      44
    ]
  }
}
]],[64,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      46
    ]
  }
}
]],[65,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      48
    ]
  }
}
]],[66,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      50
    ]
  }
}
]],[67,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      52
    ]
  }
}
]],[68,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      54
    ]
  }
}
]],[69,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      56
    ]
  }
}
]],[70,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      58
    ]
  }
}
]],[71,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      60
    ]
  }
}
]],[72,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      62
    ]
  }
}
]],[73,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/users/IEwNgnRoYUcFOwc4KSrz7MyHNvG2",
      "fields": {
        "xp": {
          "integerValue": "260"
        },
        "email": {
          "stringValue": "amanda@20dash.com"
        },
        "meetingHours": {
          "mapValue": {
            "fields": {
              "duration": {
                "integerValue": "0"
              },
              "durationExpiredAt": {
                "timestampValue": "2026-10-06T03:00:00.856Z"
              },
              "current": {
                "mapValue": {
                  "fields": {
                    "conversationId": {
                      "stringValue": "4Imdfu20mRQcsI7RKZIz"
                    },
                    "joinAt": {
                      "timestampValue": "2026-10-05T17:40:20Z"
                    }
                  }
                }
              }
            }
          }
        },
        "name": {
          "stringValue": "Amanda"
        },
        "character": {
          "mapValue": {
            "fields": {
              "slots": {
                "mapValue": {
                  "fields": {
                    "mouth": {
                      "stringValue": "characterA_mouth_mouth1+typeB"
                    },
                    "shoes": {
                      "stringValue": "characterA_shoes_crocs+brown"
                    },
                    "pants": {
                      "stringValue": "characterA_pants_bigJeans+grey"
                    },
                    "extraAccessory": {
                      "stringValue": "characterA_extraAccessory_gloves+brown"
                    },
                    "eyes": {
                      "stringValue": "characterA_eyes_eyes1+green"
                    },
                    "body": {
                      "stringValue": "characterA_body_v1+typeB"
                    },
                    "eyebrows": {
                      "stringValue": "characterA_eyebrows_eyebrows1+brown"
                    },
                    "hair": {
                      "stringValue": "characterA_hair_ophelia2+rainbow"
                    },
                    "hat": {
                      "stringValue": "characterA_hat_foneHelmet+pink"
                    },
                    "shirt": {
                      "stringValue": "characterA_shirt_punkFemaleJacket+red"
                    }
                  }
                }
              },
              "bodyType": {
                "stringValue": "characterA"
              },
              "version": {
                "integerValue": "5"
              }
            }
          }
        },
        "timezone": {
          "stringValue": "America/Sao_Paulo"
        },
        "birthday": {
          "mapValue": {
            "fields": {
              "enabled": {
                "booleanValue": true
              },
              "date": {
                "timestampValue": "2026-09-05T03:00:00Z"
              }
            }
          }
        }
      },
      "createTime": "2026-08-10T12:58:59.242059Z",
      "updateTime": "2026-10-05T17:40:21.384952Z"
    },
    "targetIds": [
      62
    ]
  }
}
]],[74,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      62
    ],
    "resumeToken": "CgkI6s2TmvallwM=",
    "readTime": "2026-10-06T17:32:50.569962Z"
  }
}
]],[75,[{
  "documentDelete": {
    "document": "projects/sage-learning-prod/databases/(default)/documents/users/RMUYRzBXvUOCqtJdxSAiMdiEZM82/private/digestReadStates",
    "readTime": "2026-10-06T17:32:50.569962Z",
    "removedTargetIds": [
      38
    ]
  }
}
]],[76,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      38
    ],
    "resumeToken": "CgkI6s2TmvallwM=",
    "readTime": "2026-10-06T17:32:50.569962Z"
  }
}
]],[77,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      48
    ],
    "resumeToken": "CgkI6s2TmvallwM=",
    "readTime": "2026-10-06T17:32:50.569962Z"
  }
}
]],[78,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/users/zE7SCh3eZDggMBNhjB5xkpeglNn2",
      "fields": {
        "groupRoles": {
          "mapValue": {
            "fields": {
              "ICvliDiy67rWNuJEzDR6": {
                "stringValue": "contributor"
              }
            }
          }
        },
   

### 131.964 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65313&AID=57&zx=ycrbpg4s4rqm&t=1 -> 200
REQ: count=1&ofs=32&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22conversations%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22ownerGroupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomDefinitionId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22yPsvwRdkb5LnGHfXFgfO%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22active%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22op%22%3A%22GREATER_THAN%22%2C%22value%22%3A%7B%22integerValue%22%3A%220%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%2C%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A64%7D%7D
RES: 9
[0,96,7]


### 131.972 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=96&CI=1&TYPE=xmlhttp&zx=1je49plaaz3l&t=1 -> 200
RES: 3604
[[97,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      64
    ]
  }
}
]],[98,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/conversations/xQiKXuEhyWgIu7i64QL1",
      "fields": {
        "privacy": {
          "stringValue": "public"
        },
        "createdAt": {
          "timestampValue": "2026-10-06T17:31:50.071Z"
        },
        "roomDefinitionId": {
          "stringValue": "yPsvwRdkb5LnGHfXFgfO"
        },
        "participants": {
          "mapValue": {}
        },
        "status": {
          "stringValue": "active"
        },
        "meetingChatId": {
          "nullValue": null
        },
        "onlineCount": {
          "integerValue": "2"
        },
        "ownerGroupId": {
          "stringValue": "v0XvoIt3uK40pdF0mQDB"
        },
        "livekitRoomId": {
          "stringValue": "RM_Tk7hEjc4aYtC"
        },
        "ownerId": {
          "nullValue": null
        },
        "sessionId": {
          "stringValue": "1e0bf030-44dd-46cb-8d0c-73dc09d1d292"
        },
        "users": {
          "mapValue": {
            "fields": {
              "zdXi8bJLG6cNbd4P87bqTAzTdTq1": {
                "mapValue": {
                  "fields": {
                    "joinAt": {
                      "timestampValue": "2026-10-06T17:31:51Z"
                    },
                    "name": {
                      "stringValue": "Stela Pagan"
                    },
                    "status": {
                      "stringValue": "online"
                    },
                    "lastJoinAt": {
                      "timestampValue": "2026-10-06T17:31:51Z"
                    }
                  }
                }
              },
              "fCfzuTuBkBSLyF1tKdMJOZSOnUu1": {
                "mapValue": {
                  "fields": {
                    "joinAt": {
                      "timestampValue": "2026-10-06T17:31:51Z"
                    },
                    "status": {
                      "stringValue": "online"
                    },
                    "lastJoinAt": {
                      "timestampValue": "2026-10-06T17:31:51Z"
                    },
                    "name": {
                      "stringValue": "Marie Cezar"
                    }
                  }
                }
              }
            }
          }
        },
        "sc": {
          "arrayValue": {
            "values": [
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:owner"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:contributor"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:guest"
              }
            ]
          }
        },
        "meetingChatProvider": {
          "stringValue": "sowork"
        },
        "zoneId": {
          "stringValue": "OgspgGHWe4AuPbgLCreJ"
        },
        "hasOnlineUsers": {
          "booleanValue": true
        },
        "roomId": {
          "stringValue": "YmAfBb3h2eY5r9FsAIfV"
        }
      },
      "createTime": "2026-10-06T17:31:50.155533Z",
      "updateTime": "2026-10-06T17:31:51.797826Z"
    },
    "targetIds": [
      64
    ]
  }
}
]],[99,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      64
    ],
    "resumeToken": "CgkIxOrYmvallwM=",
    "readTime": "2026-10-06T17:32:51.704132Z"
  }
}
]],[100,[{
  "targetChange": {
    "resumeToken": "CgkIxOrYmvallwM=",
    "readTime": "2026-10-06T17:32:51.704132Z"
  }
}
]]]

### 132.504 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65314&AID=96&zx=74urc5y12tdp&t=1 -> 200
REQ: count=7&ofs=33&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22conversations%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22ownerGroupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22unaryFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomDefinitionId%22%7D%2C%22op%22%3A%22IS_NULL%22%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22active%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22op%22%3A%22GREATER_THAN%22%2C%22value%22%3A%7B%22integerValue%22%3A%220%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%2C%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A66%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22conversations%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22ownerGroupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomDefinitionId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22yPsvwRdkb5LnGHfXFgfO%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22YmAfBb3h2eY5r9FsAIfV%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22active%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22op%22%3A%22GREATER_THAN%22%2C%22value%22%3A%7B%22integerValue%22%3A%220%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%2C%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A68%7D%7D&req2___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FRxp3M2MJzra9wQJm0VkDfRSgJgt1%22%5D%7D%2C%22targetId%22%3A70%7D%7D&req3___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FblKsp8Fxi6VGJfXfizN0iu248IB2%22%5D%7D%2C%22targetId%22%3A72%7D%7D&req4___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FG7EWVnLMQngko5v7KcgNnH06Fhg1%22%5D%7D%2C%22targetId%22%3A74%7D%7D&req5___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(de
RES: 10
[0,100,7]


### 132.506 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=100&CI=1&TYPE=xmlhttp&zx=gnlqrufl40ky&t=1 -> 200
RES: 715
[[101,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      66
    ]
  }
}
]],[102,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      68
    ]
  }
}
]],[103,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      70
    ]
  }
}
]],[104,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      72
    ]
  }
}
]],[105,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      74
    ]
  }
}
]],[106,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      76
    ]
  }
}
]],[107,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      78
    ]
  }
}
]]]

### 132.734 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=107&CI=1&TYPE=xmlhttp&zx=dg4bv5sbidtd&t=1 -> 200
RES: 51628
[[108,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/users/Rxp3M2MJzra9wQJm0VkDfRSgJgt1",
      "fields": {
        "character": {
          "mapValue": {
            "fields": {
              "slots": {
                "mapValue": {
                  "fields": {
                    "beard": {
                      "stringValue": "characterA_beard_turk+black"
                    },
                    "hair": {
                      "stringValue": "characterA_hair_barret+black"
                    },
                    "pants": {
                      "stringValue": "characterA_pants_bermudaLong+black"
                    },
                    "shoes": {
                      "stringValue": "characterA_shoes_stepLuck+green"
                    },
                    "mouth": {
                      "stringValue": "characterA_mouth_mouth1+typeA"
                    },
                    "body": {
                      "stringValue": "characterA_body_v1+typeA"
                    },
                    "eyes": {
                      "stringValue": "characterA_eyes_eyes1+grey"
                    },
                    "shirt": {
                      "stringValue": "characterA_shirt_alanShirt+blonde"
                    },
                    "bodyAccessory": {
                      "stringValue": "characterA_bodyAccessory_nateDrakeHolster+blonde"
                    },
                    "backAccessory": {
                      "stringValue": "characterA_backAccessory_fishingBackPack+brown"
                    },
                    "eyebrows": {
                      "stringValue": "characterA_eyebrows_eyebrows1+brown"
                    }
                  }
                }
              },
              "bodyType": {
                "stringValue": "characterA"
              },
              "version": {
                "integerValue": "5"
              }
            }
          }
        },
        "birthday": {
          "mapValue": {
            "fields": {
              "date": {
                "timestampValue": "2026-10-17T03:00:00Z"
              },
              "enabled": {
                "booleanValue": true
              }
            }
          }
        },
        "timezone": {
          "stringValue": "America/Sao_Paulo"
        },
        "xp": {
          "integerValue": "310"
        },
        "pets": {
          "mapValue": {
            "fields": {
              "pKWdV0wiu601x4MhA1AO": {
                "mapValue": {
                  "fields": {
                    "shouldSpawn": {
                      "booleanValue": false
                    }
                  }
                }
              }
            }
          }
        },
        "name": {
          "stringValue": "Kury"
        },
        "meetingHours": {
          "mapValue": {
            "fields": {
              "current": {
                "mapValue": {
                  "fields": {
                    "conversationId": {
                      "stringValue": "vNSK9dFjjr3tJhIX06DM"
                    },
                    "joinAt": {
                      "timestampValue": "2026-10-06T17:12:32Z"
                    }
                  }
                }
              },
              "durationExpiredAt": {
                "timestampValue": "2026-10-07T03:00:00.626Z"
              },
              "duration": {
                "integerValue": "0"
              }
            }
          }
        },
        "email": {
          "stringValue": "gabriel@20dash.com"
        }
      },
      "createTime": "2026-08-10T12:38:17.792013Z",
      "updateTime": "2026-10-06T17:12:33.076412Z"
    },
    "targetIds": [
      70
    ]
  }
}
]],[109,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      70
    ],
    "resumeToken": "CgkI4tn7mvallwM=",
    "readTime": "2026-10-06T17:32:52.275426Z"
  }
}
]],[110,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/users/blKsp8Fxi6VGJfXfizN0iu248IB2",
      "fields": {
        "name": {
          "stringValue": "Fabio"
        },
        "workingHours": {
          "mapValue": {
            "fields": {
              "ranges": {
                "mapValue": {
                  "fields": {
                    "7": {
                      "mapValue": {
                        "fields": {
                          "enabled": {
                            "booleanValue": false
                          },
                          "slots": {
                            "arrayValue": {
                              "values": [
                                {
                                  "mapValue": {
                                    "fields": {
                                      "start": {
                                        "stringValue": "09:00:00.000"
                                      },
                                      "end": {
                                        "stringValue": "17:00:00.000"
                                      },
                                      "slotId": {
                                        "stringValue": "811436cb-567c-4480-bcfc-2c50d11d4870"
                                      },
                                      "customStatus": {
                                        "nullValue": null
                                      }
                                    }
                                  }
                                }
                              ]
                            }
                          },
                          "range": {
                            "mapValue": {
                              "fields": {
                                "end": {
                                  "stringValue": "17:00:00.000"
                                },
                                "start": {
     

### 132.871 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=121&CI=1&TYPE=xmlhttp&zx=ro1zj1taa2ot&t=1 -> 200
RES: 

### 132.937 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65315&AID=121&zx=h3dmrp5grlc&t=1 -> 200
REQ: count=1&ofs=40&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FzdXi8bJLG6cNbd4P87bqTAzTdTq1%22%5D%7D%2C%22targetId%22%3A80%7D%7D
RES: 10
[1,122,7]


### 133.499 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=123&CI=1&TYPE=xmlhttp&zx=aebby8uey46z&t=1 -> 200
RES: 

### 133.720 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65316&AID=126&zx=dxypqjay1fu5&t=1 -> 200
REQ: count=1&ofs=41&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A80%7D
RES: 10
[0,126,7]


### 133.948 POST https://api.sowork.com/api/v1/gaia/listRoomByRoomDefinition -> 200
REQ: {"roomDefinitionId":"yPsvwRdkb5LnGHfXFgfO"}
RES: {"rooms":[{"id":"YmAfBb3h2eY5r9FsAIfV","name":"Escritório principal","index":1,"local":false,"serverId":"g5","server":{"url":"https://api.sowork.com/gaia/g5"},"userIds":["JLDRGMskJRXiYcHccNVaLuwr3XJ3","rjOwBlMG93QD2sjutWoF2fc8Mv63","fCfzuTuBkBSLyF1tKdMJOZSOnUu1","G7EWVnLMQngko5v7KcgNnH06Fhg1","Rxp3M2MJzra9wQJm0VkDfRSgJgt1","blKsp8Fxi6VGJfXfizN0iu248IB2","IEwNgnRoYUcFOwc4KSrz7MyHNvG2","zE7SCh3eZDggMBNhjB5xkpeglNn2","FLBKjFg1PYe5bD6cE0bdR7BhE7r1","zdXi8bJLG6cNbd4P87bqTAzTdTq1","RMUYRzBXvUOCqtJdxSAiMdiEZM82"],"userCount":11,"roomDefinitionId":"yPsvwRdkb5LnGHfXFgfO","roomDefinition":{"ownerKey":"group_v0XvoIt3uK40pdF0mQDB","name":"Misty Mountain Lodge","groupId":"v0XvoIt3uK40pdF0mQDB","zones":null,"projection":null},"isMeetingSpace":false,"createdAt":{"seconds":1786123764,"nanoseconds":340000000},"updatedAt":{"seconds":1786123764,"nanoseconds":340000000},"roomGroupId":null}]}

### 134.469 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=126&CI=1&TYPE=xmlhttp&zx=futrbzcfyway&t=1 -> 200
RES: 

### 134.471 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65317&AID=126&zx=x75n7gxz45ls&t=1 -> 200
REQ: count=6&ofs=42&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22broadcasts%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22started%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22groupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22unaryFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomDefinitionId%22%7D%2C%22op%22%3A%22IS_NULL%22%7D%7D%2C%7B%22unaryFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomId%22%7D%2C%22op%22%3A%22IS_NULL%22%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%2C%22limit%22%3A1%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A82%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22broadcasts%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22started%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22groupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomDefinitionId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22yPsvwRdkb5LnGHfXFgfO%22%7D%7D%7D%2C%7B%22unaryFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomId%22%7D%2C%22op%22%3A%22IS_NULL%22%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%2C%22limit%22%3A1%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A84%7D%7D&req2___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22broadcasts%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22started%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22groupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomDefinitionId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22yPsvwRdkb5LnGHfXFgfO%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22YmAfBb3h2eY5r9FsAIfV%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%2C%22limit%22%3A1%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A86%7D%7D&req3___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22doorbells%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3
RES: 10
[0,142,7]


### 134.730 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=142&CI=1&TYPE=xmlhttp&zx=cwgal8svkr&t=1 -> 200
RES: 2461
[[143,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      82
    ]
  }
}
]],[144,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      84
    ]
  }
}
]],[145,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      86
    ]
  }
}
]],[146,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      88
    ]
  }
}
]],[147,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      90
    ]
  }
}
]],[148,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      92
    ]
  }
}
]],[149,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      88
    ],
    "resumeToken": "CgkI3I30m/allwM=",
    "readTime": "2026-10-06T17:32:54.248156Z"
  }
}
]],[150,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      92
    ],
    "resumeToken": "CgkI3I30m/allwM=",
    "readTime": "2026-10-06T17:32:54.248156Z"
  }
}
]],[151,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      90
    ],
    "resumeToken": "CgkI3I30m/allwM=",
    "readTime": "2026-10-06T17:32:54.248156Z"
  }
}
]],[152,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      84
    ],
    "resumeToken": "CgkI3I30m/allwM=",
    "readTime": "2026-10-06T17:32:54.248156Z"
  }
}
]],[153,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      86
    ],
    "resumeToken": "CgkI3I30m/allwM=",
    "readTime": "2026-10-06T17:32:54.248156Z"
  }
}
]],[154,[{
  "targetChange": {
    "targetIds": [
      28,
      52,
      74,
      8,
      24,
      20,
      30,
      36,
      92,
      44,
      22,
      14,
      46,
      12,
      84,
      42,
      90,
      54,
      4,
      32,
      48,
      2,
      16,
      56,
      64,
      70,
      26,
      10,
      72,
      78,
      62,
      40,
      60,
      76,
      38,
      50,
      66,
      18,
      34,
      86,
      58,
      68,
      88
    ],
    "resumeToken": "CgkI3I30m/allwM=",
    "readTime": "2026-10-06T17:32:54.248156Z"
  }
}
]],[155,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      82
    ],
    "resumeToken": "CgkI3I30m/allwM=",
    "readTime": "2026-10-06T17:32:54.248156Z"
  }
}
]],[156,[{
  "targetChange": {
    "resumeToken": "CgkI3I30m/allwM=",
    "readTime": "2026-10-06T17:32:54.248156Z"
  }
}
]]]

### 134.901 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=156&CI=1&TYPE=xmlhttp&zx=x2fxh1vg2jay&t=1 -> 200
RES: 

### 135.885 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65318&AID=157&zx=wh6cy6cl5m43&t=1 -> 200
REQ: count=1&ofs=48&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A62%7D
RES: 10
[1,157,7]


### 136.020 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=158&CI=1&TYPE=xmlhttp&zx=hwbhw5fkx6s1&t=1 -> 200
RES: 106
[[159,[{
  "targetChange": {
    "targetChangeType": "REMOVE",
    "targetIds": [
      60
    ]
  }
}
]]]

### 136.021 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65319&AID=158&zx=f5m3czl7qa73&t=1 -> 200
REQ: count=1&ofs=49&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A60%7D
RES: 10
[0,158,7]


### 136.158 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=159&CI=1&TYPE=xmlhttp&zx=r3l7cwrrnekm&t=1 -> 200
RES: 

### 142.611 GET https://app.sowork.com/characters/v5/character_art_database.json -> 200
RES: {"README":"THIS IS A FILE THAT'S AUTOGENERATED BY THE CI / CD SYSTEM. DO NOT TOUCH!","lastUpdatedCommit":"79530061912db310dc167b166caaa5de72f22e29","assets":{"assistants_body_belfort+basecolor":{"preview":{"ver":1,"type":"atlas"},"layers":{"body":{"idle":{"ver":1,"type":"atlas"},"jump1":{"ver":1,"type":"atlas"},"sit":{"ver":1,"type":"atlas"},"walk":{"ver":1,"type":"atlas"},"work1":{"ver":1,"type":"atlas"},"work2":{"ver":1,"type":"atlas"},"work3":{"ver":1,"type":"atlas"},"work4":{"ver":1,"type":"atlas"},"work5":{"ver":1,"type":"atlas"},"work6":{"ver":1,"type":"atlas"},"work7":{"ver":1,"type":"atlas"},"working":{"ver":1,"type":"atlas"}}}},"assistants_body_gf+basecolor":{"preview":{"ver":1,"type":"atlas"},"layers":{"body":{"idle":{"ver":1,"type":"atlas"},"jump1":{"ver":1,"type":"atlas"},"sit":{"ver":1,"type":"atlas"},"walk":{"ver":1,"type":"atlas"},"work1":{"ver":1,"type":"atlas"},"work2":{"ver":1,"type":"atlas"},"work3":{"ver":1,"type":"atlas"},"work4":{"ver":1,"type":"atlas"},"work5":{"ver":1,"type":"atlas"},"work6":{"ver":1,"type":"atlas"},"work7":{"ver":1,"type":"atlas"},"working":{"ver":1,"type":"atlas"}}}},"assistants_body_mross+basecolor":{"preview":{"ver":1,"type":"atlas"},"layers":{"body":{"idle":{"ver":2,"type":"atlas"},"jump1":{"ver":1,"type":"atlas"},"sit":{"ver":1,"type":"atlas"},"walk":{"ver":1,"type":"atlas"},"work1":{"ver":1,"type":"atlas"},"work2":{"ver":2,"type":"atlas"},"work3":{"ver":2,"type":"atlas"},"work4":{"ver":2,"type":"atlas"},"work5":{"ver":1,"type":"atlas"},"work6":{"ver":1,"type":"atlas"},"work7":{"ver":1,"type":"atlas"},"working":{"ver":2,"type":"atlas"}}}},"cats_body_catTypeA+typeA":{"preview":{"ver":0,"type":"atlas"},"layers":{"body":{"idle":{"ver":2,"type":"atlas"},"sit":{"ver":2,"type":"atlas"},"walk":{"ver":2,"type":"atlas"}}}},"cats_body_catTypeA+typeB":{"preview":{"ver":0,"type":"atlas"},"layers":{"body":{"idle":{"ver":2,"type":"atlas"},"sit":{"ver":2,"type":"atlas"},"walk":{"ver":2,"type":"atlas"}}}},"characterA_avatarExtra_avatarExtra+basecolor":{"preview":{"ver":0,"type":"atlas"},"layers":{"avatarExtra":{"angryEmojiMeeting":{"ver":1,"type":"atlas"},"beerEmojiMeeting":{"ver":1,"type":"atlas"},"bowheEmojiMeeting":{"ver":1,"type":"atlas"},"bowsheEmojiMeeting":{"ver":1,"type":"atlas"},"cashEmojiMeeting":{"ver":1,"type":"atlas"},"champagneEmojiMeeting":{"ver":1,"type":"atlas"},"clapEmojiMeeting":{"ver":1,"type":"atlas"},"clockEmojiMeeting":{"ver":1,"type":"atlas"},"cocktailEmojiMeeting":{"ver":1,"type":"atlas"},"crazyEmojiMeeting":{"ver":1,"type":"atlas"},"cryEmojiMeeting":{"ver":1,"type":"atlas"},"eggplantEmojiMeeting":{"ver":1,"type":"atlas"},"fightEmojiMeeting":{"ver":1,"type":"atlas"},"fireEmojiMeeting":{"ver":1,"type":"atlas"},"flowersEmojiMeeting":{"ver":1,"type":"atlas"},"frozenEmojiMeeting":{"ver":1,"type":"atlas"},"glassEmojiMeeting":{"ver":1,"type":"atlas"},"handEmojiMeeting":{"ver":1,"type":"atlas"},"happyEmojiMeeting":{"ver":1,"type":"atlas"},"hilariousEmojiMeeting":{"ver":1,"type":"atlas"},"hugEmojiMeeting":{"ver":1,"type":"atlas"},"inloveEmojiMeeting":{"ver":1,"type":"atlas"},"kissEmojiMeeting":{"ver":1,"type":"atlas"},"likeEmojiMeeting":{"ver":1,"type":"atlas"},"lolEmojiMeeting":{"ver":1,"type":"atlas"},"lollypopEmojiMeeting":{"ver":1,"type":"atlas"},"loveEmojiMeeting":{"ver":1,"type":"atlas"},"normalEmojiMeeting":{"ver":1,"type":"atlas"},"oleEmojiMeeting":{"ver":1,"type":"atlas"},"partyEmojiMeeting":{"ver":1,"type":"atlas"},"peaceEmojiMeeting":{"ver":1,"type":"atlas"},"prayEmojiMeeting":{"ver":1,"type":"atlas"},"rainbowEmojiMeeting":{"ver":1,"type":"atlas"},"rocketEmojiMeeting":{"ver":1,"type":"atlas"},"sadEmojiMeeting":{"ver":1,"type":"atlas"},"seedEmojiMeeting":{"ver":1,"type":"atlas"},"silenceEmojiMeeting":{"ver":1,"type":"atlas"},"sleepEmojiMeeting":{"ver":1,"type":"atlas"},"snowEmojiMeeting":{"ver":1,"type":"atlas"},"sockyEmojiMeeting":{"ver":1,"type":"atlas"},"teethEmojiMeeting":{"ver":1,"type":"atlas"},"thinkEmojiMeeting":{"ver":1,"type":"atlas"},"valentinEmojiMeeting":{"ver":1,"type":"atlas"},"winkEmojiMeeting":{"ver":1,"type":"atlas"}}}},"characterA_avatarExtra_avatarExtra+guest":{"preview":{"ver":0,"type":"atlas"},"layers":{"avatarExtra":{"angryEmojiMeeting":{"ver":1,"type":"atlas"},"beerEmojiMeeting":{"ver":1,"type":"atlas"},"bowheEmojiMeeting":{"ver":1,"type":"atlas"},"bowsheEmojiMeeting":{"ver":1,"type":"atlas"},"cashEmojiMeeting":{"ver":2,"type":"atlas"},"champagneEmojiMeeting":{"ver":1,"type":"atlas"},"clapEmojiMeeting":{"ver":1,"type":"atlas"},"clockEmojiMeeting":{"ver":1,"type":"atlas"},"cocktailEmojiMeeting":{"ver":1,"type":"atlas"},"crazyEmojiMeeting":{"ver":1,"type":"atlas"},"cryEmojiMeeting":{"ver":1,"type":"atlas"},"eggplantEmojiMeeting":{"ver":1,"type":"atlas"},"fightEmojiMeeting":{"ver":1,"type":"atlas"},"fireEmojiMeeting":{"ver":1,"type":"atlas"},"flowersEmojiMeeting":{"ver":1,"type":"atlas"},"frozenEmojiMeeting":{"ver":1,"type":"atlas"},"glassEmojiMeeting":{"ver":1,"type":"atlas"},"handEmojiMeeting":{"ver":1,"type":"atlas"},"happyEmojiMeeting":{"ver":1,"type":"atlas"},"hilariousEmojiMeeting":{"ver":1,"type":"atlas"},"hugEmojiMeeting":{"ver":1,"type":"atlas"},"inloveEmojiMeeting":{"ver":1,"type":"atlas"},"kissEmojiMeeting":{"ver":1,"type":"atlas"},"likeEmojiMeeting":{"ver":1,"type":"atlas"},"lolEmojiMeeting":{"ver":1,"type":"atlas"},"lollypopEmojiMeeting":{"ver":1,"type":"atlas"},"loveEmojiMeeting":{"ver":1,"type":"atlas"},"negationEmojiMeeting":{"ver":1,"type":"atlas"},"nodEmojiMeeting":{"ver":1,"type":"atlas"},"normalEmojiMeeting":{"ver":1,"type":"atlas"},"oleEmojiMeeting":{"ver":1,"type":"atlas"},"partyEmojiMeeting":{"ver":1,"type":"atlas"},"peaceEmojiMeeting":{"ver":1,"type":"atlas"},"prayEmojiMeeting":{"ver":1,"type":"atlas"},"rainbowEmojiMeeting":{"ver":1,"type":"atlas"},"rocketEmojiMeeting":{"ver":1,"type":"atlas"},"sadEmojiMeeting":{"ver":1,"type":"atlas"},"seedEmojiMeeting":{"ver":1,"type":"atlas"},"silenceEmojiMeeting":{"ver":1,"type":"atlas"},"sleepEmojiMeeting":{"ver":1,"type":"atlas"},"snowEmojiMeeting

### 143.354 GET https://app.sowork.com/characters/v5/customizer_config.json -> 200
RES: {"README":"THIS IS AN AUTOGENERATED FILE BY CI, DO NOT MODIFY","bodyTypes":{"characterA":{"slots":{"backAccessory":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"category":{"id":"backAccessories","order":0},"items":{"bday":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"disabled":false,"accessKey":"PREMIUM","publishedAt":"2024-03-08T16:08:53.946Z","variations":{"basecolor":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"single","primary":"#d5b276ff"},"disabled":true},"blue":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":164,"saturationMultiplier":1.2,"valueMultiplier":1}}},"yellow":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":1.8,"valueMultiplier":0.9}}},"purple":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-120,"saturationMultiplier":1,"valueMultiplier":0.9}}},"red":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-44,"saturationMultiplier":2,"valueMultiplier":0.7}}},"green":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":81,"saturationMultiplier":1.2,"valueMultiplier":1}}}},"baseVariation":"basecolor"},"twentyfiveballoon":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"disabled":false,"accessKey":"PREMIUM","publishedAt":"2024-12-24T11:42:38.240Z","variations":{"basecolor":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"single","primary":"#d5b276ff"},"disabled":true},"blue":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":164,"saturationMultiplier":1.2,"valueMultiplier":1}}},"black":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":0,"valueMultiplier":0.35}}},"yellow":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":1.8,"valueMultiplier":0.9}}},"brown":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-20,"saturationMultiplier":1.5,"valueMultiplier":0.55}}},"orange":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-18,"saturationMultiplier":2,"valueMultiplier":0.85}}},"purple":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-120,"saturationMultiplier":1,"valueMultiplier":0.9}}},"red":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-44,"saturationMultiplier":2,"valueMultiplier":0.7}}},"turquoise":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":119,"saturationMultiplier":1,"valueMultiplier":1}}},"grey":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":0,"valueMultiplier":1}}},"white":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":0,"valueMultiplier":1.3}}},"green":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":81,"saturationMultiplier":1.2,"valueMultiplier":1}}}},"baseVariation":"basecolor"},"chineseLuck":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"disabled":false,"accessKey":"PREMIUM","publishedAt":"2025-01-23T20:36:35.648Z","variations":{"basecolor":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"single","primary":"#d5b276ff"},"disabled":true},"yellow":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":1,"valueMultiplier":1}}}},"baseVariation":"basecolor"},"chooseyou":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"disabled":false,"accessKey":"PREMIUM","publishedAt":"2025-02-07T16:24:23.811Z","variations":{"basecolor":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"single","primary":"#d5b276ff"},"disabled":true},"blue":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":1,"valueMultiplier":1}}}},"baseVariation":"basecolor"},"guitar":{"name":{"en":{"singular":"TODO","plural":"TODO"}},"disabled":false,"publishedAt":"2021-11-08T23:00:00.000Z","variations":{"brown":{"disabled":false,"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-20,"saturationMultiplier":1.5,"valueMultiplier":0.55}}},"basecolor":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"single","primary":"#d5b276ff"},"disabled":true},"blue":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":164,"saturationMultiplier":1.2,"valueMultiplier":1}}},"black":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":0,"valueMultiplier":0.35}}},"yellow":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":0,"saturationMultiplier":1.8,"valueMultiplier":0.9}}},"orange":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-18,"saturationMultiplier":2,"valueMultiplier":0.85}}},"purple":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-120,"saturationMultiplier":1,"valueMultiplier":0.9}}},"red":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transform","variation":{"hueRotationDegrees":-44,"saturationMultiplier":2,"valueMultiplier":0.7}}},"turquoise":{"name":{"en":"TODO"},"desc":{"en":"TODO"},"swatch":{"type":"transf

### 166.188 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=161&CI=1&TYPE=xmlhttp&zx=rv4tat9vtjm5&t=1 -> 200
RES: 

### 179.627 GET https://app.sowork.com/eyekia/api/v1/asset/1W0cx9o1GIeLOJID4Fqb?pathOnlyUrls=true -> 200
RES: {"name":"The Desk That Isn’t","description":"For people who refuse to desk. Looks like nothing, acts like a desk. Trust us, it works.","userHint":"","categoryId":"Bl4KXcUjIeJHdoNHREcO","thumbnail":"/kOjGlbkmAMxdhmbcZf0b/3LUAnrqTGJ0EEENdohN4O","accessKey":"","defaultInteractionType":"desk","colors":{"baseColor":"#e8cf00ff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":151.4655172413793,"saturationMultiplier":1,"valueMultiplier":1},{"hueRotationDegrees":291.4655172413793,"saturationMultiplier":0.2,"valueMultiplier":1.4},{"hueRotationDegrees":62.044464609800364,"saturationMultiplier":0.9090909090909091,"valueMultiplier":0.8196078431372549},{"hueRotationDegrees":208.4655172413793,"saturationMultiplier":1,"valueMultiplier":1},{"hueRotationDegrees":-53.53448275862069,"saturationMultiplier":1.450199203187251,"valueMultiplier":0.984313725490196},{"hueRotationDegrees":165.9777123633305,"saturationMultiplier":1.0081967213114753,"valueMultiplier":0.9568627450980393},{"hueRotationDegrees":-15.673520191775772,"saturationMultiplier":1.4841269841269842,"valueMultiplier":0.9882352941176471},{"hueRotationDegrees":-53.53448275862069,"saturationMultiplier":0,"valueMultiplier":0.6627450980392157},{"hueRotationDegrees":109.15782493368701,"saturationMultiplier":0.5279187817258884,"valueMultiplier":0.7725490196078432},{"hueRotationDegrees":0,"saturationMultiplier":0,"valueMultiplier":0.3}],"presetColorsDictionary":{"7UOELZ3loEyy4r1PbxjD":{"hueRotationDegrees":291.4655172413793,"saturationMultiplier":0.2,"valueMultiplier":1.4},"3UYK2Xi__VyeQ0MXpkIr":{"hueRotationDegrees":151.4655172413793,"saturationMultiplier":1,"valueMultiplier":1},"7f56dtvmOyesfEIhCZPb":{"hueRotationDegrees":62.044464609800364,"saturationMultiplier":0.9090909090909091,"valueMultiplier":0.8196078431372549},"Q0VsqnfZrZoHzBdvgPQb":{"hueRotationDegrees":-53.53448275862069,"saturationMultiplier":1.450199203187251,"valueMultiplier":0.984313725490196},"qSaMu4LkYPW_4BXEldqc":{"hueRotationDegrees":109.15782493368701,"saturationMultiplier":0.5279187817258884,"valueMultiplier":0.7725490196078432},"euJ5ySihIxBrw92nDxwD":{"hueRotationDegrees":-15.673520191775772,"saturationMultiplier":1.4841269841269842,"valueMultiplier":0.9882352941176471},"fCrmnj31doijJbDdLdxw":{"hueRotationDegrees":-53.53448275862069,"saturationMultiplier":0,"valueMultiplier":0.6627450980392157},"rGunPAhXwfj5T4Y0CkpC":{"hueRotationDegrees":0,"saturationMultiplier":0,"valueMultiplier":0.3},"L8RYIgk__v5arCHc86wT":{"hueRotationDegrees":208.4655172413793,"saturationMultiplier":1,"valueMultiplier":1},"eGd02gT53Ga_UU3vcjlD":{"hueRotationDegrees":165.9777123633305,"saturationMultiplier":1.0081967213114753,"valueMultiplier":0.9568627450980393}}},"projections":{"isometric":true,"orthographic":false},"published":true,"modifiedSinceLastPublish":false,"orientations":{"N":{"goToPivot":{"x":0,"y":0},"images":{"89aUzN0VYIgMASVTHcRG":{"url":"/kOjGlbkmAMxdhmbcZf0b/8IKFnRqPtzqooyxE5S5La","transform":{"position":{"x":0,"y":0},"scale":{"x":1,"y":1},"rotationRad":0,"frontPoint":{"x":-0.333038330078125,"y":3.264129638671875},"depthBand":-1},"type":"image","primitives":{},"blendMode":null},"xSNMvoWQPwLyQmYN6xe9":{"url":"/xSNMvoWQPwLyQmYN6xe9/oHsMrzRYXj5drcfu9lIzS","transform":{"position":{"x":0,"y":0},"scale":{"x":1,"y":1},"rotationRad":0,"frontPoint":{"x":0,"y":0},"depthBand":0},"type":"image","primitives":{"p6Oi6dtBp-eSwAy6_gGv":{"shape":{"rotationRad":0,"scale":{"x":127.35382843017578,"y":124.00440216064453},"position":{"x":5.303295135498047,"y":6.978008270263672},"shape":"rect"},"height":31.095836764966634,"shapeIntersectionType":"selector","type":"shape"}},"blendMode":null}}}},"config":{"frames":{}}}

### 179.627 GET https://app.sowork.com/eyekia/api/v1/floor/E5xP37vJSqXDYogbZtF2?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/floor/E5xP37vJSqXDYogbZtF2/5Rf9WTghHwr3FP3XGfn9v","pivotPosition":{"x":110,"y":67}},"E":{"url":"/floor/E5xP37vJSqXDYogbZtF2/em2Itlqzq1GQG0RpIwomy","pivotPosition":{"x":110,"y":67}}},"type":"floor","height":"","pixelDensity":211,"isCorner":false,"colors":{"baseColor":"#9ba27eff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"","name":"Moss","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 179.627 GET https://app.sowork.com/eyekia/api/v1/floor/QWaiSda3qH5Rf1OHQlQG?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/floor/QWaiSda3qH5Rf1OHQlQG/JvA6ecfq7_W8q2BOcTDJm","pivotPosition":{"x":110,"y":67}},"E":{"url":"/floor/QWaiSda3qH5Rf1OHQlQG/rIH4zNHUNtdSWR9yHAGc6","pivotPosition":{"x":110,"y":67}},"S":{"url":"/floor/QWaiSda3qH5Rf1OHQlQG/sbGhUCZKgLZ2vaUszo12y","pivotPosition":{"x":110,"y":67}},"W":{"url":"/floor/QWaiSda3qH5Rf1OHQlQG/t41ps6h8YvGk48DmkspOo","pivotPosition":{"x":110,"y":67}}},"type":"floor","height":"","pixelDensity":211,"isCorner":false,"colors":{"baseColor":"#979176ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"","name":"Moss to Dirt Straight","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 179.627 GET https://app.sowork.com/eyekia/api/v1/floor/ivRu7ugLZtuj99UGC0ZS?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/floor/ivRu7ugLZtuj99UGC0ZS/Wj8RnNShh-bdvtHOqS7hr","pivotPosition":{"x":110,"y":67}},"E":{"url":"/floor/ivRu7ugLZtuj99UGC0ZS/qPiKqHhdJLrTcSYlPjMY2","pivotPosition":{"x":110,"y":67}},"S":{"url":"/floor/ivRu7ugLZtuj99UGC0ZS/qP_BZy_BxZIW826qNriHE","pivotPosition":{"x":110,"y":67}},"W":{"url":"/floor/ivRu7ugLZtuj99UGC0ZS/sOfnrckCyWBaePBOuEdnD","pivotPosition":{"x":110,"y":67}}},"type":"floor","height":"","pixelDensity":211,"isCorner":false,"colors":{"baseColor":"#9a9d7bff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"","name":"Moss to Dirt Inside Corner","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 179.627 GET https://app.sowork.com/eyekia/api/v1/floor/fO3Bcpb1hQA3xqaqAu3w?pathOnlyUrls=true -> 404
RES: {"statusCode":404,"code":"not-found","error":"Not Found","message":"floor fO3Bcpb1hQA3xqaqAu3w could not be found."}

### 179.627 GET https://app.sowork.com/eyekia/api/v1/floor/V3CVBBA0KMmSmxpdClc2?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/floor/V3CVBBA0KMmSmxpdClc2/t_5uNwucQSlvWpdw53t_h","pivotPosition":{"x":110,"y":67}},"E":{"url":"/floor/V3CVBBA0KMmSmxpdClc2/xQbh6MQyqF9jYF2VWZtST","pivotPosition":{"x":110,"y":67}},"S":{"url":"/floor/V3CVBBA0KMmSmxpdClc2/c9WzbPYyllPnX31jzArO1","pivotPosition":{"x":110,"y":67}},"W":{"url":"/floor/V3CVBBA0KMmSmxpdClc2/vqO2n0OpDt8nAVjt8QA9v","pivotPosition":{"x":110,"y":67}}},"type":"floor","height":"","pixelDensity":211,"isCorner":false,"colors":{"baseColor":"#9ca17eff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"","name":"Moss Rounded Corder","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 179.627 GET https://app.sowork.com/eyekia/api/v1/floor/GTZ5vz4MCBe2Fi7dhxZN?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/floor/GTZ5vz4MCBe2Fi7dhxZN/1iKVhgx48n3iIujn_W3MX","pivotPosition":{"x":220,"y":134}},"S":{"url":"/floor/GTZ5vz4MCBe2Fi7dhxZN/QJWQ86Cafk7OidMLFHpIH","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#bb8e53ff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":-1.03846153846154,"saturationMultiplier":0.5,"valueMultiplier":1.1},{"hueRotationDegrees":0,"saturationMultiplier":1.2,"valueMultiplier":0.8},{"hueRotationDegrees":-6.03846153846154,"saturationMultiplier":0.86,"valueMultiplier":1}],"presetColorsDictionary":{"kA-rWLO5lWnUTR1vruaQ":{"hueRotationDegrees":-1.03846153846154,"saturationMultiplier":0.5,"valueMultiplier":1.1},"yf8jBuzfbt47SD8WakwE":{"hueRotationDegrees":-6.03846153846154,"saturationMultiplier":0.86,"valueMultiplier":1},"qhka-msVVkfbC907_eS8":{"hueRotationDegrees":0,"saturationMultiplier":1.2,"valueMultiplier":0.8}}},"accessKey":"","name":"Wood Floor06","description":"SM_2m_Floor_Wood_06a","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 179.627 GET https://app.sowork.com/eyekia/api/v1/wall/3ccOJmgDjXL7px0PJii5?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/wall/3ccOJmgDjXL7px0PJii5/KPTLLOR1X6tcmsNxuYKVI","pivotPosition":{"x":256,"y":134}},"E":{"url":"/wall/3ccOJmgDjXL7px0PJii5/-mTzMWVtXWDk8HsoY2FvW","pivotPosition":{"x":256,"y":134}},"S":{"url":"/wall/3ccOJmgDjXL7px0PJii5/YH1f30bAZ0cOJCH_XNmPs","pivotPosition":{"x":14,"y":134}},"W":{"url":"/wall/3ccOJmgDjXL7px0PJii5/5nIe7NfR9obl98kPFf3jD","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"1M","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#815221ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Cabin Wood Wall012 Tiny","description":"Cabin Wood 01 Wall 1m","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 179.627 GET https://app.sowork.com/eyekia/api/v1/wall/QxUlJJdYyM4obrTSLWVc?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/wall/QxUlJJdYyM4obrTSLWVc/8NUgvPi1_QLSIDPRc8QRD","pivotPosition":{"x":256,"y":134}},"W":{"url":"/wall/QxUlJJdYyM4obrTSLWVc/iqJaJWHMv5jg5OhWUgZF5","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"6m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#77828eff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Cabin Glass Wall015 Tall A Far Left","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 179.627 GET https://app.sowork.com/eyekia/api/v1/wall/NZoKxve6xpCxDlSrwo2G?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/wall/NZoKxve6xpCxDlSrwo2G/XztMOagUjW3lEpAUoEOMt","pivotPosition":{"x":256,"y":134}},"W":{"url":"/wall/NZoKxve6xpCxDlSrwo2G/tWESSx3ydyfbu4T8WXi1m","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"6m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#798490ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Cabin Glass Wall015 Tall A Middle Left","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 179.627 GET https://app.sowork.com/eyekia/api/v1/wall/KV9ygsHV6rAMFPEZjON6?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/wall/KV9ygsHV6rAMFPEZjON6/UDxysVm3ZmNJoI-x_S0cJ","pivotPosition":{"x":256,"y":134}},"W":{"url":"/wall/KV9ygsHV6rAMFPEZjON6/-hvSkoZSVEPtxwcLfUtVS","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"6m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#808d99ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Cabin Glass Wall015 Tall A Center","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 179.627 GET https://app.sowork.com/eyekia/api/v1/wall/svaiH50qMbUeFZcNAhGW?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/wall/svaiH50qMbUeFZcNAhGW/KKvmODBTnGXLTws0PZGau","pivotPosition":{"x":256,"y":134}},"W":{"url":"/wall/svaiH50qMbUeFZcNAhGW/3Ifis8wLABGsuLvbCcwce","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"6m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#798692ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Cabin Glass Wall015 Tall A Middle Right","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 179.627 GET https://app.sowork.com/eyekia/api/v1/wall/SyPKTxVfB6hexSmSuQQK?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/wall/SyPKTxVfB6hexSmSuQQK/GpPh9-UJKx8fCy_jnUpUC","pivotPosition":{"x":256,"y":134}},"W":{"url":"/wall/SyPKTxVfB6hexSmSuQQK/I1vqLP2RUqQCwTtmNsGt3","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"6m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#778490ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Cabin Glass Wall015 Tall A Far Right","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 179.627 GET https://app.sowork.com/eyekia/api/v1/wall/13AVKZ5OqX8hlpq5PEhZ?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/wall/13AVKZ5OqX8hlpq5PEhZ/bzTM2xn_44OfoNVmIKwRZ","pivotPosition":{"x":256,"y":134}},"E":{"url":"/wall/13AVKZ5OqX8hlpq5PEhZ/nZLRrUgDfvy5xH7J4F9dD","pivotPosition":{"x":256,"y":134}},"S":{"url":"/wall/13AVKZ5OqX8hlpq5PEhZ/ak6KtutT9280Hf9XrsLRl","pivotPosition":{"x":14,"y":134}},"W":{"url":"/wall/13AVKZ5OqX8hlpq5PEhZ/Bf6TnIwkhi4g-XIBSEd9r","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"6m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#8e581eff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Cabin Wood Wall015 Tall A","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 179.627 GET https://app.sowork.com/eyekia/api/v1/wall/nud1cv7lst8S5QjRM4jS?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/wall/nud1cv7lst8S5QjRM4jS/CodsBPGcQBuXvd-fN18M8","pivotPosition":{"x":256,"y":134}},"E":{"url":"/wall/nud1cv7lst8S5QjRM4jS/HE5u3eqlmYotdQ8HjMGlI","pivotPosition":{"x":256,"y":134}},"S":{"url":"/wall/nud1cv7lst8S5QjRM4jS/zrdbylp02CxGFuWa2cS9p","pivotPosition":{"x":14,"y":134}},"W":{"url":"/wall/nud1cv7lst8S5QjRM4jS/aA2SrZaN_8BY1u19Tu1Ys","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"0.15m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#FFFFFFFF","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":137,"saturationMultiplier":2.5,"valueMultiplier":4}],"presetColorsDictionary":{"0LwYFoXUKIUtCxDC_1II":{"hueRotationDegrees":137,"saturationMultiplier":2.5,"valueMultiplier":4}}},"accessKey":"","name":"Glass Wall011 Border","description":"Glass 01 Wall Border","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 179.627 GET https://app.sowork.com/eyekia/api/v1/wall/qLEkw2xDZXDrzL5lAAzB?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/wall/qLEkw2xDZXDrzL5lAAzB/GRBCLtuqkYesa_laGQXsW","pivotPosition":{"x":256,"y":134}},"E":{"url":"/wall/qLEkw2xDZXDrzL5lAAzB/lLPuqAiaFqxo7GrWrMyfC","pivotPosition":{"x":256,"y":134}},"S":{"url":"/wall/qLEkw2xDZXDrzL5lAAzB/MDkPSiRgdEJf4fqV6DU0S","pivotPosition":{"x":14,"y":134}},"W":{"url":"/wall/qLEkw2xDZXDrzL5lAAzB/-lSxLv4pFNKOdasefIeIv","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"6m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#826340ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Cabin Wood Wall015 Tall Window A","description":"Cabin Wood 01 Wall Window A 6m","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 179.627 GET https://app.sowork.com/eyekia/api/v1/wall/rjPVjv9zS5DPkJ1nDGpw?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/wall/rjPVjv9zS5DPkJ1nDGpw/tPWWkcA4uVQMk-R2XWSoY","pivotPosition":{"x":256,"y":134}},"E":{"url":"/wall/rjPVjv9zS5DPkJ1nDGpw/gqs_5yXSrkHfQkkAOakAL","pivotPosition":{"x":256,"y":134}},"S":{"url":"/wall/rjPVjv9zS5DPkJ1nDGpw/lFVdnFeItI-sNBTnylCfP","pivotPosition":{"x":14,"y":134}},"W":{"url":"/wall/rjPVjv9zS5DPkJ1nDGpw/EGYKJJIH9VtWKDI65q-OB","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"6m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#7a8692ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Cabin Glass Wall015 Tall A","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 179.627 GET https://app.sowork.com/eyekia/api/v1/wall/gYc0iQJU4fsSOzoaslpS?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/wall/gYc0iQJU4fsSOzoaslpS/6HUqdsBhk2n-nx-sC8ZDN","pivotPosition":{"x":220,"y":134}},"E":{"url":"/wall/gYc0iQJU4fsSOzoaslpS/KyJ4GVqdJXimL0Capoqm9","pivotPosition":{"x":220,"y":134}},"S":{"url":"/wall/gYc0iQJU4fsSOzoaslpS/8CC9V1cJ6sUJszU2dpPyD","pivotPosition":{"x":220,"y":134}},"W":{"url":"/wall/gYc0iQJU4fsSOzoaslpS/ACb33ZMz7gKmSXA4hDS37","pivotPosition":{"x":220,"y":134}}},"type":"wall","height":"0m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#e4d46aff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":107.8688524590164,"saturationMultiplier":0.88,"valueMultiplier":0.1},{"hueRotationDegrees":161.8688524590164,"saturationMultiplier":0.48,"valueMultiplier":0.28},{"hueRotationDegrees":24.868852459016395,"saturationMultiplier":0,"valueMultiplier":0},{"hueRotationDegrees":-52.131147540983605,"saturationMultiplier":0,"valueMultiplier":1.25},{"hueRotationDegrees":181.8688524590164,"saturationMultiplier":0.52,"valueMultiplier":0.15},{"hueRotationDegrees":-33.131147540983605,"saturationMultiplier":0.66,"valueMultiplier":0.38}],"presetColorsDictionary":{"BL3hzlyUEpDUyxZBIHGw":{"hueRotationDegrees":161.8688524590164,"saturationMultiplier":0.48,"valueMultiplier":0.28},"eEXJq3eGv-plKEzesQbU":{"hueRotationDegrees":181.8688524590164,"saturationMultiplier":0.52,"valueMultiplier":0.15},"L5vSSoXWCeUJ7WkQ_sWE":{"hueRotationDegrees":-52.131147540983605,"saturationMultiplier":0,"valueMultiplier":1.25},"LrGyks9sRDTblR65Agbb":{"hueRotationDegrees":-33.131147540983605,"saturationMultiplier":0.66,"valueMultiplier":0.38},"0wZ-vkaguwIN2luDP219":{"hueRotationDegrees":107.8688524590164,"saturationMultiplier":0.88,"valueMultiplier":0.1},"D2EduBBq_wYB2QUyt4xf":{"hueRotationDegrees":24.868852459016395,"saturationMultiplier":0,"valueMultiplier":0}}},"accessKey":"","name":"Plaster Wall011 Border NoCollider","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 179.627 GET https://app.sowork.com/eyekia/api/v1/floor/8P0BHQn4nWoOB0bnHH2w?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/floor/8P0BHQn4nWoOB0bnHH2w/01xUn7ESvKbepxZPGq68b","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#6fbe48ff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":-48.16949152542374,"saturationMultiplier":1,"valueMultiplier":1}],"presetColorsDictionary":{"FDu5VKJW1n8mexU_rvU4":{"hueRotationDegrees":-48.16949152542374,"saturationMultiplier":1,"valueMultiplier":1}}},"accessKey":"","name":"GrassStone Terrain01","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 179.627 GET https://app.sowork.com/eyekia/api/v1/floor/WDknDl3TmJ34rR0vIL7R?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/floor/WDknDl3TmJ34rR0vIL7R/iV2MB8_iVig6hD-TNdXAZ","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#FFFFFFFF","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"","name":"Matte Black FLoor","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 179.627 GET https://app.sowork.com/eyekia/api/v1/floor/Ic8sFy8QQeSvZAcr9xkW?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/floor/Ic8sFy8QQeSvZAcr9xkW/K1YiqlPz1NULZNKxj379_","pivotPosition":{"x":110,"y":67}},"E":{"url":"/floor/Ic8sFy8QQeSvZAcr9xkW/UHn2AlR8505se-l2DYMm2","pivotPosition":{"x":110,"y":67}},"S":{"url":"/floor/Ic8sFy8QQeSvZAcr9xkW/YnO_9HzzQVx6CAaJfsL0y","pivotPosition":{"x":110,"y":67}},"W":{"url":"/floor/Ic8sFy8QQeSvZAcr9xkW/rj7UrKHEpSMpAiGquTH3u","pivotPosition":{"x":110,"y":67}}},"type":"floor","height":"","pixelDensity":211,"isCorner":false,"colors":{"baseColor":"#FFFFFFFF","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"","name":"Moss Inner Rounded Corder","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 179.627 GET https://app.sowork.com/eyekia/api/v1/floor/ndqwVBMJKxvBj7jmtSaR?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/floor/ndqwVBMJKxvBj7jmtSaR/geffPtNgBAVHQCcj11C5j","pivotPosition":{"x":220,"y":134}},"E":{"url":"/floor/ndqwVBMJKxvBj7jmtSaR/hn8-n2cgBwSoQfVKOmMHT","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#422015ff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":0,"saturationMultiplier":0.5,"valueMultiplier":1},{"hueRotationDegrees":2.333333333333332,"saturationMultiplier":1,"valueMultiplier":1.3},{"hueRotationDegrees":0,"saturationMultiplier":0.3,"valueMultiplier":0.78}],"presetColorsDictionary":{"ZzlLOnVLx6MJQHUan9xM":{"hueRotationDegrees":0,"saturationMultiplier":0.3,"valueMultiplier":0.78},"9eJ2CC9PSh9J4zFApX4A":{"hueRotationDegrees":0,"saturationMultiplier":0.5,"valueMultiplier":1},"ICXDb4GVtDJolpLul37r":{"hueRotationDegrees":2.333333333333332,"saturationMultiplier":1,"valueMultiplier":1.3}}},"accessKey":"PREMIUM","name":"Retro Wood Floor15","description":"SM_2m_Floor_Wood_15a_N","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 179.627 GET https://app.sowork.com/eyekia/api/v1/floor/9QKOMlrx1jITZMdAaNvW?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/floor/9QKOMlrx1jITZMdAaNvW/LH7xY2K4-PqExNp0gSD8N","pivotPosition":{"x":220,"y":134}},"E":{"url":"/floor/9QKOMlrx1jITZMdAaNvW/pBoT2jwgYMTtKt30qjAnH","pivotPosition":{"x":220,"y":134}},"S":{"url":"/floor/9QKOMlrx1jITZMdAaNvW/17vaRK1B7c7kKXYbXZWGZ","pivotPosition":{"x":220,"y":134}},"W":{"url":"/floor/9QKOMlrx1jITZMdAaNvW/N0KoqInje8hsBGAfetYjW","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#cec6b3ff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":-42.22222222222222,"saturationMultiplier":0.76,"valueMultiplier":0.27},{"hueRotationDegrees":0.6713947990543758,"saturationMultiplier":2,"valueMultiplier":0.9215686274509803}],"presetColorsDictionary":{"ZDOWIWjbGrf49f0tkk2x":{"hueRotationDegrees":0.6713947990543758,"saturationMultiplier":2,"valueMultiplier":0.9215686274509803},"GybInlbb5eFxS3WMvbTU":{"hueRotationDegrees":-42.22222222222222,"saturationMultiplier":0.76,"valueMultiplier":0.27}}},"accessKey":"PREMIUM","name":"Retro Wood Floor14","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 179.627 GET https://app.sowork.com/eyekia/api/v1/wall/3JwFmhsELEM7lpDnGiHX?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/wall/3JwFmhsELEM7lpDnGiHX/CQUsYqnCzD0C3KA6S785s","pivotPosition":{"x":256,"y":134}},"E":{"url":"/wall/3JwFmhsELEM7lpDnGiHX/bVA5_0FQ1N5z-mMHb1GXi","pivotPosition":{"x":256,"y":134}},"S":{"url":"/wall/3JwFmhsELEM7lpDnGiHX/i6EEEQLM11N-x4roHiNrO","pivotPosition":{"x":14,"y":134}},"W":{"url":"/wall/3JwFmhsELEM7lpDnGiHX/EEzsi9cE7wSxyuJofwESi","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"2m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#152734ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Industrial Glass Wall013 Short","description":"Glass 02 Wall 2m","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 179.627 GET https://app.sowork.com/eyekia/api/v1/floor/hZBdo3zDFHe1raotIOy6?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/floor/hZBdo3zDFHe1raotIOy6/4ekFmWmnCIg3xRSFrIOTT","pivotPosition":{"x":220,"y":134}},"E":{"url":"/floor/hZBdo3zDFHe1raotIOy6/9yEXAOicrpP-Wt4yfHkad","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#21201fff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Marble Floor Luxury 02","description":"SM_2m_Floor_Ceramic_03a","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 179.678 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=163&CI=1&TYPE=xmlhttp&zx=kkv2fckvtz80&t=1 -> 200
RES: 

### 180.220 GET https://app.sowork.com/eyekia/api/v1/wall/hdOqw0EotFirBMmkqjny?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/wall/hdOqw0EotFirBMmkqjny/ul6-I6r2jDLRNJO0f51Rl","pivotPosition":{"x":256,"y":134}},"E":{"url":"/wall/hdOqw0EotFirBMmkqjny/987DCw_pGAfq8Iwtamwr3","pivotPosition":{"x":256,"y":134}},"S":{"url":"/wall/hdOqw0EotFirBMmkqjny/o9y9gHB5NE_hPyfoH2sAT","pivotPosition":{"x":14,"y":134}},"W":{"url":"/wall/hdOqw0EotFirBMmkqjny/WkMab5lYK_r9KG2SB7K8R","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"2m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#33677bff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Retro Wall033 Short","description":"Glass 03 Wall 2m","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 180.220 GET https://app.sowork.com/eyekia/api/v1/floor/YfEQLH4oDULWLv0ihdmd?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/floor/YfEQLH4oDULWLv0ihdmd/dfiEkckIBbcpo2Eh_Uklc","pivotPosition":{"x":220,"y":134}},"E":{"url":"/floor/YfEQLH4oDULWLv0ihdmd/wII8zBYgZ5QRA7pipMysp","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#763e23ff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":-19.518072289156628,"saturationMultiplier":0.1,"valueMultiplier":1.2},{"hueRotationDegrees":173.48192771084337,"saturationMultiplier":0.48,"valueMultiplier":0.85},{"hueRotationDegrees":-19.518072289156628,"saturationMultiplier":0.1,"valueMultiplier":2.2},{"hueRotationDegrees":-17.11807228915663,"saturationMultiplier":1.1627906976744187,"valueMultiplier":0.5058823529411764}],"presetColorsDictionary":{"yjYSXxT3N1nIIOVKKx7C":{"hueRotationDegrees":-19.518072289156628,"saturationMultiplier":0.1,"valueMultiplier":2.2},"a14VP0uqYQb-M1Qudc1u":{"hueRotationDegrees":-17.11807228915663,"saturationMultiplier":1.1627906976744187,"valueMultiplier":0.5058823529411764},"BKfH82n9HfOHx38LGETc":{"hueRotationDegrees":-19.518072289156628,"saturationMultiplier":0.1,"valueMultiplier":1.2},"MPVgpDCoQLQyXp7mmbEJ":{"hueRotationDegrees":173.48192771084337,"saturationMultiplier":0.48,"valueMultiplier":0.85}}},"accessKey":"","name":"Stone Floor09","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 180.222 GET https://app.sowork.com/eyekia/api/v1/floor/8IJBGHtZYWUImQdhKXbK?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/floor/8IJBGHtZYWUImQdhKXbK/eZpmXi6gDZczWpXbcRlF1","pivotPosition":{"x":220,"y":134}},"E":{"url":"/floor/8IJBGHtZYWUImQdhKXbK/AWLlKjpTt4CP6fcNJLFV1","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#536678ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"","name":"Stone Floor10","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 180.228 GET https://app.sowork.com/eyekia/api/v1/floor/pKTtn39tMSLg7owFUwjY?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/floor/pKTtn39tMSLg7owFUwjY/ZX_3vbWyNuSftYmJWgCYd","pivotPosition":{"x":220,"y":134}},"E":{"url":"/floor/pKTtn39tMSLg7owFUwjY/_ECFevFfV32UhJeSqkWiD","pivotPosition":{"x":220,"y":134}},"S":{"url":"/floor/pKTtn39tMSLg7owFUwjY/NbwiHCUQWWV11o3xdFx5q","pivotPosition":{"x":220,"y":134}},"W":{"url":"/floor/pKTtn39tMSLg7owFUwjY/U_IxtF0xtnIEec1YIRlwU","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#4dcdb9ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"","name":"Ocean to Grass Inside CornerB","description":"SM_2m_Floor_OceanToGrass_InsideCorner_01b","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 180.228 GET https://app.sowork.com/eyekia/api/v1/floor/0ZjuujEA8UsgGztK23XU?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/floor/0ZjuujEA8UsgGztK23XU/Jp6Sed-y_olJHH4vqknEN","pivotPosition":{"x":108,"y":67}}},"type":"floor","height":"","pixelDensity":211,"isCorner":false,"colors":{"baseColor":"#3cd1d6ff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":18.051948051948045,"saturationMultiplier":1.4,"valueMultiplier":0.95},{"hueRotationDegrees":-7.948051948051955,"saturationMultiplier":1,"valueMultiplier":1},{"hueRotationDegrees":19.051948051948045,"saturationMultiplier":0.7,"valueMultiplier":1}],"presetColorsDictionary":{"wahJYF2fc6bleF4bgqsM":{"hueRotationDegrees":-7.948051948051955,"saturationMultiplier":1,"valueMultiplier":1},"2zdNor2jI6Wg4EseLLxc":{"hueRotationDegrees":18.051948051948045,"saturationMultiplier":1.4,"valueMultiplier":0.95},"ZQ9fCGXxDoUc41t7sBsM":{"hueRotationDegrees":19.051948051948045,"saturationMultiplier":0.7,"valueMultiplier":1}}},"accessKey":"","name":"Ocean","description":"SM_2m_Floor_Ocean_01a","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 180.228 GET https://app.sowork.com/eyekia/api/v1/floor/0r7rfxu7Xfp8ByLhYYJz?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/floor/0r7rfxu7Xfp8ByLhYYJz/Ofw3aVMWaauvH-PvEW-18","pivotPosition":{"x":220,"y":134}},"E":{"url":"/floor/0r7rfxu7Xfp8ByLhYYJz/j-aFYTwBSSI1elWiiMiXO","pivotPosition":{"x":220,"y":134}},"S":{"url":"/floor/0r7rfxu7Xfp8ByLhYYJz/G0lA0rB_sJSJIJbetjahZ","pivotPosition":{"x":220,"y":134}},"W":{"url":"/floor/0r7rfxu7Xfp8ByLhYYJz/XoxzvnkiHcDy9Wz1_EUVo","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#58caa5ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"","name":"Ocean to Grass StraightB","description":"SM_2m_Floor_OceanToGrass_Straight_01b","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 180.228 GET https://app.sowork.com/eyekia/api/v1/floor/AJmshYdHAb79CVWSv90t?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/floor/AJmshYdHAb79CVWSv90t/R7BKbBTNIqsw7AOZ-0Qpr","pivotPosition":{"x":110,"y":67}},"E":{"url":"/floor/AJmshYdHAb79CVWSv90t/zFSUBL6TLgO24h0KTdJvd","pivotPosition":{"x":110,"y":67}},"S":{"url":"/floor/AJmshYdHAb79CVWSv90t/83B1cR9s44U0H3dfOvfXm","pivotPosition":{"x":110,"y":67}},"W":{"url":"/floor/AJmshYdHAb79CVWSv90t/nyKwsGEtGob9YRzuF9nvK","pivotPosition":{"x":110,"y":67}}},"type":"floor","height":"","pixelDensity":211,"isCorner":false,"colors":{"baseColor":"#a0cd5bff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":35.315789473684205,"saturationMultiplier":1.4,"valueMultiplier":0.8},{"hueRotationDegrees":-14.684210526315795,"saturationMultiplier":1.2,"valueMultiplier":1},{"hueRotationDegrees":81.3157894736842,"saturationMultiplier":1.35,"valueMultiplier":0.9},{"hueRotationDegrees":-39.684210526315795,"saturationMultiplier":1,"valueMultiplier":1}],"presetColorsDictionary":{"6PIljT3qXbpDcqJVkjb1":{"hueRotationDegrees":35.315789473684205,"saturationMultiplier":1.4,"valueMultiplier":0.8},"RBVzvxnVunOme1Bqd1zL":{"hueRotationDegrees":-39.684210526315795,"saturationMultiplier":1,"valueMultiplier":1},"NuzQh2JreF4tNs-XO1ZM":{"hueRotationDegrees":-14.684210526315795,"saturationMultiplier":1.2,"valueMultiplier":1},"h_D8d0UEr9Jsu27XGG7k":{"hueRotationDegrees":81.3157894736842,"saturationMultiplier":1.35,"valueMultiplier":0.9}}},"accessKey":"","name":"Grass Terrain01","description":"SM_2m_Floor_Grass_01a","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 180.229 GET https://app.sowork.com/eyekia/api/v1/floor/qXWWOC3HpRZ8W3NAMjO2?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/floor/qXWWOC3HpRZ8W3NAMjO2/QgHtN9OorNIz31bcYUdFn","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#b2a48aff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":-39,"saturationMultiplier":0.34,"valueMultiplier":1.1},{"hueRotationDegrees":-39,"saturationMultiplier":0,"valueMultiplier":0.34},{"hueRotationDegrees":286,"saturationMultiplier":1.3,"valueMultiplier":1},{"hueRotationDegrees":-39,"saturationMultiplier":0,"valueMultiplier":0.74},{"hueRotationDegrees":129,"saturationMultiplier":1.86,"valueMultiplier":0.75},{"hueRotationDegrees":105,"saturationMultiplier":1,"valueMultiplier":0.96}],"presetColorsDictionary":{"7b3zo9jv6PUh_gMaKo3h":{"hueRotationDegrees":-39,"saturationMultiplier":0.34,"valueMultiplier":1.1},"y9tDxZlXgyGcjJHQ6vUZ":{"hueRotationDegrees":105,"saturationMultiplier":1,"valueMultiplier":0.96},"K6jvYb_PjtmEF_-8eDCS":{"hueRotationDegrees":-39,"saturationMultiplier":0,"valueMultiplier":0.74},"FIMk3dEKeBgbamwtEA4x":{"hueRotationDegrees":286,"saturationMultiplier":1.3,"valueMultiplier":1},"CQAko1r2JIwQszzp6VtL":{"hueRotationDegrees":-39,"saturationMultiplier":0,"valueMultiplier":0.34},"xhUVK32NWQpqgYp_2O99":{"hueRotationDegrees":129,"saturationMultiplier":1.86,"valueMultiplier":0.75}}},"accessKey":"PREMIUM","name":"Retro Fabric Floor04","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 180.229 GET https://app.sowork.com/eyekia/api/v1/wall/saW1jZlEj3islAtt0Ubo?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/wall/saW1jZlEj3islAtt0Ubo/QJa8jkZ32Pggm72MDp5Ax","pivotPosition":{"x":128,"y":67}},"E":{"url":"/wall/saW1jZlEj3islAtt0Ubo/MXzHAl00wlmRflOMYTdwX","pivotPosition":{"x":128,"y":67}},"S":{"url":"/wall/saW1jZlEj3islAtt0Ubo/J3clIL5N3i2TWriSsf0FY","pivotPosition":{"x":7,"y":67}},"W":{"url":"/wall/saW1jZlEj3islAtt0Ubo/1X-89bTYIEpN1CUbLfoMG","pivotPosition":{"x":7,"y":67}}},"type":"wall","height":"2m","pixelDensity":211,"isCorner":false,"colors":{"baseColor":"#cc8d3fff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"","name":"Wood Wall013 Short","description":"Wood 01 Wall 2m","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 180.230 GET https://app.sowork.com/eyekia/api/v1/floor/Zp7ldmHdLT4hTzkjMQr7?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/floor/Zp7ldmHdLT4hTzkjMQr7/DMGyjutvluPsxRAHKSLrP","pivotPosition":{"x":108,"y":67}},"E":{"url":"/floor/Zp7ldmHdLT4hTzkjMQr7/hFHd5bRyd8yKRmFS20Wcg","pivotPosition":{"x":108,"y":67}},"W":{"url":"/floor/Zp7ldmHdLT4hTzkjMQr7/BmA_TrLLvoQEfEFUkyOJM","pivotPosition":{"x":108,"y":67}}},"type":"floor","height":"","pixelDensity":211,"isCorner":false,"colors":{"baseColor":"#76b68bff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"","name":"Wood Floor08b","description":"SM_2m_Floor_Wood_08b","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 180.232 GET https://app.sowork.com/eyekia/api/v1/floor/PhOwmTdkRzZRShXHFveA?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/floor/PhOwmTdkRzZRShXHFveA/zHB9B8dbbGU2-9AYAl-vQ","pivotPosition":{"x":220,"y":134}},"E":{"url":"/floor/PhOwmTdkRzZRShXHFveA/lY0Kl3SNwc-QXOe_ML6Bm","pivotPosition":{"x":220,"y":134}},"S":{"url":"/floor/PhOwmTdkRzZRShXHFveA/9-9T3kys9MLEphhOh6s1p","pivotPosition":{"x":220,"y":134}},"W":{"url":"/floor/PhOwmTdkRzZRShXHFveA/ewqvkvV19F-AUKNcQ12xx","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#49cdbeff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Pool Round Corner ","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 180.232 GET https://app.sowork.com/eyekia/api/v1/floor/ktXDgTLcLLfnIi9n64cK?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/floor/ktXDgTLcLLfnIi9n64cK/fsQ43sSHFuGYnfB82H17a","pivotPosition":{"x":220,"y":134}},"E":{"url":"/floor/ktXDgTLcLLfnIi9n64cK/4iaBR1XM5r8AK1afkbCCY","pivotPosition":{"x":220,"y":134}},"S":{"url":"/floor/ktXDgTLcLLfnIi9n64cK/BbH_Yra91FarEq97rL4bi","pivotPosition":{"x":220,"y":134}},"W":{"url":"/floor/ktXDgTLcLLfnIi9n64cK/zMzz6WgwInaUJwKoSRYTh","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#3cd0d6ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Pool Straight ","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 180.234 GET https://app.sowork.com/eyekia/api/v1/floor/x9WjkhBPKwPeVBJhXvaZ?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/floor/x9WjkhBPKwPeVBJhXvaZ/i-3lyTwUVsCZveWBrXRTV","pivotPosition":{"x":220,"y":134}},"E":{"url":"/floor/x9WjkhBPKwPeVBJhXvaZ/FplDERMLonunYfl1_a5nE","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#446a94ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Floor Pixelbit 01","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 180.235 GET https://app.sowork.com/eyekia/api/v1/floor/jg7BWx7uVJiVsFj86jxg?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/floor/jg7BWx7uVJiVsFj86jxg/C0ZVhnlXRLRWF9coyZ7p4","pivotPosition":{"x":220,"y":134}},"E":{"url":"/floor/jg7BWx7uVJiVsFj86jxg/v_jZqzClO5ud708a8YaNo","pivotPosition":{"x":220,"y":134}},"S":{"url":"/floor/jg7BWx7uVJiVsFj86jxg/QSwBdP9RRnpIBiP8oQHPf","pivotPosition":{"x":220,"y":134}},"W":{"url":"/floor/jg7BWx7uVJiVsFj86jxg/cqUG0C20xo00I5o1CegY7","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#a4c24dff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"BLOCKS Floor02","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 180.236 GET https://app.sowork.com/eyekia/api/v1/wall/VOxXVCZKvjX9Q1qWmwQp?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/wall/VOxXVCZKvjX9Q1qWmwQp/uzSqInAYMwnsXVTat4238","pivotPosition":{"x":256,"y":134}},"E":{"url":"/wall/VOxXVCZKvjX9Q1qWmwQp/hd20zCyb6vs92Sf0kLqL2","pivotPosition":{"x":256,"y":134}},"S":{"url":"/wall/VOxXVCZKvjX9Q1qWmwQp/oEOg0HLl8sQmuAIUGXTI9","pivotPosition":{"x":14,"y":134}},"W":{"url":"/wall/VOxXVCZKvjX9Q1qWmwQp/0OHVrAGB0AV4NJgLUdaWC","pivotPosition":{"x":14,"y":134}}},"type":"wall","height":"6m","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#858c39ff","useBaseAsPreset":true,"presetColors":[{"hueRotationDegrees":141.93975903614458,"saturationMultiplier":1,"valueMultiplier":1},{"hueRotationDegrees":234.93975903614458,"saturationMultiplier":1,"valueMultiplier":1},{"hueRotationDegrees":-57.06024096385542,"saturationMultiplier":1,"valueMultiplier":1}],"presetColorsDictionary":{"T3XPqm96d3tz9D0ds9fC":{"hueRotationDegrees":234.93975903614458,"saturationMultiplier":1,"valueMultiplier":1},"HEcqMUNxlg9wTnW64G4u":{"hueRotationDegrees":141.93975903614458,"saturationMultiplier":1,"valueMultiplier":1},"zgQ4UnyeuoW0WOF4-YLZ":{"hueRotationDegrees":-57.06024096385542,"saturationMultiplier":1,"valueMultiplier":1}}},"accessKey":"PREMIUM","name":"SoBLOCKS Wall015 Tall A","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 180.240 GET https://app.sowork.com/eyekia/api/v1/floor/klB54rvXfHrAOms5qPNY?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/floor/klB54rvXfHrAOms5qPNY/bWsw9IrhPBHK_wXa79SWU","pivotPosition":{"x":220,"y":134}},"E":{"url":"/floor/klB54rvXfHrAOms5qPNY/3zgJjbbf2hXmbd1FdMrRv","pivotPosition":{"x":220,"y":134}}},"type":"floor","height":"","pixelDensity":106,"isCorner":false,"colors":{"baseColor":"#486d95ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"PREMIUM","name":"Floor Pixelbit 02","description":"","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 180.240 GET https://app.sowork.com/eyekia/api/v1/floor/vIMm3Ipv2zBMGnPrBklM?pathOnlyUrls=true -> 200
RES: {"orientations":{"N":{"url":"/floor/vIMm3Ipv2zBMGnPrBklM/1VJZwCTwZzUW7-oaZX-xe","pivotPosition":{"x":108,"y":67}},"E":{"url":"/floor/vIMm3Ipv2zBMGnPrBklM/64Y459wuE9ihq3Qqw4Ev5","pivotPosition":{"x":108,"y":67}},"S":{"url":"/floor/vIMm3Ipv2zBMGnPrBklM/HcP7PdKS8LVXM4eX6y__g","pivotPosition":{"x":108,"y":67}},"W":{"url":"/floor/vIMm3Ipv2zBMGnPrBklM/ZHNpmYJJsSow5QUiJ5Cnv","pivotPosition":{"x":108,"y":67}}},"type":"floor","height":"","pixelDensity":211,"isCorner":false,"colors":{"baseColor":"#52c4c3ff","useBaseAsPreset":true,"presetColors":[],"presetColorsDictionary":{}},"accessKey":"","name":"Ocean to WetSand Straight","description":"SM_2m_Floor_OceanToSand_Straight_01a","thumbnail":"","published":true,"modifiedSinceLastPublish":false}

### 180.308 POST https://api.sowork.com/api/v1/user/recordLogin -> 200
REQ: {}
RES: {"success":true}

### 180.308 POST https://api.sowork.com/api/v1/userSession/upsert -> 200
REQ: {"sessionId":"tab_1791307969887_mri770eos"}
RES: {"userId":"RMUYRzBXvUOCqtJdxSAiMdiEZM82","sessionId":"tab_1791307969887_mri770eos"}

### 180.498 POST https://identitytoolkit.googleapis.com/v1/accounts:signInWithCustomToken?key=<REDACTED> -> 200
REQ: {"token":"<JWT>","returnSecureToken":true}
RES: {
  "kind": "identitytoolkit#VerifyCustomTokenResponse",
  "idToken": "<JWT>",
  "refreshToken": "AMf-vByzRvi77cRPZvVSkuFlrPp3-HZGnZExJxzWfvmrZICP9Jt4yvgg4WaqkJE31elBtBqKzDq09UvRIFK8EDwwWqsTmymKAPKF0KHhNJym7i_cNLmqpcQJqDiLChcpwygb2YsEPq2Osis4fLyml9EDS2UYsdAwRr_YF-Icsj-bYalKnnnSF_0uhKJYUq3Yn6zLdHG9fN7XMZINXBaIxpNvG6ne6cJhE6p1LC09IxIBLvfkZVzQ9Ao",
  "expiresIn": "3600",
  "isNewUser": false
}


### 180.499 OPTIONS https://identitytoolkit.googleapis.com/v1/accounts:signInWithCustomToken?key=<REDACTED> -> 200
RES: 

### 180.500 POST https://securetoken.googleapis.com/v1/token?key=<REDACTED> -> 200
REQ: grant_type=refresh_token&refresh_token=<REDACTED>
RES: {
  "access_token": "<JWT>",
  "expires_in": "3600",
  "token_type": "Bearer",
  "refresh_token": "AMf-vBwbbgsmcIA_8n4FQ7hl1s_7no08rNDENP0xPaZ4Aeene_kC-U1Ziqn5P6QDhTDfRyeUrQ_ZkczRgt3KuplGhDCqkBbp47ilXMTNzjOIyCodcJNz8yb2WCuaDn7HWCnjRGY4-tuJ_c0371F9TGBPyqhYL3JklLUdS3UH6fwLCvohEfIDKFkTk9V5zE1REYYfkBucba-TcOcjRLLaqNP9Ei-TlMWGcIWzNx-REGNZWxBMu3KEuAQ",
  "id_token": "<JWT>",
  "user_id": "RMUYRzBXvUOCqtJdxSAiMdiEZM82",
  "project_id": "537328756772"
}


### 180.500 OPTIONS https://securetoken.googleapis.com/v1/token?key=<REDACTED> -> 200
RES: 

### 180.749 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=165&CI=1&TYPE=xmlhttp&zx=6rugs8jx8fj6&t=1 -> 200
RES: 2280
[[166,[{
  "targetChange": {
    "targetIds": [
      30
    ],
    "resumeToken": "CgkIr+fysfallwM=",
    "readTime": "2026-10-06T17:33:40.364207Z"
  }
}
]],[167,[{
  "targetChange": {
    "targetIds": [
      28
    ],
    "resumeToken": "CgkIr+fysfallwM=",
    "readTime": "2026-10-06T17:33:40.364207Z"
  }
}
]],[168,[{
  "targetChange": {
    "targetIds": [
      24
    ],
    "resumeToken": "CgkIr+fysfallwM=",
    "readTime": "2026-10-06T17:33:40.364207Z"
  }
}
]],[169,[{
  "filter": {
    "targetId": 14,
    "unchangedNames": {
      "bits": {}
    }
  }
}
]],[170,[{
  "targetChange": {
    "targetIds": [
      14
    ],
    "resumeToken": "CgkIr+fysfallwM=",
    "readTime": "2026-10-06T17:33:40.364207Z"
  }
}
]],[171,[{
  "filter": {
    "targetId": 52,
    "count": 1,
    "unchangedNames": {
      "bits": {
        "bitmap": "qvoB",
        "padding": 7
      },
      "hashCount": 12
    }
  }
}
]],[172,[{
  "targetChange": {
    "targetIds": [
      52
    ],
    "resumeToken": "CgkIr+fysfallwM=",
    "readTime": "2026-10-06T17:33:40.364207Z"
  }
}
]],[173,[{
  "targetChange": {
    "targetIds": [
      56,
      84,
      40,
      4,
      54,
      26,
      90,
      38,
      76,
      78,
      70,
      30,
      72,
      88,
      10,
      52,
      36,
      20,
      50,
      66,
      28,
      34,
      86,
      58,
      18,
      12,
      46,
      92,
      22,
      14,
      44,
      24,
      42,
      74,
      8,
      2,
      48,
      16,
      32,
      82
    ],
    "resumeToken": "CgkIr+fysfallwM=",
    "readTime": "2026-10-06T17:33:40.364207Z"
  }
}
]],[174,[{
  "filter": {
    "targetId": 64,
    "count": 1,
    "unchangedNames": {
      "bits": {
        "bitmap": "PO8B",
        "padding": 7
      },
      "hashCount": 12
    }
  }
}
]],[175,[{
  "targetChange": {
    "targetIds": [
      64
    ],
    "resumeToken": "CgkIr+fysfallwM=",
    "readTime": "2026-10-06T17:33:40.364207Z"
  }
}
]],[176,[{
  "filter": {
    "targetId": 68,
    "count": 1,
    "unchangedNames": {
      "bits": {
        "bitmap": "PO8B",
        "padding": 7
      },
      "hashCount": 12
    }
  }
}
]],[177,[{
  "targetChange": {
    "resumeToken": "CgkIr+fysfallwM=",
    "readTime": "2026-10-06T17:33:40.364207Z"
  }
}
]]]

### 180.776 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65320&AID=165&zx=69rgge7qn7mc&t=1 -> 200
REQ: count=1&ofs=50&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fuser_sessions%2FRMUYRzBXvUOCqtJdxSAiMdiEZM82%22%5D%7D%2C%22targetId%22%3A94%7D%7D
RES: 10
[0,177,7]


### 180.839 POST https://identitytoolkit.googleapis.com/v1/accounts:lookup?key=<REDACTED> -> 200
REQ: {"idToken":"<JWT>"}
RES: {
  "kind": "identitytoolkit#GetAccountInfoResponse",
  "users": [
    {
      "localId": "RMUYRzBXvUOCqtJdxSAiMdiEZM82",
      "validSince": "1786365157",
      "lastLoginAt": "1791308020498",
      "createdAt": "1786365157281",
      "customAuth": true,
      "lastRefreshAt": "2026-10-06T17:33:40.498Z"
    }
  ]
}


### 180.839 OPTIONS https://identitytoolkit.googleapis.com/v1/accounts:lookup?key=<REDACTED> -> 200
RES: 

### 180.882 POST https://identitytoolkit.googleapis.com/v1/accounts:lookup?key=<REDACTED> -> 200
REQ: {"idToken":"<JWT>"}
RES: {
  "kind": "identitytoolkit#GetAccountInfoResponse",
  "users": [
    {
      "localId": "RMUYRzBXvUOCqtJdxSAiMdiEZM82",
      "validSince": "1786365157",
      "lastLoginAt": "1791308020498",
      "createdAt": "1786365157281",
      "customAuth": true,
      "lastRefreshAt": "2026-10-06T17:33:40.498Z"
    }
  ]
}


### 180.882 OPTIONS https://identitytoolkit.googleapis.com/v1/accounts:lookup?key=<REDACTED> -> 200
RES: 

### 180.890 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=177&CI=1&TYPE=xmlhttp&zx=y2rtqcpjtptv&t=1 -> 200
RES: 936
[[178,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      94
    ]
  }
}
]],[179,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/user_sessions/RMUYRzBXvUOCqtJdxSAiMdiEZM82",
      "fields": {
        "userId": {
          "stringValue": "RMUYRzBXvUOCqtJdxSAiMdiEZM82"
        },
        "sessionId": {
          "stringValue": "tab_1791307969887_mri770eos"
        }
      },
      "createTime": "2026-08-10T12:32:44.357825Z",
      "updateTime": "2026-10-06T17:33:40.365474Z"
    },
    "targetIds": [
      94
    ]
  }
}
]],[180,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      94
    ],
    "resumeToken": "CgkInPH9sfallwM=",
    "readTime": "2026-10-06T17:33:40.545692Z"
  }
}
]],[181,[{
  "targetChange": {
    "resumeToken": "CgkIl5v+sfallwM=",
    "readTime": "2026-10-06T17:33:40.551063Z"
  }
}
]]]

### 181.064 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=181&CI=1&TYPE=xmlhttp&zx=eqh5j5204q7p&t=1 -> 200
RES: 8386
[[182,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/group_membermaps/v0XvoIt3uK40pdF0mQDB_3",
      "fields": {
        "users": {
          "mapValue": {
            "fields": {
              "EsFlL0qd1kZNo0IsotREJWQWNeC3": {
                "mapValue": {
                  "fields": {
                    "inCalendarMeeting": {
                      "nullValue": null
                    },
                    "role": {
                      "stringValue": "contributor"
                    },
                    "isOnline": {
                      "booleanValue": false
                    },
                    "inPrivateConversation": {
                      "booleanValue": false
                    },
                    "userName": {
                      "stringValue": "Lucas Pacheco"
                    },
                    "mobile": {
                      "booleanValue": false
                    },
                    "status": {
                      "stringValue": "online"
                    },
                    "lastOnline": {
                      "timestampValue": "2026-09-29T18:55:00.194Z"
                    }
                  }
                }
              },
              "blKsp8Fxi6VGJfXfizN0iu248IB2": {
                "mapValue": {
                  "fields": {
                    "role": {
                      "stringValue": "contributor"
                    },
                    "userName": {
                      "stringValue": "Fabio"
                    },
                    "lastOnline": {
                      "timestampValue": "2026-10-06T17:28:35.620Z"
                    },
                    "roomDefinitionId": {
                      "stringValue": "yPsvwRdkb5LnGHfXFgfO"
                    },
                    "meetingZoneId": {
                      "stringValue": "UU2eiy61BEPJ-9G9JtW2"
                    },
                    "mobile": {
                      "booleanValue": true
                    },
                    "isOnline": {
                      "booleanValue": true
                    },
                    "inCalendarMeeting": {
                      "nullValue": null
                    },
                    "inPrivateConversation": {
                      "booleanValue": false
                    },
                    "status": {
                      "stringValue": "online"
                    },
                    "mode": {
                      "stringValue": "room"
                    }
                  }
                }
              },
              "RMUYRzBXvUOCqtJdxSAiMdiEZM82": {
                "mapValue": {
                  "fields": {
                    "inPrivateConversation": {
                      "booleanValue": false
                    },
                    "lastOnline": {
                      "timestampValue": "2026-10-06T17:33:40.188Z"
                    },
                    "roomDefinitionId": {
                      "stringValue": "yPsvwRdkb5LnGHfXFgfO"
                    },
                    "isOnline": {
                      "booleanValue": true
                    },
                    "role": {
                      "stringValue": "contributor"
                    },
                    "meetingZoneId": {
                      "stringValue": "mohybxsJ-3T8BD3hvNGx"
                    },
                    "inCalendarMeeting": {
                      "nullValue": null
                    },
                    "mode": {
                      "stringValue": "room"
                    },
                    "userName": {
                      "stringValue": "Tito Guidotti"
                    },
                    "status": {
                      "stringValue": "online"
                    },
                    "mobile": {
                      "booleanValue": false
                    }
                  }
                }
              },
              "JLDRGMskJRXiYcHccNVaLuwr3XJ3": {
                "mapValue": {
                  "fields": {
                    "roomDefinitionId": {
                      "stringValue": "yPsvwRdkb5LnGHfXFgfO"
                    },
                    "inPrivateConversation": {
                      "booleanValue": false
                    },
                    "userName": {
                      "stringValue": "Chuck"
                    },
                    "role": {
                      "stringValue": "contributor"
                    },
                    "status": {
                      "stringValue": "online"
                    },
                    "mode": {
                      "stringValue": "room"
                    },
                    "mobile": {
                      "booleanValue": true
                    },
                    "lastOnline": {
                      "timestampValue": "2026-10-06T17:01:24.767Z"
                    },
                    "meetingZoneId": {
                      "stringValue": "yAfwkWWJEuYMMWuDabPg"
                    },
                    "inCalendarMeeting": {
                      "nullValue": null
                    },
                    "isOnline": {
                      "booleanValue": true
                    }
                  }
                }
              },
              "FLBKjFg1PYe5bD6cE0bdR7BhE7r1": {
                "mapValue": {
                  "fields": {
                    "lastOnline": {
                      "timestampValue": "2026-10-06T16:15:01.829Z"
                    },
                    "userName": {
                      "stringValue": "sasa"
                    },
                    "inCalendarMeeting": {
                      "nullValue": null
                    },
                    "role": {
                      "stringValue": "owner"
                    },
                    "meetingZoneId": {
                      "stringValue": "M0ST4keNHNva-oFHk3PL"
     

### 181.110 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65321&AID=181&zx=k4zlubargine&t=1 -> 200
REQ: count=1&ofs=51&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22npcs%22%7D%5D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fgroups%2Fv0XvoIt3uK40pdF0mQDB%22%7D%2C%22targetId%22%3A96%7D%7D
RES: 10
[0,183,7]


### 181.717 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=183&CI=1&TYPE=xmlhttp&zx=6e49mt6emttx&t=1 -> 200
RES: 417
[[184,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      96
    ]
  }
}
]],[185,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      96
    ],
    "resumeToken": "CgkI3PaSsvallwM=",
    "readTime": "2026-10-06T17:33:40.890460Z"
  }
}
]],[186,[{
  "targetChange": {
    "resumeToken": "CgkI3PaSsvallwM=",
    "readTime": "2026-10-06T17:33:40.890460Z"
  }
}
]]]

### 181.767 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsophya-calendar-production%2Fdatabases%2F(default)&RID=11826&CVER=22&X-HTTP-Session-Id=gsessionid&zx=pnikq66j9vcp&t=1 -> 200
REQ: headers=X-Goog-Api-Client%3Agl-js%2F%20fire%2F9.23.0%0D%0AContent-Type%3Atext%2Fplain%0D%0AX-Firebase-GMPID%3A1%3A537328756772%3Aweb%3Ad487ca7b2e66572b9510b1%0D%0AAuthorization%3ABearer%20<JWT>%0D%0A&count=2&ofs=0&req0___data__=%7B%22database%22%3A%22projects%2Fsophya-calendar-production%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22events%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22userId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22start%22%7D%2C%22op%22%3A%22GREATER_THAN_OR_EQUAL%22%2C%22value%22%3A%7B%22timestampValue%22%3A%222026-09-29T17%3A33%3A41.449000000Z%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22start%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%2C%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsophya-calendar-production%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A2%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsophya-calendar-production%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22accounts%22%7D%5D%2C%22where%22%3A%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22userId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsophya-calendar-production%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A4%7D%7D
RES: 51
[[0,["c","AYiWFFk6YV4N591bkO7F1Q","",8,15,30000]]]


### 181.820 POST https://api.sowork.com/api/v1/user/updateUiSettings -> 200
REQ: {"userId":"RMUYRzBXvUOCqtJdxSAiMdiEZM82","data":{"mutedSounds":{"speechHigh":false,"shoot_1":false,"selectAsset":false,"shoot_2":false,"codeSuccess":false,"polaroid":false,"go":false,"countdown":false,"gong":false,"notification":false,"doorKnock":false,"speechLow":false,"deleteAsset":false,"editAsset":false,"raiseHand":false,"pencilLine":false,"battleMusic":false,"joinCall":true,"coworkingEnd1":true,"mapMakerMusic":true,"placeAsset":false,"applause":false,"requestToJoinPm":false,"lose":false,"coworkingStart2":true,"win":false,"hit":false,"jump":false,"ring":false,"textBroadcast":false,"receiveWave":false,"coworkingEnd3":true,"coworkingEnd2":true,"coworkingStart3":true,"rotateAsset":false,"shark_hitter":false,"sendWave":false,"disableVideo":false,"speechMid":false,"removeNotification":false,"leaveCall":true,"productUnlocked":false,"muteMic":false,"doorForceOpen":false,"coworkingStart1":true,"chatNotification":false,"unmuteMic":false,"privateMeeting":false,"calendarNotification":true,"enableVideo":false,"characterPicker":false,"cat_hitter":false,"duck_hitter":false,"participantJoined":false,"memo":false,"equip":false}}}
RES: {"success":true}

### 181.863 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=186&CI=1&TYPE=xmlhttp&zx=secmibc6vzvi&t=1 -> 200
RES: 

### 181.907 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=LZpPn8fHU8l56MnxY-WmYOZainSh9YrP82xawbG90Kty1lG5D2NBFQ&VER=8&database=projects%2Fsophya-calendar-production%2Fdatabases%2F(default)&RID=rpc&SID=AYiWFFk6YV4N591bkO7F1Q&AID=0&CI=0&TYPE=xmlhttp&zx=j7vununk31n4&t=1 -> 200
RES: 

### 182.713 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=188&CI=1&TYPE=xmlhttp&zx=xaymdx8u6rmn&t=1 -> 200
RES: 

### 182.866 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=197&CI=1&TYPE=xmlhttp&zx=v2zn499qim1z&t=1 -> 200
RES: 

### 186.095 POST https://api.sowork.com/api/v1/crm/track -> 200
REQ: {"userId":"RMUYRzBXvUOCqtJdxSAiMdiEZM82","groupId":"v0XvoIt3uK40pdF0mQDB","events":[{"eventName":"entered_office","timestamp":1791308020.752}]}
RES: {"success":true}

### 186.527 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=199&CI=1&TYPE=xmlhttp&zx=fzbto2d719t5&t=1 -> 200
RES: 2310
[[200,[{
  "targetChange": {
    "targetIds": [
      28
    ],
    "resumeToken": "CgkIuvvTtPallwM=",
    "readTime": "2026-10-06T17:33:46.150330Z"
  }
}
]],[201,[{
  "targetChange": {
    "targetIds": [
      24
    ],
    "resumeToken": "CgkIuvvTtPallwM=",
    "readTime": "2026-10-06T17:33:46.150330Z"
  }
}
]],[202,[{
  "targetChange": {
    "targetIds": [
      26
    ],
    "resumeToken": "CgkIuvvTtPallwM=",
    "readTime": "2026-10-06T17:33:46.150330Z"
  }
}
]],[203,[{
  "filter": {
    "targetId": 14,
    "unchangedNames": {
      "bits": {}
    }
  }
}
]],[204,[{
  "targetChange": {
    "targetIds": [
      14
    ],
    "resumeToken": "CgkIuvvTtPallwM=",
    "readTime": "2026-10-06T17:33:46.150330Z"
  }
}
]],[205,[{
  "filter": {
    "targetId": 52,
    "count": 1,
    "unchangedNames": {
      "bits": {
        "bitmap": "qvoB",
        "padding": 7
      },
      "hashCount": 12
    }
  }
}
]],[206,[{
  "targetChange": {
    "targetIds": [
      52
    ],
    "resumeToken": "CgkIuvvTtPallwM=",
    "readTime": "2026-10-06T17:33:46.150330Z"
  }
}
]],[207,[{
  "filter": {
    "targetId": 68,
    "count": 1,
    "unchangedNames": {
      "bits": {
        "bitmap": "PO8B",
        "padding": 7
      },
      "hashCount": 12
    }
  }
}
]],[208,[{
  "targetChange": {
    "targetIds": [
      68
    ],
    "resumeToken": "CgkIuvvTtPallwM=",
    "readTime": "2026-10-06T17:33:46.150330Z"
  }
}
]],[209,[{
  "targetChange": {
    "targetIds": [
      14,
      8,
      24,
      58,
      74,
      86,
      50,
      66,
      68,
      88,
      18,
      34,
      52,
      30,
      36,
      20,
      28,
      96,
      76,
      40,
      72,
      38,
      10,
      26,
      78,
      70,
      4,
      90,
      54,
      84,
      32,
      2,
      48,
      56,
      42,
      82,
      16,
      94,
      46,
      12,
      92,
      44,
      22
    ],
    "resumeToken": "CgkIuvvTtPallwM=",
    "readTime": "2026-10-06T17:33:46.150330Z"
  }
}
]],[210,[{
  "filter": {
    "targetId": 64,
    "count": 1,
    "unchangedNames": {
      "bits": {
        "bitmap": "PO8B",
        "padding": 7
      },
      "hashCount": 12
    }
  }
}
]],[211,[{
  "targetChange": {
    "resumeToken": "CgkIuvvTtPallwM=",
    "readTime": "2026-10-06T17:33:46.150330Z"
  }
}
]]]

### 186.689 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=211&CI=1&TYPE=xmlhttp&zx=yqrrmzwkb7p5&t=1 -> 200
RES: 

### 187.805 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=213&CI=1&TYPE=xmlhttp&zx=1ajrwmuzlu3y&t=1 -> 200
RES: 123
[[214,[{
  "targetChange": {
    "resumeToken": "CgkIvI6gtfallwM=",
    "readTime": "2026-10-06T17:33:47.397948Z"
  }
}
]]]

### 187.946 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=214&CI=1&TYPE=xmlhttp&zx=uf1ambntyhqk&t=1 -> 200
RES: 

### 190.952 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=216&CI=1&TYPE=xmlhttp&zx=ocq0lptyt9q8&t=1 -> 200
RES: 2280
[[217,[{
  "targetChange": {
    "targetIds": [
      24
    ],
    "resumeToken": "CgkIzPvgtvallwM=",
    "readTime": "2026-10-06T17:33:50.557644Z"
  }
}
]],[218,[{
  "targetChange": {
    "targetIds": [
      28
    ],
    "resumeToken": "CgkIzPvgtvallwM=",
    "readTime": "2026-10-06T17:33:50.557644Z"
  }
}
]],[219,[{
  "targetChange": {
    "targetIds": [
      30
    ],
    "resumeToken": "CgkIzPvgtvallwM=",
    "readTime": "2026-10-06T17:33:50.557644Z"
  }
}
]],[220,[{
  "filter": {
    "targetId": 52,
    "count": 1,
    "unchangedNames": {
      "bits": {
        "bitmap": "qvoB",
        "padding": 7
      },
      "hashCount": 12
    }
  }
}
]],[221,[{
  "targetChange": {
    "targetIds": [
      52
    ],
    "resumeToken": "CgkIzPvgtvallwM=",
    "readTime": "2026-10-06T17:33:50.557644Z"
  }
}
]],[222,[{
  "targetChange": {
    "targetIds": [
      72,
      78,
      4,
      90,
      84,
      54,
      26,
      48,
      16,
      42,
      82,
      56,
      32,
      2,
      94,
      92,
      12,
      8,
      46,
      24,
      22,
      74,
      44,
      58,
      86,
      66,
      18,
      34,
      50,
      36,
      30,
      88,
      20,
      28,
      96,
      70,
      76,
      40,
      10,
      38
    ],
    "resumeToken": "CgkIzPvgtvallwM=",
    "readTime": "2026-10-06T17:33:50.557644Z"
  }
}
]],[223,[{
  "filter": {
    "targetId": 68,
    "count": 1,
    "unchangedNames": {
      "bits": {
        "bitmap": "PO8B",
        "padding": 7
      },
      "hashCount": 12
    }
  }
}
]],[224,[{
  "targetChange": {
    "targetIds": [
      68
    ],
    "resumeToken": "CgkIzPvgtvallwM=",
    "readTime": "2026-10-06T17:33:50.557644Z"
  }
}
]],[225,[{
  "filter": {
    "targetId": 64,
    "count": 1,
    "unchangedNames": {
      "bits": {
        "bitmap": "PO8B",
        "padding": 7
      },
      "hashCount": 12
    }
  }
}
]],[226,[{
  "targetChange": {
    "targetIds": [
      64
    ],
    "resumeToken": "CgkIzPvgtvallwM=",
    "readTime": "2026-10-06T17:33:50.557644Z"
  }
}
]],[227,[{
  "filter": {
    "targetId": 14,
    "unchangedNames": {
      "bits": {}
    }
  }
}
]],[228,[{
  "targetChange": {
    "resumeToken": "CgkIzPvgtvallwM=",
    "readTime": "2026-10-06T17:33:50.557644Z"
  }
}
]]]

### 191.091 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=228&CI=1&TYPE=xmlhttp&zx=pasqunnmkmt0&t=1 -> 200
RES: 8386
[[229,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/group_membermaps/v0XvoIt3uK40pdF0mQDB_3",
      "fields": {
        "users": {
          "mapValue": {
            "fields": {
              "JLDRGMskJRXiYcHccNVaLuwr3XJ3": {
                "mapValue": {
                  "fields": {
                    "inCalendarMeeting": {
                      "nullValue": null
                    },
                    "role": {
                      "stringValue": "contributor"
                    },
                    "isOnline": {
                      "booleanValue": true
                    },
                    "roomDefinitionId": {
                      "stringValue": "yPsvwRdkb5LnGHfXFgfO"
                    },
                    "lastOnline": {
                      "timestampValue": "2026-10-06T17:01:24.767Z"
                    },
                    "inPrivateConversation": {
                      "booleanValue": false
                    },
                    "meetingZoneId": {
                      "stringValue": "yAfwkWWJEuYMMWuDabPg"
                    },
                    "userName": {
                      "stringValue": "Chuck"
                    },
                    "mobile": {
                      "booleanValue": true
                    },
                    "status": {
                      "stringValue": "online"
                    },
                    "mode": {
                      "stringValue": "room"
                    }
                  }
                }
              },
              "FLBKjFg1PYe5bD6cE0bdR7BhE7r1": {
                "mapValue": {
                  "fields": {
                    "role": {
                      "stringValue": "owner"
                    },
                    "inPrivateConversation": {
                      "booleanValue": false
                    },
                    "meetingZoneId": {
                      "stringValue": "M0ST4keNHNva-oFHk3PL"
                    },
                    "inCalendarMeeting": {
                      "nullValue": null
                    },
                    "userName": {
                      "stringValue": "sasa"
                    },
                    "roomDefinitionId": {
                      "stringValue": "yPsvwRdkb5LnGHfXFgfO"
                    },
                    "lastOnline": {
                      "timestampValue": "2026-10-06T16:15:01.829Z"
                    },
                    "isOnline": {
                      "booleanValue": true
                    },
                    "mobile": {
                      "booleanValue": true
                    },
                    "status": {
                      "stringValue": "online"
                    },
                    "mode": {
                      "stringValue": "room"
                    }
                  }
                }
              },
              "G7EWVnLMQngko5v7KcgNnH06Fhg1": {
                "mapValue": {
                  "fields": {
                    "isOnline": {
                      "booleanValue": true
                    },
                    "inCalendarMeeting": {
                      "stringValue": "external"
                    },
                    "meetingZoneId": {
                      "stringValue": "xX2fONpTxunE0VSeSqPF"
                    },
                    "role": {
                      "stringValue": "owner"
                    },
                    "status": {
                      "stringValue": "online"
                    },
                    "mobile": {
                      "booleanValue": true
                    },
                    "roomDefinitionId": {
                      "stringValue": "yPsvwRdkb5LnGHfXFgfO"
                    },
                    "mode": {
                      "stringValue": "room"
                    },
                    "lastOnline": {
                      "timestampValue": "2026-10-06T17:30:03.227Z"
                    },
                    "userName": {
                      "stringValue": "Denis Shirazi"
                    },
                    "inPrivateConversation": {
                      "booleanValue": false
                    }
                  }
                }
              },
              "blKsp8Fxi6VGJfXfizN0iu248IB2": {
                "mapValue": {
                  "fields": {
                    "userName": {
                      "stringValue": "Fabio"
                    },
                    "mobile": {
                      "booleanValue": true
                    },
                    "inPrivateConversation": {
                      "booleanValue": false
                    },
                    "status": {
                      "stringValue": "online"
                    },
                    "lastOnline": {
                      "timestampValue": "2026-10-06T17:28:35.620Z"
                    },
                    "mode": {
                      "stringValue": "room"
                    },
                    "meetingZoneId": {
                      "stringValue": "UU2eiy61BEPJ-9G9JtW2"
                    },
                    "role": {
                      "stringValue": "contributor"
                    },
                    "isOnline": {
                      "booleanValue": true
                    },
                    "inCalendarMeeting": {
                      "nullValue": null
                    },
                    "roomDefinitionId": {
                      "stringValue": "yPsvwRdkb5LnGHfXFgfO"
                    }
                  }
                }
              },
              "EsFlL0qd1kZNo0IsotREJWQWNeC3": {
                "mapValue": {
                  "fields": {
                    "status": {
                      "stringValue": "online"
                    },
                    "mobile": {
                      "booleanValue": false
                    },
       

### 191.230 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=230&CI=1&TYPE=xmlhttp&zx=rrqsgabi43lv&t=1 -> 200
RES: 

### 191.839 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 191.878 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65322&AID=231&zx=5u14yd4zw19&t=1 -> 200
REQ: count=1&ofs=52&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fconversations%2F1BQkbHroywZpI0ejYPGP%22%5D%7D%2C%22targetId%22%3A98%7D%7D
RES: 10
[1,231,7]


### 191.921 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 192.021 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=232&CI=1&TYPE=xmlhttp&zx=4moaiod0usox&t=1 -> 200
RES: 2123
[[233,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/conversations/1BQkbHroywZpI0ejYPGP",
      "fields": {
        "participants": {
          "mapValue": {}
        },
        "status": {
          "stringValue": "active"
        },
        "privacy": {
          "stringValue": "public"
        },
        "ownerGroupId": {
          "stringValue": "v0XvoIt3uK40pdF0mQDB"
        },
        "createdAt": {
          "timestampValue": "2026-10-06T17:33:51.179Z"
        },
        "livekitRoomId": {
          "stringValue": "RM_unGbwGcgK3ai"
        },
        "roomDefinitionId": {
          "stringValue": "yPsvwRdkb5LnGHfXFgfO"
        },
        "zoneId": {
          "stringValue": "XxzogfC5WwJLDYOOIcPc"
        },
        "sc": {
          "arrayValue": {
            "values": [
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:owner"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:contributor"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:guest"
              }
            ]
          }
        },
        "meetingChatProvider": {
          "stringValue": "sowork"
        },
        "meetingChatId": {
          "nullValue": null
        },
        "roomId": {
          "stringValue": "YmAfBb3h2eY5r9FsAIfV"
        },
        "hasOnlineUsers": {
          "booleanValue": false
        },
        "ownerId": {
          "nullValue": null
        },
        "sessionId": {
          "stringValue": "623b1c7b-e8f8-40e6-903e-1dd56bbd4086"
        }
      },
      "createTime": "2026-10-06T17:33:51.277834Z",
      "updateTime": "2026-10-06T17:33:51.277834Z"
    },
    "targetIds": [
      98
    ]
  }
}
]],[234,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      98
    ],
    "resumeToken": "CgkIubejt/allwM=",
    "readTime": "2026-10-06T17:33:51.646649Z"
  }
}
]],[235,[{
  "targetChange": {
    "resumeToken": "CgkIubejt/allwM=",
    "readTime": "2026-10-06T17:33:51.646649Z"
  }
}
]]]

### 192.543 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=235&CI=1&TYPE=xmlhttp&zx=3g50r12pwhwn&t=1 -> 200
RES: 6026
[[236,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/conversations/1BQkbHroywZpI0ejYPGP",
      "fields": {
        "livekitRoomId": {
          "stringValue": "RM_unGbwGcgK3ai"
        },
        "onlineCount": {
          "integerValue": "1"
        },
        "participants": {
          "mapValue": {}
        },
        "users": {
          "mapValue": {
            "fields": {
              "Rxp3M2MJzra9wQJm0VkDfRSgJgt1": {
                "mapValue": {
                  "fields": {
                    "joinAt": {
                      "timestampValue": "2026-10-06T17:33:51Z"
                    },
                    "lastJoinAt": {
                      "timestampValue": "2026-10-06T17:33:51Z"
                    },
                    "name": {
                      "stringValue": "Kury"
                    },
                    "status": {
                      "stringValue": "online"
                    }
                  }
                }
              }
            }
          }
        },
        "status": {
          "stringValue": "active"
        },
        "sessionId": {
          "stringValue": "623b1c7b-e8f8-40e6-903e-1dd56bbd4086"
        },
        "ownerId": {
          "nullValue": null
        },
        "ownerGroupId": {
          "stringValue": "v0XvoIt3uK40pdF0mQDB"
        },
        "roomDefinitionId": {
          "stringValue": "yPsvwRdkb5LnGHfXFgfO"
        },
        "meetingChatProvider": {
          "stringValue": "sowork"
        },
        "privacy": {
          "stringValue": "public"
        },
        "zoneId": {
          "stringValue": "XxzogfC5WwJLDYOOIcPc"
        },
        "meetingChatId": {
          "nullValue": null
        },
        "roomId": {
          "stringValue": "YmAfBb3h2eY5r9FsAIfV"
        },
        "hasOnlineUsers": {
          "booleanValue": true
        },
        "sc": {
          "arrayValue": {
            "values": [
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:owner"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:contributor"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:guest"
              }
            ]
          }
        },
        "createdAt": {
          "timestampValue": "2026-10-06T17:33:51.179Z"
        }
      },
      "createTime": "2026-10-06T17:33:51.277834Z",
      "updateTime": "2026-10-06T17:33:52.190220Z"
    },
    "targetIds": [
      68,
      64
    ]
  }
}
]],[237,[{
  "targetChange": {
    "targetIds": [
      54,
      48,
      10,
      2,
      40,
      72,
      26,
      70,
      78,
      96,
      76,
      38,
      20,
      28,
      52,
      36,
      88,
      30,
      18,
      68,
      58,
      66,
      86,
      50,
      34,
      44,
      12,
      46,
      24,
      22,
      14,
      74,
      94,
      42,
      16,
      92,
      64,
      8,
      82,
      32,
      84,
      56,
      4,
      90
    ],
    "resumeToken": "CgkIjM7Et/allwM=",
    "readTime": "2026-10-06T17:33:52.190220Z"
  }
}
]],[238,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/conversations/1BQkbHroywZpI0ejYPGP",
      "fields": {
        "zoneId": {
          "stringValue": "XxzogfC5WwJLDYOOIcPc"
        },
        "users": {
          "mapValue": {
            "fields": {
              "Rxp3M2MJzra9wQJm0VkDfRSgJgt1": {
                "mapValue": {
                  "fields": {
                    "name": {
                      "stringValue": "Kury"
                    },
                    "joinAt": {
                      "timestampValue": "2026-10-06T17:33:51Z"
                    },
                    "lastJoinAt": {
                      "timestampValue": "2026-10-06T17:33:51Z"
                    },
                    "status": {
                      "stringValue": "online"
                    }
                  }
                }
              }
            }
          }
        },
        "roomId": {
          "stringValue": "YmAfBb3h2eY5r9FsAIfV"
        },
        "meetingChatId": {
          "nullValue": null
        },
        "privacy": {
          "stringValue": "public"
        },
        "createdAt": {
          "timestampValue": "2026-10-06T17:33:51.179Z"
        },
        "participants": {
          "mapValue": {}
        },
        "roomDefinitionId": {
          "stringValue": "yPsvwRdkb5LnGHfXFgfO"
        },
        "hasOnlineUsers": {
          "booleanValue": true
        },
        "meetingChatProvider": {
          "stringValue": "sowork"
        },
        "livekitRoomId": {
          "stringValue": "RM_unGbwGcgK3ai"
        },
        "sc": {
          "arrayValue": {
            "values": [
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:owner"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:contributor"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:guest"
              }
            ]
          }
        },
        "onlineCount": {
          "integerValue": "1"
        },
        "ownerId": {
          "nullValue": null
        },
        "sessionId": {
          "stringValue": "623b1c7b-e8f8-40e6-903e-1dd56bbd4086"
        },
        "ownerGroupId": {
          "stringValue": "v0XvoIt3uK40pdF0mQDB"
        },
        "status": {
          "stringValue": "active"
        }
      },
      "createTime": "2026-10-06T17:33:51.277834Z",
      "updateTime": "2026-10-06T17:33:52.190220Z"
    },
    "targetIds": [
      98
    ]
  }
}
]],[239,[{
  "targetChange": {
    "targetIds": [
      98
    ],
    "resumeToken": "CgkItPfFt/allwM=",
    "readTime": "2026-10-06T17:33:52.211892Z"
  }
}
]],[240,[{
  "targetChange": {
    "resumeToken": "CgkItPfFt/allwM=",
    "readTime": "2026-10

### 192.688 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=240&CI=1&TYPE=xmlhttp&zx=2bi00n1cjo41&t=1 -> 200
RES: 3891
[[241,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/users/Rxp3M2MJzra9wQJm0VkDfRSgJgt1",
      "fields": {
        "pets": {
          "mapValue": {
            "fields": {
              "pKWdV0wiu601x4MhA1AO": {
                "mapValue": {
                  "fields": {
                    "shouldSpawn": {
                      "booleanValue": false
                    }
                  }
                }
              }
            }
          }
        },
        "xp": {
          "integerValue": "310"
        },
        "name": {
          "stringValue": "Kury"
        },
        "meetingHours": {
          "mapValue": {
            "fields": {
              "durationExpiredAt": {
                "timestampValue": "2026-10-07T03:00:00.626Z"
              },
              "current": {
                "mapValue": {
                  "fields": {
                    "conversationId": {
                      "stringValue": "1BQkbHroywZpI0ejYPGP"
                    },
                    "joinAt": {
                      "timestampValue": "2026-10-06T17:33:51Z"
                    }
                  }
                }
              },
              "duration": {
                "integerValue": "0"
              }
            }
          }
        },
        "timezone": {
          "stringValue": "America/Sao_Paulo"
        },
        "birthday": {
          "mapValue": {
            "fields": {
              "enabled": {
                "booleanValue": true
              },
              "date": {
                "timestampValue": "2026-10-17T03:00:00Z"
              }
            }
          }
        },
        "email": {
          "stringValue": "gabriel@20dash.com"
        },
        "character": {
          "mapValue": {
            "fields": {
              "bodyType": {
                "stringValue": "characterA"
              },
              "slots": {
                "mapValue": {
                  "fields": {
                    "hair": {
                      "stringValue": "characterA_hair_barret+black"
                    },
                    "beard": {
                      "stringValue": "characterA_beard_turk+black"
                    },
                    "pants": {
                      "stringValue": "characterA_pants_bermudaLong+black"
                    },
                    "mouth": {
                      "stringValue": "characterA_mouth_mouth1+typeA"
                    },
                    "shoes": {
                      "stringValue": "characterA_shoes_stepLuck+green"
                    },
                    "eyebrows": {
                      "stringValue": "characterA_eyebrows_eyebrows1+brown"
                    },
                    "body": {
                      "stringValue": "characterA_body_v1+typeA"
                    },
                    "eyes": {
                      "stringValue": "characterA_eyes_eyes1+grey"
                    },
                    "shirt": {
                      "stringValue": "characterA_shirt_alanShirt+blonde"
                    },
                    "bodyAccessory": {
                      "stringValue": "characterA_bodyAccessory_nateDrakeHolster+blonde"
                    },
                    "backAccessory": {
                      "stringValue": "characterA_backAccessory_fishingBackPack+brown"
                    }
                  }
                }
              },
              "version": {
                "integerValue": "5"
              }
            }
          }
        }
      },
      "createTime": "2026-08-10T12:38:17.792013Z",
      "updateTime": "2026-10-06T17:33:52.331518Z"
    },
    "targetIds": [
      70
    ]
  }
}
]],[242,[{
  "targetChange": {
    "resumeToken": "CgkI/p3Nt/allwM=",
    "readTime": "2026-10-06T17:33:52.331518Z"
  }
}
]]]

### 192.830 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=242&CI=1&TYPE=xmlhttp&zx=m7oghtxoduuo&t=1 -> 200
RES: 

### 193.307 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=244&CI=1&TYPE=xmlhttp&zx=ryyxcybpidit&t=1 -> 200
RES: 23067
[[245,[{
  "targetChange": {
    "targetIds": [
      86,
      66,
      44,
      22,
      46,
      14,
      12,
      94,
      74,
      24,
      8,
      92,
      42,
      82,
      48,
      32,
      2,
      16,
      84,
      90,
      54,
      4,
      56,
      64,
      40,
      72,
      26,
      10,
      96,
      38,
      78,
      28,
      76,
      20,
      68,
      36,
      70,
      34,
      30,
      88,
      18,
      52,
      50,
      58
    ],
    "resumeToken": "CgkI3Obut/allwM=",
    "readTime": "2026-10-06T17:33:52.881500Z"
  }
}
]],[246,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/conversations/1BQkbHroywZpI0ejYPGP",
      "fields": {
        "ownerGroupId": {
          "stringValue": "v0XvoIt3uK40pdF0mQDB"
        },
        "users": {
          "mapValue": {
            "fields": {
              "RMUYRzBXvUOCqtJdxSAiMdiEZM82": {
                "mapValue": {
                  "fields": {
                    "lastJoinAt": {
                      "timestampValue": "2026-10-06T17:33:52Z"
                    },
                    "joinAt": {
                      "timestampValue": "2026-10-06T17:33:52Z"
                    },
                    "name": {
                      "stringValue": "Tito Guidotti"
                    },
                    "status": {
                      "stringValue": "online"
                    }
                  }
                }
              },
              "Rxp3M2MJzra9wQJm0VkDfRSgJgt1": {
                "mapValue": {
                  "fields": {
                    "status": {
                      "stringValue": "online"
                    },
                    "name": {
                      "stringValue": "Kury"
                    },
                    "lastJoinAt": {
                      "timestampValue": "2026-10-06T17:33:51Z"
                    },
                    "joinAt": {
                      "timestampValue": "2026-10-06T17:33:51Z"
                    }
                  }
                }
              }
            }
          }
        },
        "livekitRoomId": {
          "stringValue": "RM_unGbwGcgK3ai"
        },
        "status": {
          "stringValue": "active"
        },
        "privacy": {
          "stringValue": "public"
        },
        "sessionId": {
          "stringValue": "623b1c7b-e8f8-40e6-903e-1dd56bbd4086"
        },
        "participants": {
          "mapValue": {}
        },
        "sc": {
          "arrayValue": {
            "values": [
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:owner"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:contributor"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:guest"
              }
            ]
          }
        },
        "meetingChatProvider": {
          "stringValue": "sowork"
        },
        "roomDefinitionId": {
          "stringValue": "yPsvwRdkb5LnGHfXFgfO"
        },
        "meetingChatId": {
          "nullValue": null
        },
        "hasOnlineUsers": {
          "booleanValue": true
        },
        "zoneId": {
          "stringValue": "XxzogfC5WwJLDYOOIcPc"
        },
        "onlineCount": {
          "integerValue": "2"
        },
        "ownerId": {
          "nullValue": null
        },
        "roomId": {
          "stringValue": "YmAfBb3h2eY5r9FsAIfV"
        },
        "createdAt": {
          "timestampValue": "2026-10-06T17:33:51.179Z"
        }
      },
      "createTime": "2026-10-06T17:33:51.277834Z",
      "updateTime": "2026-10-06T17:33:52.881500Z"
    },
    "targetIds": [
      98
    ]
  }
}
]],[247,[{
  "targetChange": {
    "targetIds": [
      98
    ],
    "resumeToken": "CgkIrqHyt/allwM=",
    "readTime": "2026-10-06T17:33:52.938158Z"
  }
}
]],[248,[{
  "targetChange": {
    "resumeToken": "CgkIrqHyt/allwM=",
    "readTime": "2026-10-06T17:33:52.938158Z"
  }
}
]],[249,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/users/RMUYRzBXvUOCqtJdxSAiMdiEZM82",
      "fields": {
        "role": {
          "stringValue": "Senior junior pleno executivo dedev"
        },
        "email": {
          "stringValue": "tito@20dash.com"
        },
        "timezone": {
          "stringValue": "Europe/Madrid"
        },
        "workingHours": {
          "mapValue": {
            "fields": {
              "ranges": {
                "mapValue": {
                  "fields": {
                    "6": {
                      "mapValue": {
                        "fields": {
                          "slots": {
                            "arrayValue": {
                              "values": [
                                {
                                  "mapValue": {
                                    "fields": {
                                      "customStatus": {
                                        "nullValue": null
                                      },
                                      "start": {
                                        "stringValue": "09:00:00.000"
                                      },
                                      "slotId": {
                                        "stringValue": "927320a4-4134-4580-8e80-5a8f207940ee"
                                      },
                                      "end": {
                                        "stringValue": "17:00:00.000"
                                      }
                                    }
                                  }
                                }
                              ]
                            }
                          },
                          "enabled": {
                            "booleanValue": false
                          },
                          "range": {
       

### 193.479 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=250&CI=1&TYPE=xmlhttp&zx=5afq15ufi7xo&t=1 -> 200
RES: 

### 195.749 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65323&AID=251&zx=w5mhvdsx4mjk&t=1 -> 200
REQ: count=1&ofs=53&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A58%7D
RES: 10
[1,251,7]


### 195.894 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=252&CI=1&TYPE=xmlhttp&zx=49350rcizm62&t=1 -> 200
RES: 

### 195.895 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65324&AID=252&zx=p8qklfpa51gm&t=1 -> 200
REQ: count=1&ofs=54&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A70%7D
RES: 10
[1,253,7]


### 196.046 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=254&CI=1&TYPE=xmlhttp&zx=hvpm64xu36vh&t=1 -> 200
RES: 

### 197.639 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 202.682 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 207.728 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 212.775 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 214.848 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=256&CI=1&TYPE=xmlhttp&zx=3mfa3rnb4voe&t=1 -> 200
RES: 123
[[257,[{
  "targetChange": {
    "resumeToken": "CgkIzJ6UwvallwM=",
    "readTime": "2026-10-06T17:34:14.466380Z"
  }
}
]]]

### 214.990 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=257&CI=1&TYPE=xmlhttp&zx=46yi54aebl9s&t=1 -> 200
RES: 

### 217.595 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=259&CI=1&TYPE=xmlhttp&zx=o2dr32x57e2x&t=1 -> 200
RES: 280
[[260,[{
  "targetChange": {
    "targetIds": [
      90
    ],
    "resumeToken": "CgkInZ67w/allwM=",
    "readTime": "2026-10-06T17:34:17.202461Z"
  }
}
]],[261,[{
  "targetChange": {
    "resumeToken": "CgkInZ67w/allwM=",
    "readTime": "2026-10-06T17:34:17.202461Z"
  }
}
]]]

### 217.733 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=261&CI=1&TYPE=xmlhttp&zx=h25zn2qt5rv0&t=1 -> 200
RES: 

### 217.835 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 221.514 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=263&CI=1&TYPE=xmlhttp&zx=qkcu7lcqfayk&t=1 -> 200
RES: 123
[[264,[{
  "targetChange": {
    "resumeToken": "CgkIhJ2oxfallwM=",
    "readTime": "2026-10-06T17:34:21.085316Z"
  }
}
]]]

### 221.655 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=264&CI=1&TYPE=xmlhttp&zx=shylorct8b1d&t=1 -> 200
RES: 

### 222.885 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 227.167 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65325&AID=265&zx=fy31lierddx&t=1 -> 200
REQ: count=1&ofs=55&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A72%7D
RES: 10
[1,265,7]


### 227.333 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=266&CI=1&TYPE=xmlhttp&zx=xg49m9p1me75&t=1 -> 200
RES: 

### 227.334 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65326&AID=266&zx=w4viwbc97s74&t=1 -> 200
REQ: count=3&ofs=56&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A74%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A76%7D&req2___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A78%7D
RES: 10
[0,266,7]


### 227.594 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=269&CI=1&TYPE=xmlhttp&zx=92e6iec14zdf&t=1 -> 200
RES: 

### 227.945 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 229.149 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65327&AID=270&zx=hnv9u8cu0zw&t=1 -> 200
REQ: count=1&ofs=59&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A64%7D
RES: 10
[1,270,7]


### 229.288 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65328&AID=270&zx=6fbbnjxki21v&t=1 -> 200
REQ: count=2&ofs=60&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A68%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A66%7D
RES: 10
[0,271,7]


### 229.289 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=271&CI=1&TYPE=xmlhttp&zx=w3m44t9skfqw&t=1 -> 200
RES: 16
[[272,["noop"]]]106
[[273,[{
  "targetChange": {
    "targetChangeType": "REMOVE",
    "targetIds": [
      68
    ]
  }
}
]]]

### 229.427 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=273&CI=1&TYPE=xmlhttp&zx=wfkcv6p0lurj&t=1 -> 200
RES: 106
[[274,[{
  "targetChange": {
    "targetChangeType": "REMOVE",
    "targetIds": [
      66
    ]
  }
}
]]]

### 229.602 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=274&CI=1&TYPE=xmlhttp&zx=9ngsgzpker37&t=1 -> 200
RES: 

### 232.990 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 238.059 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 243.187 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 248.301 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 253.569 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 253.611 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=276&CI=1&TYPE=xmlhttp&zx=23j95oiy4ftm&t=1 -> 200
RES: 280
[[277,[{
  "targetChange": {
    "targetIds": [
      96
    ],
    "resumeToken": "CgkIuunQ1PallwM=",
    "readTime": "2026-10-06T17:34:53.207738Z"
  }
}
]],[278,[{
  "targetChange": {
    "resumeToken": "CgkIuunQ1PallwM=",
    "readTime": "2026-10-06T17:34:53.207738Z"
  }
}
]]]

### 253.788 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=278&CI=1&TYPE=xmlhttp&zx=oh0ykzih7vrd&t=1 -> 200
RES: 

### 254.148 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65329&AID=279&zx=drefu9403yln&t=1 -> 200
REQ: count=1&ofs=62&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A56%7D
RES: 10
[1,279,7]


### 254.310 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=280&CI=1&TYPE=xmlhttp&zx=szbw1f99c4nx&t=1 -> 200
RES: 

### 257.000 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=LZpPn8fHU8l56MnxY-WmYOZainSh9YrP82xawbG90Kty1lG5D2NBFQ&VER=8&database=projects%2Fsophya-calendar-production%2Fdatabases%2F(default)&RID=rpc&SID=AYiWFFk6YV4N591bkO7F1Q&AID=8&CI=0&TYPE=xmlhttp&zx=sx4w70eo3q&t=1 -> 200
RES: 

### 258.651 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 259.564 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=282&CI=1&TYPE=xmlhttp&zx=bv98wibtbvfc&t=1 -> 200
RES: 123
[[283,[{
  "targetChange": {
    "resumeToken": "CgkIsbS91/allwM=",
    "readTime": "2026-10-06T17:34:59.181105Z"
  }
}
]]]

### 259.749 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=283&CI=1&TYPE=xmlhttp&zx=yupxt75v71le&t=1 -> 200
RES: 

### 263.779 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 270.474 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 275.704 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 278.375 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65330&AID=284&zx=ksaynxgtqciu&t=1 -> 200
REQ: count=1&ofs=63&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A98%7D
RES: 10
[1,284,7]


### 278.521 POST https://api.sowork.com/api/v1/gaia/rooms -> 200
REQ: {"groupId":"v0XvoIt3uK40pdF0mQDB"}
RES: {"roomDefinitionMap":{"yPsvwRdkb5LnGHfXFgfO":{"roomDefinition":{"id":"yPsvwRdkb5LnGHfXFgfO","groupId":"v0XvoIt3uK40pdF0mQDB","name":"Escritório principal","alias":"mistymountainlodge","mapDefinitionId":"V1vAAHj80gty4MwBqEFp","ownerKey":"group_v0XvoIt3uK40pdF0mQDB","primary":true,"isMeetingSpace":false,"zones":null,"createdAt":{"seconds":1786123761,"nanoseconds":850000000},"updatedAt":{"seconds":1786123761,"nanoseconds":850000000},"roomObjects":{},"roomGroupId":null,"pricingType":"premium"},"rooms":[{"id":"YmAfBb3h2eY5r9FsAIfV","name":"Escritório principal","index":1,"local":false,"serverId":"g5","server":{"url":"https://api.sowork.com/gaia/g5"},"userIds":["JLDRGMskJRXiYcHccNVaLuwr3XJ3","rjOwBlMG93QD2sjutWoF2fc8Mv63","fCfzuTuBkBSLyF1tKdMJOZSOnUu1","G7EWVnLMQngko5v7KcgNnH06Fhg1","Rxp3M2MJzra9wQJm0VkDfRSgJgt1","blKsp8Fxi6VGJfXfizN0iu248IB2","IEwNgnRoYUcFOwc4KSrz7MyHNvG2","zE7SCh3eZDggMBNhjB5xkpeglNn2","FLBKjFg1PYe5bD6cE0bdR7BhE7r1","zdXi8bJLG6cNbd4P87bqTAzTdTq1","RMUYRzBXvUOCqtJdxSAiMdiEZM82"],"userCount":11,"roomDefinitionId":"yPsvwRdkb5LnGHfXFgfO","roomDefinition":{"ownerKey":"group_v0XvoIt3uK40pdF0mQDB","name":"Misty Mountain Lodge","groupId":"v0XvoIt3uK40pdF0mQDB","zones":null,"projection":null},"isMeetingSpace":false,"createdAt":{"seconds":1786123764,"nanoseconds":340000000},"updatedAt":{"seconds":1786123764,"nanoseconds":340000000},"roomGroupId":null}]}}}

### 278.695 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=285&CI=1&TYPE=xmlhttp&zx=gtjhsrbo6g91&t=1 -> 200
RES: 1801
[[286,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/group_membermaps/v0XvoIt3uK40pdF0mQDB_2",
      "fields": {
        "users": {
          "mapValue": {
            "fields": {
              "Rxp3M2MJzra9wQJm0VkDfRSgJgt1": {
                "mapValue": {
                  "fields": {
                    "roomDefinitionId": {
                      "stringValue": "yPsvwRdkb5LnGHfXFgfO"
                    },
                    "isOnline": {
                      "booleanValue": true
                    },
                    "inCalendarMeeting": {
                      "nullValue": null
                    },
                    "role": {
                      "stringValue": "contributor"
                    },
                    "lastOnline": {
                      "timestampValue": "2026-10-06T17:35:17.693Z"
                    },
                    "mode": {
                      "stringValue": "room"
                    },
                    "inPrivateConversation": {
                      "booleanValue": false
                    },
                    "userName": {
                      "stringValue": "Kury"
                    },
                    "status": {
                      "stringValue": "online"
                    },
                    "mobile": {
                      "booleanValue": false
                    }
                  }
                }
              }
            }
          }
        }
      },
      "createTime": "2026-08-10T12:38:18.104479Z",
      "updateTime": "2026-10-06T17:35:18.302674Z"
    },
    "targetIds": [
      28
    ]
  }
}
]],[287,[{
  "targetChange": {
    "resumeToken": "CgkI0r/M4PallwM=",
    "readTime": "2026-10-06T17:35:18.302674Z"
  }
}
]]]

### 278.709 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65331&AID=285&zx=bb69ajt7et8&t=1 -> 200
REQ: count=5&ofs=64&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22conversations%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22ownerGroupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomDefinitionId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22yPsvwRdkb5LnGHfXFgfO%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22active%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22op%22%3A%22GREATER_THAN%22%2C%22value%22%3A%7B%22integerValue%22%3A%220%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%2C%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A100%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22conversations%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22ownerGroupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22unaryFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomDefinitionId%22%7D%2C%22op%22%3A%22IS_NULL%22%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22active%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22op%22%3A%22GREATER_THAN%22%2C%22value%22%3A%7B%22integerValue%22%3A%220%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%2C%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A102%7D%7D&req2___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22conversations%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22ownerGroupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomDefinitionId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22yPsvwRdkb5LnGHfXFgfO%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22YmAfBb3h2eY5r9FsAIfV%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22active%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22op%22%3A%22GREATER_THAN%22%2C%22value%22%3A%7B%22integerValue%22%3A%220%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22
RES: 10
[0,287,7]


### 278.848 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=287&CI=1&TYPE=xmlhttp&zx=i28njrorbxme&t=1 -> 200
RES: 18082
[[288,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      100
    ]
  }
}
]],[289,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      102
    ]
  }
}
]],[290,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      104
    ]
  }
}
]],[291,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      106
    ]
  }
}
]],[292,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      108
    ]
  }
}
]],[293,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/users/EsFlL0qd1kZNo0IsotREJWQWNeC3",
      "fields": {
        "email": {
          "stringValue": "lucas@20dash.com"
        },
        "timezone": {
          "stringValue": "America/Sao_Paulo"
        },
        "birthday": {
          "mapValue": {
            "fields": {
              "date": {
                "timestampValue": "2026-04-25T03:00:00Z"
              },
              "enabled": {
                "booleanValue": true
              }
            }
          }
        },
        "role": {
          "stringValue": "severino"
        },
        "meetingHours": {
          "mapValue": {
            "fields": {
              "duration": {
                "integerValue": "0"
              },
              "current": {
                "mapValue": {
                  "fields": {
                    "joinAt": {
                      "timestampValue": "2026-09-25T21:00:42Z"
                    },
                    "conversationId": {
                      "stringValue": "coRBRS7NPl4Bys2IgPKM"
                    }
                  }
                }
              },
              "durationExpiredAt": {
                "timestampValue": "2026-09-26T03:00:00.815Z"
              }
            }
          }
        },
        "xp": {
          "integerValue": "275"
        },
        "character": {
          "mapValue": {
            "fields": {
              "bodyType": {
                "stringValue": "characterA"
              },
              "slots": {
                "mapValue": {
                  "fields": {
                    "headAccessory": {
                      "stringValue": "characterA_headAccessory_pixelSunglasses+black"
                    },
                    "eyes": {
                      "stringValue": "characterA_eyes_eyes1+green"
                    },
                    "shirt": {
                      "stringValue": "characterA_shirt_punkMaleJacket+red"
                    },
                    "eyebrows": {
                      "stringValue": "characterA_eyebrows_eyebrows1+brown"
                    },
                    "pants": {
                      "stringValue": "characterA_pants_nathanPants+white"
                    },
                    "shoes": {
                      "stringValue": "characterA_shoes_waterBoots+black"
                    },
                    "mouth": {
                      "stringValue": "characterA_mouth_mouth1+typeC"
                    },
                    "hair": {
                      "stringValue": "characterA_hair_sarah+black"
                    },
                    "beard": {
                      "stringValue": "characterA_beard_sugar+black"
                    },
                    "body": {
                      "stringValue": "characterA_body_v1+typeC"
                    }
                  }
                }
              },
              "version": {
                "integerValue": "5"
              }
            }
          }
        },
        "name": {
          "stringValue": "Lucas Pacheco"
        }
      },
      "createTime": "2026-08-10T13:53:57.207496Z",
      "updateTime": "2026-09-25T21:00:44.991470Z"
    },
    "targetIds": [
      108
    ]
  }
}
]],[294,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      108
    ],
    "resumeToken": "CgkInpzX4PallwM=",
    "readTime": "2026-10-06T17:35:18.478366Z"
  }
}
]],[295,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/group_games/v0XvoIt3uK40pdF0mQDB",
      "fields": {
        "rooms": {
          "mapValue": {}
        }
      },
      "createTime": "2026-08-07T17:29:51.296825Z",
      "updateTime": "2026-08-07T17:29:51.296825Z"
    },
    "targetIds": [
      106
    ]
  }
}
]],[296,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      106
    ],
    "resumeToken": "CgkInpzX4PallwM=",
    "readTime": "2026-10-06T17:35:18.478366Z"
  }
}
]],[297,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      102
    ],
    "resumeToken": "CgkInpzX4PallwM=",
    "readTime": "2026-10-06T17:35:18.478366Z"
  }
}
]],[298,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/conversations/1BQkbHroywZpI0ejYPGP",
      "fields": {
        "roomDefinitionId": {
          "stringValue": "yPsvwRdkb5LnGHfXFgfO"
        },
        "onlineCount": {
          "integerValue": "2"
        },
        "ownerGroupId": {
          "stringValue": "v0XvoIt3uK40pdF0mQDB"
        },
        "hasOnlineUsers": {
          "booleanValue": true
        },
        "meetingChatProvider": {
          "stringValue": "sowork"
        },
        "status": {
          "stringValue": "active"
        },
        "livekitRoomId": {
          "stringValue": "RM_unGbwGcgK3ai"
        },
        "sessionId": {
          "stringValue": "623b1c7b-e8f8-40e6-903e-1dd56bbd4086"
        },
        "ownerId": {
          "nullValue": null
        },
        "participants": {
          "mapValue": {}
        },
        "meetingChatId": {
          "nullValue": null
        },
        "roomId": {
          "stringValue": "YmAfBb3h2eY5r9FsAIfV"
        },
        "zoneId": {
          "stringValue": "XxzogfC5WwJLDYOOIcPc"
        },
        "users": {
 

### 278.916 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65332&AID=287&zx=a1mg1p52y2s9&t=1 -> 200
REQ: count=9&ofs=69&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FzdXi8bJLG6cNbd4P87bqTAzTdTq1%22%5D%7D%2C%22targetId%22%3A110%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FFLBKjFg1PYe5bD6cE0bdR7BhE7r1%22%5D%7D%2C%22targetId%22%3A112%7D%7D&req2___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FfCfzuTuBkBSLyF1tKdMJOZSOnUu1%22%5D%7D%2C%22targetId%22%3A114%7D%7D&req3___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FRxp3M2MJzra9wQJm0VkDfRSgJgt1%22%5D%7D%2C%22targetId%22%3A116%7D%7D&req4___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FzE7SCh3eZDggMBNhjB5xkpeglNn2%22%5D%7D%2C%22targetId%22%3A118%7D%7D&req5___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FblKsp8Fxi6VGJfXfizN0iu248IB2%22%5D%7D%2C%22targetId%22%3A120%7D%7D&req6___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FG7EWVnLMQngko5v7KcgNnH06Fhg1%22%5D%7D%2C%22targetId%22%3A122%7D%7D&req7___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FJLDRGMskJRXiYcHccNVaLuwr3XJ3%22%5D%7D%2C%22targetId%22%3A124%7D%7D&req8___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FrjOwBlMG93QD2sjutWoF2fc8Mv63%22%5D%7D%2C%22targetId%22%3A126%7D%7D
RES: 10
[0,304,7]


### 278.986 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=304&CI=1&TYPE=xmlhttp&zx=ysdju423h112&t=1 -> 200
RES: 

### 279.055 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65333&AID=304&zx=648i6p46ytat&t=1 -> 200
REQ: count=1&ofs=78&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A108%7D
RES: 10
[0,331,7]


### 279.136 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=331&CI=1&TYPE=xmlhttp&zx=zfj76ml8c0o2&t=1 -> 200
RES: 229
[[332,[{
  "targetChange": {
    "resumeToken": "CgkIy+7j4PallwM=",
    "readTime": "2026-10-06T17:35:18.685515Z"
  }
}
]],[333,[{
  "targetChange": {
    "targetChangeType": "REMOVE",
    "targetIds": [
      108
    ]
  }
}
]]]

### 279.283 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=333&CI=1&TYPE=xmlhttp&zx=4adi9o74f8fl&t=1 -> 200
RES: 3516
[[334,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/conversations/1BQkbHroywZpI0ejYPGP",
      "fields": {
        "privacy": {
          "stringValue": "public"
        },
        "users": {
          "mapValue": {
            "fields": {
              "RMUYRzBXvUOCqtJdxSAiMdiEZM82": {
                "mapValue": {
                  "fields": {
                    "lastJoinAt": {
                      "timestampValue": "2026-10-06T17:33:52Z"
                    },
                    "status": {
                      "stringValue": "online"
                    },
                    "name": {
                      "stringValue": "Tito Guidotti"
                    },
                    "joinAt": {
                      "timestampValue": "2026-10-06T17:33:52Z"
                    }
                  }
                }
              },
              "Rxp3M2MJzra9wQJm0VkDfRSgJgt1": {
                "mapValue": {
                  "fields": {
                    "status": {
                      "stringValue": "offline"
                    },
                    "name": {
                      "stringValue": "Kury"
                    },
                    "joinAt": {
                      "timestampValue": "2026-10-06T17:33:51Z"
                    },
                    "lastJoinAt": {
                      "nullValue": null
                    },
                    "duration": {
                      "integerValue": "87"
                    },
                    "leftAt": {
                      "timestampValue": "2026-10-06T17:35:18Z"
                    }
                  }
                }
              }
            }
          }
        },
        "meetingChatId": {
          "nullValue": null
        },
        "sessionId": {
          "stringValue": "623b1c7b-e8f8-40e6-903e-1dd56bbd4086"
        },
        "status": {
          "stringValue": "active"
        },
        "onlineCount": {
          "integerValue": "1"
        },
        "sc": {
          "arrayValue": {
            "values": [
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:owner"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:contributor"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:guest"
              }
            ]
          }
        },
        "hasOnlineUsers": {
          "booleanValue": true
        },
        "createdAt": {
          "timestampValue": "2026-10-06T17:33:51.179Z"
        },
        "participants": {
          "mapValue": {}
        },
        "ownerId": {
          "nullValue": null
        },
        "ownerGroupId": {
          "stringValue": "v0XvoIt3uK40pdF0mQDB"
        },
        "roomDefinitionId": {
          "stringValue": "yPsvwRdkb5LnGHfXFgfO"
        },
        "zoneId": {
          "stringValue": "XxzogfC5WwJLDYOOIcPc"
        },
        "meetingChatProvider": {
          "stringValue": "sowork"
        },
        "roomId": {
          "stringValue": "YmAfBb3h2eY5r9FsAIfV"
        },
        "livekitRoomId": {
          "stringValue": "RM_unGbwGcgK3ai"
        }
      },
      "createTime": "2026-10-06T17:33:51.277834Z",
      "updateTime": "2026-10-06T17:35:18.928083Z"
    },
    "targetIds": [
      104,
      100
    ]
  }
}
]],[335,[{
  "targetChange": {
    "resumeToken": "CgkI09Xy4PallwM=",
    "readTime": "2026-10-06T17:35:18.928083Z"
  }
}
]]]

### 279.432 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=335&CI=1&TYPE=xmlhttp&zx=8a6r61rzjcwx&t=1 -> 200
RES: 

### 279.677 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=338&CI=1&TYPE=xmlhttp&zx=pazoj7dx1g36&t=1 -> 200
RES: 

### 280.124 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=340&CI=1&TYPE=xmlhttp&zx=kzswqihjk3ne&t=1 -> 200
RES: 123
[[341,[{
  "targetChange": {
    "resumeToken": "CgkIzr+i4fallwM=",
    "readTime": "2026-10-06T17:35:19.711694Z"
  }
}
]]]

### 280.397 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=341&CI=1&TYPE=xmlhttp&zx=hfyvjsj7m6r3&t=1 -> 200
RES: 

### 280.397 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65334&AID=341&zx=19ownkzgjpaq&t=1 -> 200
REQ: count=1&ofs=79&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fconversations%2F1BQkbHroywZpI0ejYPGP%22%5D%7D%2C%22targetId%22%3A1%7D%2C%22labels%22%3A%7B%22goog-listen-tags%22%3A%22limbo-document%22%7D%7D
RES: 10
[1,342,7]


### 280.557 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=343&CI=1&TYPE=xmlhttp&zx=4nx1cbo14ado&t=1 -> 200
RES: 3891
[[344,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/conversations/1BQkbHroywZpI0ejYPGP",
      "fields": {
        "ownerGroupId": {
          "stringValue": "v0XvoIt3uK40pdF0mQDB"
        },
        "status": {
          "stringValue": "active"
        },
        "meetingChatId": {
          "nullValue": null
        },
        "privacy": {
          "stringValue": "public"
        },
        "meetingChatProvider": {
          "stringValue": "sowork"
        },
        "users": {
          "mapValue": {
            "fields": {
              "RMUYRzBXvUOCqtJdxSAiMdiEZM82": {
                "mapValue": {
                  "fields": {
                    "status": {
                      "stringValue": "offline"
                    },
                    "leftAt": {
                      "timestampValue": "2026-10-06T17:35:18Z"
                    },
                    "duration": {
                      "integerValue": "86"
                    },
                    "lastJoinAt": {
                      "nullValue": null
                    },
                    "joinAt": {
                      "timestampValue": "2026-10-06T17:33:52Z"
                    },
                    "name": {
                      "stringValue": "Tito Guidotti"
                    }
                  }
                }
              },
              "Rxp3M2MJzra9wQJm0VkDfRSgJgt1": {
                "mapValue": {
                  "fields": {
                    "leftAt": {
                      "timestampValue": "2026-10-06T17:35:18Z"
                    },
                    "joinAt": {
                      "timestampValue": "2026-10-06T17:33:51Z"
                    },
                    "status": {
                      "stringValue": "offline"
                    },
                    "duration": {
                      "integerValue": "87"
                    },
                    "name": {
                      "stringValue": "Kury"
                    },
                    "lastJoinAt": {
                      "nullValue": null
                    }
                  }
                }
              }
            }
          }
        },
        "sessionId": {
          "stringValue": "623b1c7b-e8f8-40e6-903e-1dd56bbd4086"
        },
        "participants": {
          "mapValue": {}
        },
        "hasOnlineUsers": {
          "booleanValue": false
        },
        "ownerId": {
          "nullValue": null
        },
        "sc": {
          "arrayValue": {
            "values": [
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:owner"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:contributor"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:guest"
              }
            ]
          }
        },
        "onlineCount": {
          "integerValue": "0"
        },
        "livekitRoomId": {
          "stringValue": "RM_unGbwGcgK3ai"
        },
        "roomId": {
          "stringValue": "YmAfBb3h2eY5r9FsAIfV"
        },
        "roomDefinitionId": {
          "stringValue": "yPsvwRdkb5LnGHfXFgfO"
        },
        "zoneId": {
          "stringValue": "XxzogfC5WwJLDYOOIcPc"
        },
        "createdAt": {
          "timestampValue": "2026-10-06T17:33:51.179Z"
        }
      },
      "createTime": "2026-10-06T17:33:51.277834Z",
      "updateTime": "2026-10-06T17:35:19.711694Z"
    },
    "targetIds": [
      1
    ]
  }
}
]],[345,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      1
    ],
    "resumeToken": "CgkIyLu+4fallwM=",
    "readTime": "2026-10-06T17:35:20.169928Z"
  }
}
]],[346,[{
  "targetChange": {
    "resumeToken": "CgkIyLu+4fallwM=",
    "readTime": "2026-10-06T17:35:20.169928Z"
  }
}
]]]

### 280.743 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=346&CI=1&TYPE=xmlhttp&zx=8id9ttt0v0zh&t=1 -> 200
RES: 105
[[347,[{
  "targetChange": {
    "targetChangeType": "REMOVE",
    "targetIds": [
      1
    ]
  }
}
]]]

### 280.743 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65335&AID=346&zx=v0vd4mv44iet&t=1 -> 200
REQ: count=1&ofs=80&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A1%7D
RES: 10
[0,346,7]


### 280.791 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 280.906 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65336&AID=346&zx=pf10lc4yp0xd&t=1 -> 200
REQ: count=1&ofs=81&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FIEwNgnRoYUcFOwc4KSrz7MyHNvG2%22%5D%7D%2C%22targetId%22%3A128%7D%7D
RES: 10
[0,347,7]


### 280.907 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=347&CI=1&TYPE=xmlhttp&zx=226kqe4qasc4&t=1 -> 200
RES: 16
[[348,["noop"]]]104
[[349,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      128
    ]
  }
}
]]]

### 281.434 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=349&CI=1&TYPE=xmlhttp&zx=jndtqiv3cnve&t=1 -> 200
RES: 

### 281.656 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=352&CI=1&TYPE=xmlhttp&zx=z0xcjuau2ka9&t=1 -> 200
RES: 

### 281.974 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=354&CI=1&TYPE=xmlhttp&zx=q590jhu8t4lc&t=1 -> 200
RES: 1897
[[355,[{
  "targetChange": {
    "targetIds": [
      30
    ],
    "resumeToken": "CgkIlKaV4vallwM=",
    "readTime": "2026-10-06T17:35:21.592596Z"
  }
}
]],[356,[{
  "targetChange": {
    "targetIds": [
      24
    ],
    "resumeToken": "CgkIlKaV4vallwM=",
    "readTime": "2026-10-06T17:35:21.592596Z"
  }
}
]],[357,[{
  "targetChange": {
    "targetIds": [
      26
    ],
    "resumeToken": "CgkIlKaV4vallwM=",
    "readTime": "2026-10-06T17:35:21.592596Z"
  }
}
]],[358,[{
  "filter": {
    "targetId": 52,
    "count": 1,
    "unchangedNames": {
      "bits": {
        "bitmap": "qvoB",
        "padding": 7
      },
      "hashCount": 12
    }
  }
}
]],[359,[{
  "targetChange": {
    "targetIds": [
      52
    ],
    "resumeToken": "CgkIlKaV4vallwM=",
    "readTime": "2026-10-06T17:35:21.592596Z"
  }
}
]],[360,[{
  "filter": {
    "targetId": 14,
    "unchangedNames": {
      "bits": {}
    }
  }
}
]],[361,[{
  "targetChange": {
    "targetIds": [
      14
    ],
    "resumeToken": "CgkIlKaV4vallwM=",
    "readTime": "2026-10-06T17:35:21.592596Z"
  }
}
]],[362,[{
  "filter": {
    "targetId": 100,
    "count": 1,
    "unchangedNames": {
      "bits": {
        "bitmap": "JSkB",
        "padding": 5
      },
      "hashCount": 13
    }
  }
}
]],[363,[{
  "targetChange": {
    "targetIds": [
      100
    ],
    "resumeToken": "CgkIlKaV4vallwM=",
    "readTime": "2026-10-06T17:35:21.592596Z"
  }
}
]],[364,[{
  "filter": {
    "targetId": 104,
    "count": 1,
    "unchangedNames": {
      "bits": {
        "bitmap": "JSkB",
        "padding": 5
      },
      "hashCount": 13
    }
  }
}
]],[365,[{
  "targetChange": {
    "targetIds": [
      104
    ],
    "resumeToken": "CgkIlKaV4vallwM=",
    "readTime": "2026-10-06T17:35:21.592596Z"
  }
}
]],[366,[{
  "targetChange": {
    "resumeToken": "CgkIlKaV4vallwM=",
    "readTime": "2026-10-06T17:35:21.592596Z"
  }
}
]]]

### 282.141 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=366&CI=1&TYPE=xmlhttp&zx=9fmv3k5d2hrz&t=1 -> 200
RES: 

### 282.416 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=368&CI=1&TYPE=xmlhttp&zx=gyfgoeazajj4&t=1 -> 200
RES: 2345
[[369,[{
  "filter": {
    "targetId": 14,
    "unchangedNames": {
      "bits": {}
    }
  }
}
]],[370,[{
  "targetChange": {
    "targetIds": [
      14
    ],
    "resumeToken": "CgkIyL2w4vallwM=",
    "readTime": "2026-10-06T17:35:22.037960Z"
  }
}
]],[371,[{
  "targetChange": {
    "targetIds": [
      24
    ],
    "resumeToken": "CgkIyL2w4vallwM=",
    "readTime": "2026-10-06T17:35:22.037960Z"
  }
}
]],[372,[{
  "targetChange": {
    "targetIds": [
      30
    ],
    "resumeToken": "CgkIyL2w4vallwM=",
    "readTime": "2026-10-06T17:35:22.037960Z"
  }
}
]],[373,[{
  "targetChange": {
    "targetIds": [
      26
    ],
    "resumeToken": "CgkIyL2w4vallwM=",
    "readTime": "2026-10-06T17:35:22.037960Z"
  }
}
]],[374,[{
  "targetChange": {
    "targetIds": [
      82,
      102,
      16,
      118,
      32,
      2,
      48,
      120,
      90,
      54,
      112,
      84,
      4,
      44,
      128,
      22,
      14,
      46,
      24,
      94,
      110,
      8,
      92,
      42,
      122,
      12,
      30,
      88,
      114,
      20,
      36,
      28,
      106,
      116,
      18,
      10,
      86,
      34,
      50,
      26,
      124,
      40,
      96,
      38,
      126
    ],
    "resumeToken": "CgkIyL2w4vallwM=",
    "readTime": "2026-10-06T17:35:22.037960Z"
  }
}
]],[375,[{
  "filter": {
    "targetId": 104,
    "count": 1,
    "unchangedNames": {
      "bits": {
        "bitmap": "PO8B",
        "padding": 7
      },
      "hashCount": 12
    }
  }
}
]],[376,[{
  "targetChange": {
    "targetIds": [
      104
    ],
    "resumeToken": "CgkIyL2w4vallwM=",
    "readTime": "2026-10-06T17:35:22.037960Z"
  }
}
]],[377,[{
  "filter": {
    "targetId": 52,
    "count": 1,
    "unchangedNames": {
      "bits": {
        "bitmap": "qvoB",
        "padding": 7
      },
      "hashCount": 12
    }
  }
}
]],[378,[{
  "targetChange": {
    "targetIds": [
      52
    ],
    "resumeToken": "CgkIyL2w4vallwM=",
    "readTime": "2026-10-06T17:35:22.037960Z"
  }
}
]],[379,[{
  "filter": {
    "targetId": 100,
    "count": 1,
    "unchangedNames": {
      "bits": {
        "bitmap": "PO8B",
        "padding": 7
      },
      "hashCount": 12
    }
  }
}
]],[380,[{
  "targetChange": {
    "resumeToken": "CgkIyL2w4vallwM=",
    "readTime": "2026-10-06T17:35:22.037960Z"
  }
}
]]]

### 282.580 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=380&CI=1&TYPE=xmlhttp&zx=qsgw1tqvqwpp&t=1 -> 200
RES: 

### 282.944 HEAD https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud/ -> 200
RES: 

### 282.946 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 282.960 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65337&AID=381&zx=g6vj6rnd4xkd&t=1 -> 200
REQ: count=1&ofs=82&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fconversations%2FbkD5sDVZwkC8fbozpabM%22%5D%7D%2C%22targetId%22%3A130%7D%7D
RES: 10
[0,383,7]


### 283.278 POST https://api.sowork.com/api/v1/crm/track -> 200
REQ: {"userId":"RMUYRzBXvUOCqtJdxSAiMdiEZM82","groupId":"v0XvoIt3uK40pdF0mQDB","events":[{"eventName":"meeting","timestamp":1791308117.961,"properties":{"appSessionId":"1qklZjXtqu-CXhlggjD3G","inMeeting":true,"meetingId":"1BQkbHroywZpI0ejYPGP","meetingType":"public","audio":true,"video":true,"audioOnly":false,"clientVersion":"11.202.0","gaiaServerVersion":"11.202.0","clientOutdated":false,"versionsOutdated":0,"gameEngineVersion":"86.4.2","groupId":"v0XvoIt3uK40pdF0mQDB","groupName":"20DASH","planType":"basic","plan":"basic","role":"contributor","spaceType":"office","sessionId":"RM_unGbwGcgK3ai","duration":86}}]}
RES: {"success":true}

### 283.448 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=383&CI=1&TYPE=xmlhttp&zx=cbc5u41bym8a&t=1 -> 200
RES: 2228
[[384,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      130
    ]
  }
}
]],[385,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/conversations/bkD5sDVZwkC8fbozpabM",
      "fields": {
        "privacy": {
          "stringValue": "public"
        },
        "roomDefinitionId": {
          "stringValue": "yPsvwRdkb5LnGHfXFgfO"
        },
        "createdAt": {
          "timestampValue": "2026-10-06T17:35:22.286Z"
        },
        "zoneId": {
          "stringValue": "XGdh6pmi1U6jxpgfZ2dF"
        },
        "roomId": {
          "stringValue": "YmAfBb3h2eY5r9FsAIfV"
        },
        "meetingChatId": {
          "nullValue": null
        },
        "meetingChatProvider": {
          "stringValue": "sowork"
        },
        "ownerId": {
          "nullValue": null
        },
        "sc": {
          "arrayValue": {
            "values": [
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:owner"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:contributor"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:guest"
              }
            ]
          }
        },
        "hasOnlineUsers": {
          "booleanValue": false
        },
        "status": {
          "stringValue": "active"
        },
        "livekitRoomId": {
          "stringValue": "RM_TwewBEvxgMhP"
        },
        "participants": {
          "mapValue": {}
        },
        "ownerGroupId": {
          "stringValue": "v0XvoIt3uK40pdF0mQDB"
        },
        "sessionId": {
          "stringValue": "ec5f992e-087c-4f87-af39-e57eb6f70222"
        }
      },
      "createTime": "2026-10-06T17:35:22.358378Z",
      "updateTime": "2026-10-06T17:35:22.358378Z"
    },
    "targetIds": [
      130
    ]
  }
}
]],[386,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      130
    ],
    "resumeToken": "CgkIyr7b4vallwM=",
    "readTime": "2026-10-06T17:35:22.742602Z"
  }
}
]],[387,[{
  "targetChange": {
    "resumeToken": "CgkIyr7b4vallwM=",
    "readTime": "2026-10-06T17:35:22.742602Z"
  }
}
]]]

### 283.482 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65338&AID=383&zx=9jevfrr9v2sx&t=1 -> 200
REQ: count=1&ofs=83&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FEsFlL0qd1kZNo0IsotREJWQWNeC3%22%5D%7D%2C%22targetId%22%3A132%7D%7D
RES: 10
[0,387,7]


### 283.598 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=387&CI=1&TYPE=xmlhttp&zx=787xr6mbljt2&t=1 -> 200
RES: 3755
[[388,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      132
    ]
  }
}
]],[389,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/users/EsFlL0qd1kZNo0IsotREJWQWNeC3",
      "fields": {
        "timezone": {
          "stringValue": "America/Sao_Paulo"
        },
        "birthday": {
          "mapValue": {
            "fields": {
              "date": {
                "timestampValue": "2026-04-25T03:00:00Z"
              },
              "enabled": {
                "booleanValue": true
              }
            }
          }
        },
        "character": {
          "mapValue": {
            "fields": {
              "version": {
                "integerValue": "5"
              },
              "slots": {
                "mapValue": {
                  "fields": {
                    "shoes": {
                      "stringValue": "characterA_shoes_waterBoots+black"
                    },
                    "mouth": {
                      "stringValue": "characterA_mouth_mouth1+typeC"
                    },
                    "pants": {
                      "stringValue": "characterA_pants_nathanPants+white"
                    },
                    "eyes": {
                      "stringValue": "characterA_eyes_eyes1+green"
                    },
                    "beard": {
                      "stringValue": "characterA_beard_sugar+black"
                    },
                    "headAccessory": {
                      "stringValue": "characterA_headAccessory_pixelSunglasses+black"
                    },
                    "body": {
                      "stringValue": "characterA_body_v1+typeC"
                    },
                    "eyebrows": {
                      "stringValue": "characterA_eyebrows_eyebrows1+brown"
                    },
                    "hair": {
                      "stringValue": "characterA_hair_sarah+black"
                    },
                    "shirt": {
                      "stringValue": "characterA_shirt_punkMaleJacket+red"
                    }
                  }
                }
              },
              "bodyType": {
                "stringValue": "characterA"
              }
            }
          }
        },
        "name": {
          "stringValue": "Lucas Pacheco"
        },
        "role": {
          "stringValue": "severino"
        },
        "meetingHours": {
          "mapValue": {
            "fields": {
              "current": {
                "mapValue": {
                  "fields": {
                    "joinAt": {
                      "timestampValue": "2026-09-25T21:00:42Z"
                    },
                    "conversationId": {
                      "stringValue": "coRBRS7NPl4Bys2IgPKM"
                    }
                  }
                }
              },
              "duration": {
                "integerValue": "0"
              },
              "durationExpiredAt": {
                "timestampValue": "2026-09-26T03:00:00.815Z"
              }
            }
          }
        },
        "email": {
          "stringValue": "lucas@20dash.com"
        },
        "xp": {
          "integerValue": "275"
        }
      },
      "createTime": "2026-08-10T13:53:57.207496Z",
      "updateTime": "2026-09-25T21:00:44.991470Z"
    },
    "targetIds": [
      132
    ]
  }
}
]],[390,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      132
    ],
    "resumeToken": "CgkIm6L+4vallwM=",
    "readTime": "2026-10-06T17:35:23.312411Z"
  }
}
]],[391,[{
  "targetChange": {
    "resumeToken": "CgkIm6L+4vallwM=",
    "readTime": "2026-10-06T17:35:23.312411Z"
  }
}
]]]

### 283.733 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=391&CI=1&TYPE=xmlhttp&zx=u0sjqikuei61&t=1 -> 200
RES: 6095
[[392,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/conversations/bkD5sDVZwkC8fbozpabM",
      "fields": {
        "zoneId": {
          "stringValue": "XGdh6pmi1U6jxpgfZ2dF"
        },
        "users": {
          "mapValue": {
            "fields": {
              "Rxp3M2MJzra9wQJm0VkDfRSgJgt1": {
                "mapValue": {
                  "fields": {
                    "lastJoinAt": {
                      "timestampValue": "2026-10-06T17:35:23Z"
                    },
                    "status": {
                      "stringValue": "online"
                    },
                    "name": {
                      "stringValue": "Kury"
                    },
                    "joinAt": {
                      "timestampValue": "2026-10-06T17:35:23Z"
                    }
                  }
                }
              }
            }
          }
        },
        "meetingChatId": {
          "nullValue": null
        },
        "meetingChatProvider": {
          "stringValue": "sowork"
        },
        "sc": {
          "arrayValue": {
            "values": [
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:owner"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:contributor"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:guest"
              }
            ]
          }
        },
        "sessionId": {
          "stringValue": "ec5f992e-087c-4f87-af39-e57eb6f70222"
        },
        "onlineCount": {
          "integerValue": "1"
        },
        "roomId": {
          "stringValue": "YmAfBb3h2eY5r9FsAIfV"
        },
        "roomDefinitionId": {
          "stringValue": "yPsvwRdkb5LnGHfXFgfO"
        },
        "hasOnlineUsers": {
          "booleanValue": true
        },
        "privacy": {
          "stringValue": "public"
        },
        "createdAt": {
          "timestampValue": "2026-10-06T17:35:22.286Z"
        },
        "ownerGroupId": {
          "stringValue": "v0XvoIt3uK40pdF0mQDB"
        },
        "participants": {
          "mapValue": {}
        },
        "livekitRoomId": {
          "stringValue": "RM_TwewBEvxgMhP"
        },
        "ownerId": {
          "nullValue": null
        },
        "status": {
          "stringValue": "active"
        }
      },
      "createTime": "2026-10-06T17:35:22.358378Z",
      "updateTime": "2026-10-06T17:35:23.379114Z"
    },
    "targetIds": [
      104,
      100
    ]
  }
}
]],[393,[{
  "targetChange": {
    "targetIds": [
      132,
      2,
      48,
      16,
      102,
      82,
      4,
      118,
      54,
      90,
      112,
      32,
      104,
      84,
      120,
      12,
      128,
      22,
      46,
      42,
      100,
      92,
      110,
      8,
      24,
      94,
      14,
      30,
      44,
      88,
      106,
      122,
      114,
      20,
      36,
      52,
      28,
      116,
      86,
      40,
      126,
      34,
      50,
      18,
      10,
      26,
      96,
      38,
      124
    ],
    "resumeToken": "CgkIqquC4/allwM=",
    "readTime": "2026-10-06T17:35:23.379114Z"
  }
}
]],[394,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/conversations/bkD5sDVZwkC8fbozpabM",
      "fields": {
        "ownerId": {
          "nullValue": null
        },
        "onlineCount": {
          "integerValue": "1"
        },
        "ownerGroupId": {
          "stringValue": "v0XvoIt3uK40pdF0mQDB"
        },
        "sessionId": {
          "stringValue": "ec5f992e-087c-4f87-af39-e57eb6f70222"
        },
        "sc": {
          "arrayValue": {
            "values": [
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:owner"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:contributor"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:guest"
              }
            ]
          }
        },
        "livekitRoomId": {
          "stringValue": "RM_TwewBEvxgMhP"
        },
        "users": {
          "mapValue": {
            "fields": {
              "Rxp3M2MJzra9wQJm0VkDfRSgJgt1": {
                "mapValue": {
                  "fields": {
                    "lastJoinAt": {
                      "timestampValue": "2026-10-06T17:35:23Z"
                    },
                    "name": {
                      "stringValue": "Kury"
                    },
                    "status": {
                      "stringValue": "online"
                    },
                    "joinAt": {
                      "timestampValue": "2026-10-06T17:35:23Z"
                    }
                  }
                }
              }
            }
          }
        },
        "participants": {
          "mapValue": {}
        },
        "status": {
          "stringValue": "active"
        },
        "meetingChatProvider": {
          "stringValue": "sowork"
        },
        "hasOnlineUsers": {
          "booleanValue": true
        },
        "createdAt": {
          "timestampValue": "2026-10-06T17:35:22.286Z"
        },
        "zoneId": {
          "stringValue": "XGdh6pmi1U6jxpgfZ2dF"
        },
        "privacy": {
          "stringValue": "public"
        },
        "roomId": {
          "stringValue": "YmAfBb3h2eY5r9FsAIfV"
        },
        "roomDefinitionId": {
          "stringValue": "yPsvwRdkb5LnGHfXFgfO"
        },
        "meetingChatId": {
          "nullValue": null
        }
      },
      "createTime": "2026-10-06T17:35:22.358378Z",
      "updateTime": "2026-10-06T17:35:23.379114Z"
    },
    "targetIds": [
      130
    ]
  }
}
]],[395,[{
  "targetChange": {
    "targetIds": [
      130
    ],
    "resumeToken": "CgkI492D4/allwM=",
    "readTime": "2026-10-06T17:35:23.401955Z"
  }
}
]],[396,[{
  "targetChang

### 283.881 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=396&CI=1&TYPE=xmlhttp&zx=7bqfl8wnhqaf&t=1 -> 200
RES: 3892
[[397,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/users/Rxp3M2MJzra9wQJm0VkDfRSgJgt1",
      "fields": {
        "meetingHours": {
          "mapValue": {
            "fields": {
              "current": {
                "mapValue": {
                  "fields": {
                    "joinAt": {
                      "timestampValue": "2026-10-06T17:35:23Z"
                    },
                    "conversationId": {
                      "stringValue": "bkD5sDVZwkC8fbozpabM"
                    }
                  }
                }
              },
              "durationExpiredAt": {
                "timestampValue": "2026-10-07T03:00:00.626Z"
              },
              "duration": {
                "integerValue": "0"
              }
            }
          }
        },
        "pets": {
          "mapValue": {
            "fields": {
              "pKWdV0wiu601x4MhA1AO": {
                "mapValue": {
                  "fields": {
                    "shouldSpawn": {
                      "booleanValue": false
                    }
                  }
                }
              }
            }
          }
        },
        "name": {
          "stringValue": "Kury"
        },
        "xp": {
          "integerValue": "310"
        },
        "email": {
          "stringValue": "gabriel@20dash.com"
        },
        "birthday": {
          "mapValue": {
            "fields": {
              "date": {
                "timestampValue": "2026-10-17T03:00:00Z"
              },
              "enabled": {
                "booleanValue": true
              }
            }
          }
        },
        "timezone": {
          "stringValue": "America/Sao_Paulo"
        },
        "character": {
          "mapValue": {
            "fields": {
              "bodyType": {
                "stringValue": "characterA"
              },
              "version": {
                "integerValue": "5"
              },
              "slots": {
                "mapValue": {
                  "fields": {
                    "mouth": {
                      "stringValue": "characterA_mouth_mouth1+typeA"
                    },
                    "shoes": {
                      "stringValue": "characterA_shoes_stepLuck+green"
                    },
                    "bodyAccessory": {
                      "stringValue": "characterA_bodyAccessory_nateDrakeHolster+blonde"
                    },
                    "pants": {
                      "stringValue": "characterA_pants_bermudaLong+black"
                    },
                    "eyes": {
                      "stringValue": "characterA_eyes_eyes1+grey"
                    },
                    "backAccessory": {
                      "stringValue": "characterA_backAccessory_fishingBackPack+brown"
                    },
                    "beard": {
                      "stringValue": "characterA_beard_turk+black"
                    },
                    "body": {
                      "stringValue": "characterA_body_v1+typeA"
                    },
                    "hair": {
                      "stringValue": "characterA_hair_barret+black"
                    },
                    "eyebrows": {
                      "stringValue": "characterA_eyebrows_eyebrows1+brown"
                    },
                    "shirt": {
                      "stringValue": "characterA_shirt_alanShirt+blonde"
                    }
                  }
                }
              }
            }
          }
        }
      },
      "createTime": "2026-08-10T12:38:17.792013Z",
      "updateTime": "2026-10-06T17:35:23.499129Z"
    },
    "targetIds": [
      116
    ]
  }
}
]],[398,[{
  "targetChange": {
    "resumeToken": "CgkI+dSJ4/allwM=",
    "readTime": "2026-10-06T17:35:23.499129Z"
  }
}
]]]

### 284.019 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=398&CI=1&TYPE=xmlhttp&zx=eee3ys30ozm0&t=1 -> 200
RES: 

### 284.335 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=400&CI=1&TYPE=xmlhttp&zx=kx40xmqp3e8l&t=1 -> 200
RES: 

### 284.472 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=406&CI=1&TYPE=xmlhttp&zx=44fibp9vsgq0&t=1 -> 200
RES: 

### 286.919 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65339&AID=407&zx=alkkjvxszvrf&t=1 -> 200
REQ: count=1&ofs=84&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A132%7D
RES: 10
[1,407,7]


### 287.058 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=408&CI=1&TYPE=xmlhttp&zx=d09x7tmp7wzz&t=1 -> 200
RES: 

### 288.772 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 291.859 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=410&CI=1&TYPE=xmlhttp&zx=ajubtz86e88a&t=1 -> 200
RES: 123
[[411,[{
  "targetChange": {
    "resumeToken": "CgkIrNTv5vallwM=",
    "readTime": "2026-10-06T17:35:31.461676Z"
  }
}
]]]

### 291.995 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=411&CI=1&TYPE=xmlhttp&zx=eoezd5f5c1sw&t=1 -> 200
RES: 

### 293.835 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 296.291 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=413&CI=1&TYPE=xmlhttp&zx=o9500piqjrrc&t=1 -> 200
RES: 280
[[414,[{
  "targetChange": {
    "targetIds": [
      90
    ],
    "resumeToken": "CgkItu/+6PallwM=",
    "readTime": "2026-10-06T17:35:35.905206Z"
  }
}
]],[415,[{
  "targetChange": {
    "resumeToken": "CgkItu/+6PallwM=",
    "readTime": "2026-10-06T17:35:35.905206Z"
  }
}
]]]

### 296.427 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=415&CI=1&TYPE=xmlhttp&zx=ukhd3aoylzh1&t=1 -> 200
RES: 

### 298.988 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 299.266 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=417&CI=1&TYPE=xmlhttp&zx=ejmjf7w8n07f&t=1 -> 200
RES: 315
[[418,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      82
    ],
    "resumeToken": "CgkIhp616vallwM=",
    "readTime": "2026-10-06T17:35:38.893062Z"
  }
}
]],[419,[{
  "targetChange": {
    "resumeToken": "CgkIhp616vallwM=",
    "readTime": "2026-10-06T17:35:38.893062Z"
  }
}
]]]

### 299.405 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=419&CI=1&TYPE=xmlhttp&zx=6ot0albakpk4&t=1 -> 200
RES: 

### 304.034 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 309.095 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 314.151 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 316.980 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65340&AID=420&zx=adr9n0q31r6a&t=1 -> 200
REQ: count=1&ofs=85&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A110%7D
RES: 10
[1,420,7]


### 317.044 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=LZpPn8fHU8l56MnxY-WmYOZainSh9YrP82xawbG90Kty1lG5D2NBFQ&VER=8&database=projects%2Fsophya-calendar-production%2Fdatabases%2F(default)&RID=rpc&SID=AYiWFFk6YV4N591bkO7F1Q&AID=12&CI=0&TYPE=xmlhttp&zx=ubsv1mkmglpn&t=1 -> 200
RES: 

### 317.119 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65341&AID=420&zx=7m9ub4nheo9f&t=1 -> 200
REQ: count=9&ofs=86&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A112%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A114%7D&req2___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A116%7D&req3___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A118%7D&req4___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A120%7D&req5___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A122%7D&req6___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A124%7D&req7___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A126%7D&req8___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A128%7D
RES: 10
[0,421,7]


### 317.144 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=421&CI=1&TYPE=xmlhttp&zx=njg5fw7nps8w&t=1 -> 200
RES: 955
[[422,[{
  "targetChange": {
    "targetChangeType": "REMOVE",
    "targetIds": [
      112
    ]
  }
}
]],[423,[{
  "targetChange": {
    "targetChangeType": "REMOVE",
    "targetIds": [
      114
    ]
  }
}
]],[424,[{
  "targetChange": {
    "targetChangeType": "REMOVE",
    "targetIds": [
      116
    ]
  }
}
]],[425,[{
  "targetChange": {
    "targetChangeType": "REMOVE",
    "targetIds": [
      118
    ]
  }
}
]],[426,[{
  "targetChange": {
    "targetChangeType": "REMOVE",
    "targetIds": [
      120
    ]
  }
}
]],[427,[{
  "targetChange": {
    "targetChangeType": "REMOVE",
    "targetIds": [
      122
    ]
  }
}
]],[428,[{
  "targetChange": {
    "targetChangeType": "REMOVE",
    "targetIds": [
      124
    ]
  }
}
]],[429,[{
  "targetChange": {
    "targetChangeType": "REMOVE",
    "targetIds": [
      126
    ]
  }
}
]],[430,[{
  "targetChange": {
    "targetChangeType": "REMOVE",
    "targetIds": [
      128
    ]
  }
}
]]]

### 317.318 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=430&CI=1&TYPE=xmlhttp&zx=xy7e912jyd9v&t=1 -> 200
RES: 

### 318.941 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65342&AID=431&zx=a48knaiv64ns&t=1 -> 200
REQ: count=1&ofs=95&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A100%7D
RES: 10
[1,431,7]


### 319.078 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=432&CI=1&TYPE=xmlhttp&zx=tkwnc9zc0jfh&t=1 -> 200
RES: 213
[[433,[{
  "targetChange": {
    "targetChangeType": "REMOVE",
    "targetIds": [
      104
    ]
  }
}
]],[434,[{
  "targetChange": {
    "targetChangeType": "REMOVE",
    "targetIds": [
      102
    ]
  }
}
]]]

### 319.079 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65343&AID=432&zx=yteuij9r29cw&t=1 -> 200
REQ: count=2&ofs=96&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A104%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A102%7D
RES: 10
[0,432,7]


### 319.815 POST https://api.sowork.com/api/v1/gaia/rooms -> 200
REQ: {"groupId":"v0XvoIt3uK40pdF0mQDB"}
RES: {"roomDefinitionMap":{"yPsvwRdkb5LnGHfXFgfO":{"roomDefinition":{"id":"yPsvwRdkb5LnGHfXFgfO","groupId":"v0XvoIt3uK40pdF0mQDB","name":"Escritório principal","alias":"mistymountainlodge","mapDefinitionId":"V1vAAHj80gty4MwBqEFp","ownerKey":"group_v0XvoIt3uK40pdF0mQDB","primary":true,"isMeetingSpace":false,"zones":null,"createdAt":{"seconds":1786123761,"nanoseconds":850000000},"updatedAt":{"seconds":1786123761,"nanoseconds":850000000},"roomObjects":{},"roomGroupId":null,"pricingType":"premium"},"rooms":[{"id":"YmAfBb3h2eY5r9FsAIfV","name":"Escritório principal","index":1,"local":false,"serverId":"g5","server":{"url":"https://api.sowork.com/gaia/g5"},"userIds":["JLDRGMskJRXiYcHccNVaLuwr3XJ3","rjOwBlMG93QD2sjutWoF2fc8Mv63","fCfzuTuBkBSLyF1tKdMJOZSOnUu1","G7EWVnLMQngko5v7KcgNnH06Fhg1","Rxp3M2MJzra9wQJm0VkDfRSgJgt1","blKsp8Fxi6VGJfXfizN0iu248IB2","IEwNgnRoYUcFOwc4KSrz7MyHNvG2","zE7SCh3eZDggMBNhjB5xkpeglNn2","FLBKjFg1PYe5bD6cE0bdR7BhE7r1","zdXi8bJLG6cNbd4P87bqTAzTdTq1","RMUYRzBXvUOCqtJdxSAiMdiEZM82"],"userCount":11,"roomDefinitionId":"yPsvwRdkb5LnGHfXFgfO","roomDefinition":{"ownerKey":"group_v0XvoIt3uK40pdF0mQDB","name":"Misty Mountain Lodge","groupId":"v0XvoIt3uK40pdF0mQDB","zones":null,"projection":null},"isMeetingSpace":false,"createdAt":{"seconds":1786123764,"nanoseconds":340000000},"updatedAt":{"seconds":1786123764,"nanoseconds":340000000},"roomGroupId":null}]}}}

### 319.831 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 319.923 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65344&AID=432&zx=gal11r857j8w&t=1 -> 200
REQ: count=3&ofs=98&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22conversations%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22ownerGroupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomDefinitionId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22yPsvwRdkb5LnGHfXFgfO%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22active%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22op%22%3A%22GREATER_THAN%22%2C%22value%22%3A%7B%22integerValue%22%3A%220%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%2C%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A134%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22conversations%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22ownerGroupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22unaryFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomDefinitionId%22%7D%2C%22op%22%3A%22IS_NULL%22%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22active%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22op%22%3A%22GREATER_THAN%22%2C%22value%22%3A%7B%22integerValue%22%3A%220%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%2C%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A136%7D%7D&req2___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22conversations%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22ownerGroupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomDefinitionId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22yPsvwRdkb5LnGHfXFgfO%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22roomId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22YmAfBb3h2eY5r9FsAIfV%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22active%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22op%22%3A%22GREATER_THAN%22%2C%22value%22%3A%7B%22integerValue%22%3A%220%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22onlineCount%22%7D%2C%22
RES: 10
[0,434,7]


### 319.924 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=434&CI=1&TYPE=xmlhttp&zx=cds8rt5d23xl&t=1 -> 200
RES: 310
[[435,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      134
    ]
  }
}
]],[436,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      136
    ]
  }
}
]],[437,[{
  "targetChange": {
    "targetChangeType": "ADD",
    "targetIds": [
      138
    ]
  }
}
]]]

### 320.069 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65345&AID=434&zx=ytpi6jym1o5k&t=1 -> 200
REQ: count=11&ofs=101&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FEsFlL0qd1kZNo0IsotREJWQWNeC3%22%5D%7D%2C%22targetId%22%3A140%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FIEwNgnRoYUcFOwc4KSrz7MyHNvG2%22%5D%7D%2C%22targetId%22%3A142%7D%7D&req2___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FzdXi8bJLG6cNbd4P87bqTAzTdTq1%22%5D%7D%2C%22targetId%22%3A144%7D%7D&req3___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FFLBKjFg1PYe5bD6cE0bdR7BhE7r1%22%5D%7D%2C%22targetId%22%3A146%7D%7D&req4___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FfCfzuTuBkBSLyF1tKdMJOZSOnUu1%22%5D%7D%2C%22targetId%22%3A148%7D%7D&req5___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FRxp3M2MJzra9wQJm0VkDfRSgJgt1%22%5D%7D%2C%22targetId%22%3A150%7D%7D&req6___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FzE7SCh3eZDggMBNhjB5xkpeglNn2%22%5D%7D%2C%22targetId%22%3A152%7D%7D&req7___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FblKsp8Fxi6VGJfXfizN0iu248IB2%22%5D%7D%2C%22targetId%22%3A154%7D%7D&req8___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FG7EWVnLMQngko5v7KcgNnH06Fhg1%22%5D%7D%2C%22targetId%22%3A156%7D%7D&req9___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FJLDRGMskJRXiYcHccNVaLuwr3XJ3%22%5D%7D%2C%22targetId%22%3A158%7D%7D&req10___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FrjOwBlMG93QD2sjutWoF2fc8Mv63%22%5D%7D%2C%22targetId%22%3A160%7D%7D
RES: 10
[0,437,7]


### 320.073 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=437&CI=1&TYPE=xmlhttp&zx=bz8m6oukf2p7&t=1 -> 200
RES: 14589
[[438,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      136
    ],
    "resumeToken": "CgkIp+aq9PallwM=",
    "readTime": "2026-10-06T17:35:59.693607Z"
  }
}
]],[439,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/conversations/bkD5sDVZwkC8fbozpabM",
      "fields": {
        "privacy": {
          "stringValue": "public"
        },
        "createdAt": {
          "timestampValue": "2026-10-06T17:35:22.286Z"
        },
        "roomDefinitionId": {
          "stringValue": "yPsvwRdkb5LnGHfXFgfO"
        },
        "participants": {
          "mapValue": {}
        },
        "status": {
          "stringValue": "active"
        },
        "meetingChatId": {
          "nullValue": null
        },
        "onlineCount": {
          "integerValue": "2"
        },
        "ownerGroupId": {
          "stringValue": "v0XvoIt3uK40pdF0mQDB"
        },
        "livekitRoomId": {
          "stringValue": "RM_TwewBEvxgMhP"
        },
        "ownerId": {
          "nullValue": null
        },
        "users": {
          "mapValue": {
            "fields": {
              "RMUYRzBXvUOCqtJdxSAiMdiEZM82": {
                "mapValue": {
                  "fields": {
                    "lastJoinAt": {
                      "timestampValue": "2026-10-06T17:35:23Z"
                    },
                    "status": {
                      "stringValue": "online"
                    },
                    "joinAt": {
                      "timestampValue": "2026-10-06T17:35:23Z"
                    },
                    "name": {
                      "stringValue": "Tito Guidotti"
                    }
                  }
                }
              },
              "Rxp3M2MJzra9wQJm0VkDfRSgJgt1": {
                "mapValue": {
                  "fields": {
                    "joinAt": {
                      "timestampValue": "2026-10-06T17:35:23Z"
                    },
                    "lastJoinAt": {
                      "timestampValue": "2026-10-06T17:35:23Z"
                    },
                    "name": {
                      "stringValue": "Kury"
                    },
                    "status": {
                      "stringValue": "online"
                    }
                  }
                }
              }
            }
          }
        },
        "sessionId": {
          "stringValue": "ec5f992e-087c-4f87-af39-e57eb6f70222"
        },
        "sc": {
          "arrayValue": {
            "values": [
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:owner"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:contributor"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:guest"
              }
            ]
          }
        },
        "meetingChatProvider": {
          "stringValue": "sowork"
        },
        "zoneId": {
          "stringValue": "XGdh6pmi1U6jxpgfZ2dF"
        },
        "hasOnlineUsers": {
          "booleanValue": true
        },
        "roomId": {
          "stringValue": "YmAfBb3h2eY5r9FsAIfV"
        }
      },
      "createTime": "2026-10-06T17:35:22.358378Z",
      "updateTime": "2026-10-06T17:35:23.885789Z"
    },
    "targetIds": [
      134
    ]
  }
}
]],[440,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/conversations/xQiKXuEhyWgIu7i64QL1",
      "fields": {
        "meetingChatId": {
          "nullValue": null
        },
        "privacy": {
          "stringValue": "public"
        },
        "sc": {
          "arrayValue": {
            "values": [
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:owner"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:contributor"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:guest"
              }
            ]
          }
        },
        "createdAt": {
          "timestampValue": "2026-10-06T17:31:50.071Z"
        },
        "status": {
          "stringValue": "active"
        },
        "zoneId": {
          "stringValue": "OgspgGHWe4AuPbgLCreJ"
        },
        "users": {
          "mapValue": {
            "fields": {
              "zdXi8bJLG6cNbd4P87bqTAzTdTq1": {
                "mapValue": {
                  "fields": {
                    "status": {
                      "stringValue": "online"
                    },
                    "name": {
                      "stringValue": "Stela Pagan"
                    },
                    "joinAt": {
                      "timestampValue": "2026-10-06T17:31:51Z"
                    },
                    "lastJoinAt": {
                      "timestampValue": "2026-10-06T17:31:51Z"
                    }
                  }
                }
              },
              "fCfzuTuBkBSLyF1tKdMJOZSOnUu1": {
                "mapValue": {
                  "fields": {
                    "name": {
                      "stringValue": "Marie Cezar"
                    },
                    "joinAt": {
                      "timestampValue": "2026-10-06T17:31:51Z"
                    },
                    "lastJoinAt": {
                      "timestampValue": "2026-10-06T17:31:51Z"
                    },
                    "status": {
                      "stringValue": "online"
                    }
                  }
                }
              }
            }
          }
        },
        "ownerId": {
          "nullValue": null
        },
        "roomDefinitionId": {
          "stringValue": "yPsvwRdkb5LnGHfXFgfO"
        },
        "onlineCount": {
          "integerValue": "2"
        },
        "ownerGroupId": {
          "stringValue": "v0XvoIt3uK40pdF0mQDB"
        },
        "meetingChatProvider": {
          "stringV

### 320.213 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=456&CI=1&TYPE=xmlhttp&zx=fvfp5pthm22u&t=1 -> 200
RES: 115240
[[457,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/users/zE7SCh3eZDggMBNhjB5xkpeglNn2",
      "fields": {
        "groupRoles": {
          "mapValue": {
            "fields": {
              "ICvliDiy67rWNuJEzDR6": {
                "stringValue": "contributor"
              }
            }
          }
        },
        "timezone": {
          "stringValue": "America/Sao_Paulo"
        },
        "meetingHours": {
          "mapValue": {
            "fields": {
              "current": {
                "mapValue": {
                  "fields": {
                    "joinAt": {
                      "timestampValue": "2026-10-06T16:15:32Z"
                    },
                    "conversationId": {
                      "stringValue": "T5iLAdYzqEYPEGta7n4N"
                    }
                  }
                }
              },
              "duration": {
                "integerValue": "0"
              },
              "durationExpiredAt": {
                "timestampValue": "2026-10-07T03:00:00.869Z"
              }
            }
          }
        },
        "updatedAt": {
          "timestampValue": "2025-10-17T16:14:21.023Z"
        },
        "birthday": {
          "mapValue": {
            "fields": {
              "enabled": {
                "booleanValue": true
              },
              "date": {
                "timestampValue": "2026-03-17T03:00:00Z"
              }
            }
          }
        },
        "xp": {
          "integerValue": "260"
        },
        "character": {
          "mapValue": {
            "fields": {
              "bodyType": {
                "stringValue": "characterA"
              },
              "version": {
                "integerValue": "5"
              },
              "slots": {
                "mapValue": {
                  "fields": {
                    "headAccessory": {
                      "stringValue": "characterA_headAccessory_bat+black"
                    },
                    "extraAccessory": {
                      "stringValue": "characterA_extraAccessory_cap+black"
                    },
                    "eyes": {
                      "stringValue": "characterA_eyes_eyes1+brown"
                    },
                    "shirt": {
                      "stringValue": "characterA_shirt_standardShirt+green"
                    },
                    "mouth": {
                      "stringValue": "characterA_mouth_mouth1+typeE"
                    },
                    "shoes": {
                      "stringValue": "characterA_shoes_sandals+white"
                    },
                    "body": {
                      "stringValue": "characterA_body_v1+typeE"
                    },
                    "pants": {
                      "stringValue": "characterA_pants_midSwatPants+black"
                    },
                    "eyebrows": {
                      "stringValue": "characterA_eyebrows_eyebrows1+brown"
                    },
                    "hair": {
                      "stringValue": "characterA_hair_sidney+black"
                    }
                  }
                }
              }
            }
          }
        },
        "email": {
          "stringValue": "kevin@20dash.com"
        },
        "affiliation": {
          "stringValue": "20DASH"
        },
        "name": {
          "stringValue": "Kevin"
        }
      },
      "createTime": "2025-10-17T16:14:22.010485Z",
      "updateTime": "2026-10-06T16:15:33.669388Z"
    },
    "targetIds": [
      152
    ]
  }
}
]],[458,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      152
    ],
    "resumeToken": "CgkI6daz9PallwM=",
    "readTime": "2026-10-06T17:35:59.839081Z"
  }
}
]],[459,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/users/EsFlL0qd1kZNo0IsotREJWQWNeC3",
      "fields": {
        "meetingHours": {
          "mapValue": {
            "fields": {
              "durationExpiredAt": {
                "timestampValue": "2026-09-26T03:00:00.815Z"
              },
              "duration": {
                "integerValue": "0"
              },
              "current": {
                "mapValue": {
                  "fields": {
                    "joinAt": {
                      "timestampValue": "2026-09-25T21:00:42Z"
                    },
                    "conversationId": {
                      "stringValue": "coRBRS7NPl4Bys2IgPKM"
                    }
                  }
                }
              }
            }
          }
        },
        "character": {
          "mapValue": {
            "fields": {
              "slots": {
                "mapValue": {
                  "fields": {
                    "pants": {
                      "stringValue": "characterA_pants_nathanPants+white"
                    },
                    "eyes": {
                      "stringValue": "characterA_eyes_eyes1+green"
                    },
                    "shoes": {
                      "stringValue": "characterA_shoes_waterBoots+black"
                    },
                    "mouth": {
                      "stringValue": "characterA_mouth_mouth1+typeC"
                    },
                    "beard": {
                      "stringValue": "characterA_beard_sugar+black"
                    },
                    "headAccessory": {
                      "stringValue": "characterA_headAccessory_pixelSunglasses+black"
                    },
                    "eyebrows": {
                      "stringValue": "characterA_eyebrows_eyebrows1+brown"
                    },
                    "hair": {
                      "stringValue": "characterA_hair_sarah+black"
                    },
                    "shirt": {
                      "stringValue": "characterA_shirt_punkMaleJacket+red"
                 

### 320.851 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=479&CI=1&TYPE=xmlhttp&zx=4uca0kwnntbj&t=1 -> 200
RES: 

### 323.253 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65346&AID=480&zx=he3si3pixrg1&t=1 -> 200
REQ: count=1&ofs=112&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A140%7D
RES: 10
[1,480,7]


### 323.961 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=481&CI=1&TYPE=xmlhttp&zx=87mehgeuohu3&t=1 -> 200
RES: 1017
[[482,[{
  "targetChange": {
    "targetIds": [
      26,
      150,
      90,
      134,
      54,
      4,
      84,
      48,
      148,
      2,
      16,
      82,
      32,
      138,
      42,
      92,
      8,
      94,
      130,
      24,
      46,
      142,
      44,
      158,
      12,
      22,
      86,
      50,
      14,
      34,
      52,
      136,
      88,
      30,
      152,
      106,
      28,
      18,
      160,
      96,
      144,
      38,
      20,
      40,
      146,
      154,
      10,
      156
    ],
    "resumeToken": "CgkIjNqz9PallwM=",
    "readTime": "2026-10-06T17:35:59.839500Z"
  }
}
]],[483,[{
  "filter": {
    "targetId": 36,
    "unchangedNames": {
      "bits": {}
    }
  }
}
]],[484,[{
  "targetChange": {
    "targetIds": [
      36
    ],
    "resumeToken": "CgkIuKH29fallwM=",
    "readTime": "2026-10-06T17:36:03.027128Z"
  }
}
]],[485,[{
  "targetChange": {
    "resumeToken": "CgkIuKH29fallwM=",
    "readTime": "2026-10-06T17:36:03.027128Z"
  }
}
]]]

### 324.110 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=485&CI=1&TYPE=xmlhttp&zx=6ai3alfbd7j4&t=1 -> 200
RES: 

### 325.009 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 329.733 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=487&CI=1&TYPE=xmlhttp&zx=kdgqa5rwz9l0&t=1 -> 200
RES: 315
[[488,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      84
    ],
    "resumeToken": "CgkIgY7V+PallwM=",
    "readTime": "2026-10-06T17:36:08.775425Z"
  }
}
]],[489,[{
  "targetChange": {
    "resumeToken": "CgkIgY7V+PallwM=",
    "readTime": "2026-10-06T17:36:08.775425Z"
  }
}
]]]

### 329.894 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=489&CI=1&TYPE=xmlhttp&zx=6euf3d5qwyzy&t=1 -> 200
RES: 

### 330.051 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 330.283 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=491&CI=1&TYPE=xmlhttp&zx=9hfq5prvp2y9&t=1 -> 200
RES: 123
[[492,[{
  "targetChange": {
    "resumeToken": "CgkIw+qZ+fallwM=",
    "readTime": "2026-10-06T17:36:09.901379Z"
  }
}
]]]

### 330.463 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=492&CI=1&TYPE=xmlhttp&zx=kh2iwjec6fea&t=1 -> 200
RES: 

### 335.105 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 336.984 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65347&AID=493&zx=eqnqcyym5lo&t=1 -> 200
REQ: count=1&ofs=113&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A130%7D
RES: 10
[1,493,7]


### 337.116 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=494&CI=1&TYPE=xmlhttp&zx=3fpysy93uxlu&t=1 -> 200
RES: 

### 337.600 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=496&CI=1&TYPE=xmlhttp&zx=r2ny57hahgfq&t=1 -> 200
RES: 496
[[497,[{
  "targetChange": {
    "resumeToken": "CgkI+rvS/PallwM=",
    "readTime": "2026-10-06T17:36:17.120762Z"
  }
}
]],[498,[{
  "documentRemove": {
    "document": "projects/sage-learning-prod/databases/(default)/documents/conversations/bkD5sDVZwkC8fbozpabM",
    "removedTargetIds": [
      138,
      134
    ],
    "readTime": "2026-10-06T17:36:17.262690Z"
  }
}
]],[499,[{
  "targetChange": {
    "resumeToken": "CgkI4pDb/PallwM=",
    "readTime": "2026-10-06T17:36:17.262690Z"
  }
}
]]]

### 337.741 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=499&CI=1&TYPE=xmlhttp&zx=5n93uyw45o6w&t=1 -> 200
RES: 

### 337.741 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65348&AID=499&zx=4zr5nhupsrav&t=1 -> 200
REQ: count=1&ofs=114&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fconversations%2FbkD5sDVZwkC8fbozpabM%22%5D%7D%2C%22targetId%22%3A3%7D%2C%22labels%22%3A%7B%22goog-listen-tags%22%3A%22limbo-document%22%7D%7D
RES: 10
[1,500,7]


### 337.884 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=501&CI=1&TYPE=xmlhttp&zx=g3qdyoe5ke8l&t=1 -> 200
RES: 3891
[[502,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/conversations/bkD5sDVZwkC8fbozpabM",
      "fields": {
        "privacy": {
          "stringValue": "public"
        },
        "ownerGroupId": {
          "stringValue": "v0XvoIt3uK40pdF0mQDB"
        },
        "roomId": {
          "stringValue": "YmAfBb3h2eY5r9FsAIfV"
        },
        "meetingChatId": {
          "nullValue": null
        },
        "roomDefinitionId": {
          "stringValue": "yPsvwRdkb5LnGHfXFgfO"
        },
        "createdAt": {
          "timestampValue": "2026-10-06T17:35:22.286Z"
        },
        "users": {
          "mapValue": {
            "fields": {
              "RMUYRzBXvUOCqtJdxSAiMdiEZM82": {
                "mapValue": {
                  "fields": {
                    "joinAt": {
                      "timestampValue": "2026-10-06T17:35:23Z"
                    },
                    "lastJoinAt": {
                      "nullValue": null
                    },
                    "leftAt": {
                      "timestampValue": "2026-10-06T17:36:16Z"
                    },
                    "name": {
                      "stringValue": "Tito Guidotti"
                    },
                    "status": {
                      "stringValue": "offline"
                    },
                    "duration": {
                      "integerValue": "53"
                    }
                  }
                }
              },
              "Rxp3M2MJzra9wQJm0VkDfRSgJgt1": {
                "mapValue": {
                  "fields": {
                    "name": {
                      "stringValue": "Kury"
                    },
                    "leftAt": {
                      "timestampValue": "2026-10-06T17:36:16Z"
                    },
                    "joinAt": {
                      "timestampValue": "2026-10-06T17:35:23Z"
                    },
                    "duration": {
                      "integerValue": "53"
                    },
                    "status": {
                      "stringValue": "offline"
                    },
                    "lastJoinAt": {
                      "nullValue": null
                    }
                  }
                }
              }
            }
          }
        },
        "zoneId": {
          "stringValue": "XGdh6pmi1U6jxpgfZ2dF"
        },
        "participants": {
          "mapValue": {}
        },
        "hasOnlineUsers": {
          "booleanValue": false
        },
        "livekitRoomId": {
          "stringValue": "RM_TwewBEvxgMhP"
        },
        "onlineCount": {
          "integerValue": "0"
        },
        "status": {
          "stringValue": "active"
        },
        "ownerId": {
          "nullValue": null
        },
        "meetingChatProvider": {
          "stringValue": "sowork"
        },
        "sessionId": {
          "stringValue": "ec5f992e-087c-4f87-af39-e57eb6f70222"
        },
        "sc": {
          "arrayValue": {
            "values": [
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:owner"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:contributor"
              },
              {
                "stringValue": "group:v0XvoIt3uK40pdF0mQDB:guest"
              }
            ]
          }
        }
      },
      "createTime": "2026-10-06T17:35:22.358378Z",
      "updateTime": "2026-10-06T17:36:17.262690Z"
    },
    "targetIds": [
      3
    ]
  }
}
]],[503,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      3
    ],
    "resumeToken": "CgkImp3q/PallwM=",
    "readTime": "2026-10-06T17:36:17.510042Z"
  }
}
]],[504,[{
  "targetChange": {
    "resumeToken": "CgkImp3q/PallwM=",
    "readTime": "2026-10-06T17:36:17.510042Z"
  }
}
]]]

### 338.021 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=504&CI=1&TYPE=xmlhttp&zx=zhiolz64no0u&t=1 -> 200
RES: 

### 338.021 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65349&AID=504&zx=dqo76yq74zwn&t=1 -> 200
REQ: count=1&ofs=115&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A3%7D
RES: 10
[1,505,7]


### 338.163 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=506&CI=1&TYPE=xmlhttp&zx=nty5xdqsonqj&t=1 -> 200
RES: 

### 338.208 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65350&AID=506&zx=bih6pfqnmu8z&t=1 -> 200
REQ: count=1&ofs=116&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FEsFlL0qd1kZNo0IsotREJWQWNeC3%22%5D%7D%2C%22targetId%22%3A162%7D%7D
RES: 10
[1,507,7]


### 338.359 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=508&CI=1&TYPE=xmlhttp&zx=e359d6terf28&t=1 -> 200
RES: 3652
[[509,[{
  "documentChange": {
    "document": {
      "name": "projects/sage-learning-prod/databases/(default)/documents/users/EsFlL0qd1kZNo0IsotREJWQWNeC3",
      "fields": {
        "character": {
          "mapValue": {
            "fields": {
              "slots": {
                "mapValue": {
                  "fields": {
                    "hair": {
                      "stringValue": "characterA_hair_sarah+black"
                    },
                    "beard": {
                      "stringValue": "characterA_beard_sugar+black"
                    },
                    "pants": {
                      "stringValue": "characterA_pants_nathanPants+white"
                    },
                    "body": {
                      "stringValue": "characterA_body_v1+typeC"
                    },
                    "shoes": {
                      "stringValue": "characterA_shoes_waterBoots+black"
                    },
                    "mouth": {
                      "stringValue": "characterA_mouth_mouth1+typeC"
                    },
                    "shirt": {
                      "stringValue": "characterA_shirt_punkMaleJacket+red"
                    },
                    "eyes": {
                      "stringValue": "characterA_eyes_eyes1+green"
                    },
                    "eyebrows": {
                      "stringValue": "characterA_eyebrows_eyebrows1+brown"
                    },
                    "headAccessory": {
                      "stringValue": "characterA_headAccessory_pixelSunglasses+black"
                    }
                  }
                }
              },
              "version": {
                "integerValue": "5"
              },
              "bodyType": {
                "stringValue": "characterA"
              }
            }
          }
        },
        "email": {
          "stringValue": "lucas@20dash.com"
        },
        "meetingHours": {
          "mapValue": {
            "fields": {
              "durationExpiredAt": {
                "timestampValue": "2026-09-26T03:00:00.815Z"
              },
              "duration": {
                "integerValue": "0"
              },
              "current": {
                "mapValue": {
                  "fields": {
                    "conversationId": {
                      "stringValue": "coRBRS7NPl4Bys2IgPKM"
                    },
                    "joinAt": {
                      "timestampValue": "2026-09-25T21:00:42Z"
                    }
                  }
                }
              }
            }
          }
        },
        "xp": {
          "integerValue": "275"
        },
        "name": {
          "stringValue": "Lucas Pacheco"
        },
        "role": {
          "stringValue": "severino"
        },
        "birthday": {
          "mapValue": {
            "fields": {
              "enabled": {
                "booleanValue": true
              },
              "date": {
                "timestampValue": "2026-04-25T03:00:00Z"
              }
            }
          }
        },
        "timezone": {
          "stringValue": "America/Sao_Paulo"
        }
      },
      "createTime": "2026-08-10T13:53:57.207496Z",
      "updateTime": "2026-09-25T21:00:44.991470Z"
    },
    "targetIds": [
      162
    ]
  }
}
]],[510,[{
  "targetChange": {
    "targetChangeType": "CURRENT",
    "targetIds": [
      162
    ],
    "resumeToken": "CgkIm8qH/fallwM=",
    "readTime": "2026-10-06T17:36:17.990939Z"
  }
}
]],[511,[{
  "targetChange": {
    "resumeToken": "CgkI1NaH/fallwM=",
    "readTime": "2026-10-06T17:36:17.992532Z"
  }
}
]]]

### 338.496 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=511&CI=1&TYPE=xmlhttp&zx=rd8tp7ga5xfu&t=1 -> 200
RES: 

### 340.285 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 340.556 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=513&CI=1&TYPE=xmlhttp&zx=vum2l8r6zkaq&t=1 -> 200
RES: 123
[[514,[{
  "targetChange": {
    "resumeToken": "CgkI5fKJ/vallwM=",
    "readTime": "2026-10-06T17:36:20.126053Z"
  }
}
]]]

### 340.777 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=iboNN4rB6cuI2nnXp8nuig&AID=514&CI=1&TYPE=xmlhttp&zx=c5mq62rlucw0&t=1 -> 200
RES: 

### 341.925 POST https://api.sowork.com/api/v1/crm/track -> 200
REQ: {"userId":"RMUYRzBXvUOCqtJdxSAiMdiEZM82","groupId":"v0XvoIt3uK40pdF0mQDB","events":[{"eventName":"meeting","timestamp":1791308176.609,"properties":{"appSessionId":"1qklZjXtqu-CXhlggjD3G","inMeeting":true,"meetingId":"bkD5sDVZwkC8fbozpabM","meetingType":"public","audio":true,"video":true,"audioOnly":false,"clientVersion":"11.202.0","gaiaServerVersion":"11.202.0","clientOutdated":false,"versionsOutdated":0,"gameEngineVersion":"86.4.2","groupId":"v0XvoIt3uK40pdF0mQDB","groupName":"20DASH","planType":"basic","plan":"basic","role":"contributor","spaceType":"office","sessionId":"RM_TwewBEvxgMhP","duration":54}}]}
RES: {"success":true}

### 343.577 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=ctEEkl0Hn1ioVTnBa9D52QsQOj22jGYLAucylFshhhwDhvl1oDoqiQ&SID=iboNN4rB6cuI2nnXp8nuig&RID=65351&TYPE=terminate&zx=arqcyrnwlgda -> 200
RES: 

### 343.577 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsophya-calendar-production%2Fdatabases%2F(default)&gsessionid=LZpPn8fHU8l56MnxY-WmYOZainSh9YrP82xawbG90Kty1lG5D2NBFQ&SID=AYiWFFk6YV4N591bkO7F1Q&RID=11827&TYPE=terminate&zx=jpe7isy2u545 -> 200
RES: 

### 343.583 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=42331&CVER=22&X-HTTP-Session-Id=gsessionid&zx=u7evvnm7xpfd&t=1 -> 200
REQ: headers=X-Goog-Api-Client%3Agl-js%2F%20fire%2F9.23.0%0D%0AContent-Type%3Atext%2Fplain%0D%0AX-Firebase-GMPID%3A1%3A69834214729%3Aweb%3Ac3e2769cb392ca9203de98%0D%0AAuthorization%3ABearer%20<JWT>%0D%0A&count=49&ofs=0&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FRMUYRzBXvUOCqtJdxSAiMdiEZM82%22%5D%7D%2C%22targetId%22%3A2%2C%22resumeToken%22%3A%22CgkI5fKJ%2FvallwM%3D%22%2C%22expectedCount%22%3A1%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers_sys%2FRMUYRzBXvUOCqtJdxSAiMdiEZM82%22%5D%7D%2C%22targetId%22%3A4%2C%22resumeToken%22%3A%22CgkI5fKJ%2FvallwM%3D%22%2C%22expectedCount%22%3A1%7D%7D&req2___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22groupJoinRequests%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22groupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22requestorUserId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A8%2C%22resumeToken%22%3A%22CgkI5fKJ%2FvallwM%3D%22%2C%22expectedCount%22%3A0%7D%7D&req3___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22groupJoinRequests%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22pending%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22requestorUserId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A10%2C%22resumeToken%22%3A%22CgkI5fKJ%2FvallwM%3D%22%2C%22expectedCount%22%3A0%7D%7D&req4___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22groupJoinRequests%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22seenReqAnswer%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22booleanValue%22%3Afalse%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22requestorUserId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A12%2C%22resumeToken%22%3A%22CgkI5fKJ%2FvallwM%3D%22%2C%22expectedCount%22%3A0%7D%7
RES: 51
[[0,["c","777aEO1kczCR8-y8J8IqHA","",8,15,30000]]]


### 343.584 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsophya-calendar-production%2Fdatabases%2F(default)&RID=80789&CVER=22&X-HTTP-Session-Id=gsessionid&zx=fz7ay8pwp9kc&t=1 -> 200
REQ: headers=X-Goog-Api-Client%3Agl-js%2F%20fire%2F9.23.0%0D%0AContent-Type%3Atext%2Fplain%0D%0AX-Firebase-GMPID%3A1%3A537328756772%3Aweb%3Ad487ca7b2e66572b9510b1%0D%0AAuthorization%3ABearer%20<JWT>%0D%0A&count=2&ofs=0&req0___data__=%7B%22database%22%3A%22projects%2Fsophya-calendar-production%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22events%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22userId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22start%22%7D%2C%22op%22%3A%22GREATER_THAN_OR_EQUAL%22%2C%22value%22%3A%7B%22timestampValue%22%3A%222026-09-29T17%3A33%3A41.449000000Z%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22start%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%2C%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsophya-calendar-production%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A2%2C%22resumeToken%22%3A%22CgkI6qDx8vallwM%3D%22%2C%22expectedCount%22%3A0%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsophya-calendar-production%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22accounts%22%7D%5D%2C%22where%22%3A%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22userId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsophya-calendar-production%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A4%2C%22resumeToken%22%3A%22CgkI6qDx8vallwM%3D%22%2C%22expectedCount%22%3A0%7D%7D
RES: 51
[[0,["c","31f7RaVoHz3oC596s46WZw","",8,15,30000]]]


### 343.694 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=72850&CVER=22&X-HTTP-Session-Id=gsessionid&zx=beyo5gqqn37a&t=1 -> 200
REQ: headers=X-Goog-Api-Client%3Agl-js%2F%20fire%2F9.23.0%0D%0AContent-Type%3Atext%2Fplain%0D%0AX-Firebase-GMPID%3A1%3A69834214729%3Aweb%3Ac3e2769cb392ca9203de98%0D%0AAuthorization%3ABearer%20<JWT>%0D%0A&count=49&ofs=0&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FRMUYRzBXvUOCqtJdxSAiMdiEZM82%22%5D%7D%2C%22targetId%22%3A2%2C%22resumeToken%22%3A%22CgkI5fKJ%2FvallwM%3D%22%2C%22expectedCount%22%3A1%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers_sys%2FRMUYRzBXvUOCqtJdxSAiMdiEZM82%22%5D%7D%2C%22targetId%22%3A4%2C%22resumeToken%22%3A%22CgkI5fKJ%2FvallwM%3D%22%2C%22expectedCount%22%3A1%7D%7D&req2___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22groupJoinRequests%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22groupId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22v0XvoIt3uK40pdF0mQDB%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22requestorUserId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A8%2C%22resumeToken%22%3A%22CgkI5fKJ%2FvallwM%3D%22%2C%22expectedCount%22%3A0%7D%7D&req3___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22groupJoinRequests%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22status%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22pending%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22requestorUserId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A10%2C%22resumeToken%22%3A%22CgkI5fKJ%2FvallwM%3D%22%2C%22expectedCount%22%3A0%7D%7D&req4___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22groupJoinRequests%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22seenReqAnswer%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22booleanValue%22%3Afalse%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22requestorUserId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A12%2C%22resumeToken%22%3A%22CgkI5fKJ%2FvallwM%3D%22%2C%22expectedCount%22%3A0%7D%7
RES: 51
[[0,["c","D2lTcApBOMLJc86jBzrGCg","",8,15,30000]]]


### 343.694 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsophya-calendar-production%2Fdatabases%2F(default)&RID=83486&CVER=22&X-HTTP-Session-Id=gsessionid&zx=blfzfbk1ydlo&t=1 -> 200
REQ: headers=X-Goog-Api-Client%3Agl-js%2F%20fire%2F9.23.0%0D%0AContent-Type%3Atext%2Fplain%0D%0AX-Firebase-GMPID%3A1%3A537328756772%3Aweb%3Ad487ca7b2e66572b9510b1%0D%0AAuthorization%3ABearer%20<JWT>%0D%0A&count=2&ofs=0&req0___data__=%7B%22database%22%3A%22projects%2Fsophya-calendar-production%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22events%22%7D%5D%2C%22where%22%3A%7B%22compositeFilter%22%3A%7B%22op%22%3A%22AND%22%2C%22filters%22%3A%5B%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22userId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%2C%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22start%22%7D%2C%22op%22%3A%22GREATER_THAN_OR_EQUAL%22%2C%22value%22%3A%7B%22timestampValue%22%3A%222026-09-29T17%3A33%3A41.449000000Z%22%7D%7D%7D%5D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22start%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%2C%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsophya-calendar-production%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A2%2C%22resumeToken%22%3A%22CgkI6qDx8vallwM%3D%22%2C%22expectedCount%22%3A0%7D%7D&req1___data__=%7B%22database%22%3A%22projects%2Fsophya-calendar-production%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22query%22%3A%7B%22structuredQuery%22%3A%7B%22from%22%3A%5B%7B%22collectionId%22%3A%22accounts%22%7D%5D%2C%22where%22%3A%7B%22fieldFilter%22%3A%7B%22field%22%3A%7B%22fieldPath%22%3A%22userId%22%7D%2C%22op%22%3A%22EQUAL%22%2C%22value%22%3A%7B%22stringValue%22%3A%22RMUYRzBXvUOCqtJdxSAiMdiEZM82%22%7D%7D%7D%2C%22orderBy%22%3A%5B%7B%22field%22%3A%7B%22fieldPath%22%3A%22__name__%22%7D%2C%22direction%22%3A%22ASCENDING%22%7D%5D%7D%2C%22parent%22%3A%22projects%2Fsophya-calendar-production%2Fdatabases%2F(default)%2Fdocuments%22%7D%2C%22targetId%22%3A4%2C%22resumeToken%22%3A%22CgkI6qDx8vallwM%3D%22%2C%22expectedCount%22%3A0%7D%7D
RES: 51
[[0,["c","f9JWmFz9BibHZDV_iL53vA","",8,15,30000]]]


### 345.053 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=3bq91PrZPWnNXJ8R5EjRbl5B3r5dU1v5mIR6HV6mGVQMimN1FPww6w&VER=8&database=projects%2Fsophya-calendar-production%2Fdatabases%2F(default)&RID=rpc&SID=f9JWmFz9BibHZDV_iL53vA&AID=0&CI=0&TYPE=xmlhttp&zx=gqf1xlye76e8&t=1 -> 200
RES: 

### 345.080 GET https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?gsessionid=NhVt2M31yPyzCt8VhDvZhFNClZJ_j1TipSVJHfT_-8c3SguB8C47Dw&VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&RID=rpc&SID=D2lTcApBOMLJc86jBzrGCg&AID=0&CI=0&TYPE=xmlhttp&zx=wc5gq2ibu6vd&t=1 -> 200
RES: 

### 345.359 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 348.925 HEAD https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud/ -> 200
RES: 

### 348.928 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 348.941 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=NhVt2M31yPyzCt8VhDvZhFNClZJ_j1TipSVJHfT_-8c3SguB8C47Dw&SID=D2lTcApBOMLJc86jBzrGCg&RID=72851&AID=100&zx=8q7r0zi8o14a&t=1 -> 200
REQ: count=1&ofs=49&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fconversations%2FIoLD3YELUcNSlOXm2UiQ%22%5D%7D%2C%22targetId%22%3A164%7D%7D
RES: 10
[1,100,7]


### 354.242 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 355.078 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=NhVt2M31yPyzCt8VhDvZhFNClZJ_j1TipSVJHfT_-8c3SguB8C47Dw&SID=D2lTcApBOMLJc86jBzrGCg&RID=72852&AID=120&zx=grlummlozscr&t=1 -> 200
REQ: count=1&ofs=50&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A162%7D
RES: 10
[1,120,7]


### 359.088 POST https://api.sowork.com/api/v1/user-presence/scheduleClear -> 200
REQ: {"type":"cancel"}
RES: {"success":true}

### 359.089 POST https://api.sowork.com/api/v1/user/updateUiSettings -> 200
REQ: {"userId":"RMUYRzBXvUOCqtJdxSAiMdiEZM82","data":{"presenceStatus":{"clearAfter":"never"}}}
RES: {"success":true}

### 359.331 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 359.608 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=NhVt2M31yPyzCt8VhDvZhFNClZJ_j1TipSVJHfT_-8c3SguB8C47Dw&SID=D2lTcApBOMLJc86jBzrGCg&RID=72853&AID=121&zx=z6bo34ng7xrf&t=1 -> 200
REQ: count=1&ofs=51&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A164%7D
RES: 10
[1,126,7]


### 360.475 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=NhVt2M31yPyzCt8VhDvZhFNClZJ_j1TipSVJHfT_-8c3SguB8C47Dw&SID=D2lTcApBOMLJc86jBzrGCg&RID=72854&AID=136&zx=pn3z78d3raen&t=1 -> 200
REQ: count=1&ofs=52&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fconversations%2FIoLD3YELUcNSlOXm2UiQ%22%5D%7D%2C%22targetId%22%3A5%7D%2C%22labels%22%3A%7B%22goog-listen-tags%22%3A%22limbo-document%22%7D%7D
RES: 10
[1,136,7]


### 360.714 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=NhVt2M31yPyzCt8VhDvZhFNClZJ_j1TipSVJHfT_-8c3SguB8C47Dw&SID=D2lTcApBOMLJc86jBzrGCg&RID=72855&AID=140&zx=4z3mimvf4mn1&t=1 -> 200
REQ: count=1&ofs=53&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A5%7D
RES: 10
[1,140,7]


### 360.888 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=NhVt2M31yPyzCt8VhDvZhFNClZJ_j1TipSVJHfT_-8c3SguB8C47Dw&SID=D2lTcApBOMLJc86jBzrGCg&RID=72856&AID=140&zx=hn3on5nw1k62&t=1 -> 200
REQ: count=1&ofs=54&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FEsFlL0qd1kZNo0IsotREJWQWNeC3%22%5D%7D%2C%22targetId%22%3A166%7D%7D
RES: 10
[1,141,7]


### 361.103 POST https://api.sowork.com/api/v1/user-presence/update -> 200
REQ: {"status":"busy"}
RES: {"success":true}

### 363.569 HEAD https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud/ -> 200
RES: 

### 363.571 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 363.584 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=NhVt2M31yPyzCt8VhDvZhFNClZJ_j1TipSVJHfT_-8c3SguB8C47Dw&SID=D2lTcApBOMLJc86jBzrGCg&RID=72857&AID=160&zx=wi0666730t5j&t=1 -> 200
REQ: count=1&ofs=55&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fconversations%2FN9Jb2ykxgg484vuK3D1u%22%5D%7D%2C%22targetId%22%3A168%7D%7D
RES: 10
[1,160,7]


### 364.204 POST https://api.sowork.com/api/v1/user-presence/update -> 200
REQ: {"status":"online"}
RES: {"success":true}

### 367.954 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=NhVt2M31yPyzCt8VhDvZhFNClZJ_j1TipSVJHfT_-8c3SguB8C47Dw&SID=D2lTcApBOMLJc86jBzrGCg&RID=72858&AID=192&zx=l156ot7ur60x&t=1 -> 200
REQ: count=1&ofs=56&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A166%7D
RES: 10
[1,192,7]


### 369.494 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}

### 373.321 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=NhVt2M31yPyzCt8VhDvZhFNClZJ_j1TipSVJHfT_-8c3SguB8C47Dw&SID=D2lTcApBOMLJc86jBzrGCg&RID=72859&AID=207&zx=ho27nvxugylm&t=1 -> 200
REQ: count=1&ofs=57&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A168%7D
RES: 10
[1,207,7]


### 373.955 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=NhVt2M31yPyzCt8VhDvZhFNClZJ_j1TipSVJHfT_-8c3SguB8C47Dw&SID=D2lTcApBOMLJc86jBzrGCg&RID=72860&AID=214&zx=g4r1a39e9vht&t=1 -> 200
REQ: count=1&ofs=58&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fconversations%2FN9Jb2ykxgg484vuK3D1u%22%5D%7D%2C%22targetId%22%3A7%7D%2C%22labels%22%3A%7B%22goog-listen-tags%22%3A%22limbo-document%22%7D%7D
RES: 10
[1,214,7]


### 374.144 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=NhVt2M31yPyzCt8VhDvZhFNClZJ_j1TipSVJHfT_-8c3SguB8C47Dw&SID=D2lTcApBOMLJc86jBzrGCg&RID=72861&AID=218&zx=iu37sw8ua9kl&t=1 -> 200
REQ: count=1&ofs=59&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22removeTarget%22%3A7%7D
RES: 10
[1,218,7]


### 374.323 POST https://firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel?VER=8&database=projects%2Fsage-learning-prod%2Fdatabases%2F(default)&gsessionid=NhVt2M31yPyzCt8VhDvZhFNClZJ_j1TipSVJHfT_-8c3SguB8C47Dw&SID=D2lTcApBOMLJc86jBzrGCg&RID=72862&AID=219&zx=wuq1dmx1ia55&t=1 -> 200
REQ: count=1&ofs=60&req0___data__=%7B%22database%22%3A%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%22%2C%22addTarget%22%3A%7B%22documents%22%3A%7B%22documents%22%3A%5B%22projects%2Fsage-learning-prod%2Fdatabases%2F(default)%2Fdocuments%2Fusers%2FEsFlL0qd1kZNo0IsotREJWQWNeC3%22%5D%7D%2C%22targetId%22%3A170%7D%7D
RES: 10
[1,219,7]


### 374.629 GET https://sowork-production-7a7l3zs5.livekit.cloud/settings/regions -> 200
RES: {"regions":[{"region":"ofrankfurt1b","url":"https://sowork-production-7a7l3zs5.ofrankfurt1b.production.livekit.cloud","distance":"429694"},{"region":"omarseille1b","url":"https://sowork-production-7a7l3zs5.omarseille1b.production.livekit.cloud","distance":"1186458"},{"region":"ojerusalem1a","url":"https://sowork-production-7a7l3zs5.ojerusalem1a.production.livekit.cloud","distance":"2955545"},{"region":"ojeddah1a","url":"https://sowork-production-7a7l3zs5.ojeddah1a.production.livekit.cloud","distance":"4098992"},{"region":"odubai1a","url":"https://sowork-production-7a7l3zs5.odubai1a.production.livekit.cloud","distance":"4631691"},{"region":"omontreal1a","url":"https://sowork-production-7a7l3zs5.omontreal1a.production.livekit.cloud","distance":"6005667"}]}
