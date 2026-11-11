#!/usr/bin/env python3
"""Build the soworker Minecraft mod with plain `javac` + `jar`, offline, against the installed game.

Minecraft 26.x ships unobfuscated (Mojang names), so a Fabric mod needs no remapping: compiling against the game jar,
the libraries the launcher downloaded, Fabric Loader, Mixin and the Fabric API modules is enough. No Gradle/Loom, no
network. Usage:

    python3 mc/build.py --bin linux=<soworker> --bin windows=<soworker.exe> [--allow-missing] [--install]
                        [--minecraft ~/.minecraft]

The soworker binaries go into the jar under natives/<platform>/ with their sha256 next to them (the mod extracts and
starts the one for this machine). A platform without --bin is an error unless --allow-missing (quick local builds).
The jar lands in mc/build/soworker-mc-<version>.jar, the version taken from Cargo.toml; --install copies it into
<minecraft>/mods (replacing any other soworker-mc jar there). `make mc` passes both binaries.
"""
import argparse, glob, hashlib, json, os, re, shutil, subprocess, sys, zipfile

MC_VERSION = "26.3"
LOADER = "fabric-loader-0.20.1-26.3"
HERE = os.path.dirname(os.path.abspath(__file__))
BUILD = os.path.join(HERE, "build")
# platform -> name of the binary inside natives/<platform>/
PLATFORMS = {"linux": ("linux-x86_64", "soworker"), "windows": ("windows-x86_64", "soworker.exe")}


def version():
    """The package version of Cargo.toml: the mod and the binary inside it carry one number."""
    with open(os.path.join(HERE, "..", "Cargo.toml")) as f:
        m = re.search(r'^version = "([^"]+)"', f.read(), re.M)
    if not m:
        sys.exit("no version in Cargo.toml")
    return m.group(1)


def binaries(specs, allow_missing):
    """--bin platform=path list -> {platform: path}, checked."""
    out = {}
    for spec in specs:
        plat, sep, path = spec.partition("=")
        if not sep or plat not in PLATFORMS:
            sys.exit(f"--bin {spec!r}: expected linux=<path> or windows=<path>")
        if not os.path.isfile(path):
            sys.exit(f"--bin {plat}: no file {path}")
        out[plat] = path
    missing = [p for p in PLATFORMS if p not in out]
    if missing and not allow_missing:
        sys.exit(f"no soworker binary for {', '.join(missing)} (--bin {missing[0]}=<path>, or --allow-missing)")
    for p in missing:
        print(f"warning: the jar has no soworker for {p}", file=sys.stderr)
    return out


def lib_path(root, name):
    """Maven coordinate group:artifact:version[:classifier] -> path under libraries/."""
    parts = name.split(":")
    g, a, v = parts[:3]
    cls = f"-{parts[3]}" if len(parts) > 3 else ""
    return os.path.join(root, "libraries", *g.split("."), a, v, f"{a}-{v}{cls}.jar")


def classpath(mc):
    cp = [os.path.join(mc, "versions", MC_VERSION, f"{MC_VERSION}.jar")]
    for ver in (MC_VERSION, LOADER):
        for f in glob.glob(os.path.join(mc, "versions", ver, "*.json")):
            for lib in json.load(open(f)).get("libraries", []):
                if "natives" in lib["name"]:
                    continue
                p = lib_path(mc, lib["name"])
                if os.path.exists(p):
                    cp.append(p)
    # Fabric API is a jar of jars: unpack its modules to compile against them
    api = glob.glob(os.path.join(mc, "mods", "fabric-api-*.jar"))
    if not api:
        sys.exit("no fabric-api jar in mods/")
    out = os.path.join(BUILD, "fabric-api")
    stamp = os.path.join(out, ".from")
    if not os.path.exists(stamp) or open(stamp).read() != api[0]:
        shutil.rmtree(out, ignore_errors=True)
        os.makedirs(out)
        with zipfile.ZipFile(api[0]) as z:
            for n in z.namelist():
                if n.startswith("META-INF/jars/") and n.endswith(".jar"):
                    with open(os.path.join(out, os.path.basename(n)), "wb") as f:
                        f.write(z.read(n))
        open(stamp, "w").write(api[0])
    cp += sorted(glob.glob(os.path.join(out, "*.jar")))
    return cp


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--minecraft", default=os.path.expanduser("~/.minecraft"))
    ap.add_argument("--install", action="store_true")
    ap.add_argument("--bin", action="append", default=[], metavar="PLATFORM=PATH", help="linux=<path> or windows=<path>")
    ap.add_argument("--allow-missing", action="store_true", help="build without a binary for some platform")
    a = ap.parse_args()
    bins = binaries(a.bin, a.allow_missing)
    ver = version()
    os.makedirs(BUILD, exist_ok=True)
    cp = classpath(a.minecraft)
    classes = os.path.join(BUILD, "classes")
    shutil.rmtree(classes, ignore_errors=True)
    os.makedirs(classes)
    srcs = sorted(glob.glob(os.path.join(HERE, "src", "main", "java", "**", "*.java"), recursive=True))
    r = subprocess.run(["javac", "--release", "25", "-encoding", "UTF-8", "-Xlint:-options", "-proc:none", "-nowarn",
                        "-cp", os.pathsep.join(cp), "-d", classes] + srcs)
    if r.returncode:
        sys.exit(r.returncode)
    name = f"soworker-mc-{ver}.jar"
    jar = os.path.join(BUILD, name)
    for old in glob.glob(os.path.join(BUILD, "soworker-mc*.jar")):
        os.remove(old)
    res = os.path.join(HERE, "src", "main", "resources")
    with zipfile.ZipFile(jar, "w", zipfile.ZIP_DEFLATED) as z:
        for base in (classes, res):
            for dirpath, _, files in os.walk(base):
                for f in files:
                    p = os.path.join(dirpath, f)
                    rel = os.path.relpath(p, base).replace(os.sep, "/")
                    if rel == "fabric.mod.json":
                        meta = json.load(open(p))
                        meta["version"] = ver
                        z.writestr(rel, json.dumps(meta, indent=2) + "\n")
                    else:
                        z.write(p, rel)
        for plat, path in sorted(bins.items()):
            folder, binary = PLATFORMS[plat]
            data = open(path, "rb").read()
            digest = hashlib.sha256(data).hexdigest()
            info = zipfile.ZipInfo(f"natives/{folder}/{binary}", date_time=(2026, 1, 1, 0, 0, 0))
            info.external_attr = 0o755 << 16
            info.compress_type = zipfile.ZIP_DEFLATED
            z.writestr(info, data)
            z.writestr(f"natives/{folder}/{binary}.sha256", f"{digest}  {binary}\n")
            print(f"  {plat}: {path} ({len(data) / 1e6:.1f} MB, sha256 {digest[:16]})")
    print(f"built {jar} ({os.path.getsize(jar) / 1e6:.1f} MB)")
    if a.install:
        mods = os.path.join(a.minecraft, "mods")
        for old in glob.glob(os.path.join(mods, "soworker-mc*.jar")):
            os.remove(old)  # two copies of the mod would not load
        dst = os.path.join(mods, name)
        shutil.copy(jar, dst)
        print("installed", dst)


if __name__ == "__main__":
    main()
