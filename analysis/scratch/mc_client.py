#!/usr/bin/env python3
"""Stand-in for the Minecraft mod: connects to `soworker --mc-child`, signs in with the token and prints what arrives.

Start the child yourself with a token of your choosing; it prints `port <n>` first:

    export SOWORKER_MC_TOKEN=$(openssl rand -hex 24)
    target/mc/release/soworker --mc-child --mc-demo        # port 41234
    python3 analysis/scratch/mc_client.py --port 41234 [--map] [--frames DIR] [--quiet-users]

Type on stdin to talk back:
    pos X Z          the player stands at block (X, Z)   (`pos! X Z` = with a movement key pressed)
    mic | share | rec | cam | desk | away | hand | leave | status | login | logout
    wave NAME | say TEXT
    {"t": ...}       any raw JSON message
No dependencies: a minimal WebSocket client on a plain socket.
"""
import argparse, base64, json, os, socket, struct, sys, threading

BASE = [0, 8368696, 16247203, 13092807, 16711680, 10526975, 10987431, 31744, 16777215, 10791096, 9923917, 7368816, 4210943,
        9402184, 16776437, 14188339, 11685080, 6724056, 15066419, 8375321, 15892389, 5000268, 10066329, 5013401, 8339378,
        3361970, 6704179, 6717235, 10040115, 1644825, 16445005, 6085589, 4882687, 55610, 8476209, 7340544, 13742497,
        10441252, 9787244, 7367818, 12223780, 6780213, 10505550, 3746083, 8874850, 5725276, 8014168, 4996700, 4993571,
        5001770, 9321518, 2430480, 12398641, 9715553, 6035741, 1474182, 3837580, 5647422, 1356933, 6579300, 14200723, 8365974]
SHADE = [180, 220, 255, 135]


