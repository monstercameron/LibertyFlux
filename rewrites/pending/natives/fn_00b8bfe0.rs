// original: 0x00b8bfe0 DISPLAY_ALTIMETER_THIS_FRAME
/// Script native `DISPLAY_ALTIMETER_THIS_FRAME` (hash 0x50C13702).
///
/// Body is a single tail jump to a shared implementation taking the call
/// context. The rewrite forwards the context and returns the answer.
export!(cdecl, rw_00b8bfe0(ctx: *const u8) -> u32 {
    unsafe { callee_cdecl!(1, u32, ctx as u32) }
});
