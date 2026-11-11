package dev.soworker.mc;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;
import java.security.MessageDigest;
import java.security.SecureRandom;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.HexFormat;
import java.util.List;
import java.util.Locale;
import java.util.concurrent.TimeUnit;

/**
 * The soworker process this mod carries ({@code natives/<os>-x86_64/soworker[.exe]} in the jar): extracted to the
 * soworker config folder on first use, started as {@code soworker --mc-child}, and owned through its stdin. The child
 * ends when stdin closes, so it goes with Minecraft even when Minecraft crashes. It prints {@code port <n>} first; the
 * token it wants is handed over in {@code SOWORKER_MC_TOKEN}, fresh for every start.
 *
 * <p>Development: {@code -Dsoworker.bin=<path>} runs that binary instead of the bundled one, and
 * {@code -Dsoworker.args="--mc-demo"} adds arguments.
 */
public final class Child {
    /** Restarts after an unexpected exit allowed per minute; past that the user has to {@code /sw enable} again. */
    static final int MAX_RESTARTS = 3;

    private Process proc;
    private OutputStream stdin;
    private volatile int port;
    private volatile String token = "";
    /** Bumped on every start, so output and exits of an earlier process are told apart. */
    private volatile int generation;
    /** We stopped it on purpose (or never started it): its exit is not a crash. */
    private volatile boolean stopping = true;
    private final ArrayDeque<Long> restarts = new ArrayDeque<>();
    private static final List<Child> ALL = new ArrayList<>();

    static {
        // a clean quit of Minecraft (the stdin pipe already covers a crash)
        Runtime.getRuntime().addShutdownHook(new Thread(() -> {
            synchronized (ALL) {
                for (Child c : new ArrayList<>(ALL)) c.stop();
            }
        }, "soworker-stop"));
    }

    Child() {}

    /** The port the child listens on, 0 until it said. */
    public int port() {
        return port;
    }

    public String token() {
        return token;
    }

    public synchronized boolean running() {
        return proc != null && proc.isAlive();
    }

    /** Started, not stopped on purpose, and gone: it crashed (or quit by itself). Reported once per start. */
    public synchronized boolean died() {
        if (proc == null || proc.isAlive() || stopping) return false;
        SoworkerMod.LOG.warn("soworker: the soworker process exited with code {}", proc.exitValue());
        proc = null;
        stdin = null;
        port = 0;
        return true;
    }

    /** May it be started again after a crash? At most {@link #MAX_RESTARTS} times a minute. */
    public synchronized boolean mayRestart() {
        long now = System.currentTimeMillis();
        while (!restarts.isEmpty() && now - restarts.peekFirst() > 60_000) restarts.removeFirst();
        if (restarts.size() >= MAX_RESTARTS) return false;
        restarts.addLast(now);
        return true;
    }

    /** The platform's folder name under {@code natives/}, or null where there is no build. */
    static String platform() {
        String os = System.getProperty("os.name", "").toLowerCase(Locale.ROOT);
        String arch = System.getProperty("os.arch", "").toLowerCase(Locale.ROOT);
        if (!arch.equals("amd64") && !arch.equals("x86_64")) return null;
        if (os.contains("win")) return "windows-x86_64";
        if (os.contains("linux")) return "linux-x86_64";
        return null;
    }

    static boolean windows() {
        return System.getProperty("os.name", "").toLowerCase(Locale.ROOT).contains("win");
    }

    /** soworker's config folder: where its saved sign-in lives too. */
    static Path configDir() {
        String win = System.getenv("APPDATA");
        if (windows() && win != null && !win.isEmpty()) return Path.of(win, "soworker");
        String xdg = System.getenv("XDG_CONFIG_HOME");
        Path base = xdg != null && !xdg.isEmpty() ? Path.of(xdg) : Path.of(System.getProperty("user.home"), ".config");
        return base.resolve("soworker");
    }

    /** soworker's log file (it writes its diagnostics there; also copied into the game log). */
    static Path logPath() {
        if (windows()) {
            String local = System.getenv("LOCALAPPDATA");
            Path base = local != null && !local.isEmpty() ? Path.of(local) : Path.of(System.getProperty("java.io.tmpdir"));
            return base.resolve("soworker").resolve("soworker.log");
        }
        String xdg = System.getenv("XDG_STATE_HOME");
        Path base = xdg != null && !xdg.isEmpty() ? Path.of(xdg) : Path.of(System.getProperty("user.home"), ".local", "state");
        return base.resolve("soworker").resolve("soworker.log");
    }

    /** Whether this jar carries a binary for this machine (or one is set with {@code -Dsoworker.bin}). */
    static boolean supported() {
        return System.getProperty("soworker.bin") != null || platform() != null;
    }

