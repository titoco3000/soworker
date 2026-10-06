// SoWorker canvas client: top-down office, WASD / arrows to walk. All state arrives from the local bridge over SSE.
const G = 105.46875, T = 2 * G;           // tile pitch (world units)
const SPEED = 800;                        // world units per second, same as the official client
const TICK_MS = 205;                      // official client sends `move` about every 205 ms while walking
const MARGIN = 28;                        // keep this far from a walled tile edge
const N = 1, E = 2, S = 4, W = 8;
const ZK = 1.1;                           // meeting zone half-extent = ZK x stored size, centred on x,y (fitted on the recording: 98.8% of samples)

const cv = document.getElementById('c'), ctx = cv.getContext('2d'), hud = document.getElementById('hud'), dbg = document.getElementById('dbg');
const params = new URLSearchParams(location.search);
let world = { floors: new Set(), walls: new Map(), soft: new Map(), zones: [], assets: [] };
const users = new Map();                  // id -> {id,name,x,y,tx,ty,state}
let meId = null, room = '', dry = false, status = 'connecting…';
let scale = 0.35, spawn = null;
const keys = new Set();
let moving = false, lastSend = 0, lastSent = null, dirty = true;
let showDebug = params.get('debug') !== '0', stats = null, fps = 0, frames = 0, fpsT = performance.now();

