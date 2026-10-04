// original: 0x00bb2810 RELEASE_TWO_PLAYER_DISTANCE
/// Script native `RELEASE_TWO_PLAYER_DISTANCE` (hash 0x6423636D).
///
/// Tail-jumps to a shared implementation.
/// The rewrite forwards the context pointer through the intercepted callee and returns its answer.
export!(cdecl, rw_00bb2810(ctx: *const u8) -> u32 {
    // Tail call in the original: forward the whole context.
    callee_cdecl!(1, u32, ctx as u32)
});
