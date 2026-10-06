// original: 0x00a333c0 dispatch_indexed_items
//
// Scan the item table selected by the u16 at [this+0x2E] and submit each
// item whose kind probe and descriptor match the wanted id.
//
// `this` is the owner object, `arg_id` the wanted descriptor id (a stored
// id of -1 matches anything), `arg_mat` a float matrix the transform path
// combines with per-item factors. Each item is probed through vtable slot
// +4: kind 1 submits the slot +0x1C descriptor directly when its type word
// (+0x24) is 4; kind 2 submits the slot +0x24 descriptor through the float
// transform when its type word is 2. The transform reads three factor
// floats from the item (+4/+8/+0xC) and folds them with rows of the matrix
// into three results passed (with a -1.0/+1.0 pair and flag words) to the
// 19-argument sink. Two polled gates decide whether the done bit is OR-ed.
//
// Counts are SIGNED (the loop entry uses `jle` on the count and the latch
// uses `jl` on counter minus count). Every mid-body mismatch jumps to the
// loop latch (next item), not to the function exit; only the three entry
// checks return early, with EAX holding the value the check examined.
//
// Original: 0x00A333C0 (thiscall, ECX=this, two stack words).

use lf_checker_rt::{callee_cdecl, callee_thiscall};

// Callee ids (see contract).
const C_COUNT: u32 = 1; // thiscall/0: item count (SIGNED)
const C_ITEM: u32 = 2; // thiscall/1: item by index
const C_VT_PROBE: u32 = 3; // item vtable slot +4 (both probe sites share it)
const C_VT_DIRECT: u32 = 4; // item vtable slot +0x1C (direct-submit descriptor)
const C_VT_XFORM: u32 = 5; // item vtable slot +0x24 (transform-submit descriptor)
const C_DIRECT: u32 = 6; // thiscall/3 on the shared global object
const C_EMIT: u32 = 7; // cdecl/19 sink for the transformed rows
const C_GATE1: u32 = 8; // cdecl/0 flag gate, AL channel
const C_GATE2: u32 = 9; // cdecl/0 flag gate, AL channel

const DISPATCH_TABLE: u32 = 0x0129_5CD8; // indexed by the u16 at [this+0x2E]
const GLOBAL_OBJ: u32 = 0x0171_DEE8; // `this` of the direct-submit call
const ACTIVE_BYTE_TABLE: u32 = 0x012F_8498; // stride 0x70, indexed by [+0x20]
const GLOBAL_DWORD: u32 = 0x0117_35B4;

const VT_PROBE: u32 = 0x04;
const VT_DIRECT: u32 = 0x1C;
const VT_XFORM: u32 = 0x24;

const ENTRY_FLAG: u8 = 0x10; // required bit in entry byte +0x5B
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
#[inline(always)]
unsafe fn wr32(a: u32, v: u32) {
    unsafe { (a as *mut u32).write_unaligned(v) }
}
#[inline(always)]
fn fmul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}
#[inline(always)]
fn fadd(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
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

lf_checker_rt::export!(thiscall, rw_a333c0(this: u32, arg_id: u32, arg_mat: u32) -> u32 {
    unsafe {
        let w = rd16(this + 0x2E);
        if w == 0xFFFF {
            return w as u32;
        }
        let table = lf_checker_rt::relocated(DISPATCH_TABLE);
        let entry = rd32(table + (w as u32) * 4);
        if rd8(entry + 0x5B) & ENTRY_FLAG == 0 {
            return entry;
        }
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
                if rd32(desc + 0x24) == 4 {
                    let id = rd32(desc + 0x28);
                    if id == arg_id || id == 0xFFFF_FFFF {
                        let obj = lf_checker_rt::relocated(GLOBAL_OBJ);
                        callee_thiscall!(C_DIRECT, u32, obj, this, desc, arg_mat);
                    }
                }
            } else {
                let kind2 = vcall0(vt, VT_PROBE, item);
                if (kind2 as u8) == 2 {
                    let desc = vcall0(vt, VT_XFORM, item);
                    if rd32(desc + 0x24) == 2 {
                        let id = rd32(desc + 0x28);
                        if id == arg_id || id == 0xFFFF_FFFF {
                            let val = rd32(desc + 0x20);
                            let flags = rd32(this + 0x28);
                            let dl = ((flags >> 0x14) & 1) as u8;
                            let btab = lf_checker_rt::relocated(ACTIVE_BYTE_TABLE);
                            let active = rd8(btab.wrapping_add(val.wrapping_mul(0x70)));
                            let mut run = true;
                            if active == 0 {
                                if flags & MODE_MASK == MODE_WANT {
                                    let o = rd32(this + 0x280);
                                    if o != 0 && rd32(o + 0x28) & DONE_BIT != 0 {
                                        run = false;
                                    }
                                }
                                if dl != 0 {
                                    run = false;
                                }
                            }
                            if run {
                                // Three transformed rows. Each is
                                // ((rowA*f + rowB*f) + rowC*f) + rowD with the
                                // original's association and operand order.
                                let r3 = fadd(
                                    fadd(
                                        fadd(fmul(rdf(arg_mat + 0x10), f1), fmul(rdf(arg_mat), f0)),
                                        fmul(rdf(arg_mat + 0x20), f2),
                                    ),
                                    rdf(arg_mat + 0x30),
                                );
                                let r2 = fadd(
                                    fadd(
                                        fadd(fmul(rdf(arg_mat + 0x14), f1), fmul(rdf(arg_mat + 4), f0)),
                                        fmul(rdf(arg_mat + 0x24), f2),
                                    ),
                                    rdf(arg_mat + 0x34),
                                );
                                let r1 = fadd(
                                    fadd(
                                        fadd(fmul(rdf(arg_mat + 0x18), f1), fmul(rdf(arg_mat + 8), f0)),
                                        fmul(rdf(arg_mat + 0x28), f2),
                                    ),
                                    rdf(arg_mat + 0x38),
                                );
                                // The original passes a frame block of [r3,
                                // r2, r1, fill]; the fourth word is the
                                // defined stack fill (0), never written.
                                let quad = [r3, r2, r1, 0.0f32];
                                let mode = (flags >> 6) & 0xF;
                                let mut edx: u32 = 0;
                                if mode == 4 {
                                    edx = rd32(this + 0x27C);
                                    if edx == 0 {
                                        let o = rd32(this + 0x280);
                                        if o != 0 && rd32(o + 0x28) & MODE_MASK == MODE_WANT {
                                            edx = rd32(o + 0x27C);
                                        }
                                    }
                                }
                                let mut extra: u32 = 0;
                                if mode == 4 || mode == 2 || mode == 3 {
                                    let gd = lf_checker_rt::relocated(GLOBAL_DWORD);
                                    if rd32(this + 0x1E8) == rd32(gd) {
                                        extra = rd32(this + 0x1E4);
                                    }
                                }
                                let sete = if edx == 0 { 1u32 } else { 0u32 };
                                callee_cdecl!(
                                    C_EMIT, u32, 0, extra, val, F_ONE,
                                    quad.as_ptr() as u32, 0, 0, 1, F_NEG_ONE, 0, 0,
                                    arg_mat.wrapping_add(0x20), this, arg_id, 0, sete, edx, 0,
                                    0xFFFF_FFFF
                                );
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
                    }
                }
            }
            // Loop latch: signed counter-minus-count decides.
            idx = idx.wrapping_add(1);
            let n = callee_thiscall!(C_COUNT, u32, entry);
            if (idx as i32) < (n as i32) {
                continue;
            }
            return n;
        }
    }
});