// ---- network
function act(a) { fetch('/act', { method: 'POST', body: JSON.stringify(a), keepalive: true }).catch(() => {}); }
const es = new EventSource('/events');
es.onopen = () => (status = '');
es.onerror = () => (status = 'bridge offline');
es.onmessage = (m) => {
  const e = JSON.parse(m.data);
  if (e.t !== 'stats' || showDebug) dirty = true;   // stats ticks only matter while the panel is visible
  if (e.t === 'world') {
    meId = e.me; room = e.room; dry = e.dry; status = ''; spawn = e.spawn;
    if (typeof onCluster === 'function') onCluster(e.cluster ?? null);
    world.floors = new Set(e.floors.map(([i, j]) => `${i},${j}`));
    world.walls = new Map(e.walls.map(([i, j, m]) => [`${i},${j}`, m]));
    world.soft = new Map(e.soft.map(([i, j, m]) => [`${i},${j}`, m]));
    world.zones = e.zones; world.assets = e.assets;
    const seen = new Set();
    for (const u of e.users) {
      seen.add(u.id);
      const old = users.get(u.id);
      if (old && u.id === meId) continue;       // keep local position authoritative for ourselves
      users.set(u.id, { ...u, tx: u.x, ty: u.y });
    }
    for (const id of [...users.keys()]) if (!seen.has(id)) users.delete(id);
    const me = users.get(meId);
    if (me && blocked(me.x, me.y)) respawn(me);   // saved position is off the map (or inside a wall)
  } else if (e.t === 'move') { const u = users.get(e.id); if (u) { u.tx = e.x; u.ty = e.y; if (Math.hypot(u.tx - u.x, u.ty - u.y) > 1000) { u.x = u.tx; u.y = u.ty; } } }
  else if (e.t === 'state') { const u = users.get(e.id); if (u) u.state = e.state; }
  else if (e.t === 'join') users.set(e.user.id, { ...e.user, tx: e.user.x, ty: e.user.y });
  else if (e.t === 'leave') users.delete(e.id);
  else if (e.t === 'status') status = e.text;
  else if (e.t === 'cluster') onCluster(e.cluster);
  else if (e.t === 'speaking') { const u = users.get(e.id); if (u) u.speaking = e.on; }
  else if (e.t === 'stats') stats = e;
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

// teleport to the room's spawn point (also bound to R)
function respawn(me) {
  if (!spawn) return;
  me.x = spawn.x; me.y = spawn.y; me.tx = me.x; me.ty = me.y; path = null;
  act({ ev: 'move', x: me.x, y: me.y });
}

// ---- click to move: A* over tiles (walls and missing floors block), then string-pulled waypoints
let path = null, pathGoal = null, stuckMs = 0;
const DIRS = [[0, -1, N, S], [1, 0, E, W], [0, 1, S, N], [-1, 0, W, E]];
const canStep = (i, j, di, dj) => {
  const ni = i + di, nj = j + dj;
  if (!world.floors.has(`${ni},${nj}`)) return false;
  for (const [dx, dy, out, back] of DIRS) if (dx === di && dy === dj) return !(wall(i, j) & out) && !(wall(ni, nj) & back);
  return true;
};
function tilePath(si, sj, ti, tj) {
  const key = (i, j) => i * 100000 + j, open = [[0, si, sj]], g = new Map([[key(si, sj), 0]]), from = new Map();
  const h = (i, j) => Math.hypot(i - ti, j - tj);
  for (let guard = 0; open.length && guard < 20000; guard++) {
    open.sort((a, b) => a[0] - b[0]);
    const [, i, j] = open.shift();
    if (i === ti && j === tj) { const out = [[i, j]]; let k = key(i, j); while (from.has(k)) { const [pi, pj] = from.get(k); out.push([pi, pj]); k = key(pi, pj); } return out.reverse(); }
    for (let di = -1; di <= 1; di++) for (let dj = -1; dj <= 1; dj++) {
      if (!di && !dj) continue;
      if (!canStep(i, j, di, dj)) continue;
      if (di && dj && !(canStep(i, j, di, 0) && canStep(i, j, 0, dj))) continue;   // no corner cutting
      const ng = g.get(key(i, j)) + Math.hypot(di, dj), nk = key(i + di, j + dj);
      if (ng < (g.get(nk) ?? Infinity)) { g.set(nk, ng); from.set(nk, [i, j]); open.push([ng + h(i + di, j + dj), i + di, j + dj]); }
    }
  }
  return null;
}
const clear = (ax, ay, bx, by) => {          // straight line free of walls and void
  const n = Math.ceil(Math.hypot(bx - ax, by - ay) / 10);
  for (let k = 1; k <= n; k++) if (blocked(ax + ((bx - ax) * k) / n, ay + ((by - ay) * k) / n)) return false;
  return true;
};
function startPath(me, gx, gy) {
  let ti = Math.floor(gx / T), tj = Math.floor(gy / T);
  if (!world.floors.has(`${ti},${tj}`)) return;
  if (blocked(gx, gy)) { gx = (ti + 0.5) * T; gy = (tj + 0.5) * T; }
  const tiles = tilePath(Math.floor(me.x / T), Math.floor(me.y / T), ti, tj);
  if (!tiles) return;
  const pts = tiles.slice(1, -1).map(([i, j]) => [(i + 0.5) * T, (j + 0.5) * T]);
  pts.push([gx, gy]);
  const out = [];
  let ax = me.x, ay = me.y, idx = 0;          // string pulling: jump to the farthest waypoint we can see
  while (idx < pts.length) {
    let far = idx;
    for (let k = pts.length - 1; k > idx; k--) if (clear(ax, ay, pts[k][0], pts[k][1])) { far = k; break; }
    out.push(pts[far]); [ax, ay] = pts[far]; idx = far + 1;
  }
  path = out; pathGoal = [gx, gy]; stuckMs = 0;
}
function teleport(me, gx, gy) {             // the server does not validate positions, so this works anywhere on the floor
  if (!world.floors.has(`${Math.floor(gx / T)},${Math.floor(gy / T)}`)) return;
  if (blocked(gx, gy)) { gx = (Math.floor(gx / T) + 0.5) * T; gy = (Math.floor(gy / T) + 0.5) * T; }
  me.x = gx; me.y = gy; me.tx = gx; me.ty = gy; path = null;
  act({ ev: 'move', x: gx, y: gy });
}
let toast = '', toastUntil = 0;
cv.addEventListener('pointerdown', (e) => {
  dirty = true;
  const me = users.get(meId); if (!me || e.button !== 0) return;
  const r = cv.getBoundingClientRect();
  const gx = me.x + (e.clientX - r.left - r.width / 2) / scale, gy = me.y + (e.clientY - r.top - r.height / 2) / scale;
  if (e.shiftKey) return teleport(me, gx, gy);
  startPath(me, gx, gy);
  if (!path && world.floors.has(`${Math.floor(gx / T)},${Math.floor(gy / T)}`)) { toast = 'no walking route: Shift+click to teleport'; toastUntil = performance.now() + 3000; setTimeout(() => (dirty = true), 3100); }
});

// ---- input
const KEYMAP = { w: 'u', arrowup: 'u', s: 'd', arrowdown: 'd', a: 'l', arrowleft: 'l', d: 'r', arrowright: 'r' };
addEventListener('keydown', (e) => {
  dirty = true;
  if (e.key.toLowerCase() === 'r' && users.get(meId)) { respawn(users.get(meId)); return; }
  if (e.key === 'F3') { showDebug = !showDebug; e.preventDefault(); return; }
  const k = KEYMAP[e.key.toLowerCase()];
  if (k) { keys.add(k); path = null; e.preventDefault(); }
});
addEventListener('keyup', (e) => { const k = KEYMAP[e.key.toLowerCase()]; if (k) keys.delete(k); });
addEventListener('blur', () => keys.clear());
addEventListener('resize', () => (dirty = true));
addEventListener('wheel', (e) => {
  dirty = true; scale = Math.min(1.5, Math.max(0.08, scale * (e.deltaY < 0 ? 1.1 : 1 / 1.1))); }, { passive: true });

// scripted input for tests: ?script=r:1200,d:600  (hold key, milliseconds)
if (params.get('script')) {
  let t = 1500;
  for (const part of params.get('script').split(',')) {
    const [k, ms] = part.split(':');
    setTimeout(() => keys.add(k), t); t += Number(ms);
    setTimeout(() => keys.delete(k), t); t += 100;
  }
}

// ---- meeting zones: the client decides which zone it stands in and tells the server (which answers with a `cluster`)
let curZone;                              // undefined = not reported yet
function zoneAt(x, y) {
  let best = null, bd = Infinity;
  for (const z of world.zones) {
    if (z.kind !== 'meeting') continue;
    const dx = Math.abs(x - z.x) / (z.sx * ZK), dy = Math.abs(y - z.y) / (z.sy * ZK);
    if (dx <= 1 && dy <= 1 && Math.max(dx, dy) < bd) { best = z.id; bd = Math.max(dx, dy); }
  }
  return best;
}
function updateZone(me) {
  const z = zoneAt(me.x, me.y);
  if (z === (curZone ?? null) && curZone !== undefined) return;
  const first = curZone === undefined;
  curZone = z;
  if (!first || z) act({ ev: 'zone', zoneId: z });
}

// ---- frame loop
let prev = performance.now();
function frame(now) {
  const dt = Math.min(0.05, (now - prev) / 1000); prev = now;
  const me = users.get(meId);
  if (me) {
    let dx = (keys.has('r') ? 1 : 0) - (keys.has('l') ? 1 : 0), dy = (keys.has('d') ? 1 : 0) - (keys.has('u') ? 1 : 0);
    if (!dx && !dy && path) {                   // follow the click-to-move path
      while (path.length && Math.hypot(path[0][0] - me.x, path[0][1] - me.y) < 12) path.shift();
      if (!path.length) path = null;
      else { dx = path[0][0] - me.x; dy = path[0][1] - me.y; }
    }
    const want = dx !== 0 || dy !== 0;
    if (want) {
      const l = Math.hypot(dx, dy), bx = me.x, by = me.y;
      step(me, (dx / l) * SPEED * dt, (dy / l) * SPEED * dt); me.state = 'walk';
      if (path) { stuckMs = Math.hypot(me.x - bx, me.y - by) < SPEED * dt * 0.2 ? stuckMs + dt * 1000 : 0; if (stuckMs > 400) path = null; }
    }
    if (want !== moving) {                      // walk/idle flip: state + an immediate position, like the official client
      moving = want; me.state = want ? 'walk' : 'idle';
      act({ ev: 'move', x: me.x, y: me.y }); act({ ev: 'state', state: me.state }); lastSend = now; lastSent = [me.x, me.y];
    } else if (moving && now - lastSend >= TICK_MS) { act({ ev: 'move', x: me.x, y: me.y }); lastSend = now; lastSent = [me.x, me.y]; }
    me.tx = me.x; me.ty = me.y;
    updateZone(me);
  }
  let active = moving || !!path;
  for (const u of users.values()) {            // glide remote avatars toward their last reported position
    if (u.id === meId) continue;
    const d = Math.hypot(u.tx - u.x, u.ty - u.y);
    if (d > 0.5) { active = true; const k = Math.min(1, (SPEED * 1.2 * dt) / d); u.x += (u.tx - u.x) * k; u.y += (u.ty - u.y) * k; }
  }
  frames++;
  if (now - fpsT >= 1000) { fps = frames; frames = 0; fpsT = now; }
  // redraw only when something moves or an event marked the screen dirty; no periodic redraw
  if (active || dirty) { draw(me); dirty = false; }
  requestAnimationFrame(frame);
}

// ---- debug panel (F3 toggles): page heap from Chrome, process memory from the bridge
const mb = (b) => (b / 1048576).toFixed(1) + ' MB';
function debugText() {
  const pm = performance.memory;
  const lines = [`fps ${fps}   objects: ${world.floors.size} floors, ${world.walls.size} solid + ${world.soft.size} soft walls, ${world.assets.length} props, ${users.size} users`];
  lines.push(pm ? `page JS heap  ${mb(pm.usedJSHeapSize)} used / ${mb(pm.totalJSHeapSize)} reserved` : 'page JS heap  n/a');
  if (stats) {
    lines.push(`bridge (node) ${mb(stats.bridgeRss)} rss, ${mb(stats.bridgeHeap)} heap`);
    lines.push(stats.chromePss != null ? `chrome        ${mb(stats.chromePss)} (${stats.chromeProcs} processes, PSS)` : 'chrome        n/a');
    if (stats.chromeByType) lines.push('  ' + Object.entries(stats.chromeByType).sort((a, b) => b[1] - a[1]).map(([k, v]) => `${k} ${(v / 1048576).toFixed(0)}`).join(' | '));
    if (stats.chromePss != null) lines.push(`TOTAL         ${mb(stats.bridgeRss + stats.chromePss)}`);
  }
  return lines.join('\n');
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
  const edges = (map, color, width) => {      // tile-edge wall segments
    ctx.strokeStyle = color; ctx.lineWidth = width; ctx.beginPath();
    for (let j = j0; j <= j1; j++) for (let i = i0; i <= i1; i++) {
      const m = map.get(`${i},${j}`) ?? 0; if (!m) continue;
      const x = X(i * T), y = Y(j * T);
      if (m & N) { ctx.moveTo(x, y); ctx.lineTo(x + px, y); }
      if (m & S) { ctx.moveTo(x, y + px); ctx.lineTo(x + px, y + px); }
      if (m & W) { ctx.moveTo(x, y); ctx.lineTo(x, y + px); }
      if (m & E) { ctx.moveTo(x + px, y); ctx.lineTo(x + px, y + px); }
    }
    ctx.stroke();
  };
  edges(world.soft, '#74809c', Math.max(1.5, 2.5 * scale));  // walk-through walls: faint
  edges(world.walls, '#e8c36a', Math.max(2, 5 * scale));     // solid walls
  ctx.fillStyle = '#4a5a6e';
  for (const [ax, ay] of world.assets) { const sx = X(ax), sy = Y(ay); if (sx > -5 && sx < w + 5 && sy > -5 && sy < h + 5) ctx.fillRect(sx - 1, sy - 1, 3, 3); }
  ctx.font = '11px system-ui'; ctx.textAlign = 'center';
  for (const z of world.zones) {
    const zx = X(z.x - z.sx * ZK), zy = Y(z.y - z.sy * ZK), zw = z.sx * ZK * 2 * scale, zh = z.sy * ZK * 2 * scale;
    if (z.id === curZone) { ctx.fillStyle = 'rgba(217,87,127,0.18)'; ctx.fillRect(zx, zy, zw, zh); }
    ctx.strokeStyle = z.kind === 'meeting' ? '#d9577f' : '#667'; ctx.lineWidth = 1;
    ctx.strokeRect(zx, zy, zw, zh);
    if (z.name) { ctx.fillStyle = '#d9577f'; ctx.fillText(z.name, X(z.x), zy - 3); }
  }
  if (path && me) {
    ctx.strokeStyle = '#7fd6a8'; ctx.lineWidth = 2; ctx.setLineDash([5, 5]); ctx.beginPath(); ctx.moveTo(X(me.x), Y(me.y));
    for (const [px_, py_] of path) ctx.lineTo(X(px_), Y(py_));
    ctx.stroke(); ctx.setLineDash([]);
    ctx.beginPath(); ctx.arc(X(pathGoal[0]), Y(pathGoal[1]), 6, 0, 7); ctx.stroke();
  }
  for (const u of users.values()) {
    const sx = X(u.x), sy = Y(u.y), r = Math.max(6, 40 * scale);
    ctx.fillStyle = `hsl(${hue(u.id)} 60% 55%)`;
    ctx.beginPath(); ctx.arc(sx, sy, r, 0, 7); ctx.fill();
    if (u.speaking) { ctx.strokeStyle = '#7fd6a8'; ctx.lineWidth = 3; ctx.beginPath(); ctx.arc(sx, sy, r + 4, 0, 7); ctx.stroke(); }
    if (u.id === meId) { ctx.strokeStyle = '#fff'; ctx.lineWidth = 2; ctx.beginPath(); ctx.arc(sx, sy, r, 0, 7); ctx.stroke(); }
    ctx.fillStyle = '#dde'; ctx.fillText(u.name + (u.state === 'walk' ? ' ·' : ''), sx, sy - r - 4);
  }
  hud.textContent = `${room}${dry ? '  [dry run]' : ''}\n${users.size} people${status ? '  ' + status : ''}` + (performance.now() < toastUntil ? `\n${toast}` : '') + (me ? `\n${Math.round(me.x)}, ${Math.round(me.y)}` : '');
  dbg.style.display = showDebug ? '' : 'none';
  if (showDebug) dbg.textContent = debugText();
}
requestAnimationFrame(frame);
window.__test = { users, world, zoneAt, get curZone() { return curZone; }, get me() { return users.get(meId); } };
