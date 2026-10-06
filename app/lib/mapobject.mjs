// Decoder for PbSetMapObject (protobuf) carried by gaia 'b_set_obj'. See docs/gaia-protocol.md 3.9.
function varint(b, i) {
  let r = 0, mul = 1;
  for (;;) {
    const c = b[i++];
    r += (c & 0x7f) * mul;
    if (!(c & 0x80)) return [r, i];
    mul *= 128;
  }
}
function fields(b) {
  const out = [];
  let i = 0;
  while (i < b.length) {
    let k; [k, i] = varint(b, i);
    const no = Math.floor(k / 8), wt = k & 7;
    let v;
    if (wt === 0) [v, i] = varint(b, i);
    else if (wt === 1) { v = b.subarray(i, i + 8); i += 8; }
    else if (wt === 5) { v = b.subarray(i, i + 4); i += 4; }
    else if (wt === 2) { let l; [l, i] = varint(b, i); v = b.subarray(i, i + l); i += l; }
    else throw new Error('wire type ' + wt);
    out.push([no, v]);
  }
  return out;
}
function value(b) { // google.protobuf.Value
  for (const [no, v] of fields(b)) {
    if (no === 2) return v.readDoubleLE(0);
    if (no === 3) return v.toString();
    if (no === 4) return !!v;
    if (no === 5) return struct(v);
    if (no === 6) return fields(v).filter(([n]) => n === 1).map(([, x]) => value(x));
  }
  return null;
}
function struct(b) { // google.protobuf.Struct
  const d = {};
  for (const [no, v] of fields(b)) {
    if (no !== 1) continue;
    let k, val = null;
    for (const [n2, v2] of fields(v)) { if (n2 === 1) k = v2.toString(); else if (n2 === 2) val = value(v2); }
    d[k] = val;
  }
  return d;
}
const TYPES = ['zone', 'asset', 'floor', 'wall'];
const ORI = ['N', 'NE', 'E', 'SE', 'S', 'SW', 'W', 'NW'];

export function decodeSetMapObject(buf) {
  let id = '', data = null;
  for (const [no, v] of fields(buf)) {
    if (no === 1) id = v.toString();
    else if (no === 2) {
      data = { type: 'zone', x: 0, y: 0, assetId: '', orientation: 'N', sizeX: 1, sizeY: 1, interaction: null };
      for (const [n, x] of fields(v)) {
        if (n === 1) data.type = TYPES[x] ?? 'zone';
        else if (n === 2) data.x = x.readFloatLE(0);
        else if (n === 3) data.y = x.readFloatLE(0);
        else if (n === 4) data.assetId = x.toString();
        else if (n === 5) data.orientation = ORI[x] ?? 'N';
        else if (n === 6) data.sizeX = x.readFloatLE(0);
        else if (n === 7) data.sizeY = x.readFloatLE(0);
        else if (n === 9) data.interaction = struct(x);
      }
    }
  }
  return { id, data };
}
