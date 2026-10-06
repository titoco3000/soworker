import sys;sys.path.insert(0,'.')
from pb import *
import re
ev,ids=load();T0=ev[0]['t']
def show(b,depth=0,maxd=4):
    try:
        items=parse(b)
    except Exception: return None
    return items
def fmt(b,d=0):
    if d>5: return '<..>'
    items=None
    try: items=parse(b) if len(b)>0 else None
    except Exception: items=None
    parts=[]
    for f,w,v in items or []:
        if w==0: parts.append(f"{f}={v}")
        elif w==2:
            try:
                s=v.decode('utf-8')
                if s.isprintable() and len(s)>0:
                    if s.startswith('eyJ') or len(s)>60 and re.fullmatch(r'[\w\-.=]+',s): s='<REDACTED %d>'%len(s)
                    parts.append(f'{f}="{s[:80]}"');continue
            except Exception: pass
            sub=fmt(v,d+1) if v else None
            parts.append(f"{f}={{{sub}}}" if sub is not None and len(v)>0 else f"{f}=<{len(v)}B>")
        else: parts.append(f"{f}=<{w}:{v.hex()}>")
    return ' '.join(parts) if items is not None else None
which=sys.argv[1]
for e in ev:
    if e['k'] in('ws.in','ws.out') and e.get('id')==which:
        if e['op']!=2: print((e['t']-T0)/1000,e['k'],'TEXT',str(e.get('data'))[:100]);continue
        b=base64.b64decode(e['data'])
        names=SIGRESP if e['k']=='ws.in' else SIGREQ
        try:
            it=parse(b)
            print('%.3f'%((e['t']-T0)/1000),e['k'][3:].upper(),len(b),' | '.join(names.get(f,str(f))+':'+(fmt(v) if w==2 else str(v)) for f,w,v in it)[:int(sys.argv[2]) if len(sys.argv)>2 else 600])
        except Exception as x: print('ERR',x)
