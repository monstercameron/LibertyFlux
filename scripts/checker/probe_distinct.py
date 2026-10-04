"""Distinguishing probe: prove per-callee script slots are honored.

Usage: python probe_distinct.py (needs the built worker + a pilot DLL).


k2_percal1 drives q-07 FN2's two callees with different script orders (edges
vs edges-rotated-5). On trial 2 the voice is in state 2, so the original
fuses probe(id1)=0x2 with slot(id2)=0x10000 into EAX=0x10002. A worker with
one shared script slot (v1) would answer both calls with id1's value and
produce EAX=0x2. Asserting the exact fused value proves the per-id slots.
"""
import json, os, random, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import checker2

C = json.load(open(os.path.join(HERE, "contracts", "k2_percal1.json")))
w = checker2.Worker()
sr = checker2.setup_worker(w, C, [C["export"]])
print("stubs:", sr.get("stub_addrs"))

t = 2
rng = random.Random(C["seed"])
# advance rng to trial 2 exactly as run_contract would (resolve order matters)
for tt in range(t):
    checker2.resolve_regs(C, rng, tt, w.heap)
    checker2.resolve_stack(C, rng, tt, w.heap)
    checker2.resolve_segs(C, rng, tt, w.heap, w.stubs)
    checker2.resolve_scripts(C, rng, tt, w.heap, w.stubs)
req = {
    "cmd": "trial", "fn_rva": int(C["function"], 16), "export": C["export"],
    "trial": t, "seed": C["seed"] & 0xFFFFFFFF,
    "regs": checker2.resolve_regs(C, rng, t, w.heap),
    "stack": checker2.resolve_stack(C, rng, t, w.heap),
    "heapsegs": checker2.resolve_segs(C, rng, t, w.heap, w.stubs),
    "globals_fill": checker2.resolve_globals_fill(C, rng, t, w.heap, w.stubs),
    "script_vals": checker2.resolve_scripts(C, rng, t, w.heap, w.stubs),
    "checks": C["checks"], "timeout_ms": C.get("timeout_ms", 10000),
}
print("scripts:", req["script_vals"])
r = w.call(req, timeout=60)
oeax = int(r["orig"]["eax"], 16)
reax = int(r["rw"]["eax"], 16)
print("orig EAX=0x%x rw EAX=0x%x pass=%s" % (oeax, reax, r.get("pass")))
assert r["orig"]["status"] == "ok" and r["rw"]["status"] == "ok", r
assert oeax == 0x10002, "shared-slot worker would give 0x2, got 0x%x" % oeax
assert reax == 0x10002, reax
print("DISTINGUISHED: per-callee slots honored (fused 0x10002, not shared 0x2)")
w.call({"cmd": "teardown"}, timeout=30)
w.stop()
