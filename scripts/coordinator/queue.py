"""Batch picking for production and naming lanes. Pure functions, no file IO.

The drivers load the inventory, the classification and the lists already
handed out; these functions decide what the next lanes get. Kept pure so the
rules (nothing handed out twice, holds respected, families kept together) are
tested on a synthetic inventory without any lane data.
"""

from collections import defaultdict

try:
    from common import va
except ImportError:  # run directly: find the sibling module beside this file
    import os
    import sys

    sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
    from common import va

# Library code is not rewritten: crates replace these, and Rust's standard
# library replaces the C runtime.
REPLACED_LIBRARIES = ("zlib", "jpeg", "tls", "bink", "steam", "crt")
# Blockers the current checker cannot handle wait; multi-callee and indirect
# calls are handled since checker version 2.
HANDLED_BLOCKERS = {"X-multi-callee", "X-indirect-calls"}
MIN_SIZE = 12
SMALL_UNDER = 250
# Neighbours belong to one batch only while they stay in one subsystem and
# within 32 KB of the run's start.
RUN_SPAN = 0x8000


def chunks_of(functions, full):
    """Runs of neighbouring functions from one subsystem, keyed by subsystem.

    `functions` is in address order with `start` (hex) and `sub` keys; `full`
    says when a run is a complete batch. An unfinished run at the end of a
    subsystem is dropped, exactly as the queue always did.
    """
    found = defaultdict(list)
    run = []
    for f in functions:
        if run and (f["sub"] != run[0]["sub"] or va(f["start"]) - va(run[0]["start"]) > RUN_SPAN):
            run = []
        run.append(f)
        if full(run):
            found[run[0]["sub"]].append(run)
            run = []
    return found


def pick(found, count, rng):
    """Up to `count` batches, spread round-robin over subsystems in name order,
    each drawn at random within its subsystem."""
    picked = []
    order = sorted(found)
    while len(picked) < count and any(found.values()):
        for subsystem in order:
            if found[subsystem] and len(picked) < count:
                picked.append((subsystem, found[subsystem].pop(rng.randrange(len(found[subsystem])))))
    return picked


def filter_pool(features, confirmed, *, assigned, handlers, replaced, crt_block, enc_end):
    """The functions the queue may hand out, split into small and big.

    `features` is the classifier's rows (`a`, `category`, `blockers`, `name`,
    `sub`, `level`, `n_dcallees`); `confirmed` maps address to the repaired
    inventory record. Held out: unconfirmed entries, anything already assigned,
    native handlers, replaced library and runtime code, the C runtime block,
    encrypted code, entries the checker cannot handle, and entries under 12
    bytes. Returns (small, big, held) in address order.
    """
    pool = []
    held = 0
    for row in features:
        address = va(row["a"])
        if address not in confirmed or address in assigned or address in handlers or address < enc_end:
            continue
        if address in replaced or crt_block[0] <= address < crt_block[1]:
            held += 1
            continue
        category = row.get("category")
        blockers = set(row.get("blockers") or [])
        if not (category == "as-is" or (category == "needs-extension" and blockers <= HANDLED_BLOCKERS)):
            continue
        size = confirmed[address]["size"]
        if size < MIN_SIZE:
            continue
        calls = "direct calls only" if not blockers else ", ".join(sorted(b[2:] for b in blockers))
        pool.append({"start": f"0x{address:08X}", "size": size, "name": row.get("name"), "sub": row.get("sub"),
                     "level": row.get("level"), "direct_callees": row.get("n_dcallees"), "calls": calls})
    pool.sort(key=lambda f: va(f["start"]))
    return [f for f in pool if f["size"] < SMALL_UNDER], [f for f in pool if f["size"] >= SMALL_UNDER], held


def split_native_batches(free_natives, per_lane, n_lanes):
    """Consecutive slices of the shuffled free natives, `per_lane` each."""
    return [sorted(free_natives[k * per_lane:(k + 1) * per_lane], key=lambda n: n["name"] or "")
            for k in range(n_lanes)]


def naming_batches(todo, want, per):
    """`want` batches of `per` consecutive addresses, taken from evenly spaced
    places in the address space so one campaign covers the whole program."""
    batches = []
    if todo and want:
        step = max(1, len(todo) // want)
        for i in range(want):
            batch = [a for a in todo[i * step:i * step + per]]
            if batch:
                batches.append(batch)
    return batches


def next_lane_numbers(stems, kinds=("n", "s", "b")):
    """The highest lane number already used per kind, from list-file stems."""
    import re

    last = {kind: 0 for kind in kinds}
    for stem in stems:
        match = re.fullmatch(r"r-([nsb])(\d+)", stem)
        if match and match.group(1) in last:
            last[match.group(1)] = max(last[match.group(1)], int(match.group(2)))
    return last
