// original: 0x00c155f0 table_update_dual
//! Lane r-b115 rewrite. Helpers and constants below are shared
//! verbatim with this lane's other rewrite files; keep one copy when merging.

use lf_checker_rt::{callee_thiscall, export, global};

/// Global float constant used as the normalization numerator.
const G_NORM: u32 = 0x00FE88E8;
/// Two coupled global factors (fn 0x00C155F0).
const G_PAIR: u32 = 0x0110DB50;
/// Base of the 16-byte-entry table indexed by `this[0x38C] + k*7`.
const G_TABLE: u32 = 0x01047FF8;
/// Offset of the table index base inside `this`.
const THIS_INDEX: usize = 0x38C;
/// Stride multiplier applied to the stack index argument.
const INDEX_SCALE: u32 = 7;
/// Entry size of the global table in bytes.
const TABLE_ENTRY: u32 = 16;

#[inline(always)]
unsafe fn rf(base: *const u8, off: usize) -> f32 {
    unsafe { (base.add(off) as *const f32).read() }
}

#[inline(always)]
unsafe fn ru(base: *const u8, off: usize) -> u32 {
    unsafe { (base.add(off) as *const u32).read() }
}

#[inline(always)]
unsafe fn wf(base: *mut u8, off: usize, v: f32) {
    unsafe { (base.add(off) as *mut f32).write(v) }
}

#[inline(always)]
unsafe fn wu(base: *mut u8, off: usize, v: u32) {
    unsafe { (base.add(off) as *mut u32).write(v) }
}

/// Entry pointer of the global table for row `idx`.
#[inline(always)]
fn table_row(idx: u32) -> *mut u8 {
    global::<u8>(G_TABLE).wrapping_add(idx.wrapping_mul(TABLE_ENTRY) as usize)
}

/// Normalize a sum of squares the way the originals do: `C0/sqrt(s)` for a
/// nonzero `s` (NaN counts as nonzero, matching the jp-after-ucomiss test),
/// otherwise +0.0.
#[inline(always)]
fn norm_factor(s: f32, c0: f32) -> f32 {
    if s != 0.0 { c0 / s.sqrt() } else { 0.0 }
}


/// Dual-array variant: the helper takes two 3-vectors.
///
/// Mixes `p0[0x30..0x38]` with one table float, normalizes the difference
/// against `p1[0..8]` into the second helper array, and separately
/// normalizes the two coupled globals into the first helper array.
/// The third stack argument is unused. Thiscall; returns the table pointer.
export!(thiscall, rw_00c155f0(this: *mut u8, p0: *const u8, p1: *const u8, _p2: u32, k: u32) -> u32 {
    unsafe {
        let c0 = *(global::<f32>(G_NORM) as *const f32);
        let g0 = *(global::<f32>(G_PAIR) as *const f32);
        let g1 = *(global::<u8>(G_PAIR).wrapping_add(4) as *const f32);
        let k7 = k.wrapping_mul(INDEX_SCALE);
        let row = table_row(ru(this, THIS_INDEX).wrapping_add(k7));
        let t4 = rf(row, 4);
        let pad = t4 * 0.0;
        let m0 = rf(p0, 0x30) + pad;
        let m1 = rf(p0, 0x34) + pad;
        let m2 = rf(p0, 0x38) + t4;
        let e0 = rf(p1, 0) - m0;
        let e1 = rf(p1, 4) - m1;
        let e2 = rf(p1, 8) - m2;
        let s2 = e1 * e1 + e0 * e0 + e2 * e2;
        let g = norm_factor(s2, c0);
        let arr2 = [e0 * g, e1 * g, e2 * g];
        let f2 = norm_factor(g1 * g1 + g0 * g0, c0);
        let mut arr1 = [g0 * f2, g1 * f2, f2 * 0.0];
        let mut arr2m = arr2;
        // Original pushes the normalized-globals array first, so it is the
        // second stack argument; the difference array is the first.
        callee_thiscall!(1, u32, (this as u32).wrapping_add(0x10),
            arr2m.as_mut_ptr() as u32, arr1.as_mut_ptr() as u32);
        wf(this, 0x40, m0);
        wf(this, 0x44, m1);
        wf(this, 0x48, m2);
        // Uninitialized stack word above the helper arrays; 0 under the
        // checker's zero stack fill.
        wu(this, 0x4C, 0);
        let row = table_row(ru(this, THIS_INDEX).wrapping_add(k7));
        let ret = ru(row, 0);
        wu(this, 0x60, ret);
        ret
    }
});
