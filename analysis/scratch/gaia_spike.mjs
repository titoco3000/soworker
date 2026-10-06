#!/usr/bin/env node
// Gaia spike: refresh token -> ID token -> gaia/find -> open the realtime socket and read the world.
// Read-only: sends only the same handshake messages the web client sends. Prints no secrets.
// Usage: node analysis/scratch/gaia_spike.mjs recorder/logs/<run> [seconds]
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { randomBytes } from 'node:crypto';

const L = readFileSync(join(process.argv[2], 'events.jsonl'), 'utf8').trim().split('\n').map((l) => JSON.parse(l));
const seconds = Number(process.argv[3] ?? 12);
const VERSION = '11.202.0';

// ---- minimal msgpack (decode copied from prepare.mjs; encode for plain JSON values)
function msgpackDecode(buf) {
  let o = 0;
  const rd = () => {
    const b = buf[o++];
    if (b < 0x80) return b;
    if (b >= 0xe0) return b - 256;
    if (b >= 0xa0 && b < 0xc0) return str(b & 0x1f);
    if (b >= 0x90 && b < 0xa0) return arr(b & 0x0f);
    if (b >= 0x80 && b < 0x90) return map(b & 0x0f);
    const dv = () => new DataView(buf.buffer, buf.byteOffset + o);
    switch (b) {
      case 0xc0: return null;
      case 0xc2: return false;
      case 0xc3: return true;
      case 0xc4: { const n = buf[o++]; return bin(n); }
      case 0xc5: { const n = dv().getUint16(0); o += 2; return bin(n); }
      case 0xc6: { const n = dv().getUint32(0); o += 4; return bin(n); }
      case 0xca: { const v = dv().getFloat32(0); o += 4; return +v.toFixed(4); }
      case 0xcb: { const v = dv().getFloat64(0); o += 8; return +v.toFixed(4); }
      case 0xcc: return buf[o++];
      case 0xcd: { const v = dv().getUint16(0); o += 2; return v; }
      case 0xce: { const v = dv().getUint32(0); o += 4; return v; }
      case 0xcf: { const v = Number(dv().getBigUint64(0)); o += 8; return v; }
      case 0xd0: return (buf[o++] << 24) >> 24;
      case 0xd1: { const v = dv().getInt16(0); o += 2; return v; }
      case 0xd2: { const v = dv().getInt32(0); o += 4; return v; }
      case 0xd3: { const v = Number(dv().getBigInt64(0)); o += 8; return v; }
      case 0xd9: { const n = buf[o++]; return str(n); }
      case 0xda: { const n = dv().getUint16(0); o += 2; return str(n); }
      case 0xdb: { const n = dv().getUint32(0); o += 4; return str(n); }
      case 0xdc: { const n = dv().getUint16(0); o += 2; return arr(n); }
      case 0xdd: { const n = dv().getUint32(0); o += 4; return arr(n); }
      case 0xde: { const n = dv().getUint16(0); o += 2; return map(n); }
      case 0xdf: { const n = dv().getUint32(0); o += 4; return map(n); }
      case 0xd4: case 0xd5: case 0xd6: case 0xd7: case 0xd8: { const n = 1 << (b - 0xd4); o += 1 + n; return { $ext: n }; }
      case 0xc7: { const n = buf[o]; o += 2 + n; return { $ext: n }; }
      default: throw new Error('msgpack byte 0x' + b.toString(16));
    }
  };
  const str = (n) => { const s = buf.toString('utf8', o, o + n); o += n; return s; };
  const bin = (n) => { const h = buf.subarray(o, o + n).toString('hex'); o += n; return { $bin: n, hex: h.slice(0, 96) }; };
  const arr = (n) => Array.from({ length: n }, rd);
  const map = (n) => { const r = {}; for (let i = 0; i < n; i++) { const k = rd(); r[k] = rd(); } return r; };
  return rd();
}


