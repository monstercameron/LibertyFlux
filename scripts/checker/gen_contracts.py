"""Generate the checker's k2 and k5 proof contracts (writes
contracts/k2_*.json and contracts/k5_*.json).

Usage: python gen_contracts.py (from anywhere; paths derive from this file).
Regenerates all 18 k2 contracts byte-identically (verified) and the 7 k5
contracts of the version 5 abilities (see the k5 section at the end). The pilot-lane
contracts (k2_percal1/2, k2_ptr1) reference the pilot rewrite DLLs by
repo-relative path; those DLLs are lane build outputs, so the generator
only records their conventional location (LF_CHECKER_PILOT_DIR to move it).
"""
import json, os

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.join(HERE, "contracts")


def repo_root():
    here = os.path.abspath(HERE)
    while True:
        if os.path.exists(os.path.join(here, "AGENTS.md")):
            return here
        parent = os.path.dirname(here)
        if parent == here:
            break
        here = parent
    return os.path.dirname(os.path.dirname(HERE))


ROOT = repo_root()
SHARED = os.environ.get("LF_CHECKER_PILOT_DIR", os.path.join(
    ROOT, ".artifacts", "build", "cargo", "i686-pc-windows-msvc", "release"))


def rel(p):
    return os.path.relpath(p, ROOT).replace(os.sep, "/")


Q07 = rel(os.path.join(SHARED, "lf_q07_rw.dll"))
Q08 = rel(os.path.join(SHARED, "lf_q08_rw.dll"))
Q16 = rel(os.path.join(SHARED, "lf_q16_rw.dll"))

INT_EDGES = [0, 1, 2, 3, 0xFF, 0x100, 0xFFFF, 0x10000, 0x7FFFFFFF, 0x80000000,
             0x80000001, 0xFFFFFFFE, 0xFFFFFFFF, 0x7F, 0x80, 0x8000, 0x12345678]
ROT5 = INT_EDGES[5:] + INT_EDGES[:5]
FLOAT_EDGES = [0x00000000, 0x80000000, 0x3F800000, 0xBF800000, 0x3F000000,
               0x40490FDB, 0x7F800000, 0xFF800000, 0x7FC00000, 0x00800000,
               0x007FFFFF, 0x00000001, 0x501502F9, 0x2EDBE6FF, 0xC0000000]

ANY7 = [{"any": True}] * 7


def w(name, obj):
    json.dump(obj, open(os.path.join(C, name + ".json"), "w"), indent=1)
    print("wrote", name)


def base_checks(ret="eax", fulldata=False, **kw):
    d = {"ret": ret, "esp": True, "heap": True, "stack": True, "globals": True,
         "calls": True, "undeclared": True, "fulldata": fulldata, "fp_tol": "0,0"}
    d.update(kw)
    return d


# 1. per-callee scripts on q-07 FN2 (their DLL, unchanged rewrite+mutant)
w("k2_percal1", {
    "name": "k2_percal1", "function": "0x49efa0", "conv": "thiscall",
    "dll": Q07, "export": "rw_q07_f2", "mut_export": "mut_q07_f2",
    "trials": 1000, "seed": 10702, "timeout_ms": 10000, "ret": "eax",
    "regs": [{"any": True}, {"heap": 0}, {"any": True}, {"any": True},
             {"any": True}, {"any": True}, {"any": True}],
    "stack": [{"kind": "int"}],
    "heapsegs": [{"off": 4096, "size": 256, "fill": "random"}],
    "pokes": [{"seg": 0, "at": 222, "size": 1,
               "vals": [0, 1, 2, 3, 255, 128, 127, 4]}],
    "globals": [], "globals_fill": "pristine",
    "callees": [
        {"id": 1, "conv": "thiscall", "nargs": 1, "ret": "u32", "script": INT_EDGES},
        {"id": 2, "conv": "thiscall", "nargs": 1, "ret": "u32", "script": ROT5},
    ],
    "patches": [{"site": "0x49efd0", "id": 1}, {"site": "0x49efda", "id": 2}],
    "iat": [], "checks": base_checks(),
})

