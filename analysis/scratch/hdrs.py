import json,sys,re,base64,collections
L='/home/tito/dev/repos/soworker/output/raw-logs/2026-10-06T12-48-44-206Z/events.jsonl'
def jwt(s):
    try:
        p=s.split('.')[1]; p+='='*(-len(p)%4)
        return json.loads(base64.urlsafe_b64decode(p))
    except Exception: return None
def shape(v):
    if isinstance(v,dict): return {k:shape(x) for k,x in v.items()}
    return type(v).__name__
t0=None
reqs={}
agg=collections.OrderedDict()
for line in open(L):
    e=json.loads(line)
    if t0 is None: t0=e['t']
    k=e['k']
    if k=='req': reqs[e['id']]=e
    if k in('req.extra',):
        r=reqs.get(e['id']); 
        u=(r or {}).get('url','')
        m=re.match(r'https://([^/]+)(/[^?]*)',u)
        if not m: continue
        host,path=m.groups()
        if not any(x in host for x in ('sowork','googleapis','firebase','livekit','sanity','amazonaws')): continue
        h={a.lower():b for a,b in (e.get('headers') or {}).items()}
        path=re.sub(r'[A-Za-z0-9_-]{20}','<id>',path)
        key=(host,path)
        names=sorted(h.keys())
        info=agg.setdefault(key,{'n':0,'hdrs':set(),'ex':{}})
        info['n']+=1
        for n in names:
            if n in('accept','accept-encoding','accept-language','user-agent','sec-ch-ua','sec-ch-ua-mobile','sec-ch-ua-platform','sec-fetch-dest','sec-fetch-mode','sec-fetch-site','connection','host','pragma','cache-control','priority'): continue
            info['hdrs'].add(n)
        a=h.get('authorization')
        if a and 'a' not in info['ex']:
            sch,_,tok=a.partition(' ')
            c=jwt(tok)
            info['ex']['a']=(sch,{k:(v if k in('iss','aud','firebase','sub_type') else type(v).__name__) for k,v in (c or {}).items()} if c else None, (c or {}).get('exp',0)-(c or {}).get('iat',0))
        if 'cookie' in h and 'c' not in info['ex']:
            info['ex']['c']=[x.split('=')[0].strip() for x in h['cookie'].split(';')]
        if 'origin' in h: info['ex']['o']=h['origin']
for (host,path),i in agg.items():
    print(i['n'],host,path); print('   ',sorted(i['hdrs'])); 
    for k,v in i['ex'].items(): print('   ',k,v)
