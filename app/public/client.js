// SoWorker canvas client: top-down office, WASD / arrows to walk. All state arrives from the local bridge over SSE.
const G = 105.46875, T = 2 * G;           // tile pitch (world units)
const SPEED = 800;                        // world units per second, same as the official client
const TICK_MS = 205;                      // official client sends `move` about every 205 ms while walking
const MARGIN = 28;                        // keep this far from a walled tile edge
const N = 1, E = 2, S = 4, W = 8;

const cv = document.getElementById('c'), ctx = cv.getContext('2d'), hud = document.getElementById('hud');
const params = new URLSearchParams(location.search);
let world = { floors: new Set(), walls: new Map(), zones: [], assets: [] };
const users = new Map();                  // id -> {id,name,x,y,tx,ty,state}
let meId = null, room = '', dry = false, status = 'connecting…';
let scale = 0.35;
const keys = new Set();
let moving = false, lastSend = 0, lastSent = null;

// ---- network
function act(a) { fetch('/act', { method: 'POST', body: JSON.stringify(a), keepalive: true }).catch(() => {}); }
const es = new EventSource('/events');
es.onopen = () => (status = '');
es.onerror = () => (status = 'bridge offline');
es.onmessage = (m) => {
  const e = JSON.parse(m.data);
  if (e.t === 'world') {
    meId = e.me; room = e.room; dry = e.dry; status = '';
    world.floors = new Set(e.floors.map(([i, j]) => `${i},${j}`));
    world.walls = new Map(e.walls.map(([i, j, m]) => [`${i},${j}`, m]));
    world.zones = e.zones; world.assets = e.assets;
    const seen = new Set();
    for (const u of e.users) {
      seen.add(u.id);
      const old = users.get(u.id);
      if (old && u.id === meId) continue;       // keep local position authoritative for ourselves
      users.set(u.id, { ...u, tx: u.x, ty: u.y });
    }
    for (const id of [...users.keys()]) if (!seen.has(id)) users.delete(id);
  } else if (e.t === 'move') { const u = users.get(e.id); if (u) { u.tx = e.x; u.ty = e.y; if (Math.hypot(u.tx - u.x, u.ty - u.y) > 1000) { u.x = u.tx; u.y = u.ty; } } }
  else if (e.t === 'state') { const u = users.get(e.id); if (u) u.state = e.state; }
  else if (e.t === 'join') users.set(e.user.id, { ...e.user, tx: e.user.x, ty: e.user.y });
  else if (e.t === 'leave') users.delete(e.id);
  else if (e.t === 'status') status = e.text;
};

// ---- collision: floors are walkable, walls are tile-edge segments
const wall = (i, j) => world.walls.get(`${i},${j}`) ?? 0;
function blocked(x, y) {
  const i = Math.floor(x / T), j = Math.floor(y / T);
  if (!world.floors.has(`${i},${j}`)) return true;
  const x0 = i * T, y0 = j * T;
  const here = wall(i, j);
  if (((here & N) || (wall(i, j - 1) & S)) && y < y0 + MARGIN) return true;
  if (((here & S) || (wall(i, j + 1) & N)) && y > y0 + T - MARGIN) return true;
  if (((here & W) || (wall(i - 1, j) & E)) && x < x0 + MARGIN) return true;
  if (((here & E) || (wall(i + 1, j) & W)) && x > x0 + T - MARGIN) return true;
  return false;
}
function step(me, dx, dy) {                // axis-separated sliding in small sub-steps
  const len = Math.hypot(dx, dy), n = Math.max(1, Math.ceil(len / 8));
  for (let k = 0; k < n; k++) {
    if (!blocked(me.x + dx / n, me.y)) me.x += dx / n;
    if (!blocked(me.x, me.y + dy / n)) me.y += dy / n;
  }
}

// ---- input
const KEYMAP = { w: 'u', arrowup: 'u', s: 'd', arrowdown: 'd', a: 'l', arrowleft: 'l', d: 'r', arrowright: 'r' };
addEventListener('keydown', (e) => { const k = KEYMAP[e.key.toLowerCase()]; if (k) { keys.add(k); e.preventDefault(); } });
addEventListener('keyup', (e) => { const k = KEYMAP[e.key.toLowerCase()]; if (k) keys.delete(k); });
addEventListener('blur', () => keys.clear());
addEventListener('wheel', (e) => { scale = Math.min(1.5, Math.max(0.08, scale * (e.deltaY < 0 ? 1.1 : 1 / 1.1))); }, { passive: true });

// scripted input for tests: ?script=r:1200,d:600  (hold key, milliseconds)
if (params.get('script')) {
  let t = 1500;
  for (const part of params.get('script').split(',')) {
    const [k, ms] = part.split(':');
    setTimeout(() => keys.add(k), t); t += Number(ms);
    setTimeout(() => keys.delete(k), t); t += 100;
  }
}