# 2. per-callee scripts on q-08 F1 (their DLL)
w("k2_percal2", {
    "name": "k2_percal2", "function": "0x802b70", "conv": "thiscall",
    "dll": Q08, "export": "rw_q08_f1", "mut_export": "mut_q08_f1",
    "trials": 1000, "seed": 80801, "timeout_ms": 10000, "ret": "eax",
    "regs": [{"any": True}, {"heap": 0}, {"any": True}, {"any": True},
             {"any": True}, {"any": True}, {"any": True}],
    "stack": [{"kind": "int"}, {"kind": "int"}, {"kind": "byte"}, {"kind": "byte"},
              {"kind": "byte"}, {"kind": "byte"}, {"kind": "float"}],
    "heapsegs": [{"off": 4096, "size": 32, "fill": "random"}],
    "globals": [{"rva": "0xd735a4", "size": 4}], "globals_fill": "random",
    "callees": [
        {"id": 1, "conv": "thiscall", "nargs": 1, "ret": "u32", "script": INT_EDGES},
        {"id": 2, "conv": "thiscall", "nargs": 5, "ret": "u32", "script": ROT5},
    ],
    "patches": [{"site": "0x802b75", "id": 1}, {"site": "0x802b8e", "id": 2}],
    "iat": [], "checks": base_checks(),
})

# 3. indirect call through planted vtable + x87 ST0 answer (q-07 FN4, fixed rw)
w("k2_ind1", {
    "name": "k2_ind1", "function": "0x9e09f0", "conv": "thiscall",
    "export": "rw_k2_f4", "mut_export": "mut_k2_f4",
    "trials": 1000, "seed": 10704, "timeout_ms": 10000, "ret": "none",
    "regs": [{"any": True}, {"heap": 0}, {"any": True}, {"any": True},
             {"any": True}, {"any": True}, {"any": True}],
    "stack": [{"kind": "float"}],
    "heapsegs": [
        {"off": 4096, "size": 536, "fill": "floats", "floats_at": [132],
         "links": [{"at": 0, "to_seg": 1}]},
        {"off": 8192, "size": 160, "fill": "random",
         "pin": [{"at": 152, "vals": [{"stub": 2}]}]},
    ],
    "pokes": [{"seg": 0, "at": 492, "size": 4, "vals": [0, 1]}],
    "globals": [], "globals_fill": "pristine",
    "callees": [
        {"id": 1, "conv": "thiscall", "nargs": 1, "ret": "u32", "script": "edges"},
        {"id": 2, "conv": "thiscall", "nargs": 0, "ret": "f32st0", "script": FLOAT_EDGES},
    ],
    "patches": [{"site": "0x9e0a34", "id": 1}],
    "iat": [], "checks": base_checks("none"),
})

# 4. three indirect calls + heap-pointer scripts (q-07 FN1)
w("k2_ind2", {
    "name": "k2_ind2", "function": "0x9a70d0", "conv": "thiscall",
    "export": "rw_k2_f1", "mut_export": "mut_k2_f1",
    "trials": 1000, "seed": 10701, "timeout_ms": 10000, "ret": "eax",
    "regs": [{"any": True}, {"heap": 0}, {"any": True}, {"any": True},
             {"any": True}, {"any": True}, {"any": True}],
    "stack": [{"kind": "int"}],
    "heapsegs": [
        {"off": 4096, "size": 80, "fill": "random", "links": [{"at": 8, "to_seg": 1}],
         "pin": [{"at": 0, "vals": [{"heap": 5}]}]},
        {"off": 8192, "size": 16, "fill": "random", "pin": [{"at": 0, "vals": [{"heap": 2}]}]},
        {"off": 8224, "size": 16, "fill": "random", "pin": [{"at": 12, "vals": [{"stub": 2}]}]},
        {"off": 8256, "size": 16, "fill": "random", "pin": [{"at": 0, "vals": [{"heap": 4}]}]},
        {"off": 8288, "size": 16, "fill": "random", "pin": [{"at": 12, "vals": [{"stub": 3}]}]},
        {"off": 8320, "size": 80, "fill": "random", "pin": [{"at": 72, "vals": [{"stub": 4}]}]},
    ],
    "pokes": [
        {"seg": 0, "at": 72, "size": 1, "vals": [0, 1, 1, 1]},
        {"seg": 0, "at": 68, "size": 4, "vals": [305419896, 0, 1, 0]},
    ],
    "globals": [{"rva": "0xd735b4", "size": 4}], "globals_fill": "random",
    "callees": [
        # script/phasing aligned so every path fires: kind1 passes on
        # trial%4 {0,1,2}, picked is null on trial%4==2 (with flag set),
        # and kind2 passes on odd trials (trial%4==1 reaches the handoff).
        {"id": 1, "conv": "thiscall", "nargs": 1, "ret": "u32",
         "script": [{"heap": 3}, {"heap": 3}, 0, {"heap": 3}]},
        {"id": 2, "conv": "thiscall", "nargs": 0, "ret": "u32",
         "script": [0x11d, 0x11d, 0x11d, 1]},
        {"id": 3, "conv": "thiscall", "nargs": 0, "ret": "u32", "script": [0, 0x3ae]},
        {"id": 4, "conv": "thiscall", "nargs": 1, "ret": "u32", "script": "edges"},
    ],
    "patches": [{"site": "0x9a7114", "id": 1}],
    "iat": [], "checks": base_checks(),
})

