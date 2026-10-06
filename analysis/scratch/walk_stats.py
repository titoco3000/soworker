import re,json,math,statistics as st,sys
f=sys.argv[1] if len(sys.argv)>1 else 'work/ws/07-api.sowork.com.txt'
mv=[];ins=[]
for l in open(f):
    m=re.match(r'([\d.]+) (IN|OUT) +MP (\{.*)',l)
    if not m: continue
    try: d=json.loads(m.group(3))
    except Exception: continue
    dd=d.get('data')
    if not isinstance(dd,list):continue
    t=float(m.group(1))
    if m.group(2)=='OUT' and dd[0]=='move': mv.append((t,dd[1]['x'],dd[1]['y']))
    if m.group(2)=='IN' and dd[0]=='s':
        own=[it for it in dd[1] if it[0]==1 and it[1]['userId'].startswith('RMUY')]
        if own: ins.append((t,own[0][1]['x'],own[0][1]['y']))
segs=[];cur=[mv[0]]
for p in mv[1:]:
    if p[0]-cur[-1][0]>1.0: segs.append(cur);cur=[p]
    else: cur.append(p)
segs.append(cur)
for s in segs:
    if len(s)<3: continue
    out=[]
    for a,b in zip(s,s[1:]):
        dt=b[0]-a[0];dx=b[1]-a[1];dy=b[2]-a[2]
        out.append((round(dt,3),round(dx,1),round(dy,1),round(math.hypot(dx,dy)/dt) if dt>0.1 else None))
    print(round(s[0][0],2),len(s),out[:14])
dts=[];sp=[]
for s in segs:
    for a,b in zip(s,s[1:]):
        dt=b[0]-a[0]
        if 0.15<dt<0.3: dts.append(dt); sp.append(math.hypot(b[1]-a[1],b[2]-a[2])/dt)
print('dt mean',st.mean(dts),st.pstdev(dts),'speed',st.mean(sp),st.pstdev(sp),len(dts))
print('step len', [round(math.hypot(b[1]-a[1],b[2]-a[2]),1) for s in segs for a,b in zip(s,s[1:]) if 0.15<b[0]-a[0]<0.3][:30])
d=[b[0]-a[0] for a,b in zip(ins,ins[1:])]
print('in own intervals',[round(x,3) for x in d][:40])
