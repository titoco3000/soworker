// Map objects -> compact world model for the client. Tile centres sit at odd multiples of G; tile pitch T = 2G.
export const G = 105.46875;
export const T = 2 * G;
const BIT = { N: 1, E: 2, S: 4, W: 8 };

export class World {
  objects = new Map(); // id -> decoded data
  apply({ id, data }) { data ? this.objects.set(id, data) : this.objects.delete(id); }

  snapshot() {
    const floors = [], walls = new Map(), zones = [], assets = [];
    for (const [id, o] of this.objects) {
      const i = Math.floor(o.x / T), j = Math.floor(o.y / T);
      if (o.type === 'floor') floors.push([i, j]);
      else if (o.type === 'wall') walls.set(`${i},${j}`, (walls.get(`${i},${j}`) ?? 0) | (BIT[o.orientation] ?? 0));
      else if (o.type === 'zone') {
        zones.push({ id, x: o.x, y: o.y, sx: o.sizeX, sy: o.sizeY, kind: o.interaction?.type ?? null, name: o.interaction?.data?.meeting?.name ?? null });
      } else assets.push([Math.round(o.x), Math.round(o.y)]);
    }
    return { floors, walls: [...walls].map(([k, m]) => [...k.split(',').map(Number), m]), zones, assets };
  }
}