# 5. data-pointer call slots incl. an .rdata slot (q-07 FN3)
w("k2_dslot", {
    "name": "k2_dslot", "function": "0x2f6660", "conv": "thiscall",
    "export": "rw_k2_f3", "mut_export": "mut_k2_f3",
    "trials": 1000, "seed": 10703, "timeout_ms": 10000, "ret": "eax",
    "regs": [{"any": True}, {"heap": 0}, {"any": True}, {"any": True},
             {"any": True}, {"any": True}, {"any": True}],
    "stack": [{"kind": "int"}],
    "heapsegs": [{"off": 4096, "size": 128, "fill": "random"}],
    "globals": [{"rva": "0x13acd20", "size": 4}, {"rva": "0x13acd00", "size": 4},
                {"rva": "0xa73474", "size": 4}],
    "globals_values": [
        {"rva": "0x13acd20", "words": [{"cycle": [{"stub": 2}, 0]}]},
        {"rva": "0x13acd00", "words": [{"stub": 3}]},
        {"rva": "0xa73474", "words": [{"stub": 4}]},
    ],
    "callees": [
        {"id": 1, "conv": "thiscall", "nargs": 2, "ret": "u32", "script": "edges"},
        {"id": 2, "conv": "cdecl", "nargs": 0, "ret": "u32", "script": [0]},
        {"id": 3, "conv": "cdecl", "nargs": 2, "ret": "u32", "script": [0],
         "snap": [{"kind": "arg", "idx": 0, "n": 1}, {"kind": "arg", "idx": 1, "n": 1}]},
        {"id": 4, "conv": "cdecl", "nargs": 0, "ret": "u32", "script": "edges"},
    ],
    "patches": [{"site": "0x2f66a8", "id": 1}, {"site": "0x2f66c2", "id": 1}],
    "iat": [], "checks": base_checks("eax", **{"call_skip": {"3": [0, 1]}}),
})

# 6. stub out-param writes on two callees + per-callee answers (q-07 FN5)
w("k2_stw1", {
    "name": "k2_stw1", "function": "0xa079d2", "conv": "cdecl",
    "export": "rw_k2_f5", "mut_export": "mut_k2_f5",
    "trials": 1000, "seed": 10705, "timeout_ms": 5000, "ret": "eax",
    "stack_fill": 0,
    "regs": ANY7,
    "stack": [{"kind": "cycle", "values": [0, 1, 2, 0xFF, 0x100, 0x101, 0x1234, 0x1FF00]},
              {"kind": "int"}, {"kind": "int"}],
    "heapsegs": [
        {"off": 4096, "size": 160, "fill": "random",
         "pin": [{"at": 144, "vals": [{"heap": 1}]}]},
        {"off": 8192, "size": 512, "fill": "random"},
        {"off": 12288, "size": 128, "fill": "zeros",
         "pin": [{"at": 112, "vals": [0xFFFFFFFF, 0]}]},
    ],
    "globals": [], "globals_fill": "pristine",
    "callees": [
        {"id": 1, "conv": "thiscall", "nargs": 1, "ret": "u32", "script": [0],
         "writes": [{"reg": "ecx", "at": 0, "n": 4}],
         "wscript": [[{"heap": 0}, 0xAAAAAAAA, {"heap": 2}, 0],
                     [{"heap": 0}, 0xBBBBBBBB, {"heap": 2}, 1]],
         "snap": [{"kind": "ecx", "idx": 0, "n": 4}]},
        {"id": 2, "conv": "cdecl", "nargs": 2, "ret": "u32", "script": [0, 1],
         "snap": [{"kind": "arg", "idx": 1, "n": 4}]},
        {"id": 3, "conv": "cdecl", "nargs": 7, "ret": "u32", "script": [0, 1],
         "writes": [{"arg": 4, "at": 0, "n": 1}],
         "wscript": [[0xBEEF], [0x1235]],
         "snap": [{"kind": "arg", "idx": 2, "n": 1}, {"kind": "arg", "idx": 4, "n": 1}]},
    ],
    "patches": [{"site": "0xa079df", "id": 1}, {"site": "0xa07a10", "id": 2},
                {"site": "0xa07a4e", "id": 3}],
    # stack check off: like q-13 f1, the original clobbers its own incoming
    # arg slots as scratch, which no rewrite can reproduce (same rationale).
    "iat": [], "checks": dict(base_checks("eax", **{
        "call_regs": {"1": []}, "call_skip": {"2": [1], "3": [0, 2, 4]}}),
        stack=False),
})

