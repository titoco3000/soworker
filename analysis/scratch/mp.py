import struct,json,base64,re,sys
def dec(b,i=0):
    c=b[i];i+=1
    if c<=0x7f:return c,i
    if c>=0xe0:return c-256,i
    if 0x80<=c<=0x8f:return mp_map(b,i,c&15)
    if 0x90<=c<=0x9f:return mp_arr(b,i,c&15)
    if 0xa0<=c<=0xbf:
        n=c&31;return b[i:i+n].decode('utf8','replace'),i+n
    if c==0xc0:return None,i
    if c==0xc2:return False,i
    if c==0xc3:return True,i
    if c in(0xc4,0xc5,0xc6):
        k={0xc4:1,0xc5:2,0xc6:4}[c];n=int.from_bytes(b[i:i+k],'big');i+=k;return ('bin',b[i:i+n]),i+n
    if c in(0xc7,0xc8,0xc9):
        k={0xc7:1,0xc8:2,0xc9:4}[c];n=int.from_bytes(b[i:i+k],'big');i+=k;t=b[i];return ('ext',t,b[i+1:i+1+n]),i+1+n
    if c==0xca:return struct.unpack('>f',b[i:i+4])[0],i+4
    if c==0xcb:return struct.unpack('>d',b[i:i+8])[0],i+8
    if 0xcc<=c<=0xcf:
        k=1<<(c-0xcc);return int.from_bytes(b[i:i+k],'big'),i+k
    if 0xd0<=c<=0xd3:
        k=1<<(c-0xd0);return int.from_bytes(b[i:i+k],'big',signed=True),i+k
    if 0xd4<=c<=0xd8:
        k=1<<(c-0xd4);t=b[i];return ('ext',t,b[i+1:i+1+k]),i+1+k
    if c in(0xd9,0xda,0xdb):
        k={0xd9:1,0xda:2,0xdb:4}[c];n=int.from_bytes(b[i:i+k],'big');i+=k;return b[i:i+n].decode('utf8','replace'),i+n
    if c in(0xdc,0xdd):
        k=2 if c==0xdc else 4;n=int.from_bytes(b[i:i+k],'big');return mp_arr(b,i+k,n)
    if c in(0xde,0xdf):
        k=2 if c==0xde else 4;n=int.from_bytes(b[i:i+k],'big');return mp_map(b,i+k,n)
    raise ValueError(hex(c))
def mp_arr(b,i,n):
    o=[]
    for _ in range(n):
        v,i=dec(b,i);o.append(v)
    return o,i
def mp_map(b,i,n):
    o={}
    for _ in range(n):
        k,i=dec(b,i);v,i=dec(b,i);o[k]=v
    return o,i
