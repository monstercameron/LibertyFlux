// original: 0x00d7e550 gated_check_chain
//! Gated check chain: validates two table entries chosen by the word
//! arguments, stages scaled fixed-point triples from them, then dispatches
//! on a nibble-derived case to one of several gate chains over the object
//! argument (flag bytes, table lookups, float gates, helper calls). Cases
//! with helper chains finish through a shared twelve-argument call whose
//! out word decides the answer; the remaining cases join a common tail that
//! runs two more helpers and a final dot-product gate. Returns true only
//! when every gate on the taken path passes.
//!
//! Proof notes: float gates observe only the sign of their comparison, and
//! every float operation keeps the original's exact operand order (pinned
//! with black_box) so all results including NaN payloads match bit for bit.
//! The original's jump table is a plain match on the same index range. The
//! callee at the encrypted address is intercepted by call site like every
//! other callee, so nothing about it needs the encrypted bytes.

use core::hint::black_box;
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, relocated};

/// Word scale applied to the x/y fixed-point components.
const WORD_SCALE: f32 = 0.125;
/// Word scale applied to the z fixed-point component.
const WORD_SCALE_Z: f32 = 0.015625;
/// Staged constant passed by value to the shared twelve-argument call.
const STAGED_FIVE_BITS: u32 = 0x40a00000;
/// Final tail-gate threshold, kept as the exact stored bit pattern.
const TAIL_LIMIT: f32 = f32::from_bits(0xBE4CCCCD);
/// Immediate object base passed to the status helper.
const STATUS_BASE: u32 = 0x1177a80;
/// Entry table base (indexed by the low word of a word argument).
const ENTRY_TABLE: u32 = 0x1178284;
/// Flag table base (indexed by the object's signed selector word).
const FLAG_TABLE: u32 = 0x1295cd8;
/// Stride between entries selected by the high word of a word argument.
const ENTRY_STRIDE: u32 = 32;
/// Vtable slot of the float-triple getter.
const SLOT_TRIPLE: u32 = 0x64;

/// Relocated entry-table base in the worker's mapping.
#[inline(always)]
fn tab_a() -> u32 {
    relocated(ENTRY_TABLE)
}

/// Relocated flag-table base in the worker's mapping.
#[inline(always)]
fn tab_b() -> u32 {
    relocated(FLAG_TABLE)
}

#[inline(always)]
fn sadd(a: f32, b: f32) -> f32 {
    black_box(black_box(a) + black_box(b))
}

#[inline(always)]
fn ssub(a: f32, b: f32) -> f32 {
    black_box(black_box(a) - black_box(b))
}

#[inline(always)]
fn smul(a: f32, b: f32) -> f32 {
    black_box(black_box(a) * black_box(b))
}

#[inline(always)]
unsafe fn rd_u(base: u32, off: u32) -> u32 {
    unsafe { (base.wrapping_add(off) as *const u32).read() }
}

#[inline(always)]
unsafe fn rd_b(base: u32, off: u32) -> u8 {
    unsafe { (base.wrapping_add(off) as *const u8).read() }
}

#[inline(always)]
unsafe fn rd_i16(base: u32, off: u32) -> i32 {
    unsafe { (base.wrapping_add(off) as *const i16).read() as i32 }
}

#[inline(always)]
unsafe fn rd_f(base: u32, off: u32) -> f32 {
    unsafe { (base.wrapping_add(off) as *const f32).read() }
}

/// Call a vtable slot exactly like the original: load the slot from the
/// object's table and call through it, so both sides land on the same
/// planted stub.
#[inline(always)]
unsafe fn vcall_0(obj: u32, slot: u32) -> u32 {
    unsafe {
        let table = (obj as *const u32).read();
        let target = (table.wrapping_add(slot) as *const u32).read();
        let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
        f(obj)
    }
}

#[inline(always)]
fn getf(frame: &[u32; 31], off: usize) -> f32 {
    f32::from_bits(frame[off / 4])
}

#[inline(always)]
fn setf(frame: &mut [u32; 31], off: usize, v: f32) {
    frame[off / 4] = v.to_bits();
}

