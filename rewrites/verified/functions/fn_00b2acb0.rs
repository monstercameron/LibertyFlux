// original: 0x00B2ACB0 anchored_vector_blend
//! Anchored vector blend: two-object distance gates plus a vector-blend tail
//! (cdecl/3 -> u32).
//!
//! Behaviour. The function takes three object pointers. It resolves the
//! first one through two helper calls into a working object and tests a
//! masked type word: one value selects an early distance check, any other
//! value selects the main path.
//!
//! The early path loads a secondary object from the second argument, runs
//! the same resolver over the first argument, and measures the distance
//! between a three-float point chosen by a null test (either an offset into
//! the resolver answer or into the object it points at) and an anchor triple
//! in the secondary object. When a global limit does not exceed that
//! distance it stores one state byte and returns the chosen point; otherwise
//! it stores another state byte, clamps a second state byte down to thirteen,
//! and returns the clamped byte.
//!
//! The main path fetches a vector through the working object's getter slot,
//! measures its length, and runs three sampler calls that each fill two
//! frame words (the calls take a frame pointer as object and a heap or frame
//! pointer plus zero on the stack). From the six sampled words it derives a
//! scale factor (a global divided by a root, or zero when the root input is
//! an ordered zero), two differences, and two scaled products, then runs two
//! mixer calls that each fill three frame words. Two dot products of mixer
//! outputs, a ratio of one dot over the earlier root, and a chain of three
//! global comparisons with a state-byte check select between two intermediate
//! exits (which store a state byte and return the second mixer's answer) and
//! the tail.
//!
//! The tail re-fetches through the same getter slot, dots the answer against
//! a triple from the working object's child, clamps that dot through a
//! state-byte-selected low/high/none scheme against three more globals,
//! blends the child triple scaled by the clamped dot over a base triple,
//! publishes the blend into the first object and through two finish calls,
//! and finally gates on two more distance-to-global comparisons: either can
//! exit early (returning a heap pointer or the last getter answer), otherwise
//! a byte and a relocated global word are stored into the third object and
//! the word is returned.
//!
//! Proof notes. The getter slot calls are intercepted by planting stub
//! addresses in fabricated heap tables; the first two share one slot and
//! are told apart by a per-call answer sequence, while the third runs
//! against a second object's table. Sampler and mixer out-words are declared
//! callee writes with edge-valued rows, so both sides observe identical
//! out-words; the rewrite passes its own frame slots. Frame-pointer call
//! arguments are skipped with call-time snapshots of the pointed-to words,
//! except the last getter call's argument, whose words were already
//! snapshotted at the previous getter call with no writer between. The
//! child-null case faults identically on both sides after the getter call
//! (fault parity, kept at a low rate). Every comparison follows the original
//! exactly: below-or-equal branches use the negated ordered-above test so
//! unordered counts as taken, above branches use the ordered test, and the
//! zero-gated scale uses an ordered-zero test. All float steps are pinned
//! with an opaque compiler barrier.

use core::hint::black_box;
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

/// File VAs (image base 0x400000) of the globals this function reads.
const G_EARLY_LIM: u32 = 0x00FE_8AE0;
const G_SCALE_K: u32 = 0x00FE_88E8;
const G_SQRT_LIM: u32 = 0x00FE_8A24;
const G_ROOT_LIM: u32 = 0x00FE_8B38;
const G_RATIO_LIM: u32 = 0x00E7_7F98;
const G_DIV_LIM: u32 = 0x00FE_8878;
const G_CLAMP_C0: u32 = 0x00FE_8DC0;
const G_CLAMP_LO: u32 = 0x00FE_8AD8;
const G_CLAMP_HI: u32 = 0x00FE_8DCC;
const G_TAIL_LIM: u32 = 0x00FE_8AF0;
const G_VEC_LIM: u32 = 0x00FE_8B1C;
const G_BASE_WORD: u32 = 0x0117_35B4;

/// Getter slot all three virtual calls go through (offset into the table).
const VT_GET: usize = 0xEC / 4;
/// Masked type value selecting the early path.
const TYPE_EARLY: u32 = 0xC0;
const TYPE_MASK: u32 = 0x3C0;