# 6b. buffer-content mutant as export: identical to rw_k2_f5 but for
# swapped wide-prefix bytes; the id3 buf snapshot must catch it on calls
# (positive snapshot-catch proof to pair with the q-16 negative result).
_b = json.load(open(os.path.join(C, "k2_stw1.json")))
_b["name"] = "k2_stw1b"
_b["export"] = "mut_k2_f5b"
_b.pop("mut_export", None)
_b["trials"] = 100
w("k2_stw1b", _b)

# 7. pointer comparison: q-16 f2 record queries (their DLL), skip + snapshots
Q16SCRIPTS = [{"heap": 256}, {"heap": 257}, {"heap": 272}, {"heap": 511}]
Q16CALLEES = []
for i in (1, 2, 3, 4):
    Q16CALLEES.append({"id": i, "conv": "cdecl", "nargs": 3, "ret": "u32",
                       "script": Q16SCRIPTS})
for i in (5, 6, 7, 8, 9, 10, 11):
    Q16CALLEES.append({"id": i, "conv": "cdecl", "nargs": 4, "ret": "u32",
                       "script": Q16SCRIPTS,
                       "snap": [{"kind": "arg", "idx": 0, "n": 1}]})
Q16PATCHES = [{"site": s, "id": i} for s, i in [
    ("0x7e52e0", 1), ("0x7e5305", 5), ("0x7e5367", 2), ("0x7e5388", 6),
    ("0x7e53e0", 3), ("0x7e5405", 7), ("0x7e545d", 4), ("0x7e5482", 8),
    ("0x7e54f1", 9), ("0x7e5598", 10), ("0x7e5607", 11)]]
Q16SKIP = {str(i): [0] for i in (5, 6, 7, 8, 9, 10, 11)}
w("k2_ptr1", {
    "name": "k2_ptr1", "function": "0x7e52c0", "conv": "cdecl",
    "dll": Q16, "export": "rw_q16_f2",
    "trials": 1000, "seed": 1602, "timeout_ms": 10000, "ret": "eax",
    "regs": ANY7,
    "stack": [{"kind": "ptr", "seg": 0}, {"kind": "byte"},
              {"kind": "small", "max": 3}, {"kind": "small", "max": 3},
              {"kind": "int"}],
    "heapsegs": [
        {"off": 0, "size": 64, "fill": "zeros"},
        {"off": 256, "size": 288, "fill": "floats",
         "floats_at": [1, 2, 5, 6, 9, 10]},
    ],
    "globals": [{"rva": "0x127f628", "size": 288}, {"rva": "0x1282d80", "size": 256}],
    "globals_fill": "pristine",
    "callees": Q16CALLEES, "patches": Q16PATCHES, "iat": [],
    "checks": base_checks("eax", fulldata=True, **{"call_skip": Q16SKIP}),
})

