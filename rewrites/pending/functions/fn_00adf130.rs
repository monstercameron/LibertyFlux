// original: 0x00ADF130 box_bounds
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

/// Table of class descriptors indexed by event class id.
const CLASS_TABLE: u32 = 0x1295CD8;

/// Corner transform, standard form (blocks 1, 2, 3): each output is the
/// scalar by the corner weight plus the row by the two axis weights plus
/// the translation. Operations are in the original's exact operand order
/// and association so NaN payloads propagate identically.
///
/// Each row is computed contiguously and fenced off with `black_box`: the
/// optimizer would otherwise pack the three rows into SIMD vector ops,
/// which can swap a NaN-carrying add's operands and change the payload
/// (observed: x became canonical NaN instead of propagating the input's).
#[inline(always)]
fn corner_std(
    s: f32, a: f32, c: f32, m0: f32, m4: f32, m8: f32, m10: f32, m14: f32,
    m18: f32, m20: f32, m24: f32, m28: f32, m30: f32, m34: f32, m38: f32,
) -> (f32, f32, f32) {
    let mut o1 = m10 * s;
    o1 += m0 * a;
    o1 += m20 * c;
    o1 += m30;
    let o1 = core::hint::black_box(o1);
    let mut o2 = m14 * s;
    o2 += m4 * a;
    o2 += m24 * c;
    o2 += m34;
    let o2 = core::hint::black_box(o2);
    let mut o3 = m18 * s;
    o3 += m8 * a;
    o3 += m28 * c;
    o3 += m38;
    let o3 = core::hint::black_box(o3);
    (o1, o2, o3)
}

/// Corner transform, axis-first form (blocks 4, 6, 8): the first product
/// of the x row multiplies the axis weight by the matrix element, in that
/// order; the other two rows are standard. Rows are fenced as above.
#[inline(always)]
fn corner_afirst(
    s: f32, a: f32, c: f32, m0: f32, m4: f32, m8: f32, m10: f32, m14: f32,
    m18: f32, m20: f32, m24: f32, m28: f32, m30: f32, m34: f32, m38: f32,
) -> (f32, f32, f32) {
    let mut o1 = m10 * s;
    o1 += a * m0;
    o1 += m20 * c;
    o1 += m30;
    let o1 = core::hint::black_box(o1);
    let mut o2 = m14 * s;
    o2 += m4 * a;
    o2 += m24 * c;
    o2 += m34;
    let o2 = core::hint::black_box(o2);
    let mut o3 = m18 * s;
    o3 += m8 * a;
    o3 += m28 * c;
    o3 += m38;
    let o3 = core::hint::black_box(o3);
    (o1, o2, o3)
}

/// Corner transform, row-first form (blocks 5, 7): the y and z rows add
/// the axis term before the scaled corner term; the x row is standard.
/// Rows are fenced as above.
#[inline(always)]
fn corner_rev23(
    s: f32, a: f32, c: f32, m0: f32, m4: f32, m8: f32, m10: f32, m14: f32,
    m18: f32, m20: f32, m24: f32, m28: f32, m30: f32, m34: f32, m38: f32,
) -> (f32, f32, f32) {
    let mut o1 = m10 * s;
    o1 += m0 * a;
    o1 += m20 * c;
    o1 += m30;
    let o1 = core::hint::black_box(o1);
    let mut o2 = m4 * a;
    o2 += m14 * s;
    o2 += m24 * c;
    o2 += m34;
    let o2 = core::hint::black_box(o2);
    let mut o3 = m8 * a;
    o3 += m18 * s;
    o3 += m28 * c;
    o3 += m38;
    let o3 = core::hint::black_box(o3);
    (o1, o2, o3)
}

/// Min-merge matching `comiss new, old; ja keep`: keeps the old value only
/// when the new value is strictly greater, ordered; NaN or equality takes new.
#[inline(always)]
fn merge_min(old: f32, new: f32) -> f32 {
    if new > old {
        old
    } else {
        new
    }
}

/// Max-merge matching `comiss old, new; ja keep` (block 8 spells it `jbe`
/// take-new, which is the same predicate).
#[inline(always)]
fn merge_max(old: f32, new: f32) -> f32 {
    if old > new {
        old
    } else {
        new
    }
}