/// Single-precision steps pinned against operand reorder and contraction.
#[inline(always)]
fn fadd(a: f32, b: f32) -> f32 {
    black_box(black_box(a) + black_box(b))
}
#[inline(always)]
fn fsub(a: f32, b: f32) -> f32 {
    black_box(black_box(a) - black_box(b))
}
#[inline(always)]
fn fmul(a: f32, b: f32) -> f32 {
    black_box(black_box(a) * black_box(b))
}
#[inline(always)]
fn fdiv(a: f32, b: f32) -> f32 {
    black_box(black_box(a) / black_box(b))
}
#[inline(always)]
fn fsqrt(a: f32) -> f32 {
    black_box(black_box(a).sqrt())
}

/// The object's getter call: load the table, load the slot, call through it
/// exactly like the original, so both sides land on the planted stub.
#[inline(always)]
fn vcall_get(obj: u32, arg0: u32) -> u32 {
    let table = unsafe { (obj as *const u32).read() } as *const u32;
    let target = unsafe { table.add(VT_GET).read() } as usize;
    let f: extern "thiscall" fn(u32, u32) -> u32 =
        unsafe { core::mem::transmute(target) };
    f(obj, arg0)
}

#[inline(always)]
fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read() }
}
#[inline(always)]
fn rdf(addr: u32) -> f32 {
    f32::from_bits(rd32(addr))
}

