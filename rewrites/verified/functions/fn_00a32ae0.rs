// original: 0x00a32ae0 dispatch_probed_items
//
// Resolve the wanted id through the prologue object, then scan the item
// table selected by the u16 at [this+0x2E] like dispatch_indexed_items
// (0x00A333C0), with two submit paths per kind.
//
// `this`, `arg0` and `arg1` are opaque words carried into the submit calls;
// `arg2` indexes the resolver table. The prologue calls the slot +0xA0 of
// `[this]` (skipped with id 0 when it answers null), resolves a cell
// through the +0xD4 table indexed by `arg2`, sets a flag byte when the
// float at the resolved cell compares above zero (`0.0 > m`, ordered), and
// reads the wanted id as the u16 at cell-index times 0xE0 plus 0x16. Each
// item is then probed through vtable slot +4: kind 1 submits the slot
// +0x1C descriptor directly when its type word (+0x24) is 2; kind 2 takes
// the slot +0x24 descriptor down path A (type 0: emit immediately) or path
// B (type 3: resolve through the 4-argument helper, check its answer, run
// the two frame-pointer transforms, then emit). When the flag byte is set,
// descriptors also pass a presence byte (+0x38 on the direct one, +0x2C on
// the transform one). The emit call carries the descriptor's +0x20 value,
// the id, a -1.0/+1.0 pair and, on path B only, the factor/float frame
// blocks. Counts are SIGNED at entry (`jle`) and at the latch (`jl`).
//
// Original: 0x00A32AE0 (thiscall, ECX=this, three stack words).

use lf_checker_rt::{callee_cdecl, callee_thiscall};

// Callee ids (see contract).
const C_COUNT: u32 = 1; // thiscall/0: item count (SIGNED)
const C_ITEM: u32 = 2; // thiscall/1: item by index
const C_VT_PROBE: u32 = 3; // item vtable slot +4 (both probe sites share it)
const C_VT_DIRECT: u32 = 4; // item vtable slot +0x1C (direct-submit descriptor)
const C_VT_XFORM: u32 = 5; // item vtable slot +0x24 (transform-submit descriptor)
const C_DIRECT: u32 = 6; // thiscall/5 on the shared global object
const C_EMIT: u32 = 7; // cdecl/19 sink for the submits
const C_GATE1: u32 = 8; // cdecl/0 flag gate, AL channel
const C_GATE2: u32 = 9; // cdecl/0 flag gate, AL channel
const C_V0: u32 = 10; // [this]+0xA0 slot (prologue object source)
const C_CONV5: u32 = 11; // thiscall/0 on the prologue object
const C_CONV4: u32 = 12; // cdecl/4 resolver (frame-pointer out-cell arg)
const C_CHK: u32 = 13; // thiscall/0 answer check, AL channel (0 continues)
const C_XFORM: u32 = 14; // thiscall/1 (both pointers into the frame)
const C_FIN: u32 = 15; // thiscall/1 (ECX into the frame, arg is scripted)

const DISPATCH_TABLE: u32 = 0x0129_5CD8; // indexed by the u16 at [this+0x2E]
const GLOBAL_OBJ: u32 = 0x0171_DEE8; // `this` of the direct-submit call
const ACTIVE_BYTE_TABLE: u32 = 0x012F_8498; // stride 0x70, indexed by [+0x20]

const VT_PROBE: u32 = 0x04;
const VT_DIRECT: u32 = 0x1C;
const VT_XFORM: u32 = 0x24;

const PROBE_ENTRY_FLAG: u8 = 4; // required bit in entry byte +0x5B
const V0_SLOT: u32 = 0xA0; // slot of the prologue-object source in [this]
const DONE_BIT: u32 = 0x0010_0000; // OR-ed into +0x28 words on success
const MODE_MASK: u32 = 0x3C0;
const MODE_WANT: u32 = 0x100;

const F_ONE: u32 = 0x3F80_0000;
const F_NEG_ONE: u32 = 0xBF80_0000;

