"""Run a local build/checker command within the three-process resource limit.

Atomic directory claims survive an interrupted coordinator. Uncertain claims
are kept for inspection rather than allowing a fourth worker to start.
"""
import argparse
import ctypes
from contextlib import contextmanager
import json
import os
from pathlib import Path
import subprocess
import time
from datetime import datetime, timezone

ROOT = Path(__file__).resolve().parents[2]
SLOTS = ROOT / '.artifacts/scratch/coordinator/local_slots'


def now():
    return datetime.now(timezone.utc).isoformat()


def memory_ready():
    if os.name != 'nt':
        return True
    class Status(ctypes.Structure):
        _fields_ = [('length', ctypes.c_ulong), ('load', ctypes.c_ulong)] + [
            (name, ctypes.c_ulonglong) for name in ('total_phys', 'avail_phys',
                'total_page', 'avail_page', 'total_virtual', 'avail_virtual', 'extended')]
    status = Status()
    status.length = ctypes.sizeof(status)
    if not ctypes.windll.kernel32.GlobalMemoryStatusEx(ctypes.byref(status)):
        return False
    return status.avail_phys >= 1.2 * 1024**3 and status.avail_page >= 6 * 1024**3


@contextmanager
def allocation_lock(slots):
    with (slots / 'allocation.lock').open('a+b') as handle:
        handle.seek(0, 2)
        if handle.tell() == 0:
            handle.write(b'0')
            handle.flush()
        handle.seek(0)
        if os.name == 'nt':
            import msvcrt
            msvcrt.locking(handle.fileno(), msvcrt.LK_LOCK, 1)
        else:
            import fcntl
            fcntl.flock(handle.fileno(), fcntl.LOCK_EX)
        try:
            yield
        finally:
            handle.seek(0)
            if os.name == 'nt':
                msvcrt.locking(handle.fileno(), msvcrt.LK_UNLCK, 1)
            else:
                fcntl.flock(handle.fileno(), fcntl.LOCK_UN)


def claim(lane, slots=SLOTS):
    slots.mkdir(parents=True, exist_ok=True)
    with allocation_lock(slots):
        for index in range(3):
            path = slots / str(index)
            try:
                path.mkdir()
            except FileExistsError:
                try:
                    previous = json.loads((path / 'owner.json').read_text(encoding='utf-8'))
                except (OSError, ValueError):
                    continue
                if previous.get('state') != 'released':
                    continue
            (path / 'owner.json').write_text(json.dumps({
                'lane': lane, 'wrapper_pid': os.getpid(), 'started': now(),
                'state': 'active'}), encoding='utf-8')
            return path
    return None


def release(path):
    with allocation_lock(path.parent):
        owner = json.loads((path / 'owner.json').read_text(encoding='utf-8'))
        owner.update(state='released', released_at=now())
        (path / 'owner.json').write_text(json.dumps(owner), encoding='utf-8')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--lane', required=True)
    parser.add_argument('--cwd', default=str(ROOT))
    parser.add_argument('command', nargs=argparse.REMAINDER)
    args = parser.parse_args()
    command = args.command
    if command and command[0] == '--':
        command = command[1:]
    if not command:
        parser.error('a command is required after --')
    slot = None
    last_notice = 0
    while slot is None:
        if memory_ready():
            slot = claim(args.lane)
        if slot is None:
            if time.monotonic() - last_notice >= 30:
                print(f'{now()} {args.lane}: waiting for local resource slot', flush=True)
                last_notice = time.monotonic()
            time.sleep(3)
    print(f'{now()} {args.lane}: acquired slot {slot.name}; wrapper PID {os.getpid()}', flush=True)
    try:
        process = subprocess.Popen(command, cwd=args.cwd)
    except OSError:
        release(slot)
        print(f'{now()} {args.lane}: launch failed before child creation; slot released', flush=True)
        raise
    try:
        print(f'{now()} {args.lane}: child PID {process.pid}', flush=True)
        result = process.wait()
    except BaseException:
        print(f'{now()} {args.lane}: interrupted; retaining slot for process audit', flush=True)
        raise
    else:
        release(slot)
        print(f'{now()} {args.lane}: released slot; exit {result}', flush=True)
        return result


if __name__ == '__main__':
    raise SystemExit(main())
