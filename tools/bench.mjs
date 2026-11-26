#!/usr/bin/env node
// CPU and memory of SoWorker (window and terminal clients) next to SoWork in Chrome, on the same scenario:
// sit in the office, then walk into a colleague's call with the microphone on. Each client runs in a systemd scope of its
// own, so every process it starts (Chrome's renderers and GPU process, the terminal client's pty wrapper) is
// counted. Samples once a second: CPU from the scope's cgroup (`cpu.stat`), memory as the sum of PSS over its
// processes (shared libraries split fairly; `memory.current` would also count page cache).
//
// Usage (from the repo root, outside any sandbox, with no other SoWorker or SoWork session open: one account is
// signed in once at a time, and a second client kicks the first):
//   node tools/bench.mjs login               sign the benchmark's Chrome profile in to SoWork (`run` does it when needed)
//   node tools/bench.mjs run [options]       build SoWorker (release), sign in where needed, run every client, save the results
//     --join NAME                whose call to walk into: their name as SoWork shows it (pick someone who is online
//                                and sits still for the whole run, and tell them: we join their call several times).
//                                Or set SOWORKER_BENCH_JOIN. `soworker --roster` lists the names. Without it only
//                                the office part is measured.
//     --clients gui,tui,chrome   which ones (default all three)
//     --runs N                   repeat each client N times, report the median (default 3)
//     --idle S --call S          seconds measured in the office and in the call (default 120 each)
//     --cam                      also turn the camera on in the call
//     --readme                   write the summary table into README.md (between the bench markers)
//   node tools/bench.mjs readme [DIR]        write a saved run's table into README.md (default: the latest run)
//   node tools/bench.mjs probe [--join NAME] (Chrome) print what the scenario sees on the page, for debugging
//
// Results go to bench/results/<date-time>/: samples.csv (every second of every run), results.json and summary.md.
import { spawn, execFileSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';

const ROOT = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..');
const SOWORKER = path.join(ROOT, 'target/release/soworker');
const PROFILE = path.join(os.homedir(), '.local/share/soworker/bench-chrome');
const PORT = 9333;
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

function arg(name, def) {
  const i = process.argv.indexOf(`--${name}`);
  if (i < 0) return def;
  const v = process.argv[i + 1];
  return v === undefined || v.startsWith('--') ? true : v;
}

function spaceUrl() {
  const f = path.join(os.homedir(), '.config/soworker/.space.json');
  const s = JSON.parse(fs.readFileSync(f, 'utf8'));
  return `https://app.sowork.com/s/${s.groupId}/${s.alias}`;
}

function chromeArgs(url) {
  return [
    `--user-data-dir=${PROFILE}`,
    `--remote-debugging-port=${PORT}`,
    '--no-first-run',
    '--no-default-browser-check',
    '--window-size=1280,800',
    // allow the microphone and camera without the permission prompt (the real devices, not fake ones)
    '--use-fake-ui-for-media-stream',
    url,
  ];
}

// ---- Chrome DevTools: Node's own WebSocket, no packages

async function pageTarget(timeoutMs = 30000) {
  const end = Date.now() + timeoutMs;
  while (Date.now() < end) {
    try {
      const list = await (await fetch(`http://127.0.0.1:${PORT}/json/list`)).json();
      const p = list.find((t) => t.type === 'page' && t.url.startsWith('https://app.sowork.com'));
      if (p) return p.webSocketDebuggerUrl;
    } catch {}
    await sleep(500);
  }
  throw new Error('Chrome never opened a SoWork page');
}

/// One DevTools session on the SoWork page, kept for the whole run (a script added for new documents only lives as
/// long as the session that added it).
class Page {
  static async open(timeoutMs) {
    const p = new Page();
    p.ws = new WebSocket(await pageTarget(timeoutMs));
    await new Promise((res, rej) => { p.ws.onopen = res; p.ws.onerror = () => rej(new Error('devtools connection failed')); });
    p.next = 0;
    p.pending = new Map();
    p.ws.onmessage = (m) => {
      const v = JSON.parse(m.data);
      const done = p.pending.get(v.id);
      if (done) { p.pending.delete(v.id); done(v); }
    };
    return p;
  }

  call(method, params = {}) {
    return new Promise((res, rej) => {
      const id = ++this.next;
      const t = setTimeout(() => { this.pending.delete(id); rej(new Error(`devtools timeout (${method})`)); }, 20000);
      this.pending.set(id, (v) => { clearTimeout(t); v.error ? rej(new Error(v.error.message)) : res(v.result); });
      this.ws.send(JSON.stringify({ id, method, params }));
    });
  }

  async eval(expr) {
    const r = await this.call('Runtime.evaluate', { expression: expr, awaitPromise: true, returnByValue: true });
    if (r.exceptionDetails) throw new Error(r.exceptionDetails.exception?.description ?? 'page error');
    return r.result?.value;
  }

  /// Count the remote audio tracks the page receives (`window.__benchAudio`): how we know the call is up. Reloads
  /// the page so the count starts with it.
  async watchCalls() {
    await this.call('Page.enable');
    await this.call('Page.addScriptToEvaluateOnNewDocument', { source: WEB.hook });
    await this.call('Page.reload');
  }

  close() {
    this.ws.close();
  }
}

// What the scenario needs from SoWork's page (as of 2026-11: a lobby with "Enter Office", then the people list with
// "Go to" on each online person). The page is SoWork's and changes with it: `probe` shows what these see.
const label = `(e) => (e.getAttribute('aria-label') || e.title || e.textContent || '').trim()`;
const WEB = {
  hook: `(() => {
    const Rtc = window.RTCPeerConnection;
    if (!Rtc || Rtc.__bench) return;
    window.__benchAudio = 0;
    class Counted extends Rtc {
      constructor(...a) {
        super(...a);
        this.addEventListener('track', (e) => {
          if (e.track.kind !== 'audio') return;
          window.__benchAudio++;
          e.track.addEventListener('ended', () => window.__benchAudio--);
        });
      }
    }
    Counted.__bench = true;
    window.RTCPeerConnection = Counted;
  })()`,
  // in the office: past the lobby (its "Enter Office" is pressed if it shows) and the map's controls are there
  office: `(() => {
    const name = ${label};
    const buttons = [...document.querySelectorAll('button, [role=button]')];
    const enter = buttons.find((b) => /^enter office/i.test(name(b)));
    if (enter) { enter.click(); return false; }
    return buttons.some((b) => name(b) === 'Center on me');
  })()`,
  // the people list's row for the person, and its "Go to". The name can show more than once (a label on the map, a
  // meeting tile), so every element with exactly that text is tried until one sits in a row with "Go to".
  join: (who) => `(() => {
    const name = ${label};
    const want = ${JSON.stringify(who)}.trim().toLowerCase();
    const leaves = [...document.querySelectorAll('body *')].filter((e) => e.children.length === 0 && e.textContent.trim().toLowerCase() === want);
    if (!leaves.length) return want + ' is not on the page';
    const seen = [];
    for (const leaf of leaves) {
      let row = leaf;
      while (row && row !== document.body && !row.querySelector('button, [role=button]')) row = row.parentElement;
      const buttons = [...(row?.querySelectorAll('button, [role=button]') ?? [])];
      const go = buttons.find((b) => name(b) === 'Go to');
      if (go) { go.click(); return 'ok'; }
      seen.push('[' + buttons.map(name).filter(Boolean).slice(0, 6).join(', ') + ']');
    }
    return want + ' has no "Go to" (offline?); found ' + leaves.length + ' time(s), next to ' + seen.join(' ');
  })()`,
  audio: `(() => window.__benchAudio ?? -1)()`,
  press: (re) => `(() => {
    const name = ${label};
    const b = [...document.querySelectorAll('button, [role=button]')].find((e) => ${re}.test(name(e)));
    if (!b) return 'not found';
    b.click();
    return 'pressed';
  })()`,
  mic: /^unmute mic/i,
  desk: /^go to my desk$/i,
  cam: /^turn camera on/i,
  probe: (who) => `(() => {
    const name = ${label};
    const want = ${JSON.stringify(who ?? '')}.trim().toLowerCase();
    const people = !want ? undefined : [...document.querySelectorAll('body *')]
      .filter((e) => e.textContent.trim().toLowerCase().includes(want) && ![...e.children].some((c) => c.textContent.trim().toLowerCase().includes(want)))
      .map((e) => {
        let row = e;
        while (row && row !== document.body && !row.querySelector('button, [role=button]')) row = row.parentElement;
        return {
          tag: e.tagName, text: e.textContent.trim().slice(0, 60), leaf: e.children.length === 0,
          rowButtons: [...(row?.querySelectorAll('button, [role=button]') ?? [])].map(name).map((s) => s.slice(0, 40)),
        };
      });
    return JSON.stringify({
      href: location.href,
      remoteAudio: window.__benchAudio,
      buttons: [...document.querySelectorAll('button, [role=button]')].map(name).filter(Boolean).map((s) => s.slice(0, 40)).slice(0, 120),
      people,
    }, null, 1);
  })()`,
};

// ---- measuring a systemd scope

function scopeCgroup(unit) {
  const cg = execFileSync('systemctl', ['--user', 'show', '-P', 'ControlGroup', `${unit}.scope`], { encoding: 'utf8' }).trim();
  return cg ? path.join('/sys/fs/cgroup', cg) : null;
}

function cpuUsec(cg) {
  const m = fs.readFileSync(path.join(cg, 'cpu.stat'), 'utf8').match(/usage_usec (\d+)/);
  return m ? Number(m[1]) : 0;
}

function pssKb(cg) {
  let total = 0, procs = 0;
  for (const pid of fs.readFileSync(path.join(cg, 'cgroup.procs'), 'utf8').split('\n').filter(Boolean)) {
    try {
      const m = fs.readFileSync(`/proc/${pid}/smaps_rollup`, 'utf8').match(/^Pss:\s+(\d+) kB/m);
      if (m) { total += Number(m[1]); procs++; }
    } catch {} // gone meanwhile
  }
  return { total, procs };
}

// Runs argv[1:] on a pseudo-terminal of 200x60 and reads its output until it exits (what a terminal emulator would
// draw; the emulator's own cost is not counted).
const PTY = `
import fcntl, os, pty, struct, sys, termios
pid, fd = pty.fork()
if pid == 0:
    fcntl.ioctl(0, termios.TIOCSWINSZ, struct.pack('HHHH', 60, 200, 0, 0))
    os.execv(sys.argv[1], sys.argv[1:])
while True:
    try:
        if not os.read(fd, 65536):
            break
    except OSError:
        break
os.waitpid(pid, 0)
`;

function launch(client, unit, bench) {
  const env = { ...process.env, SOWORKER_BENCH: bench.spec, SOWORKER_BENCH_EVENTS: bench.events };
  const cmd = {
    gui: [SOWORKER, '--gui'],
    // the terminal client needs a terminal: a pty (python3's), 200x60, whose output is read and dropped
    tui: ['python3', '-c', PTY, SOWORKER, '--tui'],
    chrome: ['google-chrome', ...chromeArgs(spaceUrl())],
  }[client];
  const child = spawn('systemd-run', ['--user', '--scope', '--quiet', '--collect', `--unit=${unit}`, '--', ...cmd], { env, stdio: 'ignore' });
  return child;
}

function stop(unit) {
  try { execFileSync('systemctl', ['--user', 'stop', `${unit}.scope`], { stdio: 'ignore' }); } catch {}
}

/// A failure `until` passes on at once instead of trying again.
class Fatal extends Error {}

/// Waits until `check()` is true (polled every second), or fails after `secs`.
async function until(what, secs, check) {
  const end = Date.now() + secs * 1000;
  while (Date.now() < end) {
    try { if (await check()) return; } catch (e) { if (e instanceof Fatal) throw e; } // else: not ready yet
    await sleep(1000);
  }
  throw new Error(`timed out waiting for ${what}`);
}

function events(file) {
  try { return fs.readFileSync(file, 'utf8').split('\n').filter(Boolean).map((l) => l.slice(l.indexOf(' ') + 1)); } catch { return []; }
}

/// One measured phase: `secs` of 1 Hz samples. CPU in % of one core.
async function measure(cg, secs, phase, out) {
  let [lastT, lastU] = [process.hrtime.bigint(), cpuUsec(cg)];
  const rows = [];
  for (let i = 0; i < secs; i++) {
    await sleep(1000);
    const [t, u] = [process.hrtime.bigint(), cpuUsec(cg)];
    const cpu = ((u - lastU) / (Number(t - lastT) / 1000)) * 100;
    [lastT, lastU] = [t, u];
    const mem = pssKb(cg);
    rows.push({ phase, t: i + 1, cpu, pss_mb: mem.total / 1024, procs: mem.procs });
  }
  out.push(...rows);
  const avg = (k) => rows.reduce((s, r) => s + r[k], 0) / rows.length;
  return { cpu: avg('cpu'), cpu_peak: Math.max(...rows.map((r) => r.cpu)), pss_mb: avg('pss_mb'), pss_peak_mb: Math.max(...rows.map((r) => r.pss_mb)), procs: Math.max(...rows.map((r) => r.procs)) };
}

async function runOnce(client, n, opts, dir, samples) {
  const unit = `soworker-bench-${client}-${n}-${process.pid}`;
  const evFile = path.join(dir, `${client}-${n}.events`);
  const goFile = path.join(dir, `${client}-${n}.go`);
  const spec = [opts.join && `join=${opts.join}`, 'mic', opts.cam && 'cam', `go=${goFile}`].filter(Boolean).join(',');
  log(`${client} #${n}: starting`);
  const t0 = Date.now();
  launch(client, unit, { spec, events: evFile });
  let web = null;
  try {
    let cg = null;
    await until('the scope', 15, () => (cg = scopeCgroup(unit)) && fs.existsSync(cg));
    web = client === 'chrome' ? await Page.open() : null;
    if (web) await web.watchCalls();
    await until('the office', 120, async () => (web ? await web.eval(WEB.office) : events(evFile).includes('office')));
    const startup = (Date.now() - t0) / 1000;
    log(`${client} #${n}: in the office after ${startup.toFixed(1)} s; settling 30 s`);
    // every run starts from our desk, not where the last client (same account) left the avatar: SoWorker walks there
    // by itself at start-up, before it says "office"; Chrome is sent there and waits to hear no call
    if (web) {
      log(`${client} #${n}: going to the desk ${await web.eval(WEB.press(WEB.desk))}`);
      await sleep(3000);
      await until('the desk (no call audio)', 60, async () => (await web.eval(WEB.audio)) === 0)
        .catch(() => log(`${client} #${n}: still hearing a call at the desk; measuring anyway`));
    }
    await sleep(30000);
    const rows = [];
    const idle = await measure(cg, opts.idle, 'idle', rows);
    log(`${client} #${n}: idle ${fmt(idle)}${opts.join ? `; joining ${opts.join}` : ''}`);
    for (const r of rows.splice(0)) samples.push({ client, run: n, ...r });
    if (!opts.join) return { client, run: n, startup_s: startup, idle, call: null };
    const call = await callPhase(client, n, opts, { web, cg, goFile, evFile }, rows).catch((e) => {
      log(`${client} #${n}: no call measured: ${e.message}`);
      return null;
    });
    for (const r of rows) samples.push({ client, run: n, ...r });
    return { client, run: n, startup_s: startup, idle, call };
  } finally {
    web?.close();
    stop(unit);
    await sleep(3000); // let the session drop before the next client signs in
  }
}

/// Walk into the person's call, turn the microphone on, measure.
async function callPhase(client, n, opts, { web, cg, goFile, evFile }, rows) {
  {
    let heard = 0, went = 0;
    if (web) {
      heard = await web.eval(WEB.audio);
      const r = await web.eval(WEB.join(opts.join));
      if (r !== 'ok') throw new Error(`could not walk to them: ${r}`);
      went = Date.now();
    } else {
      fs.writeFileSync(goFile, '');
    }
    await until(`the call with ${opts.join}`, 90, async () => {
      // the previous client (same account) can leave the avatar in their call, so Chrome opens already in it: then
      // the audio it hears before "Go to" counts once it is still there 5 s after
      if (web) { const now = await web.eval(WEB.audio); return now > heard || (heard > 0 && now > 0 && Date.now() - went > 5000); }
      const ev = events(evFile);
      const missing = ev.find((e) => e.startsWith('waiting:'));
      if (missing) throw new Fatal(`could not walk to them: ${missing.slice(9)}`);
      return ev.includes('joined');
    }).catch(async (e) => {
      if (!web) throw e;
      const buttons = JSON.parse(await web.eval(WEB.probe())).buttons.filter((b) => /mic|camera|meeting|leave/i.test(b));
      throw new Error(`${e.message} (remote audio tracks ${heard} before "Go to", ${await web.eval(WEB.audio)} now; meeting buttons: ${buttons.join(', ') || 'none'})`);
    });
    if (web) {
      await sleep(2000);
      log(`${client} #${n}: mic ${await web.eval(WEB.press(WEB.mic))}` + (opts.cam ? `, camera ${await web.eval(WEB.press(WEB.cam))}` : ''));
    }
    log(`${client} #${n}: in the call; settling 15 s`);
    await sleep(15000);
    const call = await measure(cg, opts.call, 'call', rows);
    log(`${client} #${n}: call ${fmt(call)}`);
    return call;
  }
}

const fmt = (p) => `CPU ${p.cpu.toFixed(1)}% (peak ${p.cpu_peak.toFixed(1)}%), memory ${p.pss_mb.toFixed(0)} MB (peak ${p.pss_peak_mb.toFixed(0)}, ${p.procs} processes)`;
const log = (s) => console.log(`[${new Date().toLocaleTimeString()}] ${s}`);
const median = (xs) => { const s = [...xs].sort((a, b) => a - b); return s.length % 2 ? s[(s.length - 1) / 2] : (s[s.length / 2 - 1] + s[s.length / 2]) / 2; };

function summary(results, opts, date = new Date().toISOString().slice(0, 10)) {
  const name = { gui: 'SoWorker (window)', tui: 'SoWorker (terminal)', chrome: 'SoWork in Chrome' };
  const cpu = execFileSync('sh', ['-c', "grep -m1 'model name' /proc/cpuinfo | cut -d: -f2"], { encoding: 'utf8' }).trim();
  const chrome = (() => { try { return execFileSync('google-chrome', ['--version'], { encoding: 'utf8' }).trim(); } catch { return 'Chrome'; } })();
  const version = `SoWorker ${fs.readFileSync(path.join(ROOT, 'Cargo.toml'), 'utf8').match(/^version = "(.*)"/m)?.[1] ?? ''}`;
  // the call columns stay when no call was measured (no --join): "—" there says so
  const lines = [`| | Idle CPU | Idle memory | Call CPU (--join) | Call memory (--join) |`, `|---|---:|---:|---:|---:|`];
  for (const c of [...new Set(results.map((r) => r.client))]) {
    const rs = results.filter((r) => r.client === c);
    const m = (f, digits, unit) => {
      const xs = rs.map(f).filter((x) => x !== undefined);
      return xs.length ? `${median(xs).toFixed(digits)}${unit}` : '—';
    };
    lines.push(`| ${name[c]} | ${m((r) => r.idle.cpu, 1, '%')} | ${m((r) => r.idle.pss_mb, 0, ' MB')} | ${m((r) => r.call?.cpu, 1, '%')} | ${m((r) => r.call?.pss_mb, 0, ' MB')} |`);
  }
  // the README's format: the machine and versions as a list under the table
  const note = [`Tested in *${date}* with ${opts.runs} run(s) on:`, `- ${cpu}`, `- ${os.type()} ${os.release()}`, `- ${version}`, `- ${chrome}`].join('\n');
  // for the terminal and summary.md only: what was measured
  const scenario = `Median of ${opts.runs} run(s): ${opts.idle} s in the office, then ${results.some((r) => r.call) ? `${opts.call} s in a call with the microphone on${opts.cam ? ' and the camera on' : ''}` : opts.join ? `no call (${opts.join} could not be reached)` : 'no call (run without --join)'}. ` +
    `CPU in % of one core; memory is the PSS of every process the client starts.`;
  return { table: lines.join('\n'), note, scenario };
}

function writeReadme(s) {
  const f = path.join(ROOT, 'README.md');
  const text = fs.readFileSync(f, 'utf8');
  const block = `<!-- bench:start -->\n${s.table}\n\n${s.note}\n<!-- bench:end -->`;
  const re = /<!-- bench:start -->[\s\S]*?<!-- bench:end -->/;
  if (!re.test(text)) throw new Error('README.md has no <!-- bench:start --> / <!-- bench:end --> markers');
  fs.writeFileSync(f, text.replace(re, block));
  log('README.md updated');
}

function preflight(clients) {
  const sw = running('target/(release|debug)/soworker|/soworker( |$)').split('\n').filter((l) => l && !l.includes('bench.mjs'));
  if (sw.length) throw new Error(`close SoWorker first (it shares the account):\n${sw.join('\n')}`);
  if (running(PROFILE)) throw new Error('the benchmark Chrome is still open: close it first');
}

const SESSION = path.join(os.homedir(), '.config/soworker/session.json');
const exited = (child) => new Promise((r) => (child.exitCode !== null ? r() : child.once('exit', r)));

/// SoWorker's own sign-in: without a saved session, starts the window once (it signs in through Chrome) and closes it
/// when the session is saved.
async function ensureSoWorkerSignedIn() {
  if (fs.existsSync(SESSION) && fs.existsSync(path.join(path.dirname(SESSION), '.space.json'))) return;
  log('SoWorker is not signed in: sign in in the browser it opens; the benchmark goes on by itself after');
  const child = spawn(SOWORKER, ['--gui'], { stdio: 'ignore' });
  try {
    await until('the SoWorker sign-in', 900, () => fs.existsSync(SESSION) && fs.existsSync(path.join(path.dirname(SESSION), '.space.json')));
    await sleep(3000);
  } finally {
    child.kill();
    await exited(child);
  }
}

/// The benchmark Chrome's sign-in: opens the office in its profile; when that ends somewhere other than the office,
/// waits for the person to sign in there, then closes Chrome.
async function ensureChromeSignedIn() {
  fs.mkdirSync(PROFILE, { recursive: true });
  const child = spawn('google-chrome', chromeArgs(spaceUrl()), { stdio: 'ignore' });
  const inOffice = async () => {
    const page = await Page.open(5000);
    try { return await page.eval(WEB.office); } finally { page.close(); }
  };
  try {
    try {
      await until('the office', 45, inOffice);
      log('benchmark Chrome is signed in');
    } catch {
      log('benchmark Chrome is not signed in: sign in to SoWork in its window; the benchmark goes on by itself after');
      await until('the Chrome sign-in', 900, inOffice);
      log('signed in');
    }
  } finally {
    child.kill();
    await exited(child);
    await until('the benchmark Chrome to close', 30, () => !running(PROFILE));
  }
}

const running = (pat) => { try { return execFileSync('pgrep', ['-af', pat], { encoding: 'utf8' }).trim(); } catch { return ''; } };

async function main() {
  const cmd = process.argv[2];
  if (cmd === 'login') {
    fs.mkdirSync(PROFILE, { recursive: true });
    log('Sign in to SoWork in the Chrome window, then close it. The benchmark reuses this profile.');
    spawn('google-chrome', chromeArgs(spaceUrl()), { stdio: 'ignore' }).on('exit', () => log('profile saved'));
    return;
  }
  if (cmd === 'probe') {
    preflight(['chrome']);
    const child = spawn('google-chrome', chromeArgs(spaceUrl()), { stdio: 'ignore' });
    try {
      const page = await Page.open();
      await page.watchCalls();
      await until('the office', 120, () => page.eval(WEB.office));
      await sleep(5000);
      console.log(await page.eval(WEB.probe(arg('join', process.env.SOWORKER_BENCH_JOIN))));
      page.close();
    } finally {
      child.kill();
    }
    return;
  }
  if (cmd === 'readme') {
    const base = path.join(ROOT, 'bench/results');
    const dir = process.argv[3] ? path.resolve(process.argv[3]) : path.join(base, fs.readdirSync(base).sort().at(-1) ?? '');
    const { opts, results } = JSON.parse(fs.readFileSync(path.join(dir, 'results.json'), 'utf8'));
    const date = path.basename(dir).slice(0, 10);
    writeReadme(summary(results, opts, date));
    return;
  }
  if (cmd !== 'run') {
    console.log(fs.readFileSync(new URL(import.meta.url), 'utf8').split('\n').filter((l) => l.startsWith('//')).map((l) => l.slice(3)).join('\n'));
    return;
  }
  const opts = {
    clients: String(arg('clients', 'gui,tui,chrome')).split(','),
    runs: Number(arg('runs', 3)),
    idle: Number(arg('idle', 120)),
    call: Number(arg('call', 120)),
    join: arg('join', process.env.SOWORKER_BENCH_JOIN),
    cam: arg('cam', false) === true,
  };
  if (typeof opts.join !== 'string' || !opts.join.trim()) {
    opts.join = null;
    log('no --join NAME (or SOWORKER_BENCH_JOIN): measuring the office only, no call');
  }
  preflight(opts.clients);
  if (opts.clients.some((c) => c !== 'chrome')) {
    log('building SoWorker (release)');
    // `make build`: the release binary with the Makefile's environment (not `make release`, which bumps and tags)
    execFileSync('make', ['build'], { cwd: ROOT, stdio: 'inherit' });
  }
  await ensureSoWorkerSignedIn();
  if (opts.clients.includes('chrome')) await ensureChromeSignedIn();
  const dir = path.join(ROOT, 'bench/results', new Date().toISOString().slice(0, 19).replace(/[:T]/g, '-'));
  fs.mkdirSync(dir, { recursive: true });
  const results = [], samples = [];
  for (let n = 1; n <= opts.runs; n++) {
    for (const c of opts.clients) {
      try {
        results.push(await runOnce(c, n, opts, dir, samples));
      } catch (e) {
        log(`${c} #${n} failed: ${e.message}`);
      }
      fs.writeFileSync(path.join(dir, 'results.json'), JSON.stringify({ opts, results }, null, 1));
      fs.writeFileSync(path.join(dir, 'samples.csv'), ['client,run,phase,t,cpu_pct,pss_mb,procs', ...samples.map((s) => [s.client, s.run, s.phase, s.t, s.cpu.toFixed(2), s.pss_mb.toFixed(1), s.procs].join(','))].join('\n') + '\n');
    }
  }
  if (!results.length) throw new Error('no run finished');
  const s = summary(results, opts);
  fs.writeFileSync(path.join(dir, 'summary.md'), `${s.table}\n\n${s.note}\n\n${s.scenario}\n`);
  console.log(`\n${s.table}\n\n${s.note}\n\n${s.scenario}\n\nSaved in ${path.relative(ROOT, dir)}/`);
  if (arg('readme', false) === true) writeReadme(s);
}

main().catch((e) => { console.error(`bench: ${e.message}`); process.exit(1); });
