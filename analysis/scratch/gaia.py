import sys;sys.path.insert(0,'.')
from pb import *
from mp import dec
ev,ids=load();T0=ev[0]['t']
gaia={}
for e in ev:
    if e['k']=='ws.open' and 'api.sowork.com/gaia' in e['url']: gaia[e['id']]=e['url']
def red(o):
    if isinstance(o,dict): return {k:red(v) for k,v in o.items()}
    if isinstance(o,list): return [red(x) for x in o]
    if isinstance(o,tuple):
        if o[0]=='ext': return '<ext type=%d len=%d>'%(o[1],len(o[2]))
        if o[0]=='bin': return '<bin len=%d>'%len(o[1])
    if isinstance(o,str) and (o.startswith('eyJ') or len(o)>200): return '<str len=%d>'%len(o)
    return o
want=set(sys.argv[1].split(','))
for e in ev:
    if e['k'] in('ws.in','ws.out') and e.get('id') in gaia and e['op']==2:
        b=base64.b64decode(e['data'])
        v,_=dec(b)
        d=v.get('data') if isinstance(v,dict) else None
        if isinstance(d,list) and d and isinstance(d[0],str) and d[0] in want:
            s=json.dumps(red(d[1:]) )
            print('%.3f'%((e['t']-T0)/1000),gaia[e['id']][-25:][:0],e['id'][-4:],e['k'][3:],d[0],s[:int(sys.argv[2])])
