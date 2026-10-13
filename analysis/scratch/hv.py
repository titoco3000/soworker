import json,re,collections
L='/home/tito/dev/repos/soworker/recorder/raw-logs/2026-10-06T12-48-44-206Z/events.jsonl'
c=collections.Counter()
for line in open(L):
    e=json.loads(line)
    if e['k']=='req.extra':
        h={a.lower():b for a,b in (e.get('headers') or {}).items()}
        for n in ('source','version','web_client_version','content-type','x-firebase-client','x-client-version','x-firebase-gmpid'):
            if n in h: c[(n,h[n][:80])]+=1
for k,v in sorted(c.items()): print(v,k)
