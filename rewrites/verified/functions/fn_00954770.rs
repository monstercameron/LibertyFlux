// original: 0x00954770 dispatch_table_builder
//! Builder for the global dispatch table: scans two gated sets of command
//! records and appends validated (key, value) pairs to a growable table.
//!
//! Calling convention: cdecl with one ignored 4-byte argument; result in EAX
//! is the tail finalizer's answer. The function takes no inputs except global
//! state and the scripted answers of its callees.
//!
//! Prologue: fetches a source token and initializes from it, snapshots two
//! seed globals into frame slots, frees the old entry table and clears the
//! table pointer, the entry count/capacity pair and the sequence slot.
//!
//! Phase 1 (gated by a count byte): for each live slot, walks a NUL-terminated
//! command string byte by byte. Each byte, minus a bias and range-checked,
//! indexes a 256-entry map into a 10-way dispatch. The arms resolve or
//! allocate small nodes, toggle flag bits on the record and the node, and
//! record running state in frame slots. One arm computes a floating-point
//! ratio from fabricated table words (exact IEEE order: signed convert,
//! biased double add, narrow, divide). After the arms, a merge region calls a
//! notifier and, for five of the command bytes, performs a data-table-driven
//! indirect call through a planted object (thiscall shape, one byte answer)
//! followed by a second notifier call. The walk advances by a scripted width
//! added to a carried offset; the base and flag it resumes with are frame
//! slots the arms maintain.
//!
//! Middle: four fixed calls; the fourth answers an object whose word at
//! offset 12 becomes the phase-2 limit.
//!
//! Phase 2 (gated by a second count byte): for each live slot, walks a record
//! whose first byte dispatches (minus a bias, range-checked) through a second
//! map into a 4-way dispatch. Arm 0 validates a record through a classifier
//! and two notifiers (with a short path for two separator bytes and a direct
//! path for one tag byte); arm 1 tests two flag bits and falls through to
//! arm 2 on failure; arm 2 appends the record to the growable table, growing
//! it by sixteen slots (allocate, copy, free) when count meets capacity;
//! arm 3 and out-of-range bytes rejoin the walk directly. The walk steps by a
//! scripted width from fixed slots. A record that fails the phase-2 limit
//! takes a slow path that refreshes the limit through the same middle
//! callees; that path only terminates when the refreshed values converge.
//!
//! Tail: two fixed calls; the second one's answer (given constant 1) is the
//! function's result.
//!
//! Frame model: the original keeps its working set in stack slots below an
//! aligned frame; several slots overlap (a flag byte lives in the top byte of
//! a saved pointer, the slot counter in the low byte of a saved base) and a
//! few reads happen before pushed arguments are cleaned, shifting which slot
//! they observe. This rewrite names each slot once and reproduces the shifts.

use core::hint::black_box;
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, relocated};

// Callee ids (see the contract): the nine stage-1 ids keep their numbers.
const C_SRC: u32 = 1; // 0x426E00 cdecl/0: source token
const C_INIT: u32 = 2; // 0x42A490 thiscall/0: init from token
const C_FREE: u32 = 3; // 0x401250 cdecl/1: release a table or node
const C_MA: u32 = 4; // 0x9523A0 cdecl/0
const C_MB: u32 = 5; // 0x9625A0 cdecl/0
const C_MC: u32 = 6; // 0x8D8570 cdecl/1
const C_MD: u32 = 7; // 0x952B30 cdecl/1: answers a readable object
const C_TAILA: u32 = 8; // 0x9543D0 cdecl/0
const C_TAILB: u32 = 9; // 0x951250 cdecl/1: finalizer, returns the result
const C_RESOLVE: u32 = 10; // 0x9528F0 cdecl/2: node lookup, NULL or node
const C_NODEA: u32 = 11; // 0xBEBDD0 thiscall/1: first node handler
const C_NODEB: u32 = 12; // 0xBEBDB0 thiscall/1: second node handler
const C_ALLOC: u32 = 13; // 0x401210 cdecl/1: allocator
const C_BUILD: u32 = 14; // 0x963080 cdecl/3: node builder
const C_NOTIFY: u32 = 15; // 0x8D8750 cdecl/3: phase-1 notifier
// 16 is the planted indirect target (no patch site; called through the object).
const C_MKOBJ: u32 = 17; // 0x499940 thiscall/0: object maker
const C_ATTACH: u32 = 18; // 0x94FB00 thiscall/1: attach node to registry
const C_WIDTH: u32 = 19; // 0x9532A0 cdecl/1: scripted walk width
const C_PAIR: u32 = 20; // 0x954190 cdecl/2: record two words
const C_SYNC: u32 = 21; // 0x965E70 cdecl/0
const C_VLD: u32 = 22; // 0xBFCEB0 thiscall/0: record validator
const C_CLASS: u32 = 23; // 0x953180 cdecl/3: classifier, NULL or node
const C_NOTE1: u32 = 24; // 0xBF4180 thiscall/3: first phase-2 notifier
const C_NOTE2: u32 = 25; // 0xBF4150 thiscall/3: second phase-2 notifier
const C_FIN3: u32 = 26; // 0x963120 cdecl/3: three-word finalizer
const C_LINK: u32 = 27; // 0x951200 cdecl/3: link node into record
const C_FIN6: u32 = 28; // 0x964BB0 cdecl/6: six-word finalizer