/// Shared twelve-argument call block: stages the out word slot and the
/// scaled-x slot, makes the call, and answers whether the out word is zero.
#[inline(always)]
unsafe fn shared_call(frame: &mut [u32; 31]) -> u8 {
    unsafe {
        let p50 = frame.as_mut_ptr().add(0x50 / 4) as u32;
        let p2c = frame.as_mut_ptr().add(0x2c / 4) as u32;
        let _: u32 = callee_cdecl!(
            2,
            u32,
            p50,
            STAGED_FIVE_BITS,
            0,
            1,
            p2c,
            2,
            0,
            0,
            1,
            0,
            0,
            0
        );
        (frame[0x2c / 4] == 0) as u8
    }
}

/// Object flag-gate pair shared by the first two cases: the marker byte
/// must differ from 2, and a null link skips the linked-object checks.
#[inline(always)]
unsafe fn entry_object_gates(obj: u32) -> bool {
    unsafe {
        if rd_b(obj, 0x10b8) == 2 {
            return false;
        }
        let link = rd_u(obj, 0xf50);
        if link != 0 && rd_b(link, 0xa60) == 2 {
            return false;
        }
        true
    }
}

/// Linked-object kind gate shared by the first two cases: a null link
/// skips the check, otherwise the kind word must differ from 0x12.
#[inline(always)]
unsafe fn link_kind_gate(obj: u32) -> bool {
    unsafe {
        let link = rd_u(obj, 0xf50);
        if link == 0 {
            return true;
        }
        rd_u(rd_u(link, 0x21c), 0x12c) != 0x12
    }
}

unsafe fn case0(
    frame: &mut [u32; 31],
    obj: u32,
    a3: u32,
    flag: u8,
    status_imm: u32,
) -> u8 {
    unsafe {
        if !entry_object_gates(obj) {
            return 0;
        }
        let sel = rd_i16(obj, 0x2e) as u32;
        let flag_slot = rd_u(tab_b().wrapping_add(sel.wrapping_mul(4)), 0);
        let flag_word = rd_u(flag_slot, 0x94);
        if ((flag_word >> 1) & 1) != 0 {
            return 0;
        }
        if flag == 0 {
            let m = rd_u(obj, 0x20).wrapping_add(0x30);
            let x1 = ssub(getf(frame, 0x50), rd_f(m, 0));
            let x0 = ssub(getf(frame, 0x54), rd_f(m, 4));
            let mut x2 = ssub(getf(frame, 0x50), getf(frame, 0x30));
            let mut x3 = ssub(getf(frame, 0x54), getf(frame, 0x4c));
            x2 = smul(x2, x0);
            x3 = smul(x3, x1);
            x2 = ssub(x2, x3);
            if 0.0 > x2 {
                return 0;
            }
        }
        if rd_b(obj, 0xf1f) & 0x20 != 0 {
            return 0;
        }
        let ok: u8 = callee_cdecl!(0, u8, sel, 0x18);
        if ok != 0 {
            return 0;
        }
        if !link_kind_gate(obj) {
            return 0;
        }
        let ok: u8 = callee_thiscall!(1, u8, status_imm, a3);
        if ok != 0 {
            return 0;
        }
        shared_call(frame)
    }
}

unsafe fn case1(
    frame: &mut [u32; 31],
    obj: u32,
    a3: u32,
    flag: u8,
    status_imm: u32,
) -> u8 {
    unsafe {
        if !entry_object_gates(obj) {
            return 0;
        }
        if flag == 0 {
            let m = rd_u(obj, 0x20);
            let f10 = ssub(getf(frame, 0x50), rd_f(m, 0x30));
            let f28 = ssub(getf(frame, 0x54), rd_f(m, 0x34));
            let f30 = ssub(getf(frame, 0x58), rd_f(m, 0x38));
            setf(frame, 0x10, f10);
            setf(frame, 0x28, f28);
            setf(frame, 0x30, f30);
            let p: u32 = callee_thiscall!(3, u32, obj, frame.as_mut_ptr().add(0x70 / 4) as u32);
            let x0 = smul(rd_f(p, 0), f10);
            let mut x1 = smul(rd_f(p, 4), f28);
            x1 = sadd(x1, x0);
            let x0b = smul(rd_f(p, 8), f30);
            x1 = sadd(x1, x0b);
            if 0.0 > x1 {
                return 0;
            }
        }
        if rd_b(obj, 0xf1f) & 0x20 != 0 {
            return 0;
        }
        let sel = rd_i16(obj, 0x2e) as u32;
        let ok: u8 = callee_cdecl!(0, u8, sel, 0x18);
        if ok != 0 {
            return 0;
        }
        let flag_slot = rd_u(tab_b().wrapping_add(sel.wrapping_mul(4)), 0);
        let flag_word = rd_u(flag_slot, 0x94);
        if ((flag_word >> 1) & 1) != 0 {
            return 0;
        }
        if !link_kind_gate(obj) {
            return 0;
        }
        let ok: u8 = callee_thiscall!(1, u8, status_imm, a3);
        if ok == 0 {
            return shared_call(frame);
        }
        0
    }
}

