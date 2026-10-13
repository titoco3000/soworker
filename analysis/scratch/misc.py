import json,re,base64
L='/home/tito/dev/repos/soworker/output/raw-logs/2026-10-06T12-48-44-206Z/events.jsonl'
def jwt(s):
    try:
        p=s.split('.')[1]; p+='='*(-len(p)%4)
        return json.loads(base64.urlsafe_b64decode(p))
    except Exception: return None
evs=[json.loads(l) for l in open(L)]
t0=evs[0]['t']
n=0
for e in evs:
    if e['k']=='ws.out' and e.get('op')==2 and n<3:
        raw=base64.b64decode(e['data'])
        m=re.search(rb'eyJ[\w-]+\.[\w-]+\.[\w-]+',raw)
        if m:
            c=jwt(m.group(0).decode()); n+=1
            print('gaia connect jwt at',round((e['t']-t0)/1000,1),{k:(v if k in('iss','aud') else type(v).__name__) for k,v in c.items()}, c['exp']-c['iat'])
ks=set()
for e in evs:
    if e['k']=='storage':
        print('storage',round((e['t']-t0)/1000,1),{k:(list(v.keys()) if isinstance(v,dict) else (v if isinstance(v,list) and len(str(v))<400 else type(v).__name__)) for k,v in e.items() if k not in('t','k')} if False else list(e.keys()))
        break
