import json,re,base64
L='/home/tito/dev/repos/soworker/recorder/raw-logs/2026-10-06T12-48-44-206Z/events.jsonl'
evs=[json.loads(l) for l in open(L)]
t0=evs[0]['t']
toks={}
for e in evs:
    if e['k']=='ws.open':
        m=re.search(r'access_token=([^&]+)',e.get('url',''))
        if m: toks[m.group(1)]=e
for tok,oe in toks.items():
    sig=tok.split('.')[1][:40].encode()
    print('token opened',round((oe['t']-t0)/1000,1))
    for e in evs:
        if e['k'] in('ws.in','ws.out','dc.in','res.body','req.postData','sse') :
            d=e.get('data') or e.get('text') or e.get('b64') or ''
            if e['k'] in('ws.in','ws.out') and e.get('op')==2:
                try: raw=base64.b64decode(d)
                except: continue
            else: raw=d.encode() if isinstance(d,str) else b''
            if sig in raw or tok.encode() in raw:
                print('  found in',e['k'],round((e['t']-t0)/1000,1),e.get('tg'),e.get('id'),len(raw))
                # print msgpack context labels only
                i=raw.find(sig); print('   ctx before:',raw[max(0,i-120):i-0][:120].decode('latin1').encode('unicode_escape')[:300])