// Globals and tables (file addresses; read through the relocated image).
const G_SEED_LO: u32 = 0x120F29C; // qword seed, low half feeds one slot
const G_SEED: u32 = 0x120F2A4; // dword seed, low byte gates the flag
const G_TABLE: u32 = 0x11FA014; // entry table pointer
const G_COUNT: u32 = 0x11FA018; // entry count (low word) and capacity (high word)
const G_SEQ: u32 = 0x11F707C; // sequence slot
const G_FLAGS1: u32 = 0x11F6FF0; // phase-1 live flags, one byte per slot
const G_PTRS1: u32 = 0x11F6F7C; // phase-1 command strings, one word per slot
const G_GATE1W: u32 = 0x11F6FF8; // word holding the phase-1 gate in its top byte
/// Phase-1 dispatch map (data table at file 0x95513C, bias 2): arm per byte.
const T1MAP: [u8; 32] = [
    0, 9, 9, 1, 2, 9, 9, 9, 3, 3, 3, 3, 3, 4, 9, 9,
    9, 9, 9, 5, 4, 6, 4, 9, 9, 7, 7, 4, 9, 9, 8, 4,
];
const G_MKARG: u32 = 0x11F6FEC; // object-maker context
const G_FTAB: u32 = 0x11FF02C; // float-table root, NULL or table
const G_DBL: u32 = 0xFE8F50; // double bias pair (0.0, 2^32)
const G_INDTAB: u32 = 0x1295CD8; // indirect-call object table
const G_NVAL: u32 = 0x12B4138; // notifier middle argument
const G_GATE2W: u32 = 0x11F6FFC; // word holding the phase-2 gate in byte 1
const G_FLAGS2: u32 = 0x11F7018; // phase-2 live flags
const G_PTRS2: u32 = 0x11F7000; // phase-2 records
/// Phase-2 dispatch map (data table at file 0x95516C, bias 0x2C).
const T2MAP: [u8; 109] = [
    0, 3, 0, 0, 0, 3, 3, 3, 0, 3, 0, 0, 3, 3, 0, 0,
    0, 3, 0, 0, 0, 3, 3, 0, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 0, 3, 3, 0, 3, 3, 0, 0, 3, 3, 3, 3, 3, 0, 3,
    3, 0, 3, 3, 3, 0, 0, 0, 0, 0, 3, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 3, 3, 2, 3, 3, 3,
    3, 3, 3, 3, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 2, 3, 3, 3, 2, 3, 3, 3, 3, 3, 3, 2,
];
const G_LIM: u32 = 0x11F7028; // append threshold for phase-2 arm 2
const G_SAV2C: u32 = 0x11F702C; // scratch word written by both phases
const G_SAV30: u32 = 0x11F7030; // scratch word written by phase 1

const NODE_TAG: u32 = 0xE8AE94; // marker the maker arm stores into nodes
const PAIR_TAG: u32 = 0x11F9FFC; // constant second argument of the pair call
const ATTACH_CTX: u32 = 0x11FF028; // constant context of the attach call
// All three are relocated immediates: read them through the worker image,
// exactly as the relocated original code does.
#[inline(always)]
fn node_tag() -> u32 {
    relocated(NODE_TAG)
}
#[inline(always)]
fn pair_tag() -> u32 {
    relocated(PAIR_TAG)
}
#[inline(always)]
fn attach_ctx() -> u32 {
    relocated(ATTACH_CTX)
}
const T1_BIAS: u32 = 2; // phase-1 map bias
const T1_LAST: u32 = 0x1F; // phase-1 map bound
const T2_BIAS: u32 = 0x2C; // phase-2 map bias
const T2_LAST: u32 = 0x6C; // phase-2 map bound
const GROW_STEP: u16 = 0x10; // table growth in slots
const SLOT_BYTES: u32 = 8; // bytes per table slot
// The notifier calls take a 24-bit constant (pushed literally, not sign
// extended to -1): 16777215, spelled identically at every site.
const K24: u32 = 0xFF_FFFF;

