"""Install the repository's git hooks (today: pre-commit, the publication check before every commit).

Usage: python scripts/hooks/install.py [--link] [--force] [--check]

The hook goes into the folder git takes hooks from (`git rev-parse --git-path hooks`, which follows
core.hooksPath), and the script says what it did:
  - by default it is copied, with LF line ends and the executable bit, whatever the checkout did to the source;
    re-run it after the hook changes. --link makes a symbolic link instead, which follows the source but needs
    symbolic-link support (on Windows, developer mode or an administrator);
  - a hook that is already the same is left alone; a different one is never replaced unless --force is given,
    and then it is kept beside the new one as <name>.bak;
  - --check changes nothing: it reports each hook as installed, missing or different and exits 1 unless all
    are installed (scripts/doctor.py asks the same through hook_state).
"""

import argparse
import os
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
HOOKS = ("pre-commit",)


def repo_root():
    """The repository's top folder, from git."""
    return Path(subprocess.run(["git", "rev-parse", "--show-toplevel"], cwd=HERE, capture_output=True, text=True,
                               check=True).stdout.strip())


def hooks_dir(root):
    """The folder git runs hooks from in the repository at `root` (core.hooksPath when it is set)."""
    out = subprocess.run(["git", "rev-parse", "--git-path", "hooks"], cwd=root, capture_output=True, text=True,
                         check=True).stdout.strip()
    return Path(out) if os.path.isabs(out) else (Path(root) / out).resolve()


def shown(path, root):
    """A path for messages: from the repository root when inside it. A hooks folder elsewhere (core.hooksPath,
    or a worktree's shared folder) is a machine path, so only the file's name is printed."""
    try:
        relative = os.path.relpath(path, root)
    except ValueError:  # another drive on Windows
        relative = ".."
    if relative == ".." or relative.startswith(".." + os.sep):
        return f"{Path(path).name} in git's hooks folder"
    return Path(relative).as_posix()


def lf(data):
    """Bytes with CRLF line ends turned into LF: sh reads a carriage return as part of the command."""
    return data.replace(b"\r\n", b"\n")


def hook_state(root, name="pre-commit"):
    """'installed' when the hook git runs is this repository's (the same content, line ends aside, or a link to
    it), 'missing' when there is none, 'different' when another hook is there."""
    source, target = Path(root) / "scripts" / "hooks" / name, hooks_dir(root) / name
    if target.is_symlink():
        return "installed" if target.resolve() == source.resolve() else "different"
    if not target.is_file():
        return "missing"
    return "installed" if lf(target.read_bytes()) == lf(source.read_bytes()) else "different"


def install(root, name, link=False, force=False):
    """Install one hook. Returns (exit status, message)."""
    source, folder = Path(root) / "scripts" / "hooks" / name, hooks_dir(root)
    target = folder / name
    state = hook_state(root, name)
    if state == "installed" and (target.is_symlink() or not link):
        return 0, f"{name}: already installed as {shown(target, root)}"
    if state == "different" and not force:
        return 1, (f"{name}: {shown(target, root)} is a different hook; left as it is. Merge it by hand, or run "
                   "with --force to replace it (the old one is kept as a .bak file)")
    backup = None
    if state == "different":
        backup = target.with_name(name + ".bak")
        if backup.exists() or backup.is_symlink():
            backup.unlink()
        target.replace(backup)
    elif state == "installed":
        target.unlink()  # this repository's own hook as a copy, about to become a link
    folder.mkdir(parents=True, exist_ok=True)
    if link:
        if b"\r\n" in source.read_bytes():
            return 1, f"{name}: the source has CRLF line ends, which sh cannot run through a link; install a copy instead"
        try:
            target.symlink_to(os.path.relpath(source, folder))
        except OSError as problem:
            return 1, f"{name}: could not make a symbolic link ({problem.strerror}); install a copy instead (no --link)"
        done = f"{name}: linked {shown(target, root)} to scripts/hooks/{name}"
    else:
        target.write_bytes(lf(source.read_bytes()))
        target.chmod(0o755)
        done = f"{name}: copied scripts/hooks/{name} to {shown(target, root)}"
    return 0, done + (f"; the previous hook is kept as {shown(backup, root)}" if backup else "")


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--link", action="store_true", help="make a symbolic link instead of a copy")
    parser.add_argument("--force", action="store_true", help="replace a different hook, keeping it as <name>.bak")
    parser.add_argument("--check", action="store_true", help="only report whether the hooks are installed")
    args = parser.parse_args(argv)
    root = repo_root()
    status = 0
    for name in HOOKS:
        if args.check:
            state = hook_state(root, name)
            print(f"{name}: {state}")
            status |= state != "installed"
            continue
        code, message = install(root, name, link=args.link, force=args.force)
        print(message)
        status |= code
    return status


if __name__ == "__main__":
    sys.exit(main())
