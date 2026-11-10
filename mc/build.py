#!/usr/bin/env python3
"""Build the soworker Minecraft mod with plain `javac` + `jar`, offline, against the installed game.

Minecraft 26.x ships unobfuscated (Mojang names), so a Fabric mod needs no remapping: compiling against the game jar,
the libraries the launcher downloaded, Fabric Loader, Mixin and the Fabric API modules is enough. No Gradle/Loom, no
network. Usage:

    python3 mc/build.py [--install] [--minecraft ~/.minecraft]

The jar lands in mc/build/soworker-mc.jar; --install copies it into <minecraft>/mods.
"""
import argparse, glob, json, os, shutil, subprocess, sys, zipfile

MC_VERSION = "26.3"
LOADER = "fabric-loader-0.20.1-26.3"
HERE = os.path.dirname(os.path.abspath(__file__))
BUILD = os.path.join(HERE, "build")


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
    a = ap.parse_args()
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
    jar = os.path.join(BUILD, "soworker-mc.jar")
    res = os.path.join(HERE, "src", "main", "resources")
    with zipfile.ZipFile(jar, "w", zipfile.ZIP_DEFLATED) as z:
        for base in (classes, res):
            for dirpath, _, files in os.walk(base):
                for f in files:
                    p = os.path.join(dirpath, f)
                    z.write(p, os.path.relpath(p, base))
    print("built", jar)
    if a.install:
        dst = os.path.join(a.minecraft, "mods", "soworker-mc.jar")
        shutil.copy(jar, dst)
        print("installed", dst)


if __name__ == "__main__":
    main()