fn body<const MUT: bool>(o0: u32, o1: u32, o2: u32) -> u32 {
    let a1 = callee_thiscall!(1, u32, o0);
    let esi_a = callee_cdecl!(2, u32, a1);
    let type_word = rd32(esi_a.wrapping_add(0x28));

    if (type_word & TYPE_MASK) == TYPE_EARLY {
        let e1 = rd32(o1.wrapping_add(0x20));
        let p = callee_thiscall!(3, u32, o0);
        let q = rd32(p.wrapping_add(0x20));
        let leaf = if q == 0 {
            p.wrapping_add(0x10)
        } else {
            q.wrapping_add(0x30)
        };
        let f0 = rdf(leaf);
        let f1 = rdf(leaf.wrapping_add(4));
        let f2 = rdf(leaf.wrapping_add(8));
        let d1 = fsub(f1, rdf(e1.wrapping_add(0x34)));
        let d0 = fsub(f0, rdf(e1.wrapping_add(0x30)));
        let d2 = fsub(f2, rdf(e1.wrapping_add(0x38)));
        let dist2 = fadd(fadd(fmul(d1, d1), fmul(d0, d0)), fmul(d2, d2));
        let lim = f32::from_bits(unsafe { global::<u32>(G_EARLY_LIM).read() });
        let dist = fsqrt(dist2);
        if !(lim > dist) {
            let b = unsafe { ((o0 + 0x27) as *const u8).read() };
            let mark: u8 = if MUT { 0x1b } else { 0x1a };
            unsafe { ((o0 + 0x26) as *mut u8).write(mark) };
            let capped = if b < 0x0d { b as u32 } else { 0x0d };
            unsafe { ((o0 + 0x27) as *mut u8).write(capped as u8) };
            return capped;
        }
        unsafe { ((o0 + 0x26) as *mut u8).write(5) };
        return leaf;
    }

    let zeros = [0u32; 3];
    let v1 = vcall_get(esi_a, zeros.as_ptr() as u32);
    let v10 = rdf(v1);
    let v11 = rdf(v1.wrapping_add(4));
    let v12 = rdf(v1.wrapping_add(8));
    let dv1 = fadd(
        fadd(fmul(v10, v10), fmul(v11, v11)),
        fmul(v12, v12),
    );
    let sqrt_v1 = fsqrt(dv1);

    let a1x = rd32(o1.wrapping_add(0x20));
    let mut s1slot = [0u32; 2];
    let _ = callee_thiscall!(
        5, u32,
        s1slot.as_mut_ptr() as u32,
        a1x.wrapping_add(0x30),
        0
    );
    let e20 = rd32(esi_a.wrapping_add(0x20));
    let s2arg = if e20 == 0 {
        esi_a.wrapping_add(0x10)
    } else {
        e20.wrapping_add(0x30)
    };
    let mut s2slot = [0u32; 2];
    let _ = callee_thiscall!(6, u32, s2slot.as_mut_ptr() as u32, s2arg, 0);
    let e20b = rd32(esi_a.wrapping_add(0x20));
    let mut s3slot = [0u32; 2];
    let _ = callee_thiscall!(
        7, u32,
        s3slot.as_mut_ptr() as u32,
        e20b.wrapping_add(0x10),
        0
    );
    let s1 = [f32::from_bits(s1slot[0]), f32::from_bits(s1slot[1])];
    let s2 = [f32::from_bits(s2slot[0]), f32::from_bits(s2slot[1])];
    let s3 = [f32::from_bits(s3slot[0]), f32::from_bits(s3slot[1])];

    let ss = fadd(fmul(s3[0], s3[0]), fmul(s3[1], s3[1]));
    let kk = f32::from_bits(unsafe { global::<u32>(G_SCALE_K).read() });
    let x2 = if ss != 0.0 { fdiv(kk, fsqrt(ss)) } else { 0.0 };
    let d_a = fsub(s1[1], s2[1]);
    let d_b = fsub(s1[0], s2[0]);
    let p3 = fmul(s3[0], x2);
    let q_a = fmul(d_a, d_a);
    let q_b = fmul(d_b, d_b);
    let p4 = fmul(s3[1], x2);
    let dd = fadd(q_a, q_b);
    let st_rq = fsqrt(dd);

    let mut t1slot = [0u32; 3];
    let pair1 = [p3.to_bits(), p4.to_bits()];
    let _ = callee_thiscall!(
        8, u32,
        t1slot.as_mut_ptr() as u32,
        pair1.as_ptr() as u32,
        1
    );
    let mut t2slot = [0u32; 3];
    let pair2 = [d_b.to_bits(), d_a.to_bits()];
    let a9 = callee_thiscall!(
        9, u32,
        t2slot.as_mut_ptr() as u32,
        pair2.as_ptr() as u32,
        1
    );
    let t1 = [
        f32::from_bits(t1slot[0]),
        f32::from_bits(t1slot[1]),
        f32::from_bits(t1slot[2]),
    ];
    let t2 = [
        f32::from_bits(t2slot[0]),
        f32::from_bits(t2slot[1]),
        f32::from_bits(t2slot[2]),
    ];
    let m0 = fmul(t2[1], t1[1]);
    let m1 = fmul(t2[0], t1[0]);
    let s_a = fadd(m1, m0);
    let m2 = fmul(t2[2], t1[2]);
    let res1 = fadd(s_a, m2);

    let g_a24 = f32::from_bits(unsafe { global::<u32>(G_SQRT_LIM).read() });
    let div = fdiv(res1, st_rq);
    if !(g_a24 > sqrt_v1) {
        return tail::<MUT>(o0, o1, o2, esi_a, a9, &t1slot);
    }
    let g_b38 = f32::from_bits(unsafe { global::<u32>(G_ROOT_LIM).read() });
    if !(g_b38 > st_rq) {
        return tail::<MUT>(o0, o1, o2, esi_a, a9, &t1slot);
    }
    let g_7798 = f32::from_bits(unsafe { global::<u32>(G_RATIO_LIM).read() });
    if !(g_7798 > div) {
        return zone03::<MUT>(o0, o1, o2, esi_a, a9, div, &t1slot);
    }
    let b26 = unsafe { ((o0 + 0x26) as *const u8).read() };
    if b26 != 0x22 {
        return zone03::<MUT>(o0, o1, o2, esi_a, a9, div, &t1slot);
    }
    unsafe { ((o0 + 0x26) as *mut u8).write(0x1c) };
    a9
}

fn zone03<const MUT: bool>(
    o0: u32, o1: u32, o2: u32, esi_a: u32, a9: u32, div: f32, t1slot: &[u32; 3],
) -> u32 {
    let g_8878 = f32::from_bits(unsafe { global::<u32>(G_DIV_LIM).read() });
    if !(div > g_8878) {
        return tail::<MUT>(o0, o1, o2, esi_a, a9, t1slot);
    }
    let b26 = unsafe { ((o0 + 0x26) as *const u8).read() };
    if b26 != 0x22 {
        unsafe { ((o0 + 0x26) as *mut u8).write(0x1c) };
        return a9;
    }
    tail::<MUT>(o0, o1, o2, esi_a, a9, t1slot)
}

