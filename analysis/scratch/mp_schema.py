"""Minimal msgpack decoder (notepack flavour: fixext type0 = undefined) + schema census for gaia socket frames.
Usage: python3 mp_schema.py <events.jsonl>  -> prints per (dir, event) key paths/types/counts. Never prints string values except short enum-like ones."""
import json,base64,struct,sys,collections
class U: pass
UNDEF=U()
def dec(b,i=0):
    c=b[i];i+=1
    if c<=0x7f:return c,i
    if c>=0xe0:return c-256,i
    if 0x80<=c<=0x8f:return dmap(b,i,c&15)
    if 0x90<=c<=0x9f:return darr(b,i,c&15)
    if 0xa0<=c<=0xbf:n=c&31;return b[i:i+n].decode('utf8'),i+n
    if c==0xc0:return None,i
    if c==0xc2:return False,i
    if c==0xc3:return True,i
    if c in(0xc4,0xc5,0xc6):
        sz={0xc4:1,0xc5:2,0xc6:4}[c];n=int.from_bytes(b[i:i+sz],'big');i+=sz;return bytes(b[i:i+n]),i+n
    if c==0xca:return struct.unpack('>f',b[i:i+4])[0],i+4
    if c==0xcb:return struct.unpack('>d',b[i:i+8])[0],i+8
    if c in(0xcc,0xcd,0xce,0xcf):sz={0xcc:1,0xcd:2,0xce:4,0xcf:8}[c];return int.from_bytes(b[i:i+sz],'big'),i+sz
    if c in(0xd0,0xd1,0xd2,0xd3):sz={0xd0:1,0xd1:2,0xd2:4,0xd3:8}[c];return int.from_bytes(b[i:i+sz],'big',signed=True),i+sz
    if c in(0xd4,0xd5,0xd6,0xd7,0xd8):
        sz={0xd4:1,0xd5:2,0xd6:4,0xd7:8,0xd8:16}[c];t=b[i];i+=1
        return (UNDEF if t==0 else ('ext',t,b[i:i+sz])),i+sz
    if c in(0xd9,0xda,0xdb):sz={0xd9:1,0xda:2,0xdb:4}[c];n=int.from_bytes(b[i:i+sz],'big');i+=sz;return b[i:i+n].decode('utf8'),i+n
    if c in(0xdc,0xdd):sz=2 if c==0xdc else 4;n=int.from_bytes(b[i:i+sz],'big');return darr(b,i+sz,n)
    if c in(0xde,0xdf):sz=2 if c==0xde else 4;n=int.from_bytes(b[i:i+sz],'big');return dmap(b,i+sz,n)
    raise ValueError(hex(c))
def darr(b,i,n):
    o=[]
    for _ in range(n):v,i=dec(b,i);o.append(v)
    return o,i
def dmap(b,i,n):
    o={}
    for _ in range(n):k,i=dec(b,i);v,i=dec(b,i);o[k]=v
    return o,i
def ty(v):
    if v is UNDEF:return 'undef'
    if isinstance(v,bool):return 'bool'
    if isinstance(v,int):return 'int'
    if isinstance(v,float):return 'float'
    if isinstance(v,str):return 'str'
    if v is None:return 'null'
    if isinstance(v,bytes):return 'bin'
    if isinstance(v,list):return 'arr'
    if isinstance(v,dict):return 'obj'
    return 'ext'
def walk(v,p,cen):
    cen[(p,ty(v))]+=1
    if isinstance(v,dict):
        for k,x in v.items():
            if p.endswith('.slots') or p.endswith('.users') or p.endswith('.roomObjects') or p.endswith('.zones'):k='<key>'
            walk(x,p+'.'+str(k),cen)
    elif isinstance(v,list):
        for x in v[:50]:walk(x,p+'[]',cen)
def frames(path):
    urls={}
    for l in open(path):
        e=json.loads(l)
        if e.get('k')=='ws.open' and 'gaia' in e['url']:urls[e['id']]=e['url']
        if e.get('k') in('ws.in','ws.out') and e.get('op')==2 and e['id'] in urls:
            yield e['t'],('IN' if e['k']=='ws.in' else 'OUT'),e['id'],base64.b64decode(e['data'])
if __name__=='__main__':
    cen=collections.defaultdict(collections.Counter);cnt=collections.Counter()
    for t,d,i,b in frames(sys.argv[1]):
        p,_=dec(b)
        data=p.get('data')
        if isinstance(data,list) and data and isinstance(data[0],str):
            ev=data[0]
            if ev=='s':
                for it in data[1]:
                    ev2='s/%d'%it[0];cnt[(d,ev2)]+=1;walk(it[1],'',cen[(d,ev2)])
                continue
            cnt[(d,ev)]+=1
            for a in data[1:]:walk(a,'',cen[(d,ev)])
    for k in sorted(cnt):
        print('==',k,cnt[k])
        for (path,t),n in sorted(cen[k].items()):print('   ',path or '.',t,n)
