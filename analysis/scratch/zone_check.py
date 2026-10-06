"""Check where the avatar was (last 'move') when 'set_meeting_zone' fired, vs the zone rectangles decoded from b_set_obj."""
import sys,glob,json,re
sys.path.insert(0,'scratch')
from pb_decode import *
Z={}
for t,pl in msgpack_frames(glob.glob('../recorder/logs/*/events.jsonl')[0]):
    f={a:v for a,w,v in raw_fields(pl)};o=pb_map_object(f[2])
    if o.get('type',0)==0: Z[f[1].decode()]=o
pos=None
for l in open('work/ws/07-api.sowork.com.txt'):
    m=re.match(r'([\d.]+) OUT +MP (\{.*)',l)
    if not m: continue
    try:d=json.loads(m.group(2))
    except: continue
    dd=d.get('data')
    if not isinstance(dd,list):continue
    if dd[0]=='move':pos=(dd[1]['x'],dd[1]['y'])
    if dd[0]=='set_meeting_zone':
        zid=dd[1]['zoneId'];z=Z.get(zid)
        if z:
            dx,dy=pos[0]-z['x'],pos[1]-z['y']
            print(m.group(1),zid[:5],'pos',tuple(round(v) for v in pos),'zone',round(z['x']),round(z['y']),round(z['sizeX']),round(z['sizeY']),'d',round(dx),round(dy),'inside(center-anchored)',abs(dx)<=z['sizeX']/2 and abs(dy)<=z['sizeY']/2)
        else: print(m.group(1),zid,'pos',pos and tuple(round(v) for v in pos))
