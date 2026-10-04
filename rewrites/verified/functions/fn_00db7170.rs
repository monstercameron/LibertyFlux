// original: 0x00DB7170 UIFontString::vf87
// Emit one indexed string row of a UI font string through the text backend.
//
// The global slot selector picks one of seventeen-word row descriptors inside
// the object. When the row's tag byte or its ready flag is clear the call
// draws nothing. Otherwise the row's style word, corner coordinates, colour
// and opacity are forwarded to the backend, the row's text is resolved either
// from a scratch conversion or from the shared text cache, and the finished
// row is submitted. The ready flag is cleared on the way out.
//
// Returns the submission answer on the drawing path. On the two early-out
// paths the original falls through with the shifted slot selector still in
// the return register, and that value is reproduced here.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

export!(thiscall, rw_db7170(this_ptr: u32) -> u32 {
    /// Global slot selector shared by the indexed string rows.
    const SLOT_INDEX: u32 = 0x017A_65AC;
    /// Global UI mode bytes gating the style-word override.
    const UI_MODE: u32 = 0x0116_C250;
    const UI_MODE_ALT: u32 = 0x0116_C253;
    const UI_MODE_DRAW: u8 = 0x6A;
    /// Shared text-cache object used by the non-scratch path.
    const TEXT_CACHE: u32 = 0x0116_BFF0;
    /// Words per row descriptor stride factor (17 words of 4 bytes).
    const ROW_STRIDE_WORDS: u32 = 17;
    /// Default opaque black forwarded when no style word applies yet.
    const DEFAULT_COLOUR: u32 = 0xFF00_0000;

    // Scratch row-text buffer handed to the converter, then submitted.
    let mut scratch = [0u32; 8];

    let g = unsafe { global::<u32>(SLOT_INDEX).read() };
    let tag = unsafe {
        (this_ptr.wrapping_add(g << 8).wrapping_add(0x30E) as *const u8).read()
    };
    if tag == 0 {
        callee_cdecl!(12, u32, );
        return g << 8;
    }
    let row = this_ptr.wrapping_add(g.wrapping_mul(ROW_STRIDE_WORDS).wrapping_mul(4));
    let read_row_u8 = |off: u32| unsafe { ((row + off) as *const u8).read() };
    let read_row_u32 = |off: u32| unsafe { ((row + off) as *const u32).read() };
    if read_row_u8(0x130) == 0 {
        callee_cdecl!(12, u32, );
        return g << 8;
    }
    let use_scratch = read_row_u8(0x12C) != 0;

    let mode = unsafe { global::<u8>(UI_MODE).read() };
    let mode_alt = unsafe { global::<u8>(UI_MODE_ALT).read() };
    let styled = unsafe { ((this_ptr + 0x208) as *const u8).read() };
    let style: u32 = if (mode == UI_MODE_DRAW || mode_alt != 0) && styled == 0 {
        2
    } else {
        read_row_u32(0x124)
    };

    callee_cdecl!(1, u32, style);
    callee_cdecl!(2, u32, read_row_u32(0x100), read_row_u32(0x104));
    callee_cdecl!(3, u32, unsafe {
        ((this_ptr + 0x1E8) as *const u32).read()
    });
    callee_cdecl!(4, u32, DEFAULT_COLOUR);
    callee_cdecl!(5, u32, read_row_u32(0x120));
    callee_cdecl!(6, u32, 1);
    callee_cdecl!(7, u32, read_row_u32(0x108), read_row_u32(0x10C));
    callee_cdecl!(8, u32, unsafe {
        ((this_ptr + 0x1EC) as *const u32).read()
    });

    let text = this_ptr.wrapping_add(g << 8).wrapping_add(0x30E);
    let submitted: u32 = if use_scratch {
        callee_cdecl!(9, u32, text, scratch.as_mut_ptr() as u32);
        callee_cdecl!(
            11,
            u32,
            read_row_u32(0x0F0),
            read_row_u32(0x0F4),
            scratch.as_ptr() as u32,
            0xFFFF_FFFF,
            0xFFFF_FFFF
        )
    } else {
        let cached = callee_thiscall!(10, u32, relocated(TEXT_CACHE), text);
        callee_cdecl!(
            11,
            u32,
            read_row_u32(0x0F0),
            read_row_u32(0x0F4),
            cached,
            0xFFFF_FFFF,
            0xFFFF_FFFF
        )
    };
    unsafe { ((row + 0x130) as *mut u8).write(0) };
    callee_cdecl!(12, u32, );
    submitted
});
