// Injected into every page and frame before any site script runs.
// Reports WebRTC and media activity to the recorder through the __rec binding.
// Must never throw into site code, and must not change behavior.
(() => {
  const g = globalThis;
  if (g.__recHooked) return;
  Object.defineProperty(g, '__recHooked', { value: true });

  const MAX = 64 * 1024; // cap on logged bytes per data channel message
  let seqN = 0;
  let pcN = 0;
  let dcN = 0;

  const send = (o) => {
    try {
      o.ts = Date.now();
      o.href = g.location && g.location.href;
      g.__rec(JSON.stringify(o));
    } catch {}
  };

  const b64 = (u8) => {
    let s = '';
    for (let i = 0; i < u8.length; i += 0x8000) s += String.fromCharCode.apply(null, u8.subarray(i, i + 0x8000));
    return btoa(s);
  };

  async function payload(d) {
    if (typeof d === 'string') return { text: d.slice(0, MAX), len: d.length };
    if (typeof Blob !== 'undefined' && d instanceof Blob) d = await d.arrayBuffer();
    const u8 = ArrayBuffer.isView(d) ? new Uint8Array(d.buffer, d.byteOffset, d.byteLength) : new Uint8Array(d);
    return { b64: b64(u8.subarray(0, MAX)), len: u8.byteLength };
  }

  function watchChannel(ch, pc, origin) {
    const dc = ++dcN;
    const meta = () => ({
      pc,
      dc,
      label: ch.label,
      dcid: ch.id,
      ordered: ch.ordered,
      protocol: ch.protocol,
      negotiated: ch.negotiated,
      maxRetransmits: ch.maxRetransmits,
      state: ch.readyState,
    });
    send({ k: 'dc.new', origin, ...meta() });
    for (const ev of ['open', 'close', 'closing', 'error']) {
      ch.addEventListener(ev, () => send({ k: 'dc.' + ev, ...meta() }));
    }
    ch.addEventListener('message', async (e) => {
      const seq = ++seqN;
      send({ k: 'dc.in', pc, dc, seq, ...(await payload(e.data)) });
    });
    const origSend = ch.send;
    ch.send = function (d) {
      const seq = ++seqN;
      payload(d).then((p) => send({ k: 'dc.out', pc, dc, seq, ...p }));
      return origSend.apply(ch, arguments);
    };
  }

  const desc = (d) => d && { type: d.type, sdp: d.sdp };
  const STAT_TYPES = new Set([
    'candidate-pair',
    'local-candidate',
    'remote-candidate',
    'codec',
    'inbound-rtp',
    'outbound-rtp',
    'transport',
  ]);

  function watchPc(pc, config) {
    const id = ++pcN;
    send({ k: 'pc.new', pc: id, config: config || null });

    pc.addEventListener('icecandidate', (e) =>
      send({ k: 'pc.icecandidate', pc: id, candidate: e.candidate ? e.candidate.toJSON() : null }),
    );
    for (const ev of [
      'iceconnectionstatechange',
      'connectionstatechange',
      'signalingstatechange',
      'icegatheringstatechange',
      'negotiationneeded',
    ]) {
      pc.addEventListener(ev, () =>
        send({
          k: 'pc.' + ev,
          pc: id,
          ice: pc.iceConnectionState,
          conn: pc.connectionState,
          sig: pc.signalingState,
          gather: pc.iceGatheringState,
        }),
      );
    }
    pc.addEventListener('track', (e) =>
      send({
        k: 'pc.track',
        pc: id,
        kind: e.track.kind,
        trackId: e.track.id,
        streams: e.streams.map((s) => s.id),
        mid: e.transceiver && e.transceiver.mid,
      }),
    );
    pc.addEventListener('datachannel', (e) => watchChannel(e.channel, id, 'remote'));

    const argOf = (name, a0) => {
      if (name === 'setLocalDescription' || name === 'setRemoteDescription') return desc(a0);
      if (name === 'addIceCandidate') return a0 && a0.toJSON ? a0.toJSON() : a0;
      return a0 || null; // createOffer/createAnswer options
    };
    const wrap = (name, after) => {
      const orig = pc[name];
      if (typeof orig !== 'function') return;
      pc[name] = function (...a) {
        send({ k: 'pc.' + name, pc: id, arg: argOf(name, a[0]) });
        const r = orig.apply(pc, a);
        if (r && typeof r.then === 'function') {
          r.then(
            (v) => after && after(v),
            (err) => send({ k: 'pc.error', pc: id, op: name, err: String(err) }),
          );
        }
        return r;
      };
    };
    wrap('createOffer', (v) => send({ k: 'pc.createOffer.result', pc: id, ...desc(v) }));
    wrap('createAnswer', (v) => send({ k: 'pc.createAnswer.result', pc: id, ...desc(v) }));
    wrap('setLocalDescription', () => send({ k: 'pc.localDescription', pc: id, ...desc(pc.localDescription) }));
    wrap('setRemoteDescription', () => send({ k: 'pc.remoteDescription', pc: id, ...desc(pc.remoteDescription) }));
    wrap('addIceCandidate');

    const addTrack = pc.addTrack;
    pc.addTrack = function (track) {
      send({ k: 'pc.addTrack', pc: id, kind: track && track.kind, label: track && track.label });
      return addTrack.apply(pc, arguments);
    };
    const addTransceiver = pc.addTransceiver;
    pc.addTransceiver = function (t, init) {
      send({
        k: 'pc.addTransceiver',
        pc: id,
        kind: typeof t === 'string' ? t : t && t.kind,
        direction: init && init.direction,
      });
      return addTransceiver.apply(pc, arguments);
    };
    const createDataChannel = pc.createDataChannel;
    pc.createDataChannel = function () {
      const ch = createDataChannel.apply(pc, arguments);
      watchChannel(ch, id, 'local');
      return ch;
    };
    const close = pc.close;
    pc.close = function () {
      send({ k: 'pc.close', pc: id });
      return close.apply(pc, arguments);
    };

    const timer = setInterval(async () => {
      if (pc.connectionState === 'closed') return clearInterval(timer);
      try {
        const stats = [];
        (await pc.getStats()).forEach((r) => STAT_TYPES.has(r.type) && stats.push(r));
        send({ k: 'pc.stats', pc: id, stats });
      } catch {}
    }, 15000);
  }

  if (g.RTCPeerConnection) {
    g.RTCPeerConnection = new Proxy(g.RTCPeerConnection, {
      construct(target, args, newTarget) {
        const pc = Reflect.construct(target, args, newTarget);
        try {
          watchPc(pc, args[0]);
        } catch {}
        return pc;
      },
    });
  }

  const md = g.navigator && g.navigator.mediaDevices;
  if (md) {
    for (const name of ['getUserMedia', 'getDisplayMedia']) {
      const orig = md[name];
      if (typeof orig !== 'function') continue;
      md[name] = function (constraints) {
        send({ k: 'media.' + name, constraints });
        return orig.apply(md, arguments);
      };
    }
  }
})();