    /** The binary to run: the development override, else the bundled one, extracted when not there yet. */
    static Path binary() throws IOException {
        String override = System.getProperty("soworker.bin");
        if (override != null && !override.isEmpty()) return Path.of(override);
        String plat = platform();
        if (plat == null) throw new IOException("no soworker build for this system");
        String name = windows() ? "soworker.exe" : "soworker";
        String res = "/natives/" + plat + "/" + name;
        String want;
        try (InputStream h = Child.class.getResourceAsStream(res + ".sha256")) {
            if (h == null) throw new IOException("this jar has no soworker for " + plat);
            want = new String(h.readAllBytes(), StandardCharsets.US_ASCII).trim().split("\\s+")[0].toLowerCase(Locale.ROOT);
        }
        Path root = configDir().resolve("mc-bin");
        Path dir = root.resolve(want.substring(0, 16));
        Path bin = dir.resolve(name);
        if (!Files.isRegularFile(bin) || !want.equals(sha256(bin))) {
            Files.createDirectories(dir);
            Path tmp = Files.createTempFile(dir, name, ".part");
            try (InputStream in = Child.class.getResourceAsStream(res)) {
                if (in == null) throw new IOException("this jar has no soworker for " + plat);
                Files.copy(in, tmp, StandardCopyOption.REPLACE_EXISTING);
            }
            String got = sha256(tmp);
            if (!want.equals(got)) {
                Files.deleteIfExists(tmp);
                throw new IOException("the bundled soworker is damaged (sha256 " + got + ", expected " + want + ")");
            }
            if (!windows()) tmp.toFile().setExecutable(true, true);
            Files.move(tmp, bin, StandardCopyOption.REPLACE_EXISTING, StandardCopyOption.ATOMIC_MOVE);
            SoworkerMod.LOG.info("soworker: extracted {}", bin);
        }
        if (!windows()) bin.toFile().setExecutable(true, true);
        // binaries of other versions of the mod (one still running, on Windows, cannot go: tried again next time)
        try (var old = Files.list(root)) {
            for (Path p : old.toList()) if (!p.equals(dir)) deleteTree(p);
        } catch (IOException ignored) {
        }
        return bin;
    }

    static String sha256(Path p) throws IOException {
        try (InputStream in = Files.newInputStream(p)) {
            MessageDigest md = MessageDigest.getInstance("SHA-256");
            byte[] buf = new byte[1 << 16];
            for (int n; (n = in.read(buf)) > 0; ) md.update(buf, 0, n);
            return HexFormat.of().formatHex(md.digest());
        } catch (java.security.NoSuchAlgorithmException e) {
            throw new IOException(e);
        }
    }

    private static void deleteTree(Path p) {
        try (var walk = Files.walk(p)) {
            for (Path q : walk.sorted(java.util.Comparator.reverseOrder()).toList()) Files.deleteIfExists(q);
        } catch (IOException ignored) {
        }
    }

    /** Start the child unless it runs. Throws with a message for the user when it cannot be started. */
    public synchronized void start() throws IOException {
        if (running()) return;
        Path bin = binary();
        // the token an earlier version kept in a file
        Files.deleteIfExists(configDir().resolve("mc-token"));
        byte[] b = new byte[24];
        new SecureRandom().nextBytes(b);
        String tok = HexFormat.of().formatHex(b);
        List<String> cmd = new ArrayList<>(List.of(bin.toString(), "--mc-child"));
        String extra = System.getProperty("soworker.args", "").trim();
        if (!extra.isEmpty()) cmd.addAll(List.of(extra.split("\\s+")));
        ProcessBuilder pb = new ProcessBuilder(cmd);
        pb.environment().put("SOWORKER_MC_TOKEN", tok);
        pb.redirectInput(ProcessBuilder.Redirect.PIPE);
        Process p = pb.start();
        int gen = ++generation;
        proc = p;
        stdin = p.getOutputStream(); // kept open: closing it ends the child
        token = tok;
        port = 0;
        stopping = false;
        synchronized (ALL) {
            if (!ALL.contains(this)) ALL.add(this);
        }
        SoworkerMod.LOG.info("soworker: started {} (pid {})", String.join(" ", cmd), p.pid());
        Thread out = new Thread(() -> {
            try (BufferedReader r = new BufferedReader(new InputStreamReader(p.getInputStream(), StandardCharsets.UTF_8))) {
                for (String line; (line = r.readLine()) != null; ) {
                    if (port == 0 && generation == gen && line.startsWith("port ")) {
                        try {
                            port = Integer.parseInt(line.substring(5).trim());
                            continue;
                        } catch (NumberFormatException ignored) {
                        }
                    }
                    SoworkerMod.LOG.info("soworker: {}", line);
                }
            } catch (IOException ignored) {
            }
        }, "soworker-out");
        Thread err = new Thread(() -> {
            try (BufferedReader r = new BufferedReader(new InputStreamReader(p.getErrorStream(), StandardCharsets.UTF_8))) {
                for (String line; (line = r.readLine()) != null; ) SoworkerMod.LOG.info("soworker: {}", line);
            } catch (IOException ignored) {
            }
        }, "soworker-err");
        out.setDaemon(true);
        err.setDaemon(true);
        out.start();
        err.start();
    }

    /** Close its stdin (it quits by itself), give it 2 s, then end it. */
    public void stop() {
        Runnable finish = detach();
        if (finish != null) finish.run();
    }

    /**
     * Let go of the process at once ({@link #start} may follow right away) and return what ends it: close its stdin,
     * give it 2 s, then kill it. Null when there was none.
     */
    public Runnable detach() {
        Process p;
        OutputStream in;
        synchronized (this) {
            stopping = true;
            p = proc;
            in = stdin;
            proc = null;
            stdin = null;
            port = 0;
        }
        synchronized (ALL) {
            ALL.remove(this);
        }
        if (p == null) return null;
        return () -> {
            try {
                if (in != null) in.close();
            } catch (IOException ignored) {
            }
            try {
                if (!p.waitFor(2, TimeUnit.SECONDS)) {
                    p.destroy();
                    if (!p.waitFor(1, TimeUnit.SECONDS)) p.destroyForcibly();
                }
            } catch (InterruptedException e) {
                p.destroyForcibly();
                Thread.currentThread().interrupt();
            }
            SoworkerMod.LOG.info("soworker: stopped the soworker process");
        };
    }
}
