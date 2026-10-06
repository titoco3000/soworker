import json, sys
from PIL import Image, ImageDraw
objs=[json.loads(l)['data'] for l in open('/tmp/claude-1000/objects.jsonl')]
G=105.46875
def iso(x,y): return ((x-y)/2, (x+y)/4)
def render(proj, path, W=1800):
    pts=[]
    for o in objs:
        pts.append(proj(o['x'],o['y']))
    xs=[p[0] for p in pts]; ys=[p[1] for p in pts]
    x0,x1,y0,y1=min(xs)-200,max(xs)+200,min(ys)-200,max(ys)+200
    s=W/(x1-x0); H=int((y1-y0)*s)
    im=Image.new('RGB',(W,H),(24,26,32)); d=ImageDraw.Draw(im)
    T=lambda x,y:((proj(x,y)[0]-x0)*s,(proj(x,y)[1]-y0)*s)
    col={'floor':(70,90,120),'wall':(230,200,120),'asset':(120,200,140),'zone':(220,80,120)}
    for typ in ['floor','wall','asset','zone']:
        for o in objs:
            if o['type']!=typ: continue
            x,y=o['x'],o['y']
            if typ=='floor':
                d.polygon([T(x-G,y-G),T(x+G,y-G),T(x+G,y+G),T(x-G,y+G)],fill=col[typ],outline=(40,50,70))
            elif typ=='zone':
                a=o.get('sizeX',282)/2; b=o.get('sizeY',282)/2
                d.polygon([T(x-a,y-b),T(x+a,y-b),T(x+a,y+b),T(x-a,y+b)],outline=col[typ])
            elif typ=='wall':
                d.ellipse([*(T(x,y)[0]-2,T(x,y)[1]-2),*(T(x,y)[0]+2,T(x,y)[1]+2)],fill=col[typ])
            else:
                px,py=T(x,y); d.ellipse([px-1.5,py-1.5,px+1.5,py+1.5],fill=col[typ])
    sx,sy=T(3260.1191,979.6182); d.ellipse([sx-6,sy-6,sx+6,sy+6],outline=(255,255,255))
    im.save(path); print(path,im.size)
render(lambda x,y:(x,y),'/tmp/claude-1000/map_topdown.png')
render(iso,'/tmp/claude-1000/map_iso.png')
