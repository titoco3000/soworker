import json,re,collections,os
L='/home/tito/dev/repos/soworker/recorder/logs/2026-10-06T12-48-44-206Z/events.jsonl'
D='/home/tito/dev/repos/soworker/recorder/logs/2026-10-06T12-48-44-206Z/bodies/'
evs=[json.loads(l) for l in open(L)]
req={}
dec=json.JSONDecoder()
docs=collections.OrderedDict()
tc=collections.Counter()
def walk(o):
    if isinstance(o,dict):
        if 'documentChange' in o or 'documentDelete' in o or 'documentRemove' in o:
            k=[x for x in ('documentChange','documentDelete','documentRemove') if x in o][0]
            d=o[k].get('document') or {'name':o[k].get('document','?'),'fields':{}}
            name=d.get('name','')
            name=re.sub(r'.*/documents/','',name) if isinstance(name,str) else '?'
            tpl=re.sub(r'(?<=/)[A-Za-z0-9_]{18,}|^[A-Za-z0-9_]{20,}','<id>',name)
            tpl=re.sub(r'<id>_[A-Za-z0-9]+$','<id>_<id>',tpl)
            e=docs.setdefault((k,tpl),{'n':0,'fields':{}})
            e['n']+=1
            for f,v in (d.get('fields') or {}).items():
                e['fields'][f]=list(v.keys())[0]
        if 'targetChange' in o: tc[o['targetChange'].get('targetChangeType','(resume)')]+=1
        for v in o.values(): walk(v)
    elif isinstance(o,list):
        for v in o: walk(v)
for e in evs:
    if e['k']=='req': req[e['id']]=e
    if e['k']=='res.body':
        r=req.get(e['id'])
        if not r or 'Listen/channel' not in r['url']: continue
        txt=e.get('text')
        if txt is None and e.get('file'): 
            try: txt=open(D+e['file']).read()
            except: continue
        if not txt: continue
        for m in re.finditer(r'\[\d+,\[',txt):
            pass
        # parse frames: size line then json
        i=0
        while i<len(txt):
            m=re.match(r'\s*(\d+)\n',txt[i:])
            if not m: break
            n=int(m.group(1)); body=txt[i+m.end():i+m.end()+n]; i+=m.end()+n
            try: walk(json.loads(body))
            except Exception: pass
for (k,t),e in docs.items():
    print(e['n'],k,t); print('     ',e['fields'])
print(tc)
