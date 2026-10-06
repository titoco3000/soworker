#!/usr/bin/env node
// Turns a raw recorder run into redacted, easier-to-read working files in analysis/work/.
// Usage: node analysis/prepare.mjs recorder/logs/<run>
// Redacts JWTs, LiveKit join_request blobs and obvious secrets. Raw logs stay untouched.
import { mkdirSync, readFileSync, writeFileSync, existsSync, copyFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const run = process.argv[2];
const out = join(dirname(fileURLToPath(import.meta.url)), 'work');
mkdirSync(join(out, 'ws'), { recursive: true });
mkdirSync(join(out, 'bodies'), { recursive: true });

const NOISE = /facebook\.|doubleclick|google-analytics|analytics\.google|googletagmanager|intercom|amplitude|sentry\.io|gstatic|googleusercontent|youtube|google\.(com|de|com\.br)\/(ccm|pagead|rmkt|search)|play\.google|apis\.google|accounts\.google|signaler-pa|\.google\./;
const redact = (s) =>
  String(s)
    .replace(/eyJ[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]*/g, '<JWT>')
    .replace(/(join_request=)[^&"\s]+/g, '$1<JOIN_REQUEST>')
    .replace(/(access_token|token|apiKey|key|code|state|rart|rapt)=([^&"\s]{12,})/g, '$1=<REDACTED>');


// Minimal MessagePack decoder (gaia uses socket.io-msgpack-parser). Binary blobs become {$bin:len,hex}.
function msgpack(buf) {
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

const L = readFileSync(join(run, 'events.jsonl'), 'utf8').trim().split('\n').map((l) => JSON.parse(l));
const t0 = L[0].t;
const rel = (t) => ((t - t0) / 1000).toFixed(3);

// ---- HTTP table
const reqs = new Map();
for (const e of L) {
  if (e.k === 'req') reqs.set(`${e.tg}:${e.id}`, { t: e.t, method: e.method, url: e.url, post: e.postData, rtype: e.rtype });
  if (e.k === 'res') Object.assign(reqs.get(`${e.tg}:${e.id}`) ?? {}, { status: e.status, mime: e.mime, proto: e.protocol });
  if (e.k === 'res.body') Object.assign(reqs.get(`${e.tg}:${e.id}`) ?? {}, { size: e.size, file: e.file, text: e.text, b64: e.b64 });
}
const rows = [];
for (const [key, r] of reqs) {
  if (!r.url || NOISE.test(r.url)) continue;
  rows.push(`${rel(r.t)}\t${key}\t${r.method}\t${r.status ?? ''}\t${r.rtype}\t${r.mime ?? ''}\t${r.size ?? ''}\t${redact(r.url)}`);
}
writeFileSync(join(out, 'http.tsv'), 'time\tkey\tmethod\tstatus\ttype\tmime\tsize\turl\n' + rows.join('\n') + '\n');

// API-ish bodies (non-static): save redacted request/response pairs
let n = 0;
const api = [];
for (const [key, r] of reqs) {
  if (!r.url || NOISE.test(r.url)) continue;
  if (/\/static\/|\.(js|css|png|svg|woff2?|ico|jpg|webp)(\?|$)/.test(r.url)) continue;
  if (!/(json|text|octet|protobuf)/.test(r.mime ?? '') && !r.post) continue;
  const body = r.text ?? (r.file && existsSync(join(run, r.file)) ? readFileSync(join(run, r.file), 'utf8').slice(0, 200000) : r.b64 ? `<b64 ${r.b64.length}>` : '');
  api.push(`### ${rel(r.t)} ${r.method} ${redact(r.url)} -> ${r.status}\n${r.post ? 'REQ: ' + redact(r.post).slice(0, 4000) + '\n' : ''}RES: ${redact(body).slice(0, 6000)}\n`);
  n++;
}
writeFileSync(join(out, 'api-bodies.md'), api.join('\n'));

// ---- WebSockets
const socks = new Map();
for (const e of L) {
  if (e.k === 'ws.open') socks.set(`${e.tg}:${e.id}`, { n: socks.size + 1, url: e.url, t: e.t, lines: [] });
  if (e.k === 'ws.in' || e.k === 'ws.out') {
    const s = socks.get(`${e.tg}:${e.id}`);
    if (!s) continue;
    let data;
    if (e.op === 2) {
      try { data = 'MP ' + redact(JSON.stringify(msgpack(Buffer.from(e.data, 'base64')))); }
      catch (err) { data = `<binary b64 ${e.data.length}> ${e.data.slice(0, 200)} (${err.message})`; }
    } else data = redact(e.data);
    s.lines.push(`${rel(e.t)} ${e.k === 'ws.in' ? 'IN ' : 'OUT'} ${data.slice(0, 6000)}`);
  }
  if (e.k === 'ws.close') socks.get(`${e.tg}:${e.id}`)?.lines.push(`${rel(e.t)} CLOSE`);
}
const index = [];
for (const s of socks.values()) {
  const host = new URL(s.url).host.replace(/[^a-z0-9.-]/gi, '_');
  const name = `${String(s.n).padStart(2, '0')}-${host}.txt`;
  writeFileSync(join(out, 'ws', name), `# opened at +${rel(s.t)}s\n# ${redact(s.url)}\n` + s.lines.join('\n') + '\n');
  index.push(`${name}\t${s.lines.length} frames\t${redact(s.url).slice(0, 160)}`);
}
writeFileSync(join(out, 'ws', 'INDEX.tsv'), index.join('\n') + '\n');

// ---- RTC + media + misc
const rtc = L.filter((e) => /^(pc|dc|media)\./.test(e.k)).map((e) => {
  const { t, k, tg, ts, href, ...rest } = e;
  return `${rel(t)} ${k} ${redact(JSON.stringify(rest)).slice(0, 3000)}`;
});
writeFileSync(join(out, 'rtc.txt'), rtc.join('\n') + '\n');

// ---- first-party static assets worth reading (JS bundles etc), copied for agents
const assets = [];
for (const [, r] of reqs) {
  if (r.url?.startsWith('https://app.sowork.com/') && /\.(js|json|html)(\?|$)/.test(r.url) && (r.file || r.text)) assets.push(r);
}
writeFileSync(join(out, 'assets.tsv'), assets.map((r) => `${r.size}\t${r.url}\t${r.file ?? '(inline)'}`).join('\n') + '\n');
console.log({ httpRows: rows.length, apiBodies: n, sockets: socks.size, rtcLines: rtc.length, assets: assets.length });