#[inline(always)]
fn r8(a: u32) -> u8 {
    unsafe { (a as *const u8).read() }
}
#[inline(always)]
fn r16(a: u32) -> u16 {
    unsafe { (a as *const u16).read() }
}
#[inline(always)]
fn r32(a: u32) -> u32 {
    unsafe { (a as *const u32).read() }
}
#[inline(always)]
fn w8(a: u32, v: u8) {
    unsafe { (a as *mut u8).write(v) }
}
#[inline(always)]
fn w16(a: u32, v: u16) {
    unsafe { (a as *mut u16).write(v) }
}
#[inline(always)]
fn w32(a: u32, v: u32) {
    unsafe { (a as *mut u32).write(v) }
}

/// Shared resolve-hit block of the phase-1 lookup arms: folds the found node
/// into the running sums, toggles one flag bit each on the record and the
/// node, notifies both node handlers and links the record into the node.
/// Returns the node's running sum for the merge region.
#[inline(always)]
fn resolve_hit(node: u32, rec: u32, bh: u8, ebp: u32, f48: u32, f3c: u32, f1c: u32) -> u32 {
    let lo = r32(node);
    let sum = r32(node.wrapping_add(4)).wrapping_add(lo);
    let diff = lo.wrapping_sub(f48);
    let ne1 = (bh != r8(node.wrapping_add(8))) as u8;
    let mut fb = r8(rec.wrapping_add(4)) ^ ne1;
    fb &= 1;
    w8(rec.wrapping_add(4), r8(rec.wrapping_add(4)) ^ fb);
    callee_thiscall!(C_NODEA, u32, rec, diff);
    let ne2 = (bh != r8(node.wrapping_add(8))) as u8;
    let mut gb = (ne2.wrapping_add(ne2)) ^ r8(sum.wrapping_add(4));
    gb &= 2;
    w8(sum.wrapping_add(4), r8(sum.wrapping_add(4)) ^ gb);
    let back = ebp.wrapping_sub(f3c);
    callee_thiscall!(C_NODEB, u32, sum, back);
    w32(node.wrapping_add(4), f1c);
    w32(node, ebp);
    w8(node.wrapping_add(8), bh);
    sum
}

/// Shared resolve-miss block of the phase-1 lookup arms: allocates and links
/// a fresh node, clears the record flag and runs the builder. Returns the
/// fresh node address.
#[inline(always)]
fn resolve_miss<const MUT: bool>(rec: u32, off: u32, bh: u8, ebp: u32, f1c: u32) -> u32 {
    let n = callee_cdecl!(C_ALLOC, u32, 0xCu32);
    if n != 0 {
        w32(n, 0);
        w32(n.wrapping_add(4), 0);
        w8(n.wrapping_add(8), 0);
    }
    w32(n.wrapping_add(4), f1c);
    w32(n, ebp);
    w8(n.wrapping_add(8), bh);
    let mask: u8 = if MUT { 0x00 } else { 0xFE };
    w8(rec.wrapping_add(4), r8(rec.wrapping_add(4)) & mask);
    callee_thiscall!(C_NODEA, u32, rec, K24);
    callee_cdecl!(C_BUILD, u32, 1u32, r32(rec.wrapping_add(off)), n);
    n
}

/// One phase-1 lookup arm (resolve with the record word at `off`, optional
/// parameter word at `woff` when the flag is set). Returns the resolve
/// answer, the parameter slot value and the carried node register.
#[inline(always)]
fn lookup_arm<const MUT: bool>(
    rec: u32, off: u32, woff: u32, bl: u8, bh: u8, ebp: u32, f48: u32, f3c: u32, f1c: u32,
) -> (u32, u32, u32) {
    let mut param = 0xFFFF_FFFFu32;
    if bl != 0 {
        param = r16(rec.wrapping_add(woff)) as u32;
    }
    let node = callee_cdecl!(C_RESOLVE, u32, 1u32, r32(rec.wrapping_add(off)));
    let carried = if node != 0 {
        resolve_hit(node, rec, bh, ebp, f48, f3c, f1c)
    } else {
        resolve_miss::<MUT>(rec, off, bh, ebp, f1c)
    };
    (node, param, carried)
}