/// axis-aligned bounding box into the eight output slots (min xyz, a
/// reserved slot, max xyz, a reserved slot).
///
/// Each corner either goes through the rotation helper (callee 1, fed the
/// corner's input triple from the rotating slots) or is transformed inline
/// from the matrix and the class descriptor, chosen per corner by the flag
/// at `this+0x28`. The first corner seeds the bounds; the rest min/max-merge
/// with `comiss` NaN semantics (NaN or equality takes the new value).
///
/// The two reserved output slots copy whatever the scratch frame holds at
/// two never-stored locations; under the checker's defined stack fill those
/// read as zero, which is what this rewrite stores.
// original: 0x00ADF130 box_bounds
export!(thiscall, rw_b198_f2(this: u32, out: u32) -> u32 {
    unsafe {
        let flag = ((this.wrapping_add(0x28)) as *const u8).read();
        let sel = ((this.wrapping_add(0x26)) as *const u8).read() as u32;
        let mat = this.wrapping_add(sel.wrapping_mul(16));
        let cls = ((this.wrapping_add(0x1a)) as *const u16).read() as u32;
        let desc =
            ((relocated(CLASS_TABLE).wrapping_add(cls.wrapping_mul(4))) as *const u32).read();
        let df = |off: u32| ((desc.wrapping_add(off)) as *const f32).read();
        let (d20, d24, d28, d30, d34, d38) =
            (df(0x20), df(0x24), df(0x28), df(0x30), df(0x34), df(0x38));
        let mf = |off: u32| ((mat.wrapping_add(off)) as *const f32).read();
        let (m0, m4, m8) = (mf(0), mf(4), mf(8));
        let (m10, m14, m18) = (mf(0x10), mf(0x14), mf(0x18));
        let (m20, m24, m28) = (mf(0x20), mf(0x24), mf(0x28));
        let (m30, m34, m38) = (mf(0x30), mf(0x34), mf(0x38));
        let outf = |off: u32| ((out.wrapping_add(off)) as *mut f32);
        // Rotating input slots; blocks 3, 5 and 7 rewrite them before branching.
        let mut f20 = d20;
        let mut f36 = d30;
        let mut f12 = d28;
        let mut f28 = d38;
        let f16 = d24;
        let f32v = d34;
        let mut ans = mat;
        macro_rules! helper {
            ($i0:expr, $i1:expr, $i2:expr) => {{
                let inp = [$i0, $i1, $i2];
                let mut triple = [0f32; 3];
                ans = callee_cdecl!(
                    1,
                    u32,
                    triple.as_mut_ptr() as u32,
                    mat,
                    inp.as_ptr() as u32
                );
                (triple[0], triple[1], triple[2])
            }};
        }
        // Block 1: seeds the bounds.
        let (x, y, z);
        if flag & 1 != 0 {
            (x, y, z) = helper!(f20, f16, f12);
        } else {
            (x, y, z) = corner_std(d24, d20, d28, m0, m4, m8, m10, m14, m18, m20, m24,
                m28, m30, m34, m38);
            ans = mat;
        }
        outf(0).write(x);
        outf(4).write(y);
        outf(8).write(z);
        outf(0x0c).write(0.0);
        outf(0x10).write(x);
        outf(0x14).write(y);
        outf(0x18).write(z);
        outf(0x1c).write(0.0);
        macro_rules! merge {
            ($x:expr, $y:expr, $z:expr) => {{
                outf(0).write(merge_min(outf(0).read(), $x));
                outf(4).write(merge_min(outf(4).read(), $y));
                outf(8).write(merge_min(outf(8).read(), $z));
                outf(0x10).write(merge_max(outf(0x10).read(), $x));
                outf(0x14).write(merge_max(outf(0x14).read(), $y));
                outf(0x18).write(merge_max(outf(0x18).read(), $z));
            }};
        }
        // Block 2.
        let (x, y, z);
        if flag & 1 != 0 {
            (x, y, z) = helper!(f36, f32v, f28);
        } else {
            (x, y, z) = corner_std(d34, d30, d38, m0, m4, m8, m10, m14, m18, m20, m24,
                m28, m30, m34, m38);
            ans = mat;
        }
        merge!(x, y, z);
        // Block 3.
        f20 = d30;
        f36 = d20;
        let (x, y, z);
        if flag & 1 != 0 {
            (x, y, z) = helper!(f20, f16, f12);
        } else {
            (x, y, z) = corner_std(d24, d30, d28, m0, m4, m8, m10, m14, m18, m20, m24,
                m28, m30, m34, m38);
            ans = mat;
        }
        merge!(x, y, z);
        // Block 4.
        let (x, y, z);
        if flag & 1 != 0 {
            (x, y, z) = helper!(f36, f32v, f28);
        } else {
            (x, y, z) = corner_afirst(d34, d20, d38, m0, m4, m8, m10, m14, m18, m20, m24,
                m28, m30, m34, m38);
            ans = mat;
        }
        merge!(x, y, z);
        // Block 5.
        f12 = d38;
        f28 = d28;
        let (x, y, z);
        if flag & 1 != 0 {
            (x, y, z) = helper!(f20, f16, f12);
        } else {
            (x, y, z) = corner_rev23(d24, d30, d38, m0, m4, m8, m10, m14, m18, m20, m24,
                m28, m30, m34, m38);
            ans = mat;
        }
        merge!(x, y, z);
        // Block 6.
        let (x, y, z);
        if flag & 1 != 0 {
            (x, y, z) = helper!(f36, f32v, f28);
        } else {
            (x, y, z) = corner_afirst(d34, d20, d28, m0, m4, m8, m10, m14, m18, m20, m24,
                m28, m30, m34, m38);
            ans = mat;
        }
        merge!(x, y, z);
        // Block 7.
        f20 = d20;
        f36 = d30;
        let (x, y, z);
        if flag & 1 != 0 {
            (x, y, z) = helper!(f20, f16, f12);
        } else {
            (x, y, z) = corner_rev23(d24, d20, d38, m0, m4, m8, m10, m14, m18, m20, m24,
                m28, m30, m34, m38);
            ans = mat;
        }
        merge!(x, y, z);
        // Block 8.
        let (x, y, z);
        if flag & 1 != 0 {
            (x, y, z) = helper!(f36, f32v, f28);
        } else {
            (x, y, z) = corner_afirst(d34, d30, d28, m0, m4, m8, m10, m14, m18, m20, m24,
                m28, m30, m34, m38);
            ans = mat;
        }
        merge!(x, y, z);
        ans
    }
});