# 7b. content mutant as export: identical but for the corrupted local
Q16NOMUT = [dict(c) for c in Q16CALLEES]
w("k2_ptr1m", {
    "name": "k2_ptr1m", "function": "0x7e52c0", "conv": "cdecl",
    "export": "mut_k2_ptr1c",
    "trials": 100, "seed": 1602, "timeout_ms": 10000, "ret": "eax",
    "regs": ANY7,
    "stack": [{"kind": "ptr", "seg": 0}, {"kind": "byte"},
              {"kind": "small", "max": 3}, {"kind": "small", "max": 3},
              {"kind": "int"}],
    "heapsegs": [
        {"off": 0, "size": 64, "fill": "zeros"},
        {"off": 256, "size": 288, "fill": "floats",
         "floats_at": [1, 2, 5, 6, 9, 10]},
    ],
    "globals": [{"rva": "0x127f628", "size": 288}, {"rva": "0x1282d80", "size": 256}],
    "globals_fill": "pristine",
    "callees": Q16CALLEES, "patches": Q16PATCHES, "iat": [],
    "checks": base_checks("eax", fulldata=False, **{"call_skip": Q16SKIP}),
})

# 7c. same mutant under skip-only (q-16 style): must PASS (vacuous)
Q16SKIPONLY = []
for i in (1, 2, 3, 4):
    Q16SKIPONLY.append({"id": i, "conv": "cdecl", "nargs": 3, "ret": "u32",
                        "script": Q16SCRIPTS})
for i in (5, 6, 7, 8, 9, 10, 11):
    Q16SKIPONLY.append({"id": i, "conv": "cdecl", "nargs": 4, "ret": "u32",
                        "script": Q16SCRIPTS})
w("k2_ptr1m_skip", {
    "name": "k2_ptr1m_skip", "function": "0x7e52c0", "conv": "cdecl",
    "export": "mut_k2_ptr1c",
    "trials": 100, "seed": 1602, "timeout_ms": 10000, "ret": "eax",
    "regs": ANY7,
    "stack": [{"kind": "ptr", "seg": 0}, {"kind": "byte"},
              {"kind": "small", "max": 3}, {"kind": "small", "max": 3},
              {"kind": "int"}],
    "heapsegs": [
        {"off": 0, "size": 64, "fill": "zeros"},
        {"off": 256, "size": 288, "fill": "floats",
         "floats_at": [1, 2, 5, 6, 9, 10]},
    ],
    "globals": [{"rva": "0x127f628", "size": 288}, {"rva": "0x1282d80", "size": 256}],
    "globals_fill": "pristine",
    "callees": Q16SKIPONLY, "patches": Q16PATCHES, "iat": [],
    "checks": base_checks("eax", fulldata=False, **{"call_skip": Q16SKIP}),
})

# 8/9. fabricated TLS slot 0 + planted virtual call
for nm, fn, seed, exp, mut, stack in [
    ("k2_tls1", "0x19c150", 20101, "rw_k2_t1", "mut_k2_t1", [{"kind": "int"}]),
    ("k2_tls2", "0x19a2b0", 20102, "rw_k2_t2", "mut_k2_t2", [{"kind": "byte"}]),
]:
    segs = [
        {"off": 4096, "size": 32, "fill": "random"},
        {"off": 8192, "size": 16, "fill": "random",
         "pin": [{"at": 8, "vals": [{"heap": 2}]}]},
        {"off": 8224, "size": 16, "fill": "random",
         "pin": [{"at": 0, "vals": [{"heap": 3}]}]},
        {"off": 8256, "size": 16, "fill": "random",
         "pin": [{"at": 12, "vals": [{"stub": 1}]}]},
    ]
    if nm == "k2_tls1":
        segs[0]["pin"] = [{"at": 0, "vals": [0, 0xAABBCCDD]}]
    w(nm, {
        "name": nm, "function": fn, "conv": "thiscall",
        "export": exp, "mut_export": mut,
        "trials": 1000, "seed": seed, "timeout_ms": 10000, "ret": "eax",
        "regs": [{"any": True}, {"heap": 0}, {"any": True}, {"any": True},
                 {"any": True}, {"any": True}, {"any": True}],
        "stack": stack, "heapsegs": segs,
        "tls": [{"slot": 0, "seg": 1}],
        "globals": [], "globals_fill": "pristine",
        "callees": [{"id": 1, "conv": "thiscall", "nargs": 1, "ret": "u32",
                     "script": "edges"}],
        "patches": [], "iat": [], "checks": base_checks(),
    })

