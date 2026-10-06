#!/usr/bin/env node
// SoWorker bridge: talks to SoWork (Firebase auth, REST, gaia socket) and serves a tiny canvas client.
// Usage: node app/server.mjs [--dry-run] [--no-open] [--port 4821] [--chrome <bin>]
//   --dry-run   connect and watch, but never send position or state (your avatar does not move)
import { createServer } from 'node:http';
import { readFileSync } from 'node:fs';
import { spawn } from 'node:child_process';
import { join, dirname, extname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { Auth } from './lib/auth.mjs';
import { Gaia, CLIENT_VERSION } from './lib/gaia.mjs';
import { World } from './lib/world.mjs';

const here = dirname(fileURLToPath(import.meta.url));
const args = process.argv.slice(2);
const opt = (n, d) => { const i = args.indexOf(n); return i >= 0 ? args[i + 1] : d; };
const DRY = args.includes('--dry-run');
const PORT = Number(opt('--port', 4821));

const auth = new Auth(join(here, '.session.json'));
await auth.idToken();
const found = await (await fetch('https://api.sowork.com/api/v1/gaia/find', {
  method: 'POST',
  headers: { source: 'web', version: CLIENT_VERSION, 'content-type': 'application/json', authorization: `Bearer ${await auth.idToken()}` },
  body: JSON.stringify(auth.session.find),
})).json();
if (!found.room) throw new Error('gaia/find failed: ' + JSON.stringify(found).slice(0, 200));

const world = new World();
const users = new Map(); // id -> {id,name,x,y,state}
let roomName = found.room.name;
let worldTimer = null;
const sse = new Set();

const publish = (obj) => { const line = `data: ${JSON.stringify(obj)}\n\n`; for (const r of sse) r.write(line); };
const fullState = () => ({ t: 'world', me: auth.uid, room: roomName, dry: DRY, ...world.snapshot(), users: [...users.values()] });
const scheduleWorld = () => { clearTimeout(worldTimer); worldTimer = setTimeout(() => publish(fullState()), 300); };
const userOf = (u) => ({ id: u.id, name: u.name ?? '?', x: u.x ?? 0, y: u.y ?? 0, state: u.characterState?.state ?? 'idle' });

const gaia = new Gaia({ auth, roomId: found.room.id });
gaia.on('error', (e) => console.error('gaia:', e.message));
gaia.on('connected', () => console.log('gaia connected'));
gaia.on('disconnected', () => { console.log('gaia disconnected'); publish({ t: 'status', text: 'reconnecting' }); });
gaia.on('room', (r) => {
  roomName = r.name;
  users.clear();
  for (const u of Object.values(r.users ?? {})) users.set(u.id, userOf(u));
  if (!users.has(auth.uid)) { const s = r.spawns?.[0] ?? { x: 0, y: 0 }; users.set(auth.uid, { id: auth.uid, name: auth.name, x: s.x, y: s.y, state: 'idle' }); }
  const me = users.get(auth.uid);
  gaia.pos = { x: me.x, y: me.y };
  if (!DRY) gaia.setClusterMode('none-away'); // we have no audio/video: show as away, not as available
  scheduleWorld();
});
gaia.on('object', (o) => { world.apply(o); scheduleWorld(); });
gaia.on('s', (batch) => {
  for (const [code, p] of batch) {
    if (code === 1 && p.userId !== auth.uid) { const u = users.get(p.userId); if (u) { u.x = p.x; u.y = p.y; publish({ t: 'move', id: u.id, x: p.x, y: p.y }); } }
    else if (code === 2 && p.userId !== auth.uid) { const u = users.get(p.userId); if (u) { u.state = p.characterState?.state ?? 'idle'; publish({ t: 'state', id: u.id, state: u.state }); } }
    else if (code === 3 && p.user && p.user.id !== auth.uid) { const u = userOf(p.user); users.set(u.id, u); publish({ t: 'join', user: u }); }
  }
});
for (const ev of ['user_disconnect', 'leave']) {
  gaia.on(ev, (p) => { if (p?.userId && p.userId !== auth.uid && users.delete(p.userId)) publish({ t: 'leave', id: p.userId }); });
}
await gaia.connect();

// ---- HTTP: static client, SSE stream, actions
const MIME = { '.html': 'text/html', '.js': 'text/javascript', '.css': 'text/css' };
createServer(async (req, res) => {
  if (req.url === '/events') {
    res.writeHead(200, { 'content-type': 'text/event-stream', 'cache-control': 'no-cache', connection: 'keep-alive' });
    sse.add(res);
    req.on('close', () => sse.delete(res));
    res.write(`data: ${JSON.stringify(fullState())}\n\n`);
    return;
  }
  if (req.url === '/act' && req.method === 'POST') {
    let body = '';
    for await (const c of req) body += c;
    try {
      const a = JSON.parse(body);
      if (!DRY) {
        if (a.ev === 'move' && Number.isFinite(a.x) && Number.isFinite(a.y)) gaia.move(a.x, a.y);
        else if (a.ev === 'state' && ['idle', 'walk'].includes(a.state)) gaia.characterState(a.state);
      }
    } catch {}
    res.writeHead(204).end();
    return;
  }
  let file = req.url.split('?')[0];
  if (file === '/') file = '/index.html';
  if (file.includes('..')) return res.writeHead(400).end();
  let body;
  try { body = readFileSync(join(here, 'public', file)); } catch { return res.writeHead(404).end(); }
  res.writeHead(200, { 'content-type': MIME[extname(file)] ?? 'application/octet-stream' });
  res.end(body);
}).listen(PORT, '127.0.0.1', () => {
  const url = `http://127.0.0.1:${PORT}/`;
  console.log(`SoWorker at ${url}${DRY ? ' (dry run: nothing is sent)' : ''}`);
  if (!args.includes('--no-open')) {
    spawn(opt('--chrome', 'google-chrome-stable'), [`--app=${url}`, `--user-data-dir=${join(here, '.chrome')}`, '--window-size=1000,700', '--no-first-run', '--disable-extensions'], { stdio: 'ignore', detached: true }).unref();
  }
});
process.on('SIGINT', () => { gaia.close(); process.exit(0); });