fn tail<const MUT: bool>(
    o0: u32, o1: u32, o2: u32, esi_a: u32, _a9: u32, t1slot: &[u32; 3],
) -> u32 {
    let _ = MUT;
    let esi_b = rd32(esi_a.wrapping_add(0x20));
    let v2 = vcall_get(esi_a, t1slot.as_ptr() as u32);
    let w1 = rdf(v2.wrapping_add(4));
    let u0 = rdf(esi_b.wrapping_add(0x10));
    let w0 = rdf(v2);
    let u1 = rdf(esi_b.wrapping_add(0x14));
    let w2 = rdf(v2.wrapping_add(8));
    let u2 = rdf(esi_b.wrapping_add(0x18));
    let dot = fadd(fadd(fmul(w1, u1), fmul(u0, w0)), fmul(w2, u2));

    let b = unsafe { ((o0 + 0x26) as *const u8).read() };
    let cc = f32::from_bits(unsafe { global::<u32>(G_CLAMP_HI).read() });
    let d8 = f32::from_bits(unsafe { global::<u32>(G_CLAMP_LO).read() });
    let clamped = if b == 0x22 {
        if dot > cc { cc } else { dot }
    } else if b == 0x21 {
        if dot > d8 { dot } else { d8 }
    } else {
        let c0 = f32::from_bits(unsafe { global::<u32>(G_CLAMP_C0).read() });
        if c0 > dot {
            if dot > cc { cc } else { dot }
        } else if dot > d8 {
            dot
        } else {
            d8
        }
    };

    let m2 = fmul(u0, clamped);
    let m3 = fmul(u1, clamped);
    let m4 = fmul(u2, clamped);
    let base = esi_b.wrapping_add(0x30);
    let r_a = fadd(rdf(base), m2);
    let r_b = fadd(rdf(base.wrapping_add(4)), m3);
    let r_c = fadd(rdf(base.wrapping_add(8)), m4);
    unsafe { ((o0 + 0x26) as *mut u8).write(0x1a) };
    let _ = callee_thiscall!(10, u32, o0, 0);
    unsafe { ((o0 + 4) as *mut u32).write(r_a.to_bits()) };
    unsafe { ((o0 + 8) as *mut u32).write(r_b.to_bits()) };
    unsafe { ((o0 + 0x0c) as *mut u32).write(r_c.to_bits()) };
    // Pushed (o0, o1) so the top-first argument order is (o1, o0).
    let _ = callee_cdecl!(11, u32, o1, o0);

    let a1e = rd32(o1.wrapping_add(0x20));
    let h0 = rdf(a1e.wrapping_add(0x30));
    let h1 = rdf(a1e.wrapping_add(0x34));
    let h2 = rdf(a1e.wrapping_add(0x38));
    let e1 = fsub(r_b, h1);
    let e0 = fsub(r_a, h0);
    let e2 = fsub(r_c, h2);
    let dd2 = fadd(fadd(fmul(e1, e1), fmul(e0, e0)), fmul(e2, e2));
    let lim2 = f32::from_bits(unsafe { global::<u32>(G_TAIL_LIM).read() });
    let sq2 = fsqrt(dd2);
    if !(lim2 > sq2) {
        return a1e;
    }

    let v3 = vcall_get(o1, t1slot.as_ptr() as u32);
    let z0 = rdf(v3);
    let z1 = rdf(v3.wrapping_add(4));
    let z2 = rdf(v3.wrapping_add(8));
    let qq = fadd(fadd(fmul(z0, z0), fmul(z1, z1)), fmul(z2, z2));
    let sq3 = fsqrt(qq);
    let lim3 = f32::from_bits(unsafe { global::<u32>(G_VEC_LIM).read() });
    if !(sq3 > lim3) {
        return v3;
    }
    let g = unsafe { global::<u32>(G_BASE_WORD).read() }.wrapping_add(0x320);
    unsafe { ((o2 + 0x2a) as *mut u8).write(6) };
    unsafe { ((o2 + 0x10) as *mut u32).write(g) };
    g
}

export!(cdecl, rw_00b2acb0(o0: u32, o1: u32, o2: u32) -> u32 {
    body::<false>(o0, o1, o2)
});

export!(cdecl, mut_00b2acb0(o0: u32, o1: u32, o2: u32) -> u32 {
    body::<true>(o0, o1, o2)
});
