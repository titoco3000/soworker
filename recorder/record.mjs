#!/usr/bin/env node
// Launches Chrome with a dedicated profile, attaches over the DevTools protocol and records
// every network exchange of every tab, iframe and worker into logs/<run>/events.jsonl:
// HTTP requests/responses (with bodies), WebSocket frames, SSE, plus WebRTC signaling and
// data channel traffic reported by hook.js. Zero dependencies (Node >= 22).
//
//   node recorder/record.mjs [start-url] [--chrome <bin>] [--profile <dir>] [--out <dir>]
//
// Env: CHROME_FLAGS="--headless=new --no-sandbox" for extra Chrome flags (used in tests).
// Logs contain session tokens and cookies: keep them local.

import { spawn } from 'node:child_process';
import { createHash, randomBytes } from 'node:crypto';
import net from 'node:net';
import { createWriteStream, existsSync, openSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { setTimeout as sleep } from 'node:timers/promises';
import { fileURLToPath } from 'node:url';

process.umask(0o077);

const root = dirname(fileURLToPath(import.meta.url));
const args = process.argv.slice(2);
const opt = (name, def) => {
  const i = args.indexOf(name);
  return i >= 0 ? args[i + 1] : def;
};
const startUrl = args.find((a) => /^https?:/.test(a)) ?? 'https://www.sowork.com/';
const chromeBin = opt('--chrome', 'google-chrome-stable');
const profileDir = opt('--profile', join(root, 'profile'));
const runName = new Date().toISOString().replace(/[:.]/g, '-');
const runDir = join(opt('--out', join(root, 'logs')), runName);
const bodiesDir = join(runDir, 'bodies');
mkdirSync(bodiesDir, { recursive: true, mode: 0o700 });
mkdirSync(profileDir, { recursive: true, mode: 0o700 });

const INLINE_BODY = 16 * 1024; // larger bodies go to bodies/<hash>
const MAX_BODY = 50 * 1024 * 1024;
const SNAPSHOT_EVERY_MS = 60_000;
const hookSource = readFileSync(join(root, 'hook.js'), 'utf8');
const SNAPSHOT_EXPR = `(async () => JSON.stringify({
  href: location.href,
  ls: Object.fromEntries(Object.entries(localStorage)),
  ss: Object.fromEntries(Object.entries(sessionStorage)),
  idb: (indexedDB.databases ? await indexedDB.databases() : []),
}))()`;

// ---- log sink -------------------------------------------------------------------------

const out = createWriteStream(join(runDir, 'events.jsonl'));
let closing = false;
let eventCount = 0;
const counts = {};
const log = (k, tg, data) => {
  eventCount++;
  counts[k] = (counts[k] ?? 0) + 1;
  out.write(JSON.stringify({ t: Date.now(), k, tg, ...data }) + '\n');
};

// ---- minimal CDP client ---------------------------------------------------------------

// Node's built-in WebSocket rejects large messages ("Max decompressed message size exceeded"),
// which large response bodies trigger. This is a minimal RFC 6455 client without extensions
// or size limits, enough for Chrome's DevTools endpoint.
const EMPTY = Buffer.alloc(0);

class Cdp {
  #id = 0;
  #pending = new Map();
  #hdr = EMPTY; // partial frame header carried between chunks
  #frameState = null; // frame whose payload is still arriving
  #parts = []; // fragments of the current message
  #opcode = 0;
  #handshake = EMPTY;
  isOpen = false;
  onEvent = () => {};
  onClose = () => {};

  constructor(url) {
    this.url = new URL(url);
  }

  open() {
    return new Promise((resolve, reject) => {
      const { hostname, port, pathname } = this.url;
      const sock = (this.sock = net.connect(Number(port), hostname));
      sock.setNoDelay(true);
      sock.on('connect', () => {
        sock.write(
          `GET ${pathname} HTTP/1.1\r\nHost: ${hostname}:${port}\r\nUpgrade: websocket\r\n` +
            `Connection: Upgrade\r\nSec-WebSocket-Key: ${randomBytes(16).toString('base64')}\r\n` +
            `Sec-WebSocket-Version: 13\r\n\r\n`,
        );
      });
      sock.on('data', (chunk) => {
        if (this.isOpen) return this.#data(chunk);
        this.#handshake = Buffer.concat([this.#handshake, chunk]);
        const end = this.#handshake.indexOf('\r\n\r\n');
        if (end < 0) return;
        if (!/^HTTP\/1.1 101/.test(this.#handshake.toString('latin1', 0, 16))) {
          return reject(new Error('CDP handshake failed'));
        }
        const rest = this.#handshake.subarray(end + 4);
        this.isOpen = true;
        resolve();
        if (rest.length) this.#data(rest);
      });
      sock.on('error', (e) => {
        this.lastError = e.message;
        reject(e);
      });
      sock.on('close', () => {
        this.isOpen = false;
        this.onClose({ code: this.closeCode ?? 1006, reason: '' });
      });
    });
  }

  #data(chunk) {
    let buf = this.#hdr.length ? Buffer.concat([this.#hdr, chunk]) : chunk;
    this.#hdr = EMPTY;
    while (buf.length) {
      if (!this.#frameState) {
        if (buf.length < 2) break;
        let len = buf[1] & 0x7f;
        let hl = 2;
        if (len === 126) {
          if (buf.length < 4) break;
          len = buf.readUInt16BE(2);
          hl = 4;
        } else if (len === 127) {
          if (buf.length < 10) break;
          len = Number(buf.readBigUInt64BE(2));
          hl = 10;
        }
        if (buf[1] & 0x80) hl += 4; // servers do not mask; tolerate the flag
        if (buf.length < hl) break;
        this.#frameState = { fin: !!(buf[0] & 0x80), op: buf[0] & 0x0f, len, got: 0, data: Buffer.allocUnsafe(len) };
        buf = buf.subarray(hl);
      }
      const f = this.#frameState;
      const take = Math.min(f.len - f.got, buf.length);
      buf.copy(f.data, f.got, 0, take);
      f.got += take;
      buf = buf.subarray(take);
      if (f.got === f.len) {
        this.#frameState = null;
        this.#frame(f);
      }
    }
    if (buf.length) this.#hdr = Buffer.from(buf);
  }

  #frame({ fin, op, data }) {
    if (op === 0x8) {
      this.closeCode = data.length >= 2 ? data.readUInt16BE(0) : 1005;
      this.#write(0x8, data.subarray(0, 2));
      this.sock.end();
    } else if (op === 0x9) {
      this.#write(0xa, data);
    } else if (op === 0x1 || op === 0x2 || op === 0x0) {
      if (op !== 0x0) this.#opcode = op;
      this.#parts.push(data);
      if (!fin) return;
      const msg = (this.#parts.length === 1 ? this.#parts[0] : Buffer.concat(this.#parts)).toString('utf8');
      this.#parts = [];
      this.#message(msg);
    }
  }

  #message(text) {
    const m = JSON.parse(text);
    if (m.id) {
      const p = this.#pending.get(m.id);
      if (!p) return;
      this.#pending.delete(m.id);
      m.error ? p.reject(new Error(`${m.error.message} (${p.method})`)) : p.resolve(m.result);
    } else {
      this.onEvent(m.method, m.params, m.sessionId);
    }
  }

  #write(op, payload) {
    const len = payload.length;
    let h;
    if (len < 126) {
      h = Buffer.from([0x80 | op, 0x80 | len]);
    } else if (len < 65536) {
      h = Buffer.from([0x80 | op, 0x80 | 126, len >> 8, len & 0xff]);
    } else {
      h = Buffer.alloc(10);
      h[0] = 0x80 | op;
      h[1] = 0x80 | 127;
      h.writeBigUInt64BE(BigInt(len), 2);
    }
    const mask = randomBytes(4);
    const body = Buffer.allocUnsafe(len);
    for (let i = 0; i < len; i++) body[i] = payload[i] ^ mask[i & 3];
    this.sock.write(Buffer.concat([h, mask, body]));
  }

  send(method, params = {}, sessionId) {
    const id = ++this.#id;
    const msg = { id, method, params };
    if (sessionId) msg.sessionId = sessionId;
    return new Promise((resolve, reject) => {
      if (!this.isOpen) return reject(new Error('CDP closed'));
      this.#pending.set(id, { resolve, reject, method });
      this.#write(0x1, Buffer.from(JSON.stringify(msg)));
    });
  }
}