# 10/11. XMM0 float arguments with rewrite-side stack transport
# (id2's script is rotated so the two answers differ: with identical
# scripts the xmm1 swap mutant would be unobservable.)
FLOAT_ROT5 = FLOAT_EDGES[5:] + FLOAT_EDGES[:5]
for nm, fn, sites, seed, exp, mut in [
    ("k2_xmm1", "0x1bf550", ["0x1bf55f", "0x1bf56f"], 20201, "rw_k2_x1", "mut_k2_x1"),
    ("k2_xmm2", "0x1bf5c0", ["0x1bf5cf", "0x1bf5df"], 20202, "rw_k2_x2", "mut_k2_x2"),
]:
    w(nm, {
        "name": nm, "function": fn, "conv": "thiscall",
        "export": exp, "mut_export": mut,
        "trials": 1000, "seed": seed, "timeout_ms": 10000, "ret": "none",
        "regs": [{"any": True}, {"heap": 0}, {"any": True}, {"any": True},
                 {"any": True}, {"any": True}, {"any": True}],
        "stack": [{"kind": "float"}],
        "heapsegs": [{"off": 4096, "size": 64, "fill": "random"}],
        "globals": [], "globals_fill": "pristine",
        "callees": [
            {"id": 1, "conv": "cdecl", "nargs": 1, "ret": "f32xmm0",
             "logxmm": True, "xmm0_from_stack": 0, "script": FLOAT_EDGES},
            {"id": 2, "conv": "cdecl", "nargs": 1, "ret": "f32xmm0",
             "logxmm": True, "xmm0_from_stack": 0, "script": FLOAT_ROT5},
        ],
        "patches": [{"site": sites[0], "id": 1}, {"site": sites[1], "id": 2}],
        "iat": [], "checks": base_checks("none"),
    })

# 12/13. E9 tail-thunk interception
for nm, fn, site, seed, exp, mut in [
    ("k2_tail1", "0x9fbd78", "0x9fbd7d", 20301, "rw_k2_tail1", "mut_k2_tail1"),
    ("k2_tail2", "0x9fbd60", "0x9fbd65", 20302, "rw_k2_tail2", "mut_k2_tail2"),
]:
    w(nm, {
        "name": nm, "function": fn, "conv": "cdecl",
        "export": exp, "mut_export": mut,
        "trials": 1000, "seed": seed, "timeout_ms": 10000, "ret": "eax",
        "regs": ANY7, "stack": [], "heapsegs": [],
        "globals": [], "globals_fill": "pristine",
        "callees": [{"id": 1, "conv": "custom", "nargs": 0, "ret": "u32",
                     "script": "edges"}],
        "patches": [], "tailpatches": [{"site": site, "id": 1}], "outer_pop": 0,
        "iat": [], "checks": base_checks("eax", **{"call_regs": {"1": ["edx"]}}),
    })

# 14. XMM1 entry + virtual call + computed tail jump (0x75EE00)
w("k2_e1", {
    "name": "k2_e1", "function": "0x35ee00", "conv": "thiscall",
    "export": "rw_k2_e1", "mut_export": "mut_k2_e1",
    "trials": 1000, "seed": 20401, "timeout_ms": 10000, "ret": "eax",
    "regs": [{"any": True}, {"heap": 0}, {"any": True}, {"any": True},
             {"any": True}, {"any": True}, {"any": True}],
    "stack": [],
    "heapsegs": [
        {"off": 4096, "size": 64, "fill": "random", "pin": [
            {"at": 0, "vals": [{"heap": 1}]},
            {"at": 24, "vals": [0, 1, 2]},
            {"at": 16, "vals": [0, 1, 2, 100]},
            {"at": 20, "vals": [0, 1, 2, 100]},
            {"at": 32, "vals": [0, 1]},
            {"at": 36, "vals": [0, 1]},
        ]},
        {"off": 8192, "size": 16, "fill": "random", "pin": [
            {"at": 8, "vals": [{"stub": 1}]},
            {"at": 4, "vals": [{"stub": 2}]},
        ]},
    ],
    "xmm": {"1": ["float", 0, 0, 0]},
    "globals": [], "globals_fill": "pristine",
    "callees": [
        {"id": 1, "conv": "thiscall", "nargs": 2, "ret": "u32", "script": "edges"},
        {"id": 2, "conv": "thiscall", "nargs": 0, "ret": "u32", "script": "edges"},
    ],
    "patches": [], "iat": [], "checks": base_checks(),
})

