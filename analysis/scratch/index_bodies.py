import json,sys,collections
L='/home/tito/dev/repos/soworker/output/raw-logs/2026-10-06T12-48-44-206Z/'
t0=None;rows=[]
for l in open(L+'events.jsonl'):
    try:e=json.loads(l)
    except: continue
    if t0 is None: t0=e['t']
    if e.get('k')=='res.body' and e.get('file'):
        rows.append(((e['t']-t0)/1000,e.get('size'),e['file'],e['url'],e.get('mime')))
for r in rows: print('\t'.join(map(str,r)))