/// Common tail: entry-flag implications, selector gates, the table-driven
/// helper, two normalising helpers, and the final dot-product gate.
unsafe fn tail(
    frame: &mut [u32; 31],
    obj: u32,
    a1: u32,
    flag: u8,
    cont1: u8,
    cont2: u8,
    src: u32,
    dst: u32,
) -> u8 {
    unsafe {
        let exit = flag == 0;
        if rd_b(obj, 0xe6e) == 1
            && rd_b(src, 0x1f) & 0x10 != 0
            && rd_b(dst, 0x1f) & 0x10 == 0
        {
            return 0;
        }
        if rd_b(src, 0x1b) & 0xe0 > rd_b(dst, 0x1b) & 0xe0 {
            return 0;
        }
        if rd_b(src, 0x1e) & 0x80 != 0 && rd_b(dst, 0x1e) & 0x80 == 0 {
            return 0;
        }
        if rd_b(src, 0x1f) & 0x04 != 0 && rd_b(dst, 0x1f) & 0x04 == 0 {
            return 0;
        }
        if cont1 == 0 || cont2 == 0 {
            return exit as u8;
        }
        if a1 & 0xffff == 0xffff {
            return exit as u8;
        }
        let base = rd_u(tab_a().wrapping_add((a1 & 0xffff).wrapping_mul(4)), 0);
        if base == 0 {
            return exit as u8;
        }
        let ecx5 = base.wrapping_add((a1 >> 16).wrapping_mul(ENTRY_STRIDE));
        let _: u32 = callee_thiscall!(5, u32, ecx5, frame.as_mut_ptr().add(0x70 / 4) as u32);
        let x0 = ssub(getf(frame, 0x10), getf(frame, 0x30));
        let x1 = ssub(getf(frame, 0x30), getf(frame, 0x70));
        let x2 = getf(frame, 0x4c);
        frame[0x68 / 4] = 0;
        frame[0x58 / 4] = 0;
        setf(frame, 0x50, x0);
        let xa = ssub(getf(frame, 0x28), x2);
        let xb = ssub(x2, getf(frame, 0x74));
        setf(frame, 0x60, x1);
        setf(frame, 0x54, xa);
        setf(frame, 0x64, xb);
        callee_thiscall!(6, u32, frame.as_mut_ptr().add(0x50 / 4) as u32);
        callee_thiscall!(7, u32, frame.as_mut_ptr().add(0x60 / 4) as u32);
        let d0 = smul(getf(frame, 0x60), getf(frame, 0x50));
        let mut d1 = smul(getf(frame, 0x64), getf(frame, 0x54));
        d1 = sadd(d1, d0);
        let d0b = smul(getf(frame, 0x58), getf(frame, 0x68));
        d1 = sadd(d1, d0b);
        if TAIL_LIMIT > d1 {
            return 0;
        }
        exit as u8
    }
}