fn body<const MUT: bool>(_arg: u32) -> u32 {
    // Prologue.
    let token = callee_cdecl!(C_SRC, u32,);
    callee_thiscall!(C_INIT, u32, token);
    let seed_lo = r32(relocated(G_SEED_LO));
    let seed_hi_b = r8(relocated(G_SEED_LO).wrapping_add(4));
    let seed = r32(relocated(G_SEED));
    let g_table = relocated(G_TABLE);
    let g_count = relocated(G_COUNT);
    let g_seq = relocated(G_SEQ);
    let g_lim = relocated(G_LIM);
    let g_sav2c = relocated(G_SAV2C);
    let g_sav30 = relocated(G_SAV30);
    callee_cdecl!(C_FREE, u32, r32(g_table));
    // Frame slots (named for their offset in the original frame).
    let mut f18: u32 = 0;
    let mut f1c: u32 = 0;
    let mut f20: u32 = 0;
    let mut f24: u32 = 0;
    let mut f28: u32 = 0;
    let mut f2c: u32 = 0;
    let mut f30: u32 = 0;
    let mut f34: u32 = 0;
    let mut f38: u32 = 0;
    let mut f3c: u32 = 0;
    let mut f40: u32 = 0;
    let mut f44: u32 = 0;
    let mut f48: u32 = seed;
    let mut f54: u32 = seed;
    let mut f58lo: u32 = seed_lo;
    let mut f15: u8 = 0;
    let mut f16: u8 = 0;
    let mut esi: u32 = 0;
    let mut ebp: u32 = 0;
    w32(g_seq, 0);
    w32(g_count, 0);
    let gate1 = r8(relocated(G_GATE1W).wrapping_add(3));
    w32(g_table, 0);
    if gate1 != 0 {
        // One-time preset from the seeds (skipped with the gate).
        f16 = seed_hi_b;
        f38 = seed_lo;
        f34 = seed;
        let seed_b = seed as u8;
        let mut bh: u8 = 0;
        loop {
            let bl: u8 = (bh == seed_b) as u8;
            let flags1 = relocated(G_FLAGS1);
            let ptrs1 = relocated(G_PTRS1);
            if r8(flags1.wrapping_add(bh as u32)) != 0 {
                let s = r32(ptrs1.wrapping_add((bh as u32).wrapping_mul(4)));
                f1c = s;
                f40 = s;
                f44 = bh as u32;
                f3c = 0;
                ebp = 0;
                if r8(s) != 0 {
                    let mut edi = s;
                    // Byte walk.
                    loop {
                        let b = r8(edi);
                        f20 = 0xFFFF_FFFF;
                        let idx = (b as u32).wrapping_sub(T1_BIAS);
                        let mut to_merge = false;
                        let mut param = 0xFFFF_FFFFu32;
                        if idx > T1_LAST {
                            // Out of range: rejoin the walk directly.
                        } else {
                            let arm = T1MAP[idx as usize];
                            match arm {
                                3 => {
                                    let (node, p, e) = lookup_arm::<MUT>(
                                        edi, 0xC, 0x16, bl, bh, ebp, f48, f3c, f1c,
                                    );
                                    f30 = node;
                                    param = p;
                                    esi = e;
                                    to_merge = true;
                                }
                                5 => {
                                    let (node, p, e) = lookup_arm::<MUT>(
                                        edi, 0x8, 0xC, bl, bh, ebp, f48, f3c, f1c,
                                    );
                                    f30 = node;
                                    param = p;
                                    esi = e;
                                    to_merge = true;
                                }
                                8 => {
                                    let (node, p, e) = lookup_arm::<MUT>(
                                        edi, 0x10, 0xA, bl, bh, ebp, f48, f3c, f1c,
                                    );
                                    f30 = node;
                                    param = p;
                                    esi = e;
                                    to_merge = true;
                                }
                                6 => {
                                    let (node, p, e) = lookup_arm::<MUT>(
                                        edi, 0xC, 0xA, bl, bh, ebp, f48, f3c, f1c,
                                    );
                                    f30 = node;
                                    param = p;
                                    esi = e;
                                    to_merge = true;
                                }
                                7 => {
                                    let (node, p, e) = lookup_arm::<MUT>(
                                        edi, 0xC, 0x14, bl, bh, ebp, f48, f3c, f1c,
                                    );
                                    f30 = node;
                                    param = p;
                                    esi = e;
                                    to_merge = true;
                                }
                                4 => {
                                    let node = callee_cdecl!(
                                        C_RESOLVE, u32, 1u32,
                                        r32(edi.wrapping_add(4))
                                    );
                                    esi = node;
                                    if node != 0 {
                                        let sum = r32(node.wrapping_add(4))
                                            .wrapping_add(r32(node));
                                        w8(
                                            sum.wrapping_add(4),
                                            r8(sum.wrapping_add(4)) & 0xFD,
                                        );
                                        callee_thiscall!(
                                            C_NODEB, u32, sum, K24
                                        );
                                        callee_cdecl!(
                                            C_LINK, u32, 1u32,
                                            r32(edi.wrapping_add(4)), esi
                                        );
                                        callee_cdecl!(C_FREE, u32, esi);
                                    }
                                    esi = f18;
                                }
                                0 => {
                                    if esi != 0 {
                                        let mut cx = f24;
                                        let mut al = r8(edi.wrapping_add(4));
                                        let bl2 = (bh != f15) as u8;
                                        esi = esi.wrapping_add(cx);
                                        cx = cx.wrapping_sub(f48);
                                        al = (al & 0xFE) | (bl2 & 1);
                                        w8(edi.wrapping_add(4), al);
                                        callee_thiscall!(C_NODEA, u32, edi, cx);
                                        let bl3 =
                                            (bl2 << 1) | (r8(esi.wrapping_add(4)) & 0xFD);
                                        let back = ebp.wrapping_sub(f3c);
                                        w8(esi.wrapping_add(4), bl3);
                                        callee_thiscall!(C_NODEB, u32, esi, back);
                                        esi = f1c;
                                        f18 = esi;
                                        f24 = ebp;
                                        f15 = bh;
                                    } else {
                                        w8(
                                            edi.wrapping_add(4),
                                            r8(edi.wrapping_add(4)) & 0xFE,
                                        );
                                        esi = edi;
                                        f1c = esi;
                                        f28 = ebp;
                                        f18 = (f18 & !0xFF00) | ((bh as u32) << 8);
                                        callee_thiscall!(C_NODEA, u32, edi, K24);
                                    }
                                }
                                2 => {
                                    let esi2 = r32(edi.wrapping_add(4));
                                    let eax2 = r32(edi.wrapping_add(8));
                                    esi = esi2;
                                    f2c = esi2;
                                    f28 = eax2;
                                    if r32(g_seq) == 0 {
                                        callee_cdecl!(C_PAIR, u32, pair_tag(), esi2);
                                        w32(g_sav30, 0);
                                        w32(g_lim, r32(edi.wrapping_add(4)));
                                        callee_cdecl!(C_SYNC, u32,);
                                    }
                                    esi = f18;
                                }
                                1 => {
                                    let ctx0 = r32(relocated(G_MKARG));
                                    let made = callee_thiscall!(C_MKOBJ, u32, ctx0);
                                    esi = made;
                                    if made != 0 {
                                        w32(made.wrapping_add(8), 0);
                                        w32(made.wrapping_add(0xC), 0);
                                        w32(made, node_tag());
                                    }
                                    let n = callee_cdecl!(C_ALLOC, u32, 0x20u32);
                                    if n != 0 {
                                        w32(n, 0);
                                        w32(n.wrapping_add(4), 0);
                                        w8(n.wrapping_add(8), 0);
                                        w32(n.wrapping_add(0xC), 0);
                                        w32(n.wrapping_add(0x14), 0);
                                        w32(n.wrapping_add(0x1C), 0xFFFF_FFFF);
                                    }
                                    let edx0 = f2c;
                                    let eax28 = f28;
                                    w32(n.wrapping_add(4), f38);
                                    w8(n.wrapping_add(8), f16);
                                    w32(n, f34);
                                    w32(n.wrapping_add(0xC), edx0);
                                    w32(n.wrapping_add(0x10), eax28);
                                    let q = r32(relocated(G_FTAB));
                                    if q == 0 {
                                        w32(n.wrapping_add(0x14), 0);
                                        w32(n.wrapping_add(0x18), 0x3F80_0000);
                                    } else {
                                        // Ratio of two table-driven differences,
                                        // computed in double precision then narrowed.
                                        let seg_p = r32(q.wrapping_add(4));
                                        let q1hi = r32(seg_p.wrapping_add(0xC));
                                        let q2lo = r32(seg_p.wrapping_add(0x10));
                                        let sub1 =
                                            (edx0 as i32).wrapping_sub(q1hi as i32);
                                        w32(n.wrapping_add(0x14), sub1 as u32);
                                        let dbl = relocated(G_DBL);
                                        let b1off = ((sub1 as u32) >> 31) << 3;
                                        let b1lo = r32(dbl.wrapping_add(b1off));
                                        let b1hi = r32(dbl.wrapping_add(b1off + 4));
                                        let bias1 = f64::from_bits(
                                            ((b1hi as u64) << 32) | b1lo as u64,
                                        );
                                        let si1 = black_box(sub1 as f64);
                                        let d1 = black_box(si1 + bias1);
                                        let f1 = black_box(d1 as f32);
                                        let sub2 = (eax28 as i32)
                                            .wrapping_sub(q2lo as i32);
                                        let b2off = ((sub2 as u32) >> 31) << 3;
                                        let b2lo = r32(dbl.wrapping_add(b2off));
                                        let b2hi = r32(dbl.wrapping_add(b2off + 4));
                                        let bias2 = f64::from_bits(
                                            ((b2hi as u64) << 32) | b2lo as u64,
                                        );
                                        let si2 = black_box(sub2 as f64);
                                        let d2 = black_box(si2 + bias2);
                                        let f2 = black_box(d2 as f32);
                                        let quot = black_box(f1 / f2);
                                        w32(n.wrapping_add(0x18), quot.to_bits());
                                    }
                                    w32(n.wrapping_add(0x1C), r32(g_seq));
                                    w32(esi.wrapping_add(4), n);
                                    callee_thiscall!(C_ATTACH, u32, attach_ctx(), esi);
                                    w32(g_sav2c, f2c);
                                    esi = f1c;
                                    // Shifted stores (one argument pushed):
                                    // the seed-double slot takes the running
                                    // pair, the base slot is rewritten equal.
                                    f48 = f3c;
                                    f40 = esi;
                                    let byte0 = r8(edi);
                                    let w1 =
                                        callee_cdecl!(C_WIDTH, u32, byte0 as u32);
                                    let eax_tail = w1.wrapping_add(ebp);
                                    w32(g_seq, r32(g_seq).wrapping_add(1));
                                    f34 = eax_tail;
                                    f3c = eax_tail;
                                    f38 = esi;
                                    f16 = bh;
                                    esi = f18;
                                }
                                _ => {
                                    // Default arm: rejoin the walk directly.
                                }
                            }
                        }
                        if to_merge {
                            f20 = param;
                            let esi_m = param;
                            if (esi_m as i32) >= 0 && bl != 0 {
                                let b2 = r8(edi);
                                let a8 =
                                    if b2 == 0x1B || b2 == 0x1C { 8u32 } else { 1u32 };
                                let nval = r32(relocated(G_NVAL));
                                callee_cdecl!(C_NOTIFY, u32, esi_m, nval, a8);
                                if b2 == 0x0A
                                    || b2 == 0x0D
                                    || b2 == 0x0C
                                    || b2 == 0x0B
                                    || b2 == 0x0E
                                {
                                    let indtab = relocated(G_INDTAB);
                                    let o = r32(
                                        indtab.wrapping_add(esi_m.wrapping_mul(4)),
                                    );
                                    if o != 0 {
                                        let vt = r32(o);
                                        let tgt = r32(vt.wrapping_add(0xC));
                                        let call: extern "thiscall" fn(u32) -> u8 =
                                            unsafe {
                                                core::mem::transmute(tgt as usize)
                                            };
                                        let al = call(o);
                                        if al == 5 {
                                            let e = r32(o.wrapping_add(0x34C));
                                            if e != 0xFFFF_FFFF {
                                                callee_cdecl!(C_NOTIFY, u32, e, nval, 1u32);
                                            }
                                        }
                                    }
                                }
                            }
                            esi = f18;
                        }
                        // Walk advance. The reads run before the pushed
                        // width argument is cleaned, so they observe the
                        // string base and the saved flag, not the slots the
                        // raw offsets name.
                        let w = callee_cdecl!(C_WIDTH, u32, r8(edi) as u32);
                        let base = f1c;
                        ebp = ebp.wrapping_add(w);
                        edi = base.wrapping_add(ebp);
                        if r8(edi) == 0 {
                            break;
                        }
                    }
                }
            }
            bh = bh.wrapping_add(1);
            if bh >= gate1 {
                break;
            }
        }
    }
    // Middle.
    callee_cdecl!(C_MA, u32,);
    callee_cdecl!(C_MB, u32,);
    callee_cdecl!(C_MC, u32, 0u32);
    ebp = 0;
    f18 = 0; // shifted store (two arguments pushed): the saved node slot
    let obj = callee_cdecl!(C_MD, u32, 1u32);
    let gate2 = r8(relocated(G_GATE2W).wrapping_add(1));
    let mut edx = r32(obj.wrapping_add(12));
    f20 = edx;
    f1c &= 0xFFFF_FF00;
    if gate2 == 0 {
        callee_cdecl!(C_TAILA, u32,);
        return callee_cdecl!(C_TAILB, u32, 1u32);
    }
    // Phase 2.
    for ch in 0..gate2 {
        if r8(relocated(G_FLAGS2).wrapping_add(ch as u32)) == 0 {
            f1c = (f1c & !0xFF) | (ch.wrapping_add(1) as u32);
            continue;
        }
        let esi2 = r32(relocated(G_PTRS2).wrapping_add((ch as u32).wrapping_mul(4)));
        f2c = esi2;
        let mut edi2: u32 = 0;
        f28 = 0;
        f38 = ebp.wrapping_add(1);
        let mut ebx = esi2;
        loop {
            let mut cl = r8(ebx);
            f16 = cl;
            if cl == 0 {
                break;
            }
            ebp = r32(ebx.wrapping_add(4));
            if ebp < edx {
                // Dispatch byte is the current walk byte, not the base:
                // a two-byte probe fires the second byte's arm.
                let idx = (cl as u32).wrapping_sub(T2_BIAS);
                if idx > T2_LAST {
                    // Out of range: rejoin the walk directly.
                } else {
                    let arm = T2MAP[idx as usize];
                    match arm {
                        0 => {
                            let edx_a = r32(ebx.wrapping_add(0xC));
                            if edx_a != 0xFFFF_FFFF {
                                let mut ebp2: u32 = 0;
                                let mut short = false;
                                if cl == 0x53 {
                                    callee_thiscall!(C_VLD, u32, ebx);
                                    ebp2 = callee_cdecl!(C_ALLOC, u32, 0xCu32);
                                    if ebp2 != 0 {
                                        w32(ebp2, 0);
                                        w32(ebp2.wrapping_add(4), 0);
                                        w8(ebp2.wrapping_add(8), 0);
                                    }
                                    short = true;
                                } else {
                                    let nn = callee_cdecl!(
                                        C_CLASS, u32,
                                        r32(ebx.wrapping_add(8)),
                                        edx_a,
                                        r8(ebx.wrapping_add(0x14)) as u32
                                    );
                                    let cl2 = r8(ebx);
                                    if cl2 == 0x2C || cl2 == 0x2F || nn == 0 {
                                        ebp2 = callee_cdecl!(C_ALLOC, u32, 0xCu32);
                                        if ebp2 != 0 {
                                            w32(ebp2, 0);
                                            w32(ebp2.wrapping_add(4), 0);
                                            w8(ebp2.wrapping_add(8), 0);
                                        }
                                        short = true;
                                    } else {
                                        let aln = r8(nn.wrapping_add(8));
                                        // The notifier takes the node pointer
                                        // with its low byte replaced by the
                                        // node's tag, plus the node's first
                                        // word: the original overwrites al of
                                        // the classifier's answer in place.
                                        let nn_tagged = (nn & 0xFFFF_FF00)
                                            | aln as u32;
                                        let w0 = r32(nn);
                                        let setne = ((f1c as u8) != aln) as u32;
                                        callee_thiscall!(
                                            C_NOTE1, u32, ebx, setne, nn_tagged,
                                            w0
                                        );
                                        callee_thiscall!(
                                            C_NOTE2, u32, ebx, 0u32, 0u32, K24
                                        );
                                        let setne2 =
                                            ((f1c as u8) != r8(nn.wrapping_add(8)))
                                                as u32;
                                        let esi_n = r32(nn.wrapping_add(4))
                                            .wrapping_add(r32(nn));
                                        callee_thiscall!(
                                            C_NOTE2, u32, esi_n, setne2, f1c, edi2
                                        );
                                        w32(nn.wrapping_add(4), f2c);
                                        w32(nn, edi2);
                                        w8(nn.wrapping_add(8), f1c as u8);
                                        if r8(ebx) == 0x53 {
                                            let esi3 = r32(ebx.wrapping_add(0xC));
                                            let edi3 = r32(ebx.wrapping_add(8));
                                            let eax3 = callee_thiscall!(
                                                C_VLD, u32, ebx
                                            );
                                            callee_cdecl!(
                                                C_FIN3, u32, edi3, esi3, eax3
                                            );
                                        } else {
                                            callee_cdecl!(
                                                C_FIN3, u32,
                                                r32(ebx.wrapping_add(8)),
                                                r32(ebx.wrapping_add(0xC)),
                                                r8(ebx.wrapping_add(0x14)) as u32
                                            );
                                        }
                                        // Full path rejoins the shared walk below.
                                    }
                                }
                                // Shared tail of the short paths. The flag
                                // read runs with one argument pushed, so it
                                // observes the slot counter, not the limit.
                                if short {
                                w32(ebp2.wrapping_add(4), f2c);
                                w32(ebp2, edi2);
                                w8(ebp2.wrapping_add(8), f1c as u8);
                                callee_thiscall!(
                                    C_NOTE1, u32, ebx, 0u32, 0u32, K24
                                );
                                callee_thiscall!(
                                    C_NOTE2, u32, ebx, 0u32, 0u32, K24
                                );
                                if r8(ebx) == 0x53 {
                                    let esi3 = r32(ebx.wrapping_add(0xC));
                                    let edi3 = r32(ebx.wrapping_add(8));
                                    let eax3 =
                                        callee_thiscall!(C_VLD, u32, ebx);
                                    callee_cdecl!(
                                        C_FIN6, u32, edi3, esi3, ebp2, 1u32, eax3, 1u32
                                    );
                                } else {
                                    callee_cdecl!(
                                        C_FIN6, u32,
                                        r32(ebx.wrapping_add(8)),
                                        r32(ebx.wrapping_add(0xC)),
                                        ebp2, 1u32,
                                        r8(ebx.wrapping_add(0x14)) as u32, 1u32
                                    );
                                }
                                }
                            }
                        }
                        1 => {
                            let ab = r8(ebx.wrapping_add(2));
                            if ab & 3 == 0 {
                                // Flag test passes: rejoin directly.
                            } else if ebp >= r32(g_lim) {
                                append_pair(g_table, g_count, ebx, ebp);
                            }
                        }
                        2 => {
                            if ebp >= r32(g_lim) {
                                append_pair(g_table, g_count, ebx, ebp);
                            }
                        }
                        _ => {
                            // Default arm: rejoin the walk directly.
                        }
                    }
                }
                // Walk advance (shifted reads: the running offset
                // and the slot base, not the raw offsets' slots).
                let w = callee_cdecl!(C_WIDTH, u32, r8(ebx) as u32);
                let edx_new = f28.wrapping_add(w);
                let esi_new = f2c.wrapping_add(edx_new);
                ebx = esi_new;
                f28 = edx_new;
                edx = f20;
                cl = r8(ebx);
                f16 = cl;
                if cl == 0 {
                    break;
                }
                edi2 = f28;
            } else {
                // Slow path: refresh the limit, then re-dispatch.
                ebp = f18;
                if r8(ebx) == 0 {
                    break;
                }
                let eax38 = f38.wrapping_add(1);
                ebp = ebp.wrapping_add(1);
                f18 = ebp;
                f38 = eax38;
                if eax38 == r32(g_seq) {
                    callee_cdecl!(C_TAILA, u32,);
                    return callee_cdecl!(C_TAILB, u32, 1u32);
                }
                let rr = callee_cdecl!(C_MD, u32, eax38);
                // Shifted store (two arguments pushed): refreshes the
                // phase-2 limit, not the running offset.
                f20 = r32(rr.wrapping_add(12));
                callee_cdecl!(C_TAILB, u32, 1u32);
                callee_cdecl!(C_TAILA, u32,);
                edx = f20;
            }
        }
        f1c = (f1c & !0xFF) | (ch.wrapping_add(1) as u32);
    }
    callee_cdecl!(C_TAILA, u32,);
    callee_cdecl!(C_TAILB, u32, 1u32)
}

