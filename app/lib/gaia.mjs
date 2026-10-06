// Socket.IO v4 + msgpack client for SoWork's gaia realtime server (see docs/gaia-protocol.md).
import { EventEmitter } from 'node:events';
import { randomBytes } from 'node:crypto';
import { encode, decode } from './msgpack.mjs';
import { decodeSetMapObject } from './mapobject.mjs';

export const CLIENT_VERSION = '11.202.0';

export class Gaia extends EventEmitter {
  constructor({ auth, roomId, mediaRegion = 'eu-central-1' }) {
    super();
    this.auth = auth;
    this.roomId = roomId;
    this.mediaRegion = mediaRegion;
    this.autoForceId = randomBytes(16).toString('base64url').slice(0, 21);
    this.clusterMode = 'none-connecting';
    this.pos = null;
    this.firstConnect = true;
    this.closed = false;
  }

  async connect() {
    const token = await this.auth.idToken();
    const q = new URLSearchParams({
      roomId: this.roomId, mediaRegion: this.mediaRegion, password: '', clientVersion: CLIENT_VERSION,
      autoForceId: this.autoForceId, clusterMode: this.clusterMode, meetingZoneId: '',
      ...(this.firstConnect ? { joinType: 'force' } : {}),
      ...(this.pos ? { x: String(this.pos.x), y: String(this.pos.y) } : {}),
      EIO: '4', transport: 'websocket',
    });
    const reconnecting = !this.firstConnect;
    this.firstConnect = false;
    const ws = (this.ws = new WebSocket(`wss://api.sowork.com/gaia/g5/v2/?${q}`));
    ws.binaryType = 'arraybuffer';
    ws.onmessage = (m) => {
      if (typeof m.data === 'string') {
        if (m.data === '2') ws.send('3');
        else if (m.data[0] === '0') this.#packet({ type: 0, data: { token }, nsp: '/' });
        return;
      }
      let p;
      try { p = decode(Buffer.from(m.data)); } catch (e) { return this.emit('error', e); }
      if (p.type === 0) {
        this.emit('connected');
        if (reconnecting) this.emit_('user_reconnected', { clusterMode: this.clusterMode, zoneId: null, shouldUpdate: true });
        this.emit_('sync_time', { clientTime: Date.now() }, 0);
      } else if (p.type === 4) this.emit('error', new Error('connect error: ' + JSON.stringify(p.data)));
      else if (p.type === 2) this.#event(p.data[0], p.data[1]);
    };
    ws.onerror = (e) => this.emit('error', new Error(e.message ?? 'socket error'));
    ws.onclose = () => {
      this.emit('disconnected');
      if (!this.closed) setTimeout(() => this.connect().catch((e) => this.emit('error', e)), 2000);
    };
  }

  close() { this.closed = true; this.ws?.close(); }

  #packet(p) { if (this.ws?.readyState === WebSocket.OPEN) this.ws.send(encode(p)); }
  emit_(name, payload, ack) {
    this.#packet({ type: 2, data: payload === undefined ? [name] : [name, payload], options: { compress: true }, ...(ack !== undefined ? { id: ack } : {}), nsp: '/' });
  }

  #event(name, payload) {
    if (name === 'init') {
      this.emit_('should_update', { v: true, t: 0, lastObjectId: '' });
      this.emit_('active_app', { activeApp: null });
    } else if (name === 'b_set_obj') this.emit('object', decodeSetMapObject(payload));
    else this.emit(name, payload);
  }

  // ---- actions (same messages the web client sends)
  move(x, y) { this.pos = { x, y }; this.emit_('move', { x, y }); }
  characterState(state) { this.emit_('character_state', { characterState: { state } }); }
  setClusterMode(mode) { this.clusterMode = mode; this.emit_('cluster_mode', { mode }); }
}