const withTimeout = (p, ms) => Promise.race([p, sleep(ms).then(() => { throw new Error('timeout'); })]);

// ---- launch Chrome --------------------------------------------------------------------

rmSync(join(profileDir, 'DevToolsActivePort'), { force: true });
const extraFlags = (process.env.CHROME_FLAGS ?? '').split(/\s+/).filter(Boolean);
const chrome = spawn(
  chromeBin,
  [
    `--user-data-dir=${profileDir}`,
    '--remote-debugging-port=0',
    '--no-first-run',
    '--no-default-browser-check',
    ...extraFlags,
    'about:blank',
  ],
  { stdio: ['ignore', 'ignore', openSync(join(runDir, 'chrome.log'), 'w')] },
);
let chromeExited = false;
chrome.on('error', (e) => {
  console.error(`cannot start ${chromeBin}: ${e.message}`);
  process.exit(1);
});
chrome.on('exit', (code, signal) => {
  chromeExited = true;
  shutdown(`chrome exited (code ${code}, signal ${signal})`);
});

async function browserWsUrl() {
  const file = join(profileDir, 'DevToolsActivePort');
  for (let i = 0; i < 300; i++) {
    if (chromeExited) throw new Error('Chrome exited early. Is another Chrome using this profile?');
    if (existsSync(file)) {
      const [port, path] = readFileSync(file, 'utf8').trim().split('\n');
      if (port && path) return `ws://127.0.0.1:${port}${path}`;
    }
    await sleep(100);
  }
  throw new Error('timed out waiting for Chrome DevTools port');
}

