// original: 0x00D75FD0 draw_rect_blend_state_pair (proposed)

/// Draw the rect, blend the pair, then program two state pairs.
///
/// Draws the rectangle at `a0` with extents `a1`/`a2`, blends the same
/// triple through the pair stage (passing the two extent words by frame
/// pointer), programs state pairs (2, 4) and (0xf, 7), and returns the
/// last answer. Thiscall: object in `ecx` (unused), three stack words.
use lf_checker_rt::{callee_cdecl, callee_stdcall, export};

const DRAW: u32 = 1;
const BLEND: u32 = 2;
const STATE: u32 = 3;

export!(thiscall, rw_00d75fd0(this: u32, a0: u32, a1b: u32, a2b: u32) -> u32 {
    unsafe {
        let _ = this;
        callee_stdcall!(DRAW, u32, a0, a1b, a2b);
        let mut pair = [a1b, a2b];
        callee_stdcall!(BLEND, u32, a0, pair.as_mut_ptr() as u32);
        callee_cdecl!(STATE, u32, 2, 4);
        callee_cdecl!(STATE, u32, 0x0f, 7)
    }
});