# 15. XMM1 entry + counted virtual broadcast (0x787390)
w("k2_e2", {
    "name": "k2_e2", "function": "0x387390", "conv": "thiscall",
    "export": "rw_k2_e2", "mut_export": "mut_k2_e2",
    "trials": 1000, "seed": 20402, "timeout_ms": 10000, "ret": "none",
    "regs": [{"any": True}, {"heap": 0}, {"any": True}, {"any": True},
             {"any": True}, {"any": True}, {"any": True}],
    "stack": [],
    "heapsegs": [
        {"off": 4096, "size": 160, "fill": "random", "pin": [
            {"at": 0, "vals": [0, 1, 2, 3, 4]},
            {"at": 132, "vals": [{"heap": 1, "plus": 0}]},
            {"at": 136, "vals": [{"heap": 1, "plus": 548}]},
            {"at": 140, "vals": [{"heap": 1, "plus": 1096}]},
        ]},
        {"off": 8192, "size": 1664, "fill": "random", "pin": [
            {"at": 0, "vals": [{"heap": 2}]},
            {"at": 548, "vals": [{"heap": 2}]},
            {"at": 1096, "vals": [{"heap": 2}]},
            {"at": 544, "vals": [0, 1]},
            {"at": 1092, "vals": [0, 1]},
            {"at": 1640, "vals": [0, 1]},
        ]},
        {"off": 16384, "size": 80, "fill": "random",
         "pin": [{"at": 76, "vals": [{"stub": 1}]}]},
    ],
    "xmm": {"1": ["float", 0, 0, 0]},
    "globals": [], "globals_fill": "pristine",
    "callees": [{"id": 1, "conv": "thiscall", "nargs": 1, "ret": "u32", "script": [0]}],
    "patches": [], "iat": [], "checks": base_checks("none"),
})


# ---------------------------------------------------------------------------
# k5: proofs of the version 5 abilities. Each correct export must pass and
# each mutant must fail; every mutant is one the v4 checker could not see.
# Three run on the worker's built-in self-test originals ("selftest:<name>",
# marked as self-tests in the verdict), because no tracked original is known
# to take x87 arguments, pass XMM2-XMM7 to a callee or read through an
# unrelocated absolute address. k5_absguard is a documented negative.
# ---------------------------------------------------------------------------

# k5 1. x87 entry values: three entries popped into f32/f64/80-bit slots.
w("k5_x87", {
    "name": "k5_x87", "function": "selftest:x87_store", "conv": "cdecl",
    "export": "rw_k5_x87", "mut_export": "mut_k5_x87",
    "trials": 1000, "seed": 50501, "timeout_ms": 10000, "ret": "eax",
    "regs": ANY7, "stack": [{"kind": "ptr", "seg": 0}],
    "heapsegs": [{"off": 4096, "size": 24, "fill": "random"}],
    "x87": [{"kind": "f64"}, {"kind": "f80"}, {"kind": "f80"}],
    "globals": [], "globals_fill": "pristine", "callees": [], "patches": [],
    "iat": [], "checks": base_checks(),
})

# k5 2. x87 stack balance: same original, no return channel compared; the
# mutant leaves a value in ST0 (only the x87 state check sees it).
w("k5_x87bal", {
    "name": "k5_x87bal", "function": "selftest:x87_store", "conv": "cdecl",
    "export": "rw_k5_x87", "mut_export": "mut_k5_x87bal",
    "trials": 1000, "seed": 50502, "timeout_ms": 10000, "ret": "none",
    "regs": ANY7, "stack": [{"kind": "ptr", "seg": 0}],
    "heapsegs": [{"off": 4096, "size": 24, "fill": "random"}],
    "x87": [{"kind": "f32"}, {"kind": "f64"}, {"kind": "f80"}],
    "globals": [], "globals_fill": "pristine", "callees": [], "patches": [],
    "iat": [], "checks": base_checks("none"),
})

# k5 3. XMM2/XMM5 call arguments with rewrite-side transports, plus the
# XMM6 entry value (entry values for all eight registers predate v5).
w("k5_xmm", {
    "name": "k5_xmm", "function": "selftest:xmm_call", "conv": "cdecl",
    "export": "rw_k5_xmm", "mut_export": "mut_k5_xmm",
    "trials": 1000, "seed": 50503, "timeout_ms": 10000, "ret": "eax",
    "regs": ANY7, "stack": [{"kind": "float"}, {"kind": "float"}],
    "heapsegs": [],
    "xmm": {"2": [0x11111111, 0x22222222, 0x33333333, 0x44444444],
            "5": [0x55555555, 0x66666666, 0x77777777, 0x88888888],
            "6": ["float", 0, 0, 0]},
    "globals": [], "globals_fill": "pristine",
    "callees": [{"id": 1, "conv": "cdecl", "nargs": 2, "ret": "u32",
                 "script": [0], "logxmm_regs": [2, 5],
                 "xmm_from_stack": {"2": 0, "5": 1}}],
    "patches": [], "iat": [], "checks": base_checks(),
})