/// Phase-2 append: stores one (key, value) pair at the count index, growing
/// the table by sixteen slots when count meets capacity.
fn append_pair(g_table: u32, g_count: u32, key: u32, val: u32) {
    let count = r16(g_count);
    let cap = r16(g_count.wrapping_add(2));
    let base = if count == cap {
        let newcap = cap.wrapping_add(GROW_STEP);
        w16(g_count.wrapping_add(2), newcap);
        let size = (newcap as u32) << 3;
        let nn = callee_cdecl!(C_ALLOC, u32, size);
        let old = r32(g_table);
        for i in 0..(count as u32) {
            let w0 = r32(old.wrapping_add(i.wrapping_mul(SLOT_BYTES)));
            w32(nn.wrapping_add(i.wrapping_mul(SLOT_BYTES)), w0);
            let w1 = r32(old.wrapping_add(i.wrapping_mul(SLOT_BYTES).wrapping_add(4)));
            w32(
                nn.wrapping_add(i.wrapping_mul(SLOT_BYTES).wrapping_add(4)),
                w1,
            );
        }
        callee_cdecl!(C_FREE, u32, old);
        w32(g_table, nn);
        nn
    } else {
        r32(g_table)
    };
    let idxa = count as u32;
    w16(g_count, count.wrapping_add(1));
    w32(base.wrapping_add(idxa.wrapping_mul(SLOT_BYTES)), key);
    w32(
        base.wrapping_add(idxa.wrapping_mul(SLOT_BYTES).wrapping_add(4)),
        val,
    );
}

export!(cdecl, rw_00954770(_arg: u32) -> u32 {
    body::<false>(_arg)
});

export!(cdecl, mut_00954770(_arg: u32) -> u32 {
    body::<true>(_arg)
});