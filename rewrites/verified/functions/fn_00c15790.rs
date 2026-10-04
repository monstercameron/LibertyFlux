// original: 0x00c15790 table_update_plain
//! Lane r-b115 rewrite. Helpers and constants below are shared
//! verbatim with this lane's other rewrite files; keep one copy when merging.

use lf_checker_rt::{callee_thiscall, export, global};

/// Global float constant used as the normalization numerator.
const G_NORM: u32 = 0x00FE88E8;
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


/// Plain variant: unconditional second accumulator, subtracted mix.
///
/// Same shape as `rw_00c15080` except the second table float is added to
/// `p2[8]` unconditionally (no flag test, no extra scale) and the three
/// mix terms are subtracted from `p0[0x30..0x38]` instead of added.
/// Thiscall with four stack arguments; returns the table pointer.
export!(thiscall, rw_00c15790(this: *mut u8, p0: *const u8, p1: *mut u8, p2: *mut u8, k: u32) -> u32 {
    unsafe {
        let c0 = *(global::<f32>(G_NORM) as *const f32);
        let x = rf(p0, 0x10);
        let y = rf(p0, 0x14);
        let f = norm_factor(x * x + y * y, c0);
        let k7 = k.wrapping_mul(INDEX_SCALE);
        let row = table_row(ru(this, THIS_INDEX).wrapping_add(k7));
        let nx = x * f;
        let ny = y * f;
        let p1_new = rf(row, 8) + rf(p1, 8);
        wf(p1, 8, p1_new);
        let row = table_row(ru(this, THIS_INDEX).wrapping_add(k7));
        let fz = f * 0.0;
        wf(p2, 8, rf(row, 8) + rf(p2, 8));
        let row = table_row(ru(this, THIS_INDEX).wrapping_add(k7));
        let t4 = rf(row, 4);
        let sx = nx * t4;
        let sy = ny * t4;
        let sz = t4 * fz;
        let v2 = rf(p0, 0x34) - sy;
        let p1_after = rf(p1, 8);
        let v6 = rf(p0, 0x38) - sz;
        let v7 = rf(p0, 0x30) - sx;
        let d1 = rf(p1, 4) - v2;
        let d0 = rf(p1, 0) - v7;
        let v6b = v6 + rf(row, 12);
        let d2 = p1_after - v6b;
        let s5 = d1 * d1 + d0 * d0 + d2 * d2;
        let g = norm_factor(s5, c0);
        let mut out = [d0 * g, d1 * g, d2 * g];
        callee_thiscall!(1, u32, (this as u32).wrapping_add(0x10), out.as_mut_ptr() as u32);
        wf(this, 0x40, v7);
        wf(this, 0x44, v2);
        wf(this, 0x48, v6b);
        // Uninitialized stack word above the helper argument; 0 under the
        // checker's zero stack fill.
        wu(this, 0x4C, 0);
        let row = table_row(ru(this, THIS_INDEX).wrapping_add(k7));
        let ret = ru(row, 0);
        wu(this, 0x60, ret);
        ret
    }
});
