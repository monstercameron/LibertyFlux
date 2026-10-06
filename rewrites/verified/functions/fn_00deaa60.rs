// original: 0x00deaa60 RP_NOMEM
//! Rebuilds the pause-menu clip viewer, or the montage editor when the mode
//! bytes select it.
//!
//! `this` points to the editor object and `arg0` selects the update path.
//! The entry gate (slot +0x140) returns at once when nonzero. Otherwise the
//! two mode bytes decide: both at their rest values continues below, any
//! other combination builds the alternate editor object through the
//! allocator, two constructors, a resolver and eight virtual steps. On the
//! main path a describe call runs first; a null `arg0` then allocates and
//! constructs the viewer object, resolves four query triples through it and
//! runs its two-step tail, while a non-null `arg0` forwards to the sibling
//! rebuild. Every path joins a shared three-call tail whose last result is
//! returned.
//!
//! Proof: one contract covers every path (entry gate plus early exit, main
//! build with all four query triples, forward, globals-differ build with all
//! eight virtual steps, shared tail). All 28 callees fired; both null-alloc
//! fault paths taken with fault parity. Narrowings: three register/argument
//! skips (resolver `ecx` is constructor-stub residue; two frame-pointer
//! `ecx` values and one frame-pointer argument are compared through
//! call-time snapshots of zeroes instead).

#![allow(unsafe_code)]
#![allow(clippy::pedantic)]

use lf_checker_rt::export;

const DE_GATE_SLOT: u32 = 0x140;
const DE_G1: u32 = 0x010376E9;
const DE_G2: u32 = 0x011F7076;
const DE_SEL_A: u32 = 0x00EFF1FF;
const DE_SEL_B: u32 = 0x00EFF310;
const DE_SEL_D: u32 = 0x00EFF1EC;
const DE_SEL_D2: u32 = 0x00EFF200;
const DE_T1_SEL: u32 = 0x00EFF31C;
const DE_T2_SEL: u32 = 0x00EFF32C;
const DE_T3_SEL: u32 = 0x00EFF340;
const DE_T4_SEL: u32 = 0x00EFF354;
const DE_F1: u32 = 0x42000000;
const DE_F2A: u32 = 0x43480000;
const DE_F2B: u32 = 0x41F00000;
const DE_T1_A0: u32 = 4;

/// `t1a0` is the first query triple's leading tag (4 in the proven export;
/// the honest mutant this proof ran against passes 5).
export!(thiscall, rw_deaa60(this: u32, arg0: u32) -> u32 {
    unsafe { run_deaa60(this, arg0, DE_T1_A0) }
});

unsafe fn run_deaa60(this: u32, arg0: u32, t1a0: u32) -> u32 {
    unsafe {
        let vt = (this as *const u32).read_unaligned();
        let gate: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((vt.wrapping_add(DE_GATE_SLOT)) as *const u32).read_unaligned());
        let g = gate(this);
        if (g & 0xFF) != 0 {
            return g;
        }
        if lf_checker_rt::global::<u8>(DE_G1).read_unaligned() == 0 {
            de_path_d(this);
        } else if lf_checker_rt::global::<u8>(DE_G2).read_unaligned() != 0 {
            de_path_d(this);
        } else if arg0 == 0 {
            lf_checker_rt::callee_thiscall!(2, u32, this.wrapping_add(0x1f8), lf_checker_rt::relocated(DE_SEL_A));
            (this.wrapping_add(0x209) as *mut u8).write_unaligned(1);
            de_path_b(this, t1a0);
        } else {
            lf_checker_rt::callee_thiscall!(2, u32, this.wrapping_add(0x1f8), lf_checker_rt::relocated(DE_SEL_A));
            (this.wrapping_add(0x209) as *mut u8).write_unaligned(1);
            lf_checker_rt::callee_thiscall!(13, u32, this, arg0);
        }
        de_tail(this)
    }
}

