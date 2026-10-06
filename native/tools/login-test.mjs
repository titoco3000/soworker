#!/usr/bin/env node
// A fake "app.sowork.com" for testing the login capture offline: serves a page that stores a Firebase-style login in
// IndexedDB after a short delay, like the real app does after Google sign-in. Usage: node native/tools/login-test.mjs [port]
import http from 'node:http';
const page = `<!doctype html><title>fake sowork</title><body>signing in...<script>
setTimeout(() => {
  const r = indexedDB.open('firebaseLocalStorageDb', 1);
  r.onupgradeneeded = () => r.result.createObjectStore('firebaseLocalStorage', { keyPath: 'fbase_key' });
  r.onsuccess = () => {
    const tx = r.result.transaction('firebaseLocalStorage', 'readwrite'), s = tx.objectStore('firebaseLocalStorage');
    s.put({ fbase_key: 'firebase:authUser:FAKEKEY2:[calendar]', value: { apiKey: 'FAKEKEY2', stsTokenManager: { refreshToken: 'calendar-token' } } });
    s.put({ fbase_key: 'firebase:authUser:FAKEKEY1:[DEFAULT]', value: { apiKey: 'FAKEKEY1', stsTokenManager: { refreshToken: 'fake-refresh-token-1234567890' } } });
  };
}, 1500);
</script>`;
http.createServer((req, res) => { res.setHeader('content-type', 'text/html'); res.end(page); }).listen(Number(process.argv[2] ?? 4900), '127.0.0.1');