def rgb(i):
    b, s = i >> 2, i & 3
    if b == 0 or b >= len(BASE):
        return (0, 0, 0)
    c = BASE[b]
    return tuple(((c >> sh) & 255) * SHADE[s] // 255 for sh in (16, 8, 0))


class WS:
    def __init__(self, host, port):
        self.s = socket.create_connection((host, port))
        key = base64.b64encode(os.urandom(16)).decode()
        self.s.sendall((f"GET / HTTP/1.1\r\nHost: {host}:{port}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\n"
                        f"Sec-WebSocket-Key: {key}\r\nSec-WebSocket-Version: 13\r\n\r\n").encode())
        head = b""
        while b"\r\n\r\n" not in head:
            chunk = self.s.recv(1)
            if not chunk:
                raise SystemExit("closed during handshake")
            head += chunk
        if b" 101 " not in head.split(b"\r\n")[0]:
            raise SystemExit("handshake refused: " + head.decode(errors="replace"))
        self.lock = threading.Lock()

    def _exact(self, n):
        b = b""
        while len(b) < n:
            c = self.s.recv(n - len(b))
            if not c:
                raise EOFError
            b += c
        return b

    def recv(self):
        """(opcode, payload) of the next whole message."""
        data, op0 = b"", None
        while True:
            h = self._exact(2)
            fin, op, ln = h[0] & 0x80, h[0] & 15, h[1] & 127
            if ln == 126:
                ln = struct.unpack(">H", self._exact(2))[0]
            elif ln == 127:
                ln = struct.unpack(">Q", self._exact(8))[0]
            mask = self._exact(4) if h[1] & 0x80 else None
            p = self._exact(ln)
            if mask:
                p = bytes(b ^ mask[i % 4] for i, b in enumerate(p))
            if op == 9:  # ping
                self._send(10, p)
                continue
            if op in (1, 2, 8):
                op0 = op
            data += p
            if fin:
                return op0, data

    def _send(self, op, payload):
        mask = os.urandom(4)
        n = len(payload)
        hdr = bytes([0x80 | op])
        if n < 126:
            hdr += bytes([0x80 | n])
        elif n < 65536:
            hdr += bytes([0x80 | 126]) + struct.pack(">H", n)
        else:
            hdr += bytes([0x80 | 127]) + struct.pack(">Q", n)
        with self.lock:
            self.s.sendall(hdr + mask + bytes(b ^ mask[i % 4] for i, b in enumerate(payload)))

    def send_json(self, v):
        self._send(1, json.dumps(v).encode())


def draw_map(m):
    """The block map as text: . grass, # cobblestone (meeting), | fence, x barrier."""
    cells = {}
    for k, ch in (("barriers", "x"), ("floors", "."), ("meeting", "#"), ("fences", "|")):
        for i, j in m[k]:
            cells[(i, j)] = ch
    if not cells:
        return "(empty map)"
    i0, i1 = min(c[0] for c in cells), max(c[0] for c in cells)
    j0, j1 = min(c[1] for c in cells), max(c[1] for c in cells)
    return "\n".join("".join(cells.get((i, j), " ") for i in range(i0, i1 + 1)) for j in range(j0, j1 + 1))


def reader(ws, args):
    last_users = None
    while True:
        try:
            op, p = ws.recv()
        except EOFError:
            print("** closed by soworker", flush=True)
            os._exit(0)
        if op == 8:
            print("** close frame", p[2:].decode(errors="replace"), flush=True)
            os._exit(0)
        if op == 2:
            kind, kl = p[0], struct.unpack("<H", p[1:3])[0]
            key = p[3:3 + kl].decode()
            w, h = struct.unpack("<HH", p[3 + kl:7 + kl])
            px = p[7 + kl:]
            if kind == 3:  # the full-screen view of a share: a JPEG
                print(f"<- full {key!r} {w}x{h} (JPEG, {len(p) - 7 - kl} bytes)")
                continue
            face = kind == 2  # a camera for a face: RGB, not map colours
            want = w * h * (3 if face else 1)
            print(f"<- {'face' if face else 'video'} {key!r} {w}x{h} ({len(px)} bytes{'' if len(px) == want else ', WRONG SIZE'})")
            if args.frames:
                os.makedirs(args.frames, exist_ok=True)
                name = os.path.join(args.frames, key.replace(":", "_").replace("/", "_") + ".ppm")
                with open(name, "wb") as f:
                    f.write(f"P6 {w} {h} 255\n".encode() + (bytes(px) if face else b"".join(bytes(rgb(i)) for i in px)))
            continue
        v = json.loads(p)
        t = v.get("t")
        if t == "world":
            print(f"<- world origin {v['origin']}: {len(v['floors'])} grass, {len(v['meeting'])} cobblestone, "
                  f"{len(v['fences'])} fences, {len(v['barriers'])} barriers, zones:")
            for z in v["zones"]:
                print(f"     {z['id']} {z['name']!r} blocks ({z['i0']},{z['j0']})..({z['i1']},{z['j1']})")
            if args.map:
                print(draw_map(v))
        elif t == "users":
            if args.quiet_users:
                names = sorted(u["name"] for u in v["list"])
                if names != last_users:
                    print(f"<- users ({len(names)}): {', '.join(names)}")
                last_users = names
            else:
                print(f"<- users ({len(v['list'])})")
                for u in v["list"]:
                    flags = [f for f in ("walking", "away", "speaking", "muted") if u.get(f)]
                    if u.get("hand"):
                        flags.append(f"hand #{u['hand']}")
                    print(f"     {u['name']:<20} ({u['x']:7.2f}, {u['z']:7.2f}) {' '.join(flags)} zone={u.get('zone')}")
        else:
            print("<-", json.dumps(v, ensure_ascii=False))


def main():
    sys.stdout.reconfigure(line_buffering=True)
    ap = argparse.ArgumentParser()
    ap.add_argument("--port", type=int, required=True, help="the port from the child's `port <n>` line")
    ap.add_argument("--token", default=os.environ.get("SOWORKER_MC_TOKEN"), help="default: $SOWORKER_MC_TOKEN")
    ap.add_argument("--map", action="store_true", help="draw the block map as text")
    ap.add_argument("--frames", help="save the latest video frame of each screen as DIR/<key>.ppm")
    ap.add_argument("--quiet-users", action="store_true", help="only print the user list when someone joins or leaves")
    ap.add_argument("--bad-token", action="store_true", help="send a wrong token (soworker should close the socket)")
    args = ap.parse_args()
    if not args.token and not args.bad_token:
        raise SystemExit("set SOWORKER_MC_TOKEN (the token the child was started with) or pass --token")
    token = "wrong" if args.bad_token else args.token
    ws = WS("127.0.0.1", args.port)
    ws.send_json({"t": "auth", "token": token})
    threading.Thread(target=reader, args=(ws, args), daemon=True).start()
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        if line.startswith("{"):
            ws.send_json(json.loads(line))
            continue
        word, _, rest = line.partition(" ")
        if word in ("pos", "pos!"):
            x, z = map(float, rest.split())
            ws.send_json({"t": "pos", "x": x, "z": z, "input": word == "pos!"})
        else:
            msg = {"t": "cmd", "name": word}
            if rest:
                msg["arg"] = rest
            ws.send_json(msg)


if __name__ == "__main__":
    main()
