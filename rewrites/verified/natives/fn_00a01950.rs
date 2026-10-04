// original: 0x00a01950 RESET_HEALTH_PICKUP_NETWORK_REGEN_TIME
/// Script native `RESET_HEALTH_PICKUP_NETWORK_REGEN_TIME` (hash 0x08237C10).
///
/// The body is a single jump to a shared implementation (a tail call), so
/// the context pointer reaches the implementation unchanged. The checker
/// intercepts the jump; the rewrite forwards the context pointer as the
/// implementation's argument and returns its answer.
///
/// Transport note: the worker rewrites the jump into a call, so its log
/// for this site starts with a return address the true original never
/// pushes. The rewrite passes a zero filler first to keep the forwarded
/// context at the same log index on both sides; the contract skips the
/// filler word and snapshots the forwarded context instead.
export!(cdecl, rw_00a01950(ctx: *const u8) -> u32 {
    callee_cdecl!(1, u32, 0, ctx as u32)
});
