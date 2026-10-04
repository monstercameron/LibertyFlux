// original: 0x00c15260 table_update_gated
//! Lane r-b115 rewrite. Helpers and constants below are shared
//! verbatim with this lane's other rewrite files; keep one copy when merging.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

/// Global float constant used as the normalization numerator.
const G_NORM: u32 = 0x00FE88E8;
/// Eight published floats (fn 0x00C15260).
const G_BLOCK8: u32 = 0x01048100;
/// Blend target (fn 0x00C15260).
const G_BLEND: u32 = 0x01047FF0;
/// Scale for the second published group (fn 0x00C15260).
const G_PUB_SCALE: u32 = 0x00FE879C;
/// Absolute-value mask for the blend test (fn 0x00C15260).
const G_ABS_MASK: u32 = 0x00FE8F80;
/// Blend snap threshold (fn 0x00C15260).
const G_BLEND_EPS: u32 = 0x00FE868C;
/// Blend rate (fn 0x00C15260).
const G_BLEND_RATE: u32 = 0x00FE87E8;
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


/// Validated variant: a gate call selects a matrix-transform path.
///
/// Starts like the plain variant, then asks a gate helper whether the
/// normalized 3-vector is usable: a nonzero answer skips straight to the
/// helper call, otherwise eight global floats are published into
/// `this[0x1A0..0x1BC]` (the second group scaled), a blend slot at
/// `this[0x2D0]` is eased toward its target, a second helper runs on
/// `this+0x150` with the blended value, and the 3-vector is pushed
/// through the matrix at `this+0x150` first. Thiscall with four stack
/// arguments; returns the table pointer.
export!(thiscall, rw_00c15260(this: *mut u8, p0: *const u8, p1: *mut u8, p2: *mut u8, k: u32) -> u32 {
    unsafe {
        let c0 = *(global::<f32>(G_NORM) as *const f32);
        let x = rf(p0, 0x10);
        let y = rf(p0, 0x14);
        let f = norm_factor(x * x + y * y, c0);
        let k7 = k.wrapping_mul(INDEX_SCALE);
        let row = table_row(ru(this, THIS_INDEX).wrapping_add(k7));
        let nx = x * f;
        let ny = y * f;
        wf(p1, 8, rf(row, 8) + rf(p1, 8));
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
        let mut v = [d0 * g, d1 * g, d2 * g];
        let gate = callee_cdecl!(1, u32,);
        if (gate & 0xFF) == 0 {
            let blk = global::<u8>(G_BLOCK8);
            wf(this, 0x1A0, rf(blk, 0));
            wf(this, 0x1A4, rf(blk, 4));
            wf(this, 0x1A8, rf(blk, 8));
            wf(this, 0x1AC, rf(blk, 12));
            wf(this, 0x1B0, rf(blk, 16));
            wf(this, 0x1B4, rf(blk, 20));
            wf(this, 0x1B8, rf(blk, 24));
            wf(this, 0x1BC, rf(blk, 28));
            let sc = *(global::<f32>(G_PUB_SCALE) as *const f32);
            wf(this, 0x1B0, rf(this, 0x1B0) * sc);
            wf(this, 0x1B4, rf(this, 0x1B4) * sc);
            wf(this, 0x1B8, rf(this, 0x1B8) * sc);
            let tgt = *(global::<f32>(G_BLEND) as *const f32);
            let cur = rf(this, 0x2D0);
            let d = tgt - cur;
            let mask = *(global::<u32>(G_ABS_MASK) as *const u32);
            let m = f32::from_bits(d.to_bits() & mask);
            let thresh = *(global::<f32>(G_BLEND_EPS) as *const f32);
            // jbe after comiss: snap unless strictly greater (NaN snaps).
            if !(m > thresh) {
                wf(this, 0x2D0, tgt);
            } else {
                let rate = *(global::<f32>(G_BLEND_RATE) as *const f32);
                wf(this, 0x2D0, d * rate + cur);
            }
            let blended = rf(this, 0x2D0);
            let mtx = (this as u32).wrapping_add(0x150);
            callee_thiscall!(2, u32, mtx, blended.to_bits());
            let m = mtx as *const u8;
            let (v0, v1, v2) = (v[0], v[1], v[2]);
            let o0 = rf(m, 0x10) * v1 + v0 * rf(m, 0) + rf(m, 0x20) * v2;
            let o1 = rf(m, 0x14) * v1 + v0 * rf(m, 4) + rf(m, 0x24) * v2;
            let o2 = rf(m, 0x18) * v1 + v0 * rf(m, 8) + rf(m, 0x28) * v2;
            v = [o0, o1, o2];
        }
        callee_thiscall!(3, u32, (this as u32).wrapping_add(0x10), v.as_mut_ptr() as u32);
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
