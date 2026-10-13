import json,re,base64
L='/home/tito/dev/repos/soworker/output/raw-logs/2026-10-06T12-48-44-206Z/events.jsonl'
def jwt(s):
    try:
        p=s.split('.')[1]; p+='='*(-len(p)%4)
        return json.loads(base64.urlsafe_b64decode(p))
    except Exception: return None
def sh(c):
    if not c: return None
    out={}
    for k,v in c.items():
        if k in('iss','aud','video','firebase','kind','claims') : out[k]= v if k!='claims' else '...' 
        else: out[k]=type(v).__name__
    return out
evs=[json.loads(l) for l in open(L)]
t0=evs[0]['t']
wsid={}
for e in evs:
    k=e['k']
    if k=='ws.open':
        u=e.get('url','')
        wsid[e['id']]=u
        q=re.search(r'token=([^&]+)',u) or re.search(r'access_token=([^&]+)',u)
        print('OPEN',round((e['t']-t0)/1000,1),re.sub(r'(token|join_request)=[^&]+',r'\1=<X>',u)[:200])
        if q:
            c=jwt(q.group(1)); print('   claims',sh(c), 'exp-iat/nbf',(c or {}).get('exp',0)-(c or {}).get('iat',(c or {}).get('nbf',0)))
    if k=='ws.hs.req':
        h={a.lower():b for a,b in (e.get('headers') or {}).items()}
        print('HS.REQ',{a:(b if a in('origin','sec-websocket-protocol') else '...') for a,b in h.items() if a not in('user-agent','accept-language','accept-encoding','sec-websocket-key','sec-websocket-version','sec-websocket-extensions','pragma','cache-control','connection','upgrade')})