# k5 4. unrelocated absolute reads served by the read-only shadow. The
# addresses are file VAs inside the first header page (identical bytes in
# the relocated image, so the relocated read is the correct rewrite).
ABS_VAS = [0x400000, 0x400002, 0x40003C, 0x400080, 0x400084, 0x400100]
w("k5_abs", {
    "name": "k5_abs", "function": "selftest:abs_read", "conv": "cdecl",
    "export": "rw_k5_abs", "mut_export": "mut_k5_abs",
    "trials": 200, "seed": 50504, "timeout_ms": 10000, "ret": "eax",
    "abs_shadow": True,
    "regs": ANY7, "stack": [{"kind": "rot", "values": ABS_VAS}],
    "heapsegs": [], "globals": [], "globals_fill": "pristine",
    "callees": [], "patches": [], "iat": [], "checks": base_checks(),
})

# k5 5. documented negative (fails by design): without abs_shadow the same
# reads fault on the original in the reserved window (the v5 guard) instead
# of silently reading worker memory as in v4; the correct rewrite then
# fails on termination, with an "abs-window" note in the fault detail.
w("k5_absguard", {
    "name": "k5_absguard", "function": "selftest:abs_read", "conv": "cdecl",
    "export": "rw_k5_abs",
    "trials": 20, "seed": 50505, "timeout_ms": 10000, "ret": "eax",
    "regs": ANY7, "stack": [{"kind": "rot", "values": ABS_VAS}],
    "heapsegs": [], "globals": [], "globals_fill": "pristine",
    "callees": [], "patches": [], "iat": [], "checks": base_checks(),
})


def k5_snap_contract(name, seed, mut, snap):
    """k2_ind1's original and rewrite (q-07 FN4), with a wider snapshot of
    the object at the virtual sample's caller-side call."""
    return {
        "name": name, "function": "0x9e09f0", "conv": "thiscall",
        "export": "rw_k2_f4", "mut_export": mut,
        "trials": 1000, "seed": seed, "timeout_ms": 10000, "ret": "none",
        "regs": [{"any": True}, {"heap": 0}, {"any": True}, {"any": True},
                 {"any": True}, {"any": True}, {"any": True}],
        "stack": [{"kind": "float"}],
        "heapsegs": [
            {"off": 4096, "size": 536, "fill": "floats", "floats_at": [132],
             "links": [{"at": 0, "to_seg": 1}]},
            {"off": 8192, "size": 160, "fill": "random",
             "pin": [{"at": 152, "vals": [{"stub": 2}]}]},
        ],
        "pokes": [{"seg": 0, "at": 492, "size": 4, "vals": [0, 1]}],
        "globals": [], "globals_fill": "pristine",
        "callees": [
            {"id": 1, "conv": "thiscall", "nargs": 1, "ret": "u32",
             "script": "edges", "snap": snap},
            {"id": 2, "conv": "thiscall", "nargs": 0, "ret": "f32st0",
             "script": FLOAT_EDGES},
        ],
        "patches": [{"site": "0x9e0a34", "id": 1}],
        "iat": [], "checks": base_checks("none"),
    }


# k5 6. 16 snapshot words 0x1D8 bytes into the object (v4: 8 words at 0);
# the mutant disturbs word 8 of the window during the call.
w("k5_snap", k5_snap_contract("k5_snap", 50506, "mut_k5_snap",
                              [{"kind": "ecx", "at": 0x1D8, "n": 16}]))

# k5 7. a negative offset, as a second ECX entry (which also exercises the
# v5 fix: v4 read a later register entry's pointer from a clobbered ECX);
# the mutant disturbs the word 8 bytes below the object during the call.
w("k5_snapneg", k5_snap_contract("k5_snapneg", 50507, "mut_k5_snapneg",
                                 [{"kind": "ecx", "at": 0, "n": 2},
                                  {"kind": "ecx", "at": -16, "n": 4}]))
