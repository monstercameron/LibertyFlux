// original: 0x00d76020 DrawMagentaRect
//! Magenta-rectangle batch draw.
//!
//! `origin` points at two f32 values (x0, y0); `w` and `h` are f32 bit
//! patterns giving the width and height. Opens a batch, programs four
//! render-state pairs, derives the four corners (x0+w and y0+h are computed
//! once and shared), emits them through the vertex helper, draws the quad
//! in solid magenta (0xff0000ff), and closes the batch. Returns the
//! batch-end helper's answer.

use lf_checker_rt::{callee_cdecl, export};

const C_BEGIN: u32 = 1;
const C_SETSTATE: u32 = 2;
const C_EMIT: u32 = 3;
const C_DRAW_QUAD: u32 = 4;
const C_END: u32 = 5;

export!(stdcall, rw_00d76020(origin: u32, w: u32, h: u32) -> u32 {
    unsafe {
        const MAGENTA: u32 = 0xff0000ff;
        callee_cdecl!(C_BEGIN, u32,);
        callee_cdecl!(C_SETSTATE, u32, 0x0au32, 0u32);
        callee_cdecl!(C_SETSTATE, u32, 2u32, 5u32);
        callee_cdecl!(C_SETSTATE, u32, 0x0fu32, 8u32);
        callee_cdecl!(C_SETSTATE, u32, 7u32, 0u32);
        let x0 = *(origin as *const f32);
        let y0 = *((origin + 4) as *const f32);
        // Operand order matches the original (base first, extent second) so
        // NaN payloads propagate bit-identically.
        let x1 = x0 + f32::from_bits(w);
        let y1 = y0 + f32::from_bits(h);
        callee_cdecl!(C_EMIT, u32, 0u32, 1u32);
        let mut verts = [x1, y1, x1, y0, x0, y1, x0, y0];
        let mut color = MAGENTA;
        let base = verts.as_mut_ptr() as u32;
        let color_ptr = &mut color as *mut u32 as u32;
        callee_cdecl!(C_DRAW_QUAD, u32, base, base + 8, base + 16, base + 24, color_ptr);
        callee_cdecl!(C_END, u32,)
    }
});
