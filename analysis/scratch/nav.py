import json,re,urllib.parse
L='/home/tito/dev/repos/soworker/output/raw-logs/2026-10-06T12-48-44-206Z/events.jsonl'
evs=[json.loads(l) for l in open(L)]; t0=evs[0]['t']
SEC=('apiKey','state','code','client_id','nonce','sessionId','eventId','id_token','key')
def red(u):
    p=urllib.parse.urlsplit(u)
    q=urllib.parse.parse_qsl(p.query,keep_blank_values=True)
    q2=[(k,(v if k not in SEC and len(v)<60 else '<%s>'%k)) for k,v in q]
    return f"{p.scheme}://{p.netloc}{p.path[:60]}?"+urllib.parse.urlencode(q2)
for e in evs:
    t=round((e['t']-t0)/1000,1)
    if e['k']=='nav': print('NAV',t,e.get('tg'),red(e.get('url',''))[:500])
    if e['k']=='req' and e.get('rtype')=='Document': print('DOC',t,e.get('tg'),e['method'],red(e['url'])[:500])
    if e['k']=='target': print('TARGET',t,e.get('tg'),red(e.get('url',''))[:200], e.get('type'))
