import json,re,base64
L='/home/tito/dev/repos/soworker/recorder/raw-logs/2026-10-06T12-48-44-206Z/events.jsonl'
def jwt(s):
    p=s.split('.')[1]; p+='='*(-len(p)%4); return json.loads(base64.urlsafe_b64decode(p))
req={}
for l in open(L):
    e=json.loads(l)
    if e['k']=='req': req[e['id']]=e
    if e['k']=='res.body' and req.get(e['id'],{}).get('url','').endswith('/calendar/token'):
        c=jwt(json.loads(e['text'])['token'])
        print({k:(v if k in('iss','aud','sub','uid') and k!='uid' and k!='sub' else type(v).__name__) for k,v in c.items()}, c['exp']-c['iat'], 'sub==iss?',c['iss']==c['sub'])
    if e['k']=='req.postData' and 'signInWithCustomToken' in req.get(e['id'],{}).get('url',''):
        d=json.loads(e.get('text') or e.get('data') or '{}'); c=jwt(d['token']); print('customToken aud',c['aud'])
    if e['k']=='res.body' and 'securetoken' in req.get(e['id'],{}).get('url',''): print('securetoken!')
