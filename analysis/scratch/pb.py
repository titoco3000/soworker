import json,base64,sys
L="/home/tito/dev/repos/soworker/recorder/raw-logs/2026-10-06T12-48-44-206Z/events.jsonl"
def varint(b,i):
    r=0;s=0
    while True:
        x=b[i];i+=1;r|=(x&0x7f)<<s;s+=7
        if x<128:return r,i
def parse(b):
    i=0;out=[]
    while i<len(b):
        k,i=varint(b,i);f,w=k>>3,k&7
        if w==0:v,i=varint(b,i)
        elif w==2:
            n,i=varint(b,i);v=b[i:i+n];i+=n
        elif w==1:v=b[i:i+8];i+=8
        elif w==5:v=b[i:i+4];i+=4
        else:raise ValueError(w)
        out.append((f,w,v))
    return out
SIGRESP={1:'join',2:'answer',3:'offer',4:'trickle',5:'update',6:'track_published',8:'leave',9:'mute',10:'speakers_changed',11:'room_update',12:'connection_quality',13:'stream_state_update',14:'subscribed_quality_update',15:'subscription_permission_update',16:'refresh_token',17:'track_unpublished',18:'pong',19:'reconnect',20:'pong_resp',21:'subscription_response',22:'request_response',23:'track_subscribed',24:'room_moved'}
SIGREQ={1:'offer',2:'answer',3:'trickle',4:'add_track',5:'mute',6:'subscription',7:'track_setting',8:'leave',9:'update_layers',10:'subscription_permission',11:'sync_state',12:'simulate',13:'ping',14:'update_metadata',15:'ping_req',16:'update_audio_track',17:'update_video_track'}
def load():
    ev=[json.loads(l) for l in open(L)]
    t0=None
    ids={}
    for e in ev:
        if e['k']=='ws.open' and 'livekit' in e['url']: ids[e['id']]=e['url'].split('/rtc')[0]
    return ev,ids
