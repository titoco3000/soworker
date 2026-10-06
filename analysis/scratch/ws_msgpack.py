#!/usr/bin/env python3
"""Iterate gaia (socket.io + notepack msgpack) frames of events.jsonl with full binary payloads.
yield (t_seconds, direction, wsid, packet) ; bin -> bytes; fixext -> ('ext', type, bytes)"""
import json, struct, base64, sys
LOG = '/home/tito/dev/repos/soworker/recorder/logs/2026-10-06T12-48-44-206Z/events.jsonl'

def dec(b, i=0):
    c = b[i]; i += 1
    if c <= 0x7f: return c, i
    if 0x80 <= c <= 0x8f: return _map(b, i, c & 15)
    if 0x90 <= c <= 0x9f: return _arr(b, i, c & 15)
    if 0xa0 <= c <= 0xbf: n = c & 31; return b[i:i+n].decode('utf8', 'replace'), i+n
    if c >= 0xe0: return c - 256, i
    if c == 0xc0: return None, i
    if c == 0xc2: return False, i
    if c == 0xc3: return True, i
    if c in (0xc4, 0xc5, 0xc6):
        w = {0xc4: 1, 0xc5: 2, 0xc6: 4}[c]; n = int.from_bytes(b[i:i+w], 'big'); i += w
        return bytes(b[i:i+n]), i+n
    if c in (0xc7, 0xc8, 0xc9):
        w = {0xc7: 1, 0xc8: 2, 0xc9: 4}[c]; n = int.from_bytes(b[i:i+w], 'big'); i += w
        return ('ext', b[i], bytes(b[i+1:i+1+n])), i+1+n
    if c == 0xca: return struct.unpack('>f', b[i:i+4])[0], i+4
    if c == 0xcb: return struct.unpack('>d', b[i:i+8])[0], i+8
    if c in (0xcc, 0xcd, 0xce, 0xcf):
        w = {0xcc: 1, 0xcd: 2, 0xce: 4, 0xcf: 8}[c]; return int.from_bytes(b[i:i+w], 'big'), i+w
    if c in (0xd0, 0xd1, 0xd2, 0xd3):
        w = {0xd0: 1, 0xd1: 2, 0xd2: 4, 0xd3: 8}[c]; return int.from_bytes(b[i:i+w], 'big', signed=True), i+w
    if c in (0xd4, 0xd5, 0xd6, 0xd7, 0xd8):
        n = {0xd4: 1, 0xd5: 2, 0xd6: 4, 0xd7: 8, 0xd8: 16}[c]
        return ('ext', b[i], bytes(b[i+1:i+1+n])), i+1+n
    if c in (0xd9, 0xda, 0xdb):
        w = {0xd9: 1, 0xda: 2, 0xdb: 4}[c]; n = int.from_bytes(b[i:i+w], 'big'); i += w
        return b[i:i+n].decode('utf8', 'replace'), i+n
    if c in (0xdc, 0xdd):
        w = 2 if c == 0xdc else 4; n = int.from_bytes(b[i:i+w], 'big'); return _arr(b, i+w, n)
    if c in (0xde, 0xdf):
        w = 2 if c == 0xde else 4; n = int.from_bytes(b[i:i+w], 'big'); return _map(b, i+w, n)
    raise ValueError(hex(c))
def _arr(b, i, n):
    r = []
    for _ in range(n): v, i = dec(b, i); r.append(v)
    return r, i
def _map(b, i, n):
    r = {}
    for _ in range(n):
        k, i = dec(b, i); v, i = dec(b, i); r[k] = v
    return r, i

def frames(match='gaia'):
    t0 = None; urls = {}
    for l in open(LOG):
        try: e = json.loads(l)
        except Exception: continue
        if t0 is None: t0 = e['t']
        if e['k'] == 'ws.open': urls[e['id']] = e['url']
        if e['k'] in ('ws.in', 'ws.out') and match in urls.get(e['id'], '') and e.get('op') == 2:
            b = base64.b64decode(e['data'])
            try: p, _ = dec(b)
            except Exception as ex: p = ('ERR', str(ex))
            yield (e['t'] - t0) / 1000, e['k'], e['id'], p
if __name__ == '__main__':
    for t, k, i, p in frames():
        print(round(t, 3), k, i, repr(p)[:300])