#[inline(always)]
unsafe fn rd8(a: u32) -> u8 {
    unsafe { (a as *const u8).read() }
}
#[inline(always)]
unsafe fn rd16(a: u32) -> u16 {
    unsafe { (a as *const u16).read_unaligned() }
}
#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}
#[inline(always)]
unsafe fn rdf(a: u32) -> f32 {
    unsafe { f32::from_bits(rd32(a)) }
}

/// Call an item-vtable slot exactly like the original: load the slot from the
/// fabricated vtable and call it as thiscall/0. Both sides land on the same
/// planted recorder stub.
#[inline(always)]
unsafe fn vcall0(vt: u32, slot: u32, this: u32) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vt + slot) as usize);
        f(this)
    }
}

/// Pick the assistance pointer for the emit call: [this+0x27C], else the
/// same field of the [this+0x280] object when its mode matches, else null.
#[inline(always)]
unsafe fn pick_ecx3(this: u32) -> u32 {
    unsafe {
        if rd32(this + 0x28) & MODE_MASK != MODE_WANT {
            return 0;
        }
        let c = rd32(this + 0x27C);
        if c != 0 {
            return c;
        }
        let o = rd32(this + 0x280);
        if o != 0 && rd32(o + 0x28) & MODE_MASK == MODE_WANT {
            return rd32(o + 0x27C);
        }
        0
    }
}

/// The two polled gates and the done-bit updates shared by both dispatchers.
#[inline(always)]
unsafe fn gates_and_flags(this: u32) {
    unsafe {
        let g1 = callee_cdecl!(C_GATE1, u32,);
        let do_or = if (g1 as u8) == 0 {
            true
        } else {
            (callee_cdecl!(C_GATE2, u32,) as u8) != 0
        };
        if do_or {
            let fl = rd32(this + 0x28) | DONE_BIT;
            wr32(this + 0x28, fl);
            if fl & MODE_MASK == MODE_WANT {
                let o = rd32(this + 0x280);
                if o != 0 {
                    wr32(o + 0x28, rd32(o + 0x28) | DONE_BIT);
                }
            }
        }
    }
}

#[inline(always)]
unsafe fn wr32(a: u32, v: u32) {
    unsafe { (a as *mut u32).write_unaligned(v) }
}

