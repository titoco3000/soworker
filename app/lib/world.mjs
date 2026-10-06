import { readFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

// Map objects -> compact world model for the client. Tile centres sit at odd multiples of G; tile pitch T = 2G.
export const G = 105.46875;
export const T = 2 * G;
const BIT = { N: 1, E: 2, S: 4, W: 8 };

// Which wall variants block walking. The catalog has no collider flag, so use its name and height:
// "NoCollider" variants and flat borders (<= 0.15 m) are decoration. Unknown wall ids block.
let wallTable = {};
try { wallTable = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), '..', 'data', 'walls.json'), 'utf8')); } catch {}
export function wallBlocks(assetId) {
  const w = wallTable[assetId];
  if (!w) return true;
  if (/no ?collider/i.test(w.name ?? '')) return false;
  const h = parseFloat(w.height);
  return !(Number.isFinite(h) && h <= 0.15);
}

export class World {
  objects = new Map(); // id -> decoded data
  apply({ id, data }) { data ? this.objects.set(id, data) : this.objects.delete(id); }

  snapshot() {
    const floors = [], walls = new Map(), soft = new Map(), zones = [], assets = [];
    for (const [id, o] of this.objects) {
      const i = Math.floor(o.x / T), j = Math.floor(o.y / T);
      if (o.type === 'floor') floors.push([i, j]);
      else if (o.type === 'wall') { const m = wallBlocks(o.assetId) ? walls : soft; m.set(`${i},${j}`, (m.get(`${i},${j}`) ?? 0) | (BIT[o.orientation] ?? 0)); }
      else if (o.type === 'zone') {
        zones.push({ id, x: o.x, y: o.y, sx: o.sizeX, sy: o.sizeY, kind: o.interaction?.type ?? null, name: o.interaction?.data?.meeting?.name ?? null });
      } else assets.push([Math.round(o.x), Math.round(o.y)]);
    }
    const list = (m) => [...m].map(([k, v]) => [...k.split(',').map(Number), v]);
    return { floors, walls: list(walls), soft: list(soft), zones, assets };
  }
}
