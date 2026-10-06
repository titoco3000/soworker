// Firebase session: keeps a refresh token on disk and hands out fresh ID tokens.
import { readFileSync, writeFileSync, existsSync, chmodSync } from 'node:fs';

export class Auth {
  #idToken = null;
  #exp = 0;
  constructor(file) {
    this.file = file;
    if (!existsSync(file)) throw new Error(`no session file ${file}; run: node app/import-token.mjs recorder/logs/<run>`);
    this.session = JSON.parse(readFileSync(file, 'utf8'));
  }

  async idToken() {
    if (this.#idToken && Date.now() < this.#exp - 5 * 60_000) return this.#idToken;
    const r = await fetch(`https://securetoken.googleapis.com/v1/token?key=${this.session.apiKey}`, {
      method: 'POST',
      headers: { 'content-type': 'application/x-www-form-urlencoded' },
      body: new URLSearchParams({ grant_type: 'refresh_token', refresh_token: this.session.refreshToken }),
    });
    const j = await r.json();
    if (!r.ok) throw new Error(`token refresh failed: ${j.error?.message ?? r.status}`);
    this.#idToken = j.id_token;
    this.#exp = Date.now() + Number(j.expires_in) * 1000;
    if (j.refresh_token && j.refresh_token !== this.session.refreshToken) {
      this.session.refreshToken = j.refresh_token;
      writeFileSync(this.file, JSON.stringify(this.session, null, 2), { mode: 0o600 });
    }
    const claims = JSON.parse(Buffer.from(this.#idToken.split('.')[1], 'base64url').toString());
    this.uid = claims.user_id;
    this.name = claims.name;
    return this.#idToken;
  }
}

export function saveSession(file, session) {
  writeFileSync(file, JSON.stringify(session, null, 2), { mode: 0o600 });
  chmodSync(file, 0o600);
}
