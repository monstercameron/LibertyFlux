"""Build the project's runnable programs and collect them into one folder for a release.

What gets packaged today:
- lf-inspect, the one inspection tool for the game's file formats (crates/tools/lf-inspect; it replaced the
  per-crate example programs), built for the machine this runs on;
- on Windows, the 32-bit developer tools as well: the checker's worker, the loader's proxy library and its
  test program.

Nothing from the game is included or needed to build. The tools read files from a copy of the game that the
person running them owns.

Usage: python scripts/ci/package.py --out dist [--debug] [--only crate,crate] [--no-win32-tools]
Writes the programs, LICENSE, a README.txt and a manifest.json into the output folder and prints its contents.
"""

import argparse
import json
import os
import platform
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True, text=True, check=True).stdout.strip())
WIN32_TARGET = "i686-pc-windows-msvc"
WIN32_TOOLS = ["lf-checker-worker", "lf-proxy", "lf-test-target"]

parser = argparse.ArgumentParser()
parser.add_argument("--out", required=True)
parser.add_argument("--debug", action="store_true", help="build without optimisation (for trying the script out)")
parser.add_argument("--only", default="", help="comma-separated crate names to limit the format tools to (lf-inspect)")
parser.add_argument("--no-win32-tools", action="store_true")
args = parser.parse_args()

out = Path(args.out).resolve()
out.mkdir(parents=True, exist_ok=True)
profile_flag = [] if args.debug else ["--release"]
profile_dir = "debug" if args.debug else "release"
exe = ".exe" if os.name == "nt" else ""


def run(*command):
    print("+", " ".join(command), flush=True)
    subprocess.run(command, cwd=ROOT, check=True)


metadata = json.loads(subprocess.run(["cargo", "metadata", "--no-deps", "--format-version", "1"], cwd=ROOT,
                                     capture_output=True, text=True, check=True).stdout)
target_dir = Path(metadata["target_directory"])
only = {name for name in args.only.split(",") if name}
packaged = []

# Format tool: lf-inspect, one program with a subcommand per format (`lf-inspect formats` lists them).
FORMAT_TOOL = "lf-inspect"
if not only or FORMAT_TOOL in only:
    run("cargo", "build", *profile_flag, "-p", FORMAT_TOOL)
    built = target_dir / profile_dir / (FORMAT_TOOL + exe)
    if not built.exists():
        print(f"missing after build: {built}", file=sys.stderr)
        sys.exit(1)
    (out / "format-tools").mkdir(exist_ok=True)
    shutil.copy2(built, out / "format-tools" / built.name)
    packaged.append({"file": f"format-tools/{built.name}", "crate": FORMAT_TOOL, "kind": "format tool"})

if os.name == "nt" and not args.no_win32_tools:
    run("cargo", "build", *profile_flag, "--target", WIN32_TARGET, *[x for tool in WIN32_TOOLS for x in ("-p", tool)])
    built_dir = target_dir / WIN32_TARGET / profile_dir
    (out / "tools-win32").mkdir(exist_ok=True)
    for name in ("lf-checker-worker.exe", "lf_proxy.dll", "lf-test-target.exe"):
        source = built_dir / name
        if not source.exists():
            print(f"missing after build: {source}", file=sys.stderr)
            sys.exit(1)
        shutil.copy2(source, out / "tools-win32" / name)
        packaged.append({"file": f"tools-win32/{name}", "crate": name.split(".")[0].replace("_", "-"), "kind": "32-bit developer tool"})

commit = subprocess.run(["git", "rev-parse", "HEAD"], cwd=ROOT, capture_output=True, text=True, check=True).stdout.strip()
shutil.copy2(ROOT / "LICENSE", out / "LICENSE")
(out / "README.txt").write_text(f"""LibertyFlux: automated build

Built from commit {commit}
for {platform.system()} {platform.machine()}.

What this is
  Tools from the LibertyFlux project, which is rewriting Grand Theft Auto IV's engine in Rust.
  format-tools/   lf-inspect, which reads and describes the game's file formats (archives, models,
                  textures, collision, scripts, text, audio configuration, saves and more); run
                  `lf-inspect formats` for the list and `lf-inspect help` for usage
  tools-win32/    (Windows builds only) the project's 32-bit developer tools: the checker's worker, the
                  loader's proxy library and its test program

What this is not
  It is not the game and it is not a playable build. No game code and no game data is included.

You need your own copy of the game
  These tools work on files from a copy of Grand Theft Auto IV that you own. Buy it on Steam, from an
  authorised key seller, or from a retailer. This project does not condone piracy and will not help
  anyone run the game without having bought it.

Licence: MIT (see LICENSE). Source: https://github.com/monstercameron/LibertyFlux
""", encoding="utf-8", newline="\n")
(out / "manifest.json").write_text(json.dumps({"commit": commit, "system": platform.system(), "machine": platform.machine(),
                                               "profile": profile_dir, "files": packaged}, indent=1) + "\n", encoding="utf-8", newline="\n")
print(f"packaged {len(packaged)} programs into {out}")
for item in packaged:
    size = (out / item["file"]).stat().st_size
    print(f"  {item['file']}  {size // 1024} KB")