function msgpackEncode(v) {
  const parts = [];
  const w = (x) => {
    if (v === undefined) {}
    if (x === null || x === undefined) parts.push(Buffer.from([0xc0]));
    else if (x === true) parts.push(Buffer.from([0xc3]));
    else if (x === false) parts.push(Buffer.from([0xc2]));
    else if (typeof x === 'number') {
      if (Number.isInteger(x) && x >= 0 && x < 128) parts.push(Buffer.from([x]));
      else if (Number.isInteger(x) && x >= 0 && x < 2 ** 32) { const b = Buffer.alloc(5); b[0] = 0xce; b.writeUInt32BE(x, 1); parts.push(b); }
      else if (Number.isInteger(x) && x >= 0) { const b = Buffer.alloc(9); b[0] = 0xcf; b.writeBigUInt64BE(BigInt(x), 1); parts.push(b); }
      else { const b = Buffer.alloc(9); b[0] = 0xcb; b.writeDoubleBE(x, 1); parts.push(b); }
    } else if (typeof x === 'string') {
      const s = Buffer.from(x);
      if (s.length < 32) parts.push(Buffer.from([0xa0 | s.length]));
      else if (s.length < 65536) { const b = Buffer.alloc(3); b[0] = 0xda; b.writeUInt16BE(s.length, 1); parts.push(b); }
      else { const b = Buffer.alloc(5); b[0] = 0xdb; b.writeUInt32BE(s.length, 1); parts.push(b); }
      parts.push(s);
    } else if (Array.isArray(x)) {
      if (x.length < 16) parts.push(Buffer.from([0x90 | x.length]));
      else { const b = Buffer.alloc(3); b[0] = 0xdc; b.writeUInt16BE(x.length, 1); parts.push(b); }
      x.forEach(w);
    } else {
      const ks = Object.keys(x).filter((k) => x[k] !== undefined);
      if (ks.length < 16) parts.push(Buffer.from([0x80 | ks.length]));
      else { const b = Buffer.alloc(3); b[0] = 0xde; b.writeUInt16BE(ks.length, 1); parts.push(b); }
      for (const k of ks) { w(k); w(x[k]); }
    }
  };
  w(v);
  return Buffer.concat(parts);
}

// ---- 1. tokens
const idp = L.find((e) => e.k === 'res.body' && /accounts:signInWithIdp/.test(e.url) && /refreshToken/.test(e.text ?? ''));
const apiKey = new URL(idp.url).searchParams.get('key');
const findReq = L.find((e) => e.k === 'req' && e.method === 'POST' && /gaia\/find/.test(e.url));
const tok = await (await fetch(`https://securetoken.googleapis.com/v1/token?key=${apiKey}`, {
  method: 'POST',
  headers: { 'content-type': 'application/x-www-form-urlencoded' },
  body: new URLSearchParams({ grant_type: 'refresh_token', refresh_token: JSON.parse(idp.text).refreshToken }),
})).json();
const idToken = tok.id_token;
console.log('id token:', idToken ? 'ok' : 'FAILED');

// ---- 2. find the room
const found = await (await fetch('https://api.sowork.com/api/v1/gaia/find', {
  method: 'POST',
  headers: { source: 'web', version: VERSION, 'content-type': 'application/json', authorization: `Bearer ${idToken}` },
  body: findReq.postData,
})).json();
const roomId = found.room.id;
console.log('room:', found.room.name, '| users in room:', found.room.userCount);

// ---- 3. gaia socket
const q = new URLSearchParams({
  roomId, mediaRegion: 'eu-central-1', password: '', clientVersion: VERSION,
  autoForceId: randomBytes(16).toString('base64url').slice(0, 21),
  clusterMode: 'none-connecting', meetingZoneId: '', joinType: 'force', EIO: '4', transport: 'websocket',
});
const ws = new WebSocket(`wss://api.sowork.com/gaia/g5/v2/?${q}`);
ws.binaryType = 'arraybuffer';
const events = {};
let nextAck = 0;
const sendPacket = (p) => ws.send(msgpackEncode(p));
const emit = (name, payload, ack) => sendPacket({ type: 2, data: payload === undefined ? [name] : [name, payload], options: { compress: true }, ...(ack !== undefined ? { id: ack } : {}), nsp: '/' });
let room;

ws.onmessage = (m) => {
  if (typeof m.data === 'string') {
    if (m.data === '2') ws.send('3'); // engine.io ping -> pong
    else if (m.data[0] === '0') { console.log('engine.io open'); sendPacket({ type: 0, data: { token: idToken }, nsp: '/' }); }
    return;
  }
  const p = msgpackDecode(Buffer.from(m.data));
  if (p.type === 0) { console.log('socket.io connected'); emit('sync_time', { clientTime: Date.now() }, nextAck++); }
  else if (p.type === 4) console.log('connect error:', JSON.stringify(p.data).slice(0, 200));
  else if (p.type === 2) {
    const [name, payload] = p.data;
    events[name] = (events[name] ?? 0) + 1;
    if (name === 'init') {
      emit('should_update', { v: true, t: 0, lastObjectId: '' });
      emit('active_app', { activeApp: null });
    } else if (name === 'room') room = payload;
    else if (name === 'server_version') console.log('server_version:', JSON.stringify(payload));
    else if (name === 'cluster') console.log('cluster event keys:', Object.keys(payload ?? {}));
  }
};
ws.onerror = (e) => console.log('ws error:', e.message ?? 'error');
ws.onclose = (e) => console.log('ws closed', e.code);

await new Promise((r) => setTimeout(r, seconds * 1000));
ws.close();
console.log('events received:', events);
if (room) console.log('room event keys:', Object.keys(room), '| spawns:', room.spawns?.length, '| mapId:', room.mapId);
process.exit(0);
