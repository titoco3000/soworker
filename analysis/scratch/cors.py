import json,re
L='/home/tito/dev/repos/soworker/recorder/logs/2026-10-06T12-48-44-206Z/events.jsonl'
evs=[json.loads(l) for l in open(L)]
req={}
seen=set()
for e in evs:
    if e['k']=='req': req[e['id']]=e
    if e['k'] in('res','res.extra'):
        r=req.get(e['id']); u=(r or {}).get('url','')
        m=re.match(r'https://([^/]+)(/[^?]*)',u)
        if not m: continue
        host,path=m.groups()
        if host not in('api.sowork.com','chat.sowork.com') and 'livekit' not in host: continue
        meth=r.get('method')
        key=(host,meth if meth=='OPTIONS' else 'x',e['k'], path if host!='app.sowork.com' else path[:12])
        if key in seen: continue
        if host=='api.sowork.com' and ('eyekia' in path or 'chat-v2' in path): 
            if (host,meth,e['k'],'eyekia') in seen: continue
            seen.add((host,meth,e['k'],'eyekia'))
        seen.add(key)
        h={a.lower():b for a,b in (e.get('headers') or {}).items()}
        keep={a:b[:120] for a,b in h.items() if a.startswith('access-control') or a in('set-cookie','server','via','vary','cache-control','content-type','x-powered-by','cf-ray','x-amz-cf-id','strict-transport-security','content-security-policy')}
        print(meth,e['k'],e.get('status'),host,path[:50]); print('    ',keep)