const cdp = new Cdp(await browserWsUrl());
await cdp.open();
cdp.onClose = (e) =>
  shutdown(`devtools connection closed (code ${e.code}, reason "${e.reason}", error ${cdp.lastError ?? 'none'})`);

const version = await cdp.send('Browser.getVersion');
writeFileSync(
  join(runDir, 'meta.json'),
  JSON.stringify({ startedAt: new Date().toISOString(), startUrl, chromeBin, version, profileDir }, null, 2),
);

// ---- targets --------------------------------------------------------------------------

const INSTRUMENTED = new Set(['page', 'iframe', 'worker', 'service_worker', 'shared_worker']);
const DISCOVER_ATTACH = new Set(['page', 'service_worker', 'shared_worker']);
const sessions = new Map(); // sessionId -> { n, targetId, type }
const seenTargets = new Set();
const requested = new Set();
const reqs = new Map(); // `${sessionId}:${requestId}` -> { url, type, mime, status }
let targetN = 0;
let navigated = false;

const resume = (sessionId) => cdp.send('Runtime.runIfWaitingForDebugger', {}, sessionId).catch(() => {});

async function setup(sessionId, type, waiting) {
  const call = (m, p) => cdp.send(m, p, sessionId).catch(() => {});
  const isDoc = type === 'page' || type === 'iframe';
  await call('Target.setAutoAttach', { autoAttach: true, waitForDebuggerOnStart: true, flatten: true });
  await call('Network.enable', {
    maxTotalBufferSize: 200e6,
    maxResourceBufferSize: 50e6,
    maxPostDataSize: 1e6,
  });
  if (isDoc) {
    // Full bodies for every load (JS bundles included), even when Chrome would use its cache.
    await call('Network.setCacheDisabled', { cacheDisabled: true });
    await call('Runtime.addBinding', { name: '__rec' });
    await call('Runtime.enable');
    await call('Page.enable');
    await call('Page.addScriptToEvaluateOnNewDocument', { source: hookSource, runImmediately: true });
  }
  if (waiting) await resume(sessionId);
}

function onTargetEvent(method, p) {
  if (method === 'Target.attachedToTarget') {
    const { sessionId, targetInfo: ti, waitingForDebugger } = p;
    if (!INSTRUMENTED.has(ti.type) || ti.url.startsWith('chrome-extension://')) {
      if (waitingForDebugger) resume(sessionId);
      return;
    }
    if (seenTargets.has(ti.targetId)) {
      // Already instrumented through another session (discovery vs auto-attach race).
      if (waitingForDebugger) resume(sessionId);
      cdp.send('Target.detachFromTarget', { sessionId }).catch(() => {});
      return;
    }
    seenTargets.add(ti.targetId);
    const s = { n: ++targetN, targetId: ti.targetId, type: ti.type };
    sessions.set(sessionId, s);
    log('target', s.n, { type: ti.type, url: ti.url, opener: ti.openerId });
    setup(sessionId, ti.type, waitingForDebugger).then(async () => {
      if (!navigated && ti.type === 'page') {
        navigated = true;
        await cdp.send('Page.navigate', { url: startUrl }, sessionId).catch(() => {});
      }
    });
  } else if (method === 'Target.detachedFromTarget') {
    const s = sessions.get(p.sessionId);
    if (s) {
      log('target.gone', s.n, {});
      sessions.delete(p.sessionId);
    }
  } else if (method === 'Target.targetCreated') {
    const ti = p.targetInfo;
    if (DISCOVER_ATTACH.has(ti.type) && !seenTargets.has(ti.targetId) && !requested.has(ti.targetId)) {
      requested.add(ti.targetId);
      cdp.send('Target.attachToTarget', { targetId: ti.targetId, flatten: true }).catch(() => {});
    }
  }
}

