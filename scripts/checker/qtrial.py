"""Single-trial query: print orig/rw status + fault + calls for a contract.

Usage: python qtrial.py CONTRACT [TRIAL] (default trial 0).
Debugging helper for checker work; needs the built worker + proof DLL.
"""
import json, os, random, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import checker2

name = sys.argv[1]
trial = int(sys.argv[2]) if len(sys.argv) > 2 else 0
C = json.load(open(os.path.join(HERE, "contracts", name + ".json")))
w = checker2.Worker()
sr = checker2.setup_worker(w, C, [C["export"]])
print("setup errors:", sr.get("errors"))
print("stubs:", sr.get("stub_addrs"))
rng = random.Random(C["seed"])
for tt in range(trial):
    checker2.resolve_regs(C, rng, tt, w.heap)
    checker2.resolve_stack(C, rng, tt, w.heap)
    checker2.resolve_segs(C, rng, tt, w.heap, w.stubs)
    checker2.resolve_scripts(C, rng, tt, w.heap, w.stubs)
req = {
    "cmd": "trial", "fn_rva": int(C["function"], 16), "export": C["export"],
    "trial": trial, "seed": C["seed"] & 0xFFFFFFFF,
    "regs": checker2.resolve_regs(C, rng, trial, w.heap),
    "stack": checker2.resolve_stack(C, rng, trial, w.heap),
    "heapsegs": checker2.resolve_segs(C, rng, trial, w.heap, w.stubs),
    "globals_fill": checker2.resolve_globals_fill(C, rng, trial, w.heap, w.stubs),
    "script_vals": checker2.resolve_scripts(C, rng, trial, w.heap, w.stubs),
    "checks": C["checks"], "timeout_ms": C.get("timeout_ms", 10000),
}
tls = checker2.resolve_tls(C, trial, w.heap)
if tls:
    req["tls"] = tls
xmm = checker2.resolve_xmm(C, rng, trial)
if xmm:
    req["xmm"] = xmm
if "stack_fill" in C:
    req["stack_fill"] = C["stack_fill"]
r = w.call(req, timeout=60)
for side in ("orig", "rw"):
    o = r.get(side, {})
    print(side, "status=", o.get("status"), "fault=", o.get("fault"),
          "eip=", o.get("fault_eip"), "badva=", o.get("fault_badva"))
    print("  calls:", json.dumps(o.get("calls"))[:600])
print("pass=", r.get("pass"), "mismatch=", r.get("first_mismatch"))
w.call({"cmd": "teardown"}, timeout=30)
w.stop()