unsafe fn body(
    frame: &mut [u32; 31],
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    flag: u8,
    cont1: u8,
    cont2: u8,
    status_imm: u32,
) -> u8 {
    unsafe {
        let src_base = rd_u(tab_a().wrapping_add((a3 & 0xffff).wrapping_mul(4)), 0);
        if src_base == 0 {
            return 0;
        }
        if a1 == a3 {
            return 0;
        }
        let dst_base = rd_u(tab_a().wrapping_add((a2 & 0xffff).wrapping_mul(4)), 0);
        let src = src_base.wrapping_add((a3 >> 16).wrapping_mul(ENTRY_STRIDE));
        let dst = dst_base.wrapping_add((a2 >> 16).wrapping_mul(ENTRY_STRIDE));
        if (rd_b(src, 0x1f) ^ rd_b(dst, 0x1f)) & 2 != 0 {
            return 0;
        }
        let sx = smul(rd_i16(src, 0x14) as f32, WORD_SCALE);
        let sy = smul(rd_i16(src, 0x16) as f32, WORD_SCALE);
        let sz = smul(rd_i16(src, 0x18) as f32, WORD_SCALE_Z);
        let dx = smul(rd_i16(dst, 0x14) as f32, WORD_SCALE);
        let dy = smul(rd_i16(dst, 0x16) as f32, WORD_SCALE);
        setf(frame, 0x50, sx);
        setf(frame, 0x10, sx);
        setf(frame, 0x54, sy);
        setf(frame, 0x28, sy);
        setf(frame, 0x58, sz);
        setf(frame, 0x30, dx);
        setf(frame, 0x4c, dy);
        let case = (rd_b(src, 0x1c) >> 4).wrapping_sub(1);
        match case {
            0 => case0(frame, a0, a3, flag, status_imm),
            1 => case1(frame, a0, a3, flag, status_imm),
            2 | 5 => {
                if flag != 0 {
                    return 0;
                }
                let sel = rd_i16(a0, 0x2e) as u32;
                let flag_slot = rd_u(tab_b().wrapping_add(sel.wrapping_mul(4)), 0);
        let flag_word = rd_u(flag_slot, 0x94);
                if ((flag_word >> 10) & 1) == 0 {
                    return 0;
                }
                tail(frame, a0, a1, flag, cont1, cont2, src, dst)
            }
            3 | 4 => {
                if flag != 0 {
                    return 0;
                }
                let p = vcall_0(a0, SLOT_TRIPLE);
                if rd_f(p, 8) > 1.5 {
                    return 0;
                }
                let p = vcall_0(a0, SLOT_TRIPLE);
                if rd_f(p, 0) > 2.0 {
                    return 0;
                }
                let p = vcall_0(a0, SLOT_TRIPLE);
                if rd_f(p, 4) > 4.0 {
                    return 0;
                }
                frame[0x28 / 4] = frame[0x54 / 4];
                frame[0x10 / 4] = frame[0x50 / 4];
                tail(frame, a0, a1, flag, cont1, cont2, src, dst)
            }
            _ => tail(frame, a0, a1, flag, cont1, cont2, src, dst),
        }
    }
}

/// Gated check chain (see module docs).
export!(cdecl, rw_00d7e550(a0: u32, a1: u32, a2: u32, a3: u32, f18: u32, f1c: u32, f20: u32) -> u8 {
    unsafe {
        let mut frame = [0u32; 31];
        body(
            &mut frame,
            a0,
            a1,
            a2,
            a3,
            f18 as u8,
            f1c as u8,
            f20 as u8,
            relocated(STATUS_BASE),
        )
    }
});

/// Deliberately wrong version: the final tail gate is inverted, so trials
/// that reach it with a passing dot product return false and trials with a
/// failing one continue to the flag-based exit.
export!(cdecl, rw_00d7e550_m1(a0: u32, a1: u32, a2: u32, a3: u32, f18: u32, f1c: u32, f20: u32) -> u8 {
    unsafe {
        // Same behaviour as the rewrite except the final gate below; the
        // shared helpers are reused so only that gate differs.
        let mut frame = [0u32; 31];
        let flag = f18 as u8;
        let src_base = rd_u(tab_a().wrapping_add((a3 & 0xffff).wrapping_mul(4)), 0);
        if src_base == 0 {
            return 0;
        }
        if a1 == a3 {
            return 0;
        }
        let dst_base = rd_u(tab_a().wrapping_add((a2 & 0xffff).wrapping_mul(4)), 0);
        let src = src_base.wrapping_add((a3 >> 16).wrapping_mul(ENTRY_STRIDE));
        let dst = dst_base.wrapping_add((a2 >> 16).wrapping_mul(ENTRY_STRIDE));
        if (rd_b(src, 0x1f) ^ rd_b(dst, 0x1f)) & 2 != 0 {
            return 0;
        }
        let sx = smul(rd_i16(src, 0x14) as f32, WORD_SCALE);
        let sy = smul(rd_i16(src, 0x16) as f32, WORD_SCALE);
        let sz = smul(rd_i16(src, 0x18) as f32, WORD_SCALE_Z);
        let dx = smul(rd_i16(dst, 0x14) as f32, WORD_SCALE);
        let dy = smul(rd_i16(dst, 0x16) as f32, WORD_SCALE);
        setf(&mut frame, 0x50, sx);
        setf(&mut frame, 0x10, sx);
        setf(&mut frame, 0x54, sy);
        setf(&mut frame, 0x28, sy);
        setf(&mut frame, 0x58, sz);
        setf(&mut frame, 0x30, dx);
        setf(&mut frame, 0x4c, dy);
        let status_imm = relocated(STATUS_BASE);
        let case = (rd_b(src, 0x1c) >> 4).wrapping_sub(1);
        match case {
            0 => case0(&mut frame, a0, a3, flag, status_imm),
            1 => case1(&mut frame, a0, a3, flag, status_imm),
            2 | 5 => {
                if flag != 0 {
                    return 0;
                }
                let sel = rd_i16(a0, 0x2e) as u32;
                let flag_slot = rd_u(tab_b().wrapping_add(sel.wrapping_mul(4)), 0);
        let flag_word = rd_u(flag_slot, 0x94);
                if ((flag_word >> 10) & 1) == 0 {
                    return 0;
                }
                tail_mut(&mut frame, a0, a1, flag, f1c as u8, f20 as u8, src, dst)
            }
            3 | 4 => {
                if flag != 0 {
                    return 0;
                }
                let p = vcall_0(a0, SLOT_TRIPLE);
                if rd_f(p, 8) > 1.5 {
                    return 0;
                }
                let p = vcall_0(a0, SLOT_TRIPLE);
                if rd_f(p, 0) > 2.0 {
                    return 0;
                }
                let p = vcall_0(a0, SLOT_TRIPLE);
                if rd_f(p, 4) > 4.0 {
                    return 0;
                }
                frame[0x28 / 4] = frame[0x54 / 4];
                frame[0x10 / 4] = frame[0x50 / 4];
                tail_mut(&mut frame, a0, a1, flag, f1c as u8, f20 as u8, src, dst)
            }
            _ => tail_mut(&mut frame, a0, a1, flag, f1c as u8, f20 as u8, src, dst),
        }
    }
});