// ---- per-session events ---------------------------------------------------------------

const textual = (mime) => !/^(image|video|audio|font)\//i.test(mime ?? '');
const wantBody = (q) =>
  textual(q.mime) &&
  !['Image', 'Media', 'Font', 'Stylesheet'].includes(q.type) &&
  !(q.status >= 300 && q.status < 400) &&
  q.status !== 204;

const extFor = (mime = '') =>
  /json/i.test(mime) ? '.json'
  : /javascript|ecmascript/i.test(mime) ? '.js'
  : /html/i.test(mime) ? '.html'
  : /wasm/i.test(mime) ? '.wasm'
  : /^text\//i.test(mime) ? '.txt'
  : '.bin';

async function saveBody(sid, tg, requestId) {
  const key = `${sid}:${requestId}`;
  const q = reqs.get(key);
  reqs.delete(key);
  if (!q || !wantBody(q)) return;
  let r;
  try {
    r = await cdp.send('Network.getResponseBody', { requestId }, sid);
  } catch (e) {
    log('res.body.err', tg, { id: requestId, url: q.url, err: e.message });
    return;
  }
  const bytes = Buffer.from(r.body, r.base64Encoded ? 'base64' : 'utf8');
  const ev = { id: requestId, url: q.url, mime: q.mime, size: bytes.length };
  if (bytes.length > MAX_BODY) {
    ev.skipped = 'too-large';
  } else if (bytes.length <= INLINE_BODY) {
    ev[r.base64Encoded ? 'b64' : 'text'] = r.body;
  } else {
    const file = createHash('sha1').update(bytes).digest('hex').slice(0, 16) + extFor(q.mime);
    const path = join(bodiesDir, file);
    if (!existsSync(path)) writeFileSync(path, bytes);
    ev.file = `bodies/${file}`;
  }
  log('res.body', tg, ev);
}

