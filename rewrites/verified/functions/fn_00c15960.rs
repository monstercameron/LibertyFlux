// original: 0x00c15960 table_update_select
//! Lane r-b115 rewrite. Helpers and constants below are shared
//! verbatim with this lane's other rewrite files; keep one copy when merging.

use lf_checker_rt::{callee_thiscall, export, global};

/// Global float constant used as the normalization numerator.
const G_NORM: u32 = 0x00FE88E8;
/// Alternate numerator selected when the index base is not 2 (fn 0x00C15960).
const G_ALT_NUM: u32 = 0x00FE8D94;
/// Extra global multiplier (fn 0x00C15960).
const G_EXTRA: u32 = 0x01048120;
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


/// Selecting variant: two normalized input pairs, index-dependent scale.
///
/// Normalizes both `p0[0x10,0x14]` and `p0[0,4]`, picks the mix scale from
/// one of two global constants depending on whether the index base is
/// exactly 2, folds the extra global into the second pair, and rewrites
/// `p1[0..8]` in place before the usual normalize-and-call tail.
/// Thiscall with four stack arguments; returns the table pointer.
export!(thiscall, rw_00c15960(this: *mut u8, p0: *const u8, p1: *mut u8, p2: *mut u8, k: u32) -> u32 {
    unsafe {
        let c0 = *(global::<f32>(G_NORM) as *const f32);
        let base = ru(this, THIS_INDEX);
        let sel = if base == 2 {
            c0
        } else {
            *(global::<f32>(G_ALT_NUM) as *const f32)
        };
        let x = rf(p0, 0x10);
        let y = rf(p0, 0x14);
        let px = rf(p0, 0);
        let py = rf(p0, 4);
        let f = norm_factor(x * x + y * y, c0);
        let mut nx = x * f;
        let mut ny = y * f;
        let fz = f * 0.0;
        let f2 = norm_factor(py * py + px * px, c0);
        let k7 = k.wrapping_mul(INDEX_SCALE);
        let row = table_row(k7.wrapping_add(base));
        let mut qy = py * f2;
        let mut qx = px * f2;
        let f2z = f2 * 0.0;
        wf(p1, 8, rf(row, 8) + rf(p1, 8));
        let row = table_row(ru(this, THIS_INDEX).wrapping_add(k7));
        let ge = *(global::<f32>(G_EXTRA) as *const f32);
        ny = ny * ge;
        let p2_new = rf(row, 8) + rf(p2, 8);
        nx = nx * ge;
        let ge_fz = ge * fz;
        wf(p2, 8, p2_new);
        let row = table_row(ru(this, THIS_INDEX).wrapping_add(k7));
        let w5 = rf(p1, 0) + nx;
        let t4 = rf(row, 4);
        qy = qy * t4;
        qx = qx * t4;
        let mut tz = t4 * f2z;
        qy = qy * sel;
        tz = tz * sel;
        qx = qx * sel;
        let w2 = rf(p0, 0x34) + qy;
        let w1 = rf(p0, 0x30) + qx;
        let w3 = rf(p0, 0x38) + tz;
        let w0 = rf(p1, 4) + ny;
        ny = ny + w2;
        nx = nx + w1;
        let w3b = w3 + rf(row, 12);
        let ge2 = ge_fz + rf(p1, 8);
        wf(p1, 4, w0);
        let z0 = w0 - ny;
        let z2 = ge_fz + w3b;
        wf(p1, 0, w5);
        let z5 = w5 - nx;
        let z1 = ge2 - z2;
        wf(p1, 8, ge2);
        let s6 = z0 * z0 + z5 * z5 + z1 * z1;
        let g = norm_factor(s6, c0);
        let mut out = [z5 * g, z0 * g, z1 * g];
        callee_thiscall!(1, u32, (this as u32).wrapping_add(0x10), out.as_mut_ptr() as u32);
        wf(this, 0x40, nx);
        wf(this, 0x44, ny);
        wf(this, 0x48, z2);
        // Uninitialized stack word above the helper argument; 0 under the
        // checker's zero stack fill.
        wu(this, 0x4C, 0);
        let row = table_row(ru(this, THIS_INDEX).wrapping_add(k7));
        let ret = ru(row, 0);
        wu(this, 0x60, ret);
        ret
    }
});