/// Mutant tail: identical to the tail except the final gate is inverted.
unsafe fn tail_mut(
    frame: &mut [u32; 31],
    obj: u32,
    a1: u32,
    flag: u8,
    cont1: u8,
    cont2: u8,
    src: u32,
    dst: u32,
) -> u8 {
    unsafe {
        let exit = flag == 0;
        if rd_b(obj, 0xe6e) == 1
            && rd_b(src, 0x1f) & 0x10 != 0
            && rd_b(dst, 0x1f) & 0x10 == 0
        {
            return 0;
        }
        if rd_b(src, 0x1b) & 0xe0 > rd_b(dst, 0x1b) & 0xe0 {
            return 0;
        }
        if rd_b(src, 0x1e) & 0x80 != 0 && rd_b(dst, 0x1e) & 0x80 == 0 {
            return 0;
        }
        if rd_b(src, 0x1f) & 0x04 != 0 && rd_b(dst, 0x1f) & 0x04 == 0 {
            return 0;
        }
        if cont1 == 0 || cont2 == 0 {
            return exit as u8;
        }
        if a1 & 0xffff == 0xffff {
            return exit as u8;
        }
        let base = rd_u(tab_a().wrapping_add((a1 & 0xffff).wrapping_mul(4)), 0);
        if base == 0 {
            return exit as u8;
        }
        let ecx5 = base.wrapping_add((a1 >> 16).wrapping_mul(ENTRY_STRIDE));
        let _: u32 = callee_thiscall!(5, u32, ecx5, frame.as_mut_ptr().add(0x70 / 4) as u32);
        let x0 = ssub(getf(frame, 0x10), getf(frame, 0x30));
        let x1 = ssub(getf(frame, 0x30), getf(frame, 0x70));
        let x2 = getf(frame, 0x4c);
        frame[0x68 / 4] = 0;
        frame[0x58 / 4] = 0;
        setf(frame, 0x50, x0);
        let xa = ssub(getf(frame, 0x28), x2);
        let xb = ssub(x2, getf(frame, 0x74));
        setf(frame, 0x60, x1);
        setf(frame, 0x54, xa);
        setf(frame, 0x64, xb);
        callee_thiscall!(6, u32, frame.as_mut_ptr().add(0x50 / 4) as u32);
        callee_thiscall!(7, u32, frame.as_mut_ptr().add(0x60 / 4) as u32);
        let d0 = smul(getf(frame, 0x60), getf(frame, 0x50));
        let mut d1 = smul(getf(frame, 0x64), getf(frame, 0x54));
        d1 = sadd(d1, d0);
        let d0b = smul(getf(frame, 0x58), getf(frame, 0x68));
        d1 = sadd(d1, d0b);
        // MUTANT: inverted final gate.
        if d1 > TAIL_LIMIT {
            return 0;
        }
        exit as u8
    }
}