function onSessionEvent(method, p, sid) {
  const s = sessions.get(sid);
  if (!s) return;
  const tg = s.n;
  switch (method) {
    case 'Network.requestWillBeSent': {
      const r = p.request;
      reqs.set(`${sid}:${p.requestId}`, { url: r.url, type: p.type });
      const ev = {
        id: p.requestId,
        url: r.url,
        method: r.method,
        headers: r.headers,
        rtype: p.type,
        init: p.initiator?.type,
        initUrl: p.initiator?.stack?.callFrames?.[0]?.url ?? p.initiator?.url,
      };
      if (p.redirectResponse) {
        ev.redirectFrom = {
          status: p.redirectResponse.status,
          url: p.redirectResponse.url,
          headers: p.redirectResponse.headers,
        };
      }
      if (r.postData) {
        ev.postData = r.postData;
      } else if (r.hasPostData) {
        cdp
          .send('Network.getRequestPostData', { requestId: p.requestId }, sid)
          .then((x) => log('req.postData', tg, { id: p.requestId, postData: x.postData, b64: x.base64Encoded }))
          .catch(() => {});
      }
      log('req', tg, ev);
      break;
    }
    case 'Network.requestWillBeSentExtraInfo':
      log('req.extra', tg, { id: p.requestId, headers: p.headers });
      break;
    case 'Network.responseReceived': {
      const r = p.response;
      const q = reqs.get(`${sid}:${p.requestId}`) ?? {};
      q.mime = r.mimeType;
      q.status = r.status;
      q.type ??= p.type;
      reqs.set(`${sid}:${p.requestId}`, q);
      log('res', tg, {
        id: p.requestId,
        url: r.url,
        status: r.status,
        statusText: r.statusText,
        headers: r.headers,
        mime: r.mimeType,
        protocol: r.protocol,
        remoteIP: r.remoteIPAddress,
        remotePort: r.remotePort,
        fromCache: r.fromDiskCache || r.fromPrefetchCache,
        fromSW: r.fromServiceWorker,
        rtype: p.type,
      });
      break;
    }
    case 'Network.responseReceivedExtraInfo':
      log('res.extra', tg, { id: p.requestId, status: p.statusCode, headers: p.headers });
      break;
    case 'Network.loadingFinished':
      saveBody(sid, tg, p.requestId);
      break;
    case 'Network.loadingFailed':
      reqs.delete(`${sid}:${p.requestId}`);
      log('fail', tg, { id: p.requestId, err: p.errorText, canceled: p.canceled, blocked: p.blockedReason });
      break;
    case 'Network.webSocketCreated':
      log('ws.open', tg, { id: p.requestId, url: p.url, initUrl: p.initiator?.stack?.callFrames?.[0]?.url });
      break;
    case 'Network.webSocketWillSendHandshakeRequest':
      log('ws.hs.req', tg, { id: p.requestId, headers: p.request.headers });
      break;
    case 'Network.webSocketHandshakeResponseReceived':
      log('ws.hs.res', tg, { id: p.requestId, status: p.response.status, headers: p.response.headers });
      break;
    case 'Network.webSocketFrameSent':
      log('ws.out', tg, { id: p.requestId, op: p.response.opcode, data: p.response.payloadData });
      break;
    case 'Network.webSocketFrameReceived':
      log('ws.in', tg, { id: p.requestId, op: p.response.opcode, data: p.response.payloadData });
      break;
    case 'Network.webSocketFrameError':
      log('ws.err', tg, { id: p.requestId, err: p.errorMessage });
      break;
    case 'Network.webSocketClosed':
      log('ws.close', tg, { id: p.requestId });
      break;
    case 'Network.eventSourceMessageReceived':
      log('sse', tg, { id: p.requestId, event: p.eventName, eventId: p.eventId, data: p.data });
      break;
    case 'Page.frameNavigated':
      log('nav', tg, { url: p.frame.url, frameId: p.frame.id, parent: p.frame.parentId });
      break;
    case 'Runtime.bindingCalled':
      if (p.name === '__rec') {
        try {
          const { k, ...rest } = JSON.parse(p.payload);
          log(k, tg, rest);
        } catch {}
      }
      break;
    default:
      if (method.startsWith('Network.webTransport')) log(method.replace('Network.', ''), tg, p);
  }
}

cdp.onEvent = (method, params, sid) => {
  if (method.startsWith('Target.')) onTargetEvent(method, params);
  else if (sid) onSessionEvent(method, params, sid);
};

// ---- snapshots, status, shutdown ------------------------------------------------------

async function snapshot(reason) {
  for (const [sid, s] of sessions) {
    if (s.type !== 'page') continue;
    try {
      const r = await withTimeout(
        cdp.send('Runtime.evaluate', { expression: SNAPSHOT_EXPR, awaitPromise: true, returnByValue: true }, sid),
        3000,
      );
      if (r.result?.value) log('storage', s.n, { reason, ...JSON.parse(r.result.value) });
    } catch {}
  }
  try {
    const c = await withTimeout(cdp.send('Storage.getCookies'), 3000);
    log('cookies', 0, { reason, cookies: c.cookies });
  } catch {}
}

async function shutdown(reason) {
  if (closing) return;
  closing = true;
  if (!chromeExited && cdp.isOpen) await snapshot('exit').catch(() => {});
  console.error(`\nstopped (${reason}): ${eventCount} events in ${runDir}`);
  chrome.kill();
  out.end(() => process.exit(0));
}
for (const ev of ['uncaughtException', 'unhandledRejection']) {
  process.on(ev, (e) => {
    console.error(`\n${ev}:`, e);
    shutdown(ev);
  });
}
process.on('SIGINT', () => shutdown('SIGINT'));
process.on('SIGTERM', () => shutdown('SIGTERM'));

setInterval(() => snapshot('periodic'), SNAPSHOT_EVERY_MS).unref();
setInterval(() => {
  const c = counts;
  process.stderr.write(
    `\r${eventCount} events | http ${c.res ?? 0} | ws in/out ${c['ws.in'] ?? 0}/${c['ws.out'] ?? 0}` +
      ` | rtc ${Object.entries(c).filter(([k]) => /^(pc|dc)\./.test(k)).reduce((a, [, v]) => a + v, 0)}   `,
  );
}, 2000).unref();

await cdp.send('Target.setDiscoverTargets', { discover: true });
console.error(`recording to ${runDir}\nuse the Chrome window that just opened; Ctrl+C here to stop.`);