#[inline(never)]
unsafe fn de_path_b(this: u32, t1a0: u32) {
    unsafe {
        let mem = lf_checker_rt::callee_cdecl!(3, u32, 0x1f8);
        let built = if mem == 0 {
            0
        } else {
            de_vcall0(this, 0x48);
            de_vcall0(this, 0x48);
            let k = lf_checker_rt::callee_thiscall!(5, u32, this, lf_checker_rt::relocated(DE_SEL_B));
            lf_checker_rt::callee_thiscall!(6, u32, mem, k)
        };
        (this.wrapping_add(0x1e8) as *mut u32).write_unaligned(built);
        lf_checker_rt::callee_thiscall!(7, u32, built);
        let obj = (this.wrapping_add(0x1e8) as *const u32).read_unaligned();
        let obv = (obj as *const u32).read_unaligned();
        let fq: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(((obv.wrapping_add(0x100)) as *const u32).read_unaligned());
        let zero = [0u32; 5];
        let zp = zero.as_ptr() as u32;
        de_triple(obj, fq, zp, t1a0, DE_T1_SEL, 0x10);
        de_triple(obj, fq, zp, 0x10, DE_T2_SEL, 4);
        de_triple(obj, fq, zp, 2, DE_T3_SEL, 2);
        de_triple(obj, fq, zp, 8, DE_T4_SEL, 8);
        de_vcall1(obj, 0x24, 1);
        de_vcall1(obj, 0x18, 1);
    }
}

#[inline(never)]
unsafe fn de_triple(
    obj: u32,
    fq: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32,
    zp: u32,
    a0: u32,
    sel: u32,
    a2: u32,
) {
    unsafe {
        let d = lf_checker_rt::callee_thiscall!(8, u32, zp, 0, 0);
        let w0 = (d as *const u32).read_unaligned();
        let w1 = (d.wrapping_add(4) as *const u32).read_unaligned();
        let w2 = (d.wrapping_add(8) as *const u32).read_unaligned();
        let w3 = (d.wrapping_add(12) as *const u32).read_unaligned();
        let w4 = (d.wrapping_add(16) as *const u32).read_unaligned();
        let w5 = (d.wrapping_add(20) as *const u32).read_unaligned();
        fq(obj, a0, lf_checker_rt::relocated(sel), a2, w0, w1, w2, w3, w4, w5);
        lf_checker_rt::callee_thiscall!(10, u32, zp);
    }
}

#[inline(never)]
unsafe fn de_path_d(this: u32) {
    unsafe {
        let mem = lf_checker_rt::callee_cdecl!(17, u32, 0x610);
        let ebx = if mem == 0 {
            0
        } else {
            de_vcall0(this, 0x48);
            de_vcall0(this, 0x48);
            let k = lf_checker_rt::callee_thiscall!(5, u32, this, lf_checker_rt::relocated(DE_SEL_D));
            lf_checker_rt::callee_thiscall!(18, u32, mem, k)
        };
        let obv = (ebx as *const u32).read_unaligned();
        let zero1 = [0u32; 1];
        let r = lf_checker_rt::callee_thiscall!(19, u32, ebx, zero1.as_ptr() as u32, 0x41);
        de_vcall2(ebx, 0x1cc, DE_F1, r);
        de_vcall2(ebx, 0x80, DE_F2A, DE_F2B);
        de_vcall2(ebx, 0x1e0, lf_checker_rt::relocated(DE_SEL_D2), 0);
        let _ = obv;
        let m1 = de_vcall0(this, 0x4c);
        de_vcall1(ebx, 0x170, m1);
        let m2 = de_vcall0(this, 0x4c);
        de_vcall1(ebx, 0x17c, m2);
        de_vcall1(ebx, 0x1fc, 0);
        de_vcall1(ebx, 0x118, 1);
        de_vcall1(ebx, 0x200, 1);
    }
}

#[inline(never)]
unsafe fn de_tail(this: u32) -> u32 {
    unsafe {
        de_vcall1(this, 0x24, 1);
        de_vcall1(this, 0x18, 1);
        de_vcall1(this, 0x13c, 1)
    }
}

#[inline(always)]
unsafe fn de_vcall0(obj: u32, slot: u32) -> u32 {
    unsafe {
        let vt = (obj as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((vt.wrapping_add(slot)) as *const u32).read_unaligned());
        f(obj)
    }
}

#[inline(always)]
unsafe fn de_vcall1(obj: u32, slot: u32, a0: u32) -> u32 {
    unsafe {
        let vt = (obj as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(((vt.wrapping_add(slot)) as *const u32).read_unaligned());
        f(obj, a0)
    }
}

#[inline(always)]
unsafe fn de_vcall2(obj: u32, slot: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        let vt = (obj as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(((vt.wrapping_add(slot)) as *const u32).read_unaligned());
        f(obj, a0, a1)
    }
}
