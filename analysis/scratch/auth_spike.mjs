#!/usr/bin/env node
// Auth spike: can a non-browser client turn the recorded Firebase refresh token into an ID token
// and call SoWork's API? Prints only statuses and key names, never token values.
// Usage: node analysis/scratch/auth_spike.mjs recorder/raw-logs/<run>
import { readFileSync } from 'node:fs';
import { join } from 'node:path';

const L = readFileSync(join(process.argv[2], 'events.jsonl'), 'utf8').trim().split('\n').map((l) => JSON.parse(l));
const ORIGIN = 'https://app.sowork.com';

const idp = L.find((e) => e.k === 'res.body' && /accounts:signInWithIdp/.test(e.url) && /refreshToken/.test(e.text ?? ''));
const apiKey = new URL(idp.url).searchParams.get('key');
const refreshToken = JSON.parse(idp.text).refreshToken;
const findReq = L.find((e) => e.k === 'req' && e.method === 'POST' && /gaia\/find/.test(e.url));
const appHeaders = { source: findReq.headers.source, version: findReq.headers.version, 'content-type': 'application/json' };
console.log('recorded app headers:', { source: appHeaders.source, version: appHeaders.version });

async function refresh(label, headers) {
  const r = await fetch(`https://securetoken.googleapis.com/v1/token?key=${apiKey}`, {
    method: 'POST',
    headers: { 'content-type': 'application/x-www-form-urlencoded', ...headers },
    body: new URLSearchParams({ grant_type: 'refresh_token', refresh_token: refreshToken }),
  });
  const j = await r.json().catch(() => ({}));
  console.log(`[refresh ${label}]`, r.status, j.error ? j.error.message : `ok, expires_in=${j.expires_in}, keys=${Object.keys(j)}`);
  return j.id_token;
}

// 1. refresh without any browser headers (what a plain Rust client would send)
let idToken = await refresh('no origin', {});
// 2. with the headers the web app sends, in case the key is referrer-restricted
if (!idToken) idToken = await refresh('with origin+referer', { origin: ORIGIN, referer: ORIGIN + '/' });
if (!idToken) process.exit(1);

// 3. call SoWork API with the fresh token
async function find(label, headers) {
  const r = await fetch('https://api.sowork.com/api/v1/gaia/find', {
    method: 'POST',
    headers: { ...appHeaders, authorization: `Bearer ${idToken}`, ...headers },
    body: findReq.postData,
  });
  const t = await r.text();
  let j; try { j = JSON.parse(t); } catch {}
  console.log(`[gaia/find ${label}]`, r.status, j ? `keys=${Object.keys(j)}` : t.slice(0, 120));
  if (j?.room) console.log('  room keys:', Object.keys(j.room), 'name:', j.room.name);
  return r.ok;
}
if (!(await find('no origin', {}))) await find('with origin', { origin: ORIGIN, referer: ORIGIN + '/' });
