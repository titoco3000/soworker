#!/usr/bin/env node
// One-time import of the Firebase refresh token and invite link from a recorder run.
// Usage: node app/import-token.mjs recorder/logs/<run>
import { readFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { saveSession } from './lib/auth.mjs';

const L = readFileSync(join(process.argv[2], 'events.jsonl'), 'utf8').trim().split('\n').map((l) => JSON.parse(l));
const idp = L.find((e) => e.k === 'res.body' && /accounts:signInWithIdp/.test(e.url) && /refreshToken/.test(e.text ?? ''));
const find = L.find((e) => e.k === 'req' && e.method === 'POST' && /gaia\/find/.test(e.url));
if (!idp || !find) throw new Error('recording has no sign-in or gaia/find request');
const file = join(dirname(fileURLToPath(import.meta.url)), '.session.json');
saveSession(file, {
  apiKey: new URL(idp.url).searchParams.get('key'),
  refreshToken: JSON.parse(idp.text).refreshToken,
  find: JSON.parse(find.postData),
});
console.log('saved', file, '(mode 600)');
