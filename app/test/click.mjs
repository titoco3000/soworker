// Headless Chrome test of click-to-move against the offline bridge: node app/test/click.mjs <port> <targetDx> <targetDy>
import { spawn } from 'node:child_process';
import { readFileSync, writeFileSync, rmSync, existsSync } from 'node:fs';
const port = process.argv[2] ?? 4830, dx = Number(process.argv[3] ?? 0), dy = Number(process.argv[4] ?? 0);
const prof = '/tmp/claude-1000/clickprof'; rmSync(prof, { recursive: true, force: true });
const c = spawn('google-chrome-stable', ['--headless=new', '--no-sandbox', '--disable-gpu', '--remote-debugging-port=0', `--user-data-dir=${prof}`, 'about:blank'], { stdio: 'ignore' });
for (let i = 0; i < 100 && !existsSync(prof + '/DevToolsActivePort'); i++) await new Promise((r) => setTimeout(r, 100));
const [cdp] = readFileSync(prof + '/DevToolsActivePort', 'utf8').split('\n');
const t = (await (await fetch(`http://127.0.0.1:${cdp}/json`)).json()).find((x) => x.type === 'page');
const ws = new WebSocket(t.webSocketDebuggerUrl); await new Promise((r) => (ws.onopen = r));
let id = 0; const pend = new Map();
ws.onmessage = (m) => { const d = JSON.parse(m.data); pend.get(d.id)?.(d.result); };
const send = (method, params = {}) => new Promise((res) => { pend.set(++id, res); ws.send(JSON.stringify({ id, method, params })); });
const ev = async (e) => (await send('Runtime.evaluate', { expression: e, returnByValue: true })).result.value;
await send('Emulation.setDeviceMetricsOverride', { width: 1000, height: 700, deviceScaleFactor: 1, mobile: false });
await send('Page.navigate', { url: `http://127.0.0.1:${port}/` });
await new Promise((r) => setTimeout(r, 1500));
const pos = () => ev('JSON.stringify({x:Math.round(__test.me.x),y:Math.round(__test.me.y),state:__test.me.state})');
console.log('start', await pos());
const scale = 0.35;   // click dx,dy world units away from the avatar (avatar is centred)
await send('Input.dispatchMouseEvent', { type: 'mousePressed', x: 500 + dx * scale, y: 350 + dy * scale, button: 'left', clickCount: 1, modifiers: process.env.SHIFT ? 8 : 0 });
await send('Input.dispatchMouseEvent', { type: 'mouseReleased', x: 500 + dx * scale, y: 350 + dy * scale, button: 'left', clickCount: 1, modifiers: process.env.SHIFT ? 8 : 0 });
for (let i = 0; i < 6; i++) { await new Promise((r) => setTimeout(r, 1000)); console.log(`t+${i + 1}s`, await pos()); }
writeFileSync(process.argv[5] ?? '/tmp/claude-1000/click.png', Buffer.from((await send('Page.captureScreenshot', { format: 'png' })).data, 'base64'));
console.log('debug panel:\n' + await ev('document.getElementById("dbg").textContent'));
c.kill(); process.exit(0);
