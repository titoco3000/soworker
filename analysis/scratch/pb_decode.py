"""Generic protobuf wire decoder + extractor of Socket.IO msgpack 'b_set_obj' payloads from recorder events.jsonl.
Usage: python3 pb_decode.py <events.jsonl> [N]   (prints structure of first N b_set_obj frames; no secrets in these)"""
import json,base64,struct,sys,collections

def rv(b,i):
    r=0;s=0
    while True:
        c=b[i];i+=1;r|=(c&0x7f)<<s;s+=7
        if not c&0x80:return r,i

def pb(b,depth=0,maxd=4):
    i=0;out=[]
    while i<len(b):
        k,i=rv(b,i);f,w=k>>3,k&7
        if w==0:v,i=rv(b,i);out.append((f,'v',v))
        elif w==1:out.append((f,'d',struct.unpack('<d',b[i:i+8])[0]));i+=8
        elif w==5:out.append((f,'f',round(struct.unpack('<f',b[i:i+4])[0],4)));i+=4
        elif w==2:
            n,i=rv(b,i);x=b[i:i+n];i+=n
            sub=None
            if depth<maxd and n>1:
                try:
                    sub=pb(x,depth+1,maxd)
                    if sub and sum(1 for _ in sub)==0: sub=None
                except Exception: sub=None
            try:
                t=x.decode('ascii')
                printable=all(32<=ord(c)<127 for c in t)
            except Exception: printable=False
            if printable and n>0: out.append((f,'s',t))
            elif sub: out.append((f,'m',sub))
            else: out.append((f,'b',x.hex()))
        else: raise ValueError('wire %d'%w)
    return out

def msgpack_frames(path):
    # minimal: find binary payload of b_set_obj in msgpack: search for the str 'b_set_obj' then bin8/16/32 marker
    for l in open(path):
        e=json.loads(l)
        if e.get('k')=='ws.in' and e.get('op')==2:
            b=base64.b64decode(e['data'])
            j=b.find(b'b_set_obj')
            if j<0: continue
            j+=9
            m=b[j]
            if m==0xc4:n=b[j+1];p=j+2
            elif m==0xc5:n=struct.unpack('>H',b[j+1:j+3])[0];p=j+3
            elif m==0xc6:n=struct.unpack('>I',b[j+1:j+5])[0];p=j+5
            else: continue
            yield e['t'],b[p:p+n]

if __name__=='__main__':
    N=int(sys.argv[2]) if len(sys.argv)>2 else 5
    fields=collections.Counter()
    for k,(t,pl) in enumerate(msgpack_frames(sys.argv[1])):
        tree=pb(pl)
        def walk(tr,path=''):
            for f,ty,v in tr:
                fields[(path+str(f),ty)]+=1
                if ty=='m':walk(v,path+str(f)+'.')
        walk(tree)
        if k<N: print(t,len(pl),tree)
    print('--- field census (path,type)->count')
    for k,v in sorted(fields.items()):print(k,v)

# --- google.protobuf.Struct decoding from RAW bytes (typed, no heuristics) ---
def raw_fields(b):
    i=0;out=[]
    while i<len(b):
        k,i=rv(b,i);f,w=k>>3,k&7
        if w==0:v,i=rv(b,i)
        elif w==1:v=struct.unpack('<d',b[i:i+8])[0];i+=8
        elif w==5:v=struct.unpack('<f',b[i:i+4])[0];i+=4
        else:
            n,i=rv(b,i);v=b[i:i+n];i+=n
        out.append((f,w,v))
    return out
def pb_value(b):
    for f,w,v in raw_fields(b):
        if f==1:return None
        if f==2:return v
        if f==3:return v.decode()
        if f==4:return bool(v)
        if f==5:return pb_struct(v)
        if f==6:return [pb_value(x[2]) for x in raw_fields(v) if x[0]==1]
def pb_struct(b):
    o={}
    for f,w,v in raw_fields(b):
        if f==1:
            e={ff:vv for ff,ww,vv in raw_fields(v)}
            o[e[1].decode()]=pb_value(e[2]) if 2 in e else None
    return o
def pb_map_object(b):
    """Decode PbMapObject (see main/httpClientFactory bundle: PbMapObject fields) into a dict."""
    names={1:'type',2:'x',3:'y',4:'assetId',5:'orientation',6:'sizeX',7:'sizeY',8:'perms',9:'interaction',10:'config',11:'spawnDirection',13:'colorPalette',14:'colorKey',15:'updatedBy',16:'updatedAt',17:'tags',18:'accessKey'}
    o={}
    for f,w,v in raw_fields(b):
        n=names.get(f,'f%d'%f)
        if f in(8,9,10):v=pb_struct(v)
        elif f in(4,14,15,18):v=v.decode()
        elif f==17:o.setdefault('tags',[]).append(v.decode());continue
        elif f==13:v={('hue','sat','val')[ff-1]:vv for ff,ww,vv in raw_fields(v)}
        o[n]=v
    return o
