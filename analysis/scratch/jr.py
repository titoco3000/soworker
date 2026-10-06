import sys;sys.path.insert(0,'.')
from pb import *
import urllib.parse as up,re,time
ev,ids=load();T0=ev[0]['t']
for e in ev:
    if e['k']=='ws.open' and 'livekit' in e['url']:
        u=up.urlparse(e['url']);q=up.parse_qs(u.query)
        print('==',(e['t']-T0)/1000,u.netloc,u.path,{k:(len(v[0]) if k in('access_token','join_request') else v) for k,v in q.items()})
        tok=q['access_token'][0].split('.')
        for i,p in enumerate(tok[:2]):
            d=json.loads(base64.urlsafe_b64decode(p+'='*(-len(p)%4)))
            if i==0: print('header',d)
            else:
                def red(o,path=''):
                    if isinstance(o,dict): return {k:red(v,k) for k,v in o.items()}
                    if path in('sub','name','email','identity','metadata','room','jti','iss'): return '<%s:%s>'%(type(o).__name__,len(str(o)))
                    return o
                print('payload',red(d))
                print('exp-nbf', d.get('exp',0)-d.get('nbf',0),'exp-iat',d.get('exp',0)-d.get('iat',0) if 'iat' in d else None, 'nbf-conn(s)',d.get('nbf',0)-(e['t']/1000))
        jr=q['join_request'][0]
        b=base64.urlsafe_b64decode(jr+'='*(-len(jr)%4))
        def walk(b,ind=0):
            for f,w,v in parse(b):
                if w==2:
                    try:
                        s=v.decode();
                        if s.isprintable() and s:
                            if len(s)>40: s='<%dchars>'%len(s)
                            print(' '*ind,f,'str',s);continue
                    except: pass
                    try:
                        parse(v);print(' '*ind,f,'msg');walk(v,ind+2)
                    except: print(' '*ind,f,'bytes',len(v))
                else: print(' '*ind,f,w,v if w==0 else v.hex())
        walk(b)
print("---- unwrapped")
import gzip
for e in ev:
    if e['k']=='ws.open' and 'livekit' in e['url']:
        q=up.parse_qs(up.urlparse(e['url']).query);jr=q['join_request'][0]
        b=base64.urlsafe_b64decode(jr+'='*(-len(jr)%4))
        inner=[v for f,w,v in parse(b) if f==2][0]
        raw=gzip.decompress(inner)
        print('== inner',len(raw))
        def walk(b,ind=0):
            for f,w,v in parse(b):
                if w==2:
                    try:
                        s=v.decode()
                        if s.isprintable() and s:
                            if len(s)>60 and not ' ' in s: s='<%dchars>'%len(s)
                            print(' '*ind,f,'str',s[:200]);continue
                    except: pass
                    try:
                        parse(v);print(' '*ind,f,'msg');walk(v,ind+2)
                    except: print(' '*ind,f,'bytes',len(v))
                else: print(' '*ind,f,w,v if w==0 else v.hex())
        walk(raw)
