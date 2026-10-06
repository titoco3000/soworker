import re,json,urllib.parse,collections
txt=open('/home/tito/dev/repos/soworker/analysis/work/api-bodies.md').read()
blocks=re.split(r'\n(?=### )',txt)
adds=[];rems=[]
for b in blocks:
    m=re.match(r'### ([\d.]+) (\w+) (\S+)',b)
    if not m or 'Listen/channel' not in m.group(3): continue
    t=float(m.group(1)); proj=re.search(r'projects%2F([\w-]+)%2F',m.group(3)).group(1)
    for d in re.findall(r'req\d+___data__=([^&\n]+)',b):
        try: j=json.loads(urllib.parse.unquote(d))
        except Exception: j={'raw':urllib.parse.unquote(d)[:300]}
        j.pop('database',None)
        adds.append((t,proj,j))
    # response docs
    for name in re.findall(r'"name": "projects/[\w-]+/databases/\(default\)/documents/([^"]+)"',b):
        rems.append((t,proj,'DOC',name))
for t,p,j in adds:
    if 'addTarget' in j:
        a=j['addTarget']
        if 'documents' in a: s='doc '+', '.join(x.split('/documents/')[1] for x in a['documents']['documents'])
        else: s='query '+json.dumps(a.get('query'))[:300]
        print(f"{t:7.1f} {p[:12]:12} +T{a.get('targetId')} {s}")
    elif 'removeTarget' in j: print(f"{t:7.1f} {p[:12]:12} -T{j['removeTarget']}")
    else: print(t,p,j)
print()
c=collections.Counter()
for t,p,k,n in rems: c[(p[:12],re.sub(r'[A-Za-z0-9]{20,}','<id>',n))]+=1
for k,v in sorted(c.items()): print(v,k)
