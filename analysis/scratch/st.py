import json,re,base64,collections
L='/home/tito/dev/repos/soworker/recorder/raw-logs/2026-10-06T12-48-44-206Z/events.jsonl'
evs=[json.loads(l) for l in open(L)]
t0=evs[0]['t']
def kinds(d):
    return {k:(type(v).__name__, len(v) if hasattr(v,'__len__') else '') for k,v in d.items()}
st=[e for e in evs if e['k']=='storage']
for e in (st[0],st[-1]):
    print('STORAGE',round((e['t']-t0)/1000,1),e['reason'],e['href'][:80])
    print(' ls keys',sorted(e['ls'].keys()) if isinstance(e['ls'],dict) else type(e['ls']))
    print(' ss keys',sorted(e['ss'].keys()) if isinstance(e['ss'],dict) else type(e['ss']))
    print(' idb',json.dumps(e['idb'])[:600])
ck=[e for e in evs if e['k']=='cookies']
print('COOKIES',len(ck)); e=ck[-1]
cs=e.get('cookies') or e.get('list') or e
print(list(e.keys()))
for c in (e.get('cookies') or []):
    print(' ',c.get('domain'),c.get('name'),'httpOnly',c.get('httpOnly'),'secure',c.get('secure'),'sameSite',c.get('sameSite'),'expires',c.get('expires'))
