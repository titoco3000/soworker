// How much of the floor is reachable from spawn on foot? Usage: node app/test/reach.mjs recorder/logs/<run>
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { decode } from '../lib/msgpack.mjs';
import { decodeSetMapObject } from '../lib/mapobject.mjs';
import { World, T } from '../lib/world.mjs';

const L = readFileSync(join(process.argv[2], 'events.jsonl'), 'utf8').trim().split('\n').map((l) => JSON.parse(l));
const world = new World();
for (const e of L) {
  if (e.k !== 'ws.in' || e.op !== 2) continue;
  let p; try { p = decode(Buffer.from(e.data, 'base64')); } catch { continue; }
  if (p?.type === 2 && p.data[0] === 'b_set_obj') world.apply(decodeSetMapObject(p.data[1]));
}
const N = 1, E = 2, S = 4, W = 8;
function reach(snap, sx, sy) {
  const floors = new Set(snap.floors.map(([i, j]) => `${i},${j}`)), walls = new Map(snap.walls.map(([i, j, m]) => [`${i},${j}`, m]));
  const wall = (i, j) => walls.get(`${i},${j}`) ?? 0;
  const D = [[0, -1, N, S], [1, 0, E, W], [0, 1, S, N], [-1, 0, W, E]];
  const seen = new Set([`${Math.floor(sx / T)},${Math.floor(sy / T)}`]), q = [[Math.floor(sx / T), Math.floor(sy / T)]];
  while (q.length) {
    const [i, j] = q.pop();
    for (const [di, dj, out, back] of D) {
      const k = `${i + di},${j + dj}`;
      if (seen.has(k) || !floors.has(k) || (wall(i, j) & out) || (wall(i + di, j + dj) & back)) continue;
      seen.add(k); q.push([i + di, j + dj]);
    }
  }
  const zones = snap.zones.filter((z) => seen.has(`${Math.floor(z.x / T)},${Math.floor(z.y / T)}`)).length;
  return { floors: floors.size, reachable: seen.size, zonesReachable: zones, zones: snap.zones.length };
}
const snap = world.snapshot();
console.log('now (non-blocking walls removed):', reach(snap, 3260, 980));
// baseline: every wall blocks
const all = new World();
for (const [id, o] of world.objects) all.objects.set(id, o.type === 'wall' ? { ...o, assetId: 'unknown' } : o);
console.log('before (every wall blocks):     ', reach(all.snapshot(), 3260, 980));
