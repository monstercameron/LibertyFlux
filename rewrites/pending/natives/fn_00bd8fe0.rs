// original: 0x00bd8fe0 REMOVE_ALL_NETWORK_RESTART_POINTS
/// Script native `REMOVE_ALL_NETWORK_RESTART_POINTS` (hash 0x3FC034EB).
///
/// Tail-jumps to a shared implementation; the rewrite forwards the call context and returns its answer.
export!(cdecl, rw_00bd8fe0(ctx: *const u8) -> u32 {
    unsafe { callee_cdecl!(1, u32, ctx as u32) }
});