// ---- frame loop
let prev = performance.now();
function frame(now) {
  const dt = Math.min(0.05, (now - prev) / 1000); prev = now;
  const me = users.get(meId);
  if (me) {
    const dx = (keys.has('r') ? 1 : 0) - (keys.has('l') ? 1 : 0), dy = (keys.has('d') ? 1 : 0) - (keys.has('u') ? 1 : 0);
    const want = dx !== 0 || dy !== 0;
    if (want) { const l = Math.hypot(dx, dy); step(me, (dx / l) * SPEED * dt, (dy / l) * SPEED * dt); me.state = 'walk'; }
    if (want !== moving) {                      // walk/idle flip: state + an immediate position, like the official client
      moving = want; me.state = want ? 'walk' : 'idle';
      act({ ev: 'move', x: me.x, y: me.y }); act({ ev: 'state', state: me.state }); lastSend = now; lastSent = [me.x, me.y];
    } else if (moving && now - lastSend >= TICK_MS) { act({ ev: 'move', x: me.x, y: me.y }); lastSend = now; lastSent = [me.x, me.y]; }
    me.tx = me.x; me.ty = me.y;
  }
  for (const u of users.values()) {            // glide remote avatars toward their last reported position
    if (u.id === meId) continue;
    const d = Math.hypot(u.tx - u.x, u.ty - u.y);
    if (d > 0.5) { const k = Math.min(1, (SPEED * 1.2 * dt) / d); u.x += (u.tx - u.x) * k; u.y += (u.ty - u.y) * k; }
  }
  draw(me);
  requestAnimationFrame(frame);
}

// ---- render
const hue = (id) => { let h = 0; for (const c of id) h = (h * 31 + c.charCodeAt(0)) % 360; return h; };
function draw(me) {
  const dpr = devicePixelRatio || 1, w = cv.clientWidth, h = cv.clientHeight;
  if (cv.width !== w * dpr || cv.height !== h * dpr) { cv.width = w * dpr; cv.height = h * dpr; }
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
  ctx.fillStyle = '#14161b'; ctx.fillRect(0, 0, w, h);
  const cx = me ? me.x : 3000, cy = me ? me.y : 3000;
  const X = (x) => (x - cx) * scale + w / 2, Y = (y) => (y - cy) * scale + h / 2;
  const i0 = Math.floor((cx - w / 2 / scale) / T), i1 = Math.floor((cx + w / 2 / scale) / T);
  const j0 = Math.floor((cy - h / 2 / scale) / T), j1 = Math.floor((cy + h / 2 / scale) / T);
  const px = T * scale;
  ctx.fillStyle = '#2b3a52';
  for (let j = j0; j <= j1; j++) for (let i = i0; i <= i1; i++) if (world.floors.has(`${i},${j}`)) ctx.fillRect(X(i * T) + 0.5, Y(j * T) + 0.5, px - 1, px - 1);
  ctx.strokeStyle = '#e8c36a'; ctx.lineWidth = Math.max(2, 5 * scale); ctx.beginPath();
  for (let j = j0; j <= j1; j++) for (let i = i0; i <= i1; i++) {
    const m = wall(i, j); if (!m) continue;
    const x = X(i * T), y = Y(j * T);
    if (m & N) { ctx.moveTo(x, y); ctx.lineTo(x + px, y); }
    if (m & S) { ctx.moveTo(x, y + px); ctx.lineTo(x + px, y + px); }
    if (m & W) { ctx.moveTo(x, y); ctx.lineTo(x, y + px); }
    if (m & E) { ctx.moveTo(x + px, y); ctx.lineTo(x + px, y + px); }
  }
  ctx.stroke();
  ctx.fillStyle = '#4a5a6e';
  for (const [ax, ay] of world.assets) { const sx = X(ax), sy = Y(ay); if (sx > -5 && sx < w + 5 && sy > -5 && sy < h + 5) ctx.fillRect(sx - 1, sy - 1, 3, 3); }
  ctx.font = '11px system-ui'; ctx.textAlign = 'center';
  for (const z of world.zones) {
    ctx.strokeStyle = z.kind === 'meeting' ? '#d9577f' : '#667'; ctx.lineWidth = 1;
    ctx.strokeRect(X(z.x - z.sx / 2), Y(z.y - z.sy / 2), z.sx * scale, z.sy * scale);
    if (z.name) { ctx.fillStyle = '#d9577f'; ctx.fillText(z.name, X(z.x), Y(z.y - z.sy / 2) - 3); }
  }
  for (const u of users.values()) {
    const sx = X(u.x), sy = Y(u.y), r = Math.max(6, 40 * scale);
    ctx.fillStyle = `hsl(${hue(u.id)} 60% 55%)`;
    ctx.beginPath(); ctx.arc(sx, sy, r, 0, 7); ctx.fill();
    if (u.id === meId) { ctx.strokeStyle = '#fff'; ctx.lineWidth = 2; ctx.stroke(); }
    ctx.fillStyle = '#dde'; ctx.fillText(u.name + (u.state === 'walk' ? ' ·' : ''), sx, sy - r - 4);
  }
  hud.textContent = `${room}${dry ? '  [dry run]' : ''}\n${users.size} people${status ? '  ' + status : ''}` + (me ? `\n${Math.round(me.x)}, ${Math.round(me.y)}` : '');
}
requestAnimationFrame(frame);
window.__test = { users, world, get me() { return users.get(meId); } };
