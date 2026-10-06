#!/usr/bin/env node
// Offline test bridge: serves public/ and a world built from a recorder run (no SoWork contact, no credentials).
// Usage: node app/test/fake-bridge.mjs recorder/logs/<run> [port]
import { createServer } from 'node:http';
import { readFileSync } from 'node:fs';
import { join, dirname, extname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { decode } from '../lib/msgpack.mjs';
import { decodeSetMapObject } from '../lib/mapobject.mjs';
import { World } from '../lib/world.mjs';

const here = dirname(fileURLToPath(import.meta.url));
const L = readFileSync(join(process.argv[2], 'events.jsonl'), 'utf8').trim().split('\n').map((l) => JSON.parse(l));
const world = new World();
for (const e of L) {
  if (e.k !== 'ws.in' || e.op !== 2) continue;
  let p;
  try { p = decode(Buffer.from(e.data, 'base64')); } catch { continue; } // LiveKit sockets are protobuf, not msgpack
  if (p?.type === 2 && p.data[0] === 'b_set_obj') world.apply(decodeSetMapObject(p.data[1]));
}
const spawn = { x: 3260, y: 980 };
const state = { t: 'world', me: 'me', spawn, room: 'offline test', dry: true, ...world.snapshot(),
  users: [{ id: 'me', name: 'Test Me', x: Number(process.env.START_X ?? 3750), y: Number(process.env.START_Y ?? 3560), state: 'idle' }, { id: 'o', name: 'Other', x: 3900, y: 3560, state: 'idle' }] };
createServer((req, res) => {
  if (req.url === '/events') {
    res.writeHead(200, { 'content-type': 'text/event-stream' });
    res.write(`data: ${JSON.stringify(state)}\n\n`);
    setInterval(() => res.write(`data: ${JSON.stringify({ t: 'stats', bridgeRss: 61e6, bridgeHeap: 9e6, chromePss: 250e6, chromeProcs: 7 })}\n\n`), 1000);
    return;
  }
  if (req.url === '/act') { global.acts = (global.acts ?? 0) + 1; return res.writeHead(204).end(); }
  let f = req.url.split('?')[0]; if (f === '/') f = '/index.html';
  try { const b = readFileSync(join(here, '..', 'public', f)); res.writeHead(200, { 'content-type': { '.html': 'text/html', '.js': 'text/javascript' }[extname(f)] }); res.end(b); }
  catch { res.writeHead(404).end(); }
}).listen(Number(process.argv[3] ?? 4830), '127.0.0.1');
