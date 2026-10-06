#!/usr/bin/env python3
"""Decode SoWork gaia 'b_set_obj' frames (PbSetMapObject, protobuf proto3).
Schema recovered from app bundle httpClientFactory-*.js (module Yu):
 PbSetMapObject{1 id:string, 2 data:PbMapObject(optional; absent = delete)}
 PbMapObject{1 type:enum(Zone0 Asset1 Floor2 Wall3) 2 x:float 3 y:float 4 assetId:string
  5 orientation:enum(N0 NE1 E2 SE3 S4 SW5 W6 NW7) 6 sizeX:float? 7 sizeY:float? 8 perms:Struct?
  9 interaction:Struct? 10 config:Struct? 11 spawnDirection:enum? 13 colorPalette:PbColorVariation?
  14 colorKey:string? 15 updatedBy:string? 16 updatedAt:int64? 17 tags:repeated string 18 accessKey:string?}
 PbColorVariation{1 hueRotationDegrees:float? 2 saturationMultiplier:float? 3 valueMultiplier:float?}
Usage: python3 decode_obj.py > objects.jsonl   (reads recorder events.jsonl via ws_msgpack.py)
"""
import sys, re, json, struct

def varint(b, i):
    r = 0; s = 0
    while True:
        c = b[i]; i += 1
        r |= (c & 0x7f) << s; s += 7
        if not c & 0x80: return r, i

def fields(b):
    i = 0; out = []
    while i < len(b):
        k, i = varint(b, i); no, wt = k >> 3, k & 7
        if wt == 0: v, i = varint(b, i)
        elif wt == 1: v = b[i:i+8]; i += 8
        elif wt == 5: v = b[i:i+4]; i += 4
        elif wt == 2:
            l, i = varint(b, i); v = b[i:i+l]; i += l
        else: raise ValueError('wt %d' % wt)
        out.append((no, wt, v))
    return out

def value(b):  # google.protobuf.Value
    for no, wt, v in fields(b):
        if no == 1: return None
        if no == 2: return struct.unpack('<d', v)[0]
        if no == 3: return v.decode()
        if no == 4: return bool(v)
        if no == 5: return struct_(v)
        if no == 6: return [value(x) for n, w, x in fields(v) if n == 1]
    return None

def struct_(b):  # google.protobuf.Struct: map<string,Value> fields = 1
    d = {}
    for no, wt, v in fields(b):
        if no == 1:
            k = None; val = None
            for n2, w2, v2 in fields(v):
                if n2 == 1: k = v2.decode()
                elif n2 == 2: val = value(v2)
            d[k] = val
    return d

TYPE = ['zone', 'asset', 'floor', 'wall']
ORI = ['N', 'NE', 'E', 'SE', 'S', 'SW', 'W', 'NW']
f32 = lambda v: struct.unpack('<f', v)[0]

def decode_obj(b):
    o = {'type': 'zone', 'x': 0.0, 'y': 0.0, 'assetId': '', 'orientation': 'N', 'tags': []}
    for no, wt, v in fields(b):
        if no == 1: o['type'] = TYPE[v]
        elif no == 2: o['x'] = f32(v)
        elif no == 3: o['y'] = f32(v)
        elif no == 4: o['assetId'] = v.decode()
        elif no == 5: o['orientation'] = ORI[v]
        elif no == 6: o['sizeX'] = f32(v)
        elif no == 7: o['sizeY'] = f32(v)
        elif no == 8: o['perms'] = struct_(v)
        elif no == 9: o['interaction'] = struct_(v)
        elif no == 10: o['config'] = struct_(v)
        elif no == 11: o['spawnDirection'] = ORI[v]
        elif no == 13:
            c = {}
            for n2, w2, v2 in fields(v):
                c[['', 'hueRotationDegrees', 'saturationMultiplier', 'valueMultiplier'][n2]] = f32(v2)
            o['colorPalette'] = c
        elif no == 14: o['colorKey'] = v.decode()
        elif no == 15: o['updatedBy'] = v.decode()
        elif no == 16: o['updatedAt'] = v
        elif no == 17: o['tags'].append(v.decode())
        elif no == 18: o['accessKey'] = v.decode()
        else: o['_unk%d' % no] = repr(v)
    return o

def decode_set(b):
    r = {}
    for no, wt, v in fields(b):
        if no == 1: r['id'] = v.decode()
        elif no == 2: r['data'] = decode_obj(v)
    return r

if __name__ == '__main__':
    # decodes every b_set_obj from the recording (full binary frames); prints one JSON per line
    import os
    sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
    from ws_msgpack import frames
    for t, k, i, p in frames():
        d = p.get('data') if isinstance(p, dict) else None
        if k == 'ws.in' and isinstance(d, list) and d and d[0] == 'b_set_obj':
            print(json.dumps({'t': t, **decode_set(d[1])}))
