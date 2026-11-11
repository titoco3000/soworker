package dev.soworker.mc;

import com.google.gson.JsonObject;
import com.google.gson.JsonParser;
import java.io.ByteArrayOutputStream;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.WebSocket;
import java.nio.ByteBuffer;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.CompletionStage;
import java.util.concurrent.ConcurrentLinkedQueue;
import java.util.concurrent.atomic.AtomicReference;

/**
 * The WebSocket client to the soworker child process ({@link Child}) on localhost. Connects once the child said its
 * port (and reconnects every 2 s), signs in with the child's token, and queues what arrives; the server thread drains
 * the queue on its tick.
 */
public final class Bridge {
    /** An incoming message: a JSON object, or a binary video frame. */
    public record In(JsonObject json, byte[] binary, boolean connected, boolean disconnected) {}

    private final ConcurrentLinkedQueue<In> inbox = new ConcurrentLinkedQueue<>();
    private final AtomicReference<WebSocket> ws = new AtomicReference<>();
    private final HttpClient http = HttpClient.newHttpClient();
    private final Child child;
    private volatile boolean connecting;
    private volatile long nextTry;
    /** The child's port of the last attempt: a new child (new port) is tried at once. */
    private int lastPort;
    private CompletableFuture<?> sending = CompletableFuture.completedFuture(null);

    public Bridge(Child child) {
        this.child = child;
    }

    public boolean connected() {
        return ws.get() != null;
    }

    public In poll() {
        return inbox.poll();
    }

    /** Call on every tick: starts a connection attempt when there is none. */
    public void tick() {
        long now = System.currentTimeMillis();
        int port = child.port();
        if (connected() || connecting || port == 0) return; // not started, or has not said its port yet
        if (now < nextTry && port == lastPort) return;
        lastPort = port;
        connecting = true;
        nextTry = now + 2000;
        String token = child.token();
        http.newWebSocketBuilder().buildAsync(URI.create("ws://127.0.0.1:" + port), new Listener()).whenComplete((w, err) -> {
            connecting = false;
            if (err != null) return;
            JsonObject auth = new JsonObject();
            auth.addProperty("t", "auth");
            auth.addProperty("token", token);
            w.sendText(auth.toString(), true);
            ws.set(w);
            inbox.add(new In(null, null, true, false));
            SoworkerMod.LOG.info("soworker: connected to the soworker process on port {}", port);
        });
    }

    /** Send a JSON message; dropped when not connected. Sends are chained (the API allows one at a time). */
    public synchronized void send(JsonObject o) {
        WebSocket w = ws.get();
        if (w == null) return;
        String s = o.toString();
        sending = sending.handle((a, b) -> null).thenCompose(x -> w.sendText(s, true));
    }

    private void lost() {
        if (ws.getAndSet(null) != null) {
            inbox.add(new In(null, null, false, true));
            SoworkerMod.LOG.info("soworker: lost the connection to the soworker process");
        }
        nextTry = System.currentTimeMillis() + 2000;
    }

    private final class Listener implements WebSocket.Listener {
        private final StringBuilder text = new StringBuilder();
        private final ByteArrayOutputStream bin = new ByteArrayOutputStream();

        @Override
        public CompletionStage<?> onText(WebSocket w, CharSequence data, boolean last) {
            text.append(data);
            if (last) {
                try {
                    inbox.add(new In(JsonParser.parseString(text.toString()).getAsJsonObject(), null, false, false));
                } catch (Exception e) {
                    SoworkerMod.LOG.warn("soworker: bad message: {}", e.toString());
                }
                text.setLength(0);
            }
            w.request(1);
            return null;
        }

        @Override
        public CompletionStage<?> onBinary(WebSocket w, ByteBuffer data, boolean last) {
            byte[] b = new byte[data.remaining()];
            data.get(b);
            bin.writeBytes(b);
            if (last) {
                inbox.add(new In(null, bin.toByteArray(), false, false));
                bin.reset();
            }
            w.request(1);
            return null;
        }

        @Override
        public CompletionStage<?> onClose(WebSocket w, int code, String reason) {
            lost();
            return null;
        }

        @Override
        public void onError(WebSocket w, Throwable error) {
            lost();
        }
    }
}
