// Minimal MessagePack for socket.io-msgpack-parser packets. Decode maps undefined (fixext type 0) to undefined.
export function decode(buf) {
  let o = 0;
  const dv = () => new DataView(buf.buffer, buf.byteOffset + o);
  const str = (n) => { const s = buf.toString('utf8', o, o + n); o += n; return s; };
  const bin = (n) => { const b = Buffer.from(buf.subarray(o, o + n)); o += n; return b; };
  const arr = (n) => Array.from({ length: n }, rd);
  const map = (n) => { const r = {}; for (let i = 0; i < n; i++) { const k = rd(); r[k] = rd(); } return r; };
  const rd = () => {
    const b = buf[o++];
    if (b < 0x80) return b;
    if (b >= 0xe0) return b - 256;
    if (b >= 0xa0 && b < 0xc0) return str(b & 0x1f);
    if (b >= 0x90 && b < 0xa0) return arr(b & 0x0f);
    if (b >= 0x80 && b < 0x90) return map(b & 0x0f);
    switch (b) {
      case 0xc0: return null;
      case 0xc2: return false;
      case 0xc3: return true;
      case 0xc4: return bin(buf[o++]);
      case 0xc5: { const n = dv().getUint16(0); o += 2; return bin(n); }
      case 0xc6: { const n = dv().getUint32(0); o += 4; return bin(n); }
      case 0xca: { const v = dv().getFloat32(0); o += 4; return v; }
      case 0xcb: { const v = dv().getFloat64(0); o += 8; return v; }
      case 0xcc: return buf[o++];
      case 0xcd: { const v = dv().getUint16(0); o += 2; return v; }
      case 0xce: { const v = dv().getUint32(0); o += 4; return v; }
      case 0xcf: { const v = Number(dv().getBigUint64(0)); o += 8; return v; }
      case 0xd0: return (buf[o++] << 24) >> 24;
      case 0xd1: { const v = dv().getInt16(0); o += 2; return v; }
      case 0xd2: { const v = dv().getInt32(0); o += 4; return v; }
      case 0xd3: { const v = Number(dv().getBigInt64(0)); o += 8; return v; }
      case 0xd9: return str(buf[o++]);
      case 0xda: { const n = dv().getUint16(0); o += 2; return str(n); }
      case 0xdb: { const n = dv().getUint32(0); o += 4; return str(n); }
      case 0xdc: { const n = dv().getUint16(0); o += 2; return arr(n); }
      case 0xdd: { const n = dv().getUint32(0); o += 4; return arr(n); }
      case 0xde: { const n = dv().getUint16(0); o += 2; return map(n); }
      case 0xdf: { const n = dv().getUint32(0); o += 4; return map(n); }
      case 0xd4: { const t = buf[o]; o += 2; return t === 0 ? undefined : { $ext: t }; }
      case 0xd5: case 0xd6: case 0xd7: case 0xd8: { o += 1 + (1 << (b - 0xd4)); return { $ext: true }; }
      default: throw new Error('msgpack byte 0x' + b.toString(16));
    }
  };
  return rd();
}

export function encode(v) {
  const parts = [];
  const u8 = (...a) => parts.push(Buffer.from(a));
  const w = (x) => {
    if (x === null || x === undefined) u8(0xc0);
    else if (x === true) u8(0xc3);
    else if (x === false) u8(0xc2);
    else if (typeof x === 'number') {
      if (Number.isInteger(x) && x >= 0 && x < 128) u8(x);
      else if (Number.isInteger(x) && x >= 0 && x < 2 ** 32) { const b = Buffer.alloc(5); b[0] = 0xce; b.writeUInt32BE(x, 1); parts.push(b); }
      else if (Number.isInteger(x) && x >= 0) { const b = Buffer.alloc(9); b[0] = 0xcf; b.writeBigUInt64BE(BigInt(x), 1); parts.push(b); }
      else { const b = Buffer.alloc(9); b[0] = 0xcb; b.writeDoubleBE(x, 1); parts.push(b); }
    } else if (typeof x === 'string') {
      const s = Buffer.from(x);
      if (s.length < 32) u8(0xa0 | s.length);
      else if (s.length < 65536) { const b = Buffer.alloc(3); b[0] = 0xda; b.writeUInt16BE(s.length, 1); parts.push(b); }
      else { const b = Buffer.alloc(5); b[0] = 0xdb; b.writeUInt32BE(s.length, 1); parts.push(b); }
      parts.push(s);
    } else if (Array.isArray(x)) {
      if (x.length < 16) u8(0x90 | x.length);
      else { const b = Buffer.alloc(3); b[0] = 0xdc; b.writeUInt16BE(x.length, 1); parts.push(b); }
      x.forEach(w);
    } else {
      const ks = Object.keys(x).filter((k) => x[k] !== undefined);
      if (ks.length < 16) u8(0x80 | ks.length);
      else { const b = Buffer.alloc(3); b[0] = 0xde; b.writeUInt16BE(ks.length, 1); parts.push(b); }
      for (const k of ks) { w(k); w(x[k]); }
    }
  };
  w(v);
  return Buffer.concat(parts);
}