lf_checker_rt::export!(thiscall, rw_a32ae0(this: u32, arg0: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        // Models the original's unwritten frame words (defined fill 0) that
        // are passed by address to the frame-pointer callees.
        let zeros = [0u32; 4];
        let w = rd16(this + 0x2E);
        if w == 0xFFFF {
            return w as u32;
        }
        let table = lf_checker_rt::relocated(DISPATCH_TABLE);
        let entry0 = rd32(table + (w as u32) * 4);
        if rd8(entry0 + 0x5B) & PROBE_ENTRY_FLAG == 0 {
            return entry0;
        }
        let mut idslot: u32 = 0;
        let mut flagbyte: u8 = 0;
        let v0 = vcall0(rd32(this), V0_SLOT, this);
        if v0 != 0 {
            let r = callee_thiscall!(C_CONV5, u32, v0);
            let u = rd32(rd32(r + 0xD4).wrapping_add(arg2.wrapping_mul(4)));
            if u == 0 {
                return r;
            }
            let q = rd32(v0 + 0x64);
            if q != 0 {
                let m = rdf(rd32(q + 0x104).wrapping_add((rd8(u + 0xC) as u32) * 4));
                // comiss 0.0,m then cmovae: set iff 0.0 exceeds m, ordered.
                if 0.0f32 > m {
                    flagbyte = 1;
                }
            }
            let sx = rd16(u + 0xE) as i16 as i32;
            if sx == -1 {
                return 0xFFFF_FFFF;
            }
            let s = rd32(r + 0x1E0);
            if s == 0 {
                return 0;
            }
            let cell = rd32(rd32(s + 4)).wrapping_add((sx as u32).wrapping_mul(0xE0));
            if cell == 0 {
                return 0;
            }
            idslot = rd16(cell + 0x16) as u32;
        }
        let ws = rd16(this + 0x2E) as i16 as i32;
        let entry = rd32(table + (ws as u32) * 4);
        let count = callee_thiscall!(C_COUNT, u32, entry);
        if (count as i32) <= 0 {
            return count;
        }
        let mut idx: u32 = 0;
        loop {
            let item = callee_thiscall!(C_ITEM, u32, entry, idx);
            let f0 = rdf(item + 4);
            let f1 = rdf(item + 8);
            let f2 = rdf(item + 12);
            let vt = rd32(item);
            let kind = vcall0(vt, VT_PROBE, item);
            if (kind as u8) == 1 {
                let desc = vcall0(vt, VT_DIRECT, item);
                let mut ok = flagbyte == 0 || rd8(desc + 0x38) == 0;
                if ok {
                    let id = rd32(desc + 0x28);
                    if id != idslot && id != 0xFFFF_FFFF {
                        ok = false;
                    } else if rd32(desc + 0x24) != 2 {
                        ok = false;
                    } else {
                        let obj = lf_checker_rt::relocated(GLOBAL_OBJ);
                        callee_thiscall!(C_DIRECT, u32, obj, this, desc, arg0, arg1, idslot);
                    }
                }
            } else if (vcall0(vt, VT_PROBE, item) as u8) == 2 {
                let desc = vcall0(vt, VT_XFORM, item);
                if flagbyte == 0 || rd8(desc + 0x2C) == 0 {
                    let id2 = rd32(desc + 0x28);
                    if id2 == idslot || id2 == 0xFFFF_FFFF {
                        let t = rd32(desc + 0x24);
                        let val = rd32(desc + 0x20);
                        let flags = rd32(this + 0x28);
                        let mut run = t == 0 || t == 3;
                        if run && flags & DONE_BIT != 0 {
                            let btab = lf_checker_rt::relocated(ACTIVE_BYTE_TABLE);
                            if rd8(btab.wrapping_add(val.wrapping_mul(0x70))) == 0 {
                                run = false;
                            }
                        }
                        if run {
                            let ecx3 = pick_ecx3(this);
                            let sete = if ecx3 == 0 { 1u32 } else { 0u32 };
                            if t == 0 {
                                callee_cdecl!(
                                    C_EMIT, u32, 0, 0, val, F_ONE, arg0, 0, 0, 1, F_NEG_ONE, 0,
                                    0, arg1, this, idslot, 0, sete, ecx3, 0, 0xFFFF_FFFF
                                );
                                gates_and_flags(this);
                            } else {
                                // Out-cell for the resolver: its byte is set
                                // to 1 here; the snapshot compares it with
                                // the flag and fill bytes beside it.
                                let mut ocell = [0u8; 8];
                                ocell[3] = 1;
                                ocell[4] = flagbyte;
                                let ret1 = callee_cdecl!(
                                    C_CONV4, u32, this, idslot, 1,
                                    ocell.as_ptr().wrapping_add(3) as u32
                                );
                                if ret1 != 0 && (callee_thiscall!(C_CHK, u32, ret1) as u8) == 0 {
                                    let quad2 = [
                                        rdf(desc + 0x10),
                                        rdf(desc + 0x14),
                                        rdf(desc + 0x18),
                                        rdf(desc + 0x1C),
                                    ];
                                    callee_thiscall!(
                                        C_XFORM, u32, zeros.as_ptr() as u32, quad2.as_ptr() as u32
                                    );
                                    callee_thiscall!(C_FIN, u32, zeros.as_ptr() as u32, ret1);
                                    let fblock = [f0, f1, f2, rdf(desc + 0x1C)];
                                    callee_cdecl!(
                                        C_EMIT, u32, 0, 0, val, F_ONE, fblock.as_ptr() as u32, 0,
                                        0, 1, F_NEG_ONE, 0, 0, zeros.as_ptr() as u32, this, idslot,
                                        0, sete, ecx3, 0, 0xFFFF_FFFF
                                    );
                                    gates_and_flags(this);
                                }
                            }
                        }
                    }
                }
            }
            idx = idx.wrapping_add(1);
            let n = callee_thiscall!(C_COUNT, u32, entry);
            if (idx as i32) < (n as i32) {
                continue;
            }
            return n;
        }
    }
});
