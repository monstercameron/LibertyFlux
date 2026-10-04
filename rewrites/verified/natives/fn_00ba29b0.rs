// original: 0x00ba29b0 SWITCH_PED_TO_RAGDOLL_WITH_FALL
/// Script native `SWITCH_PED_TO_RAGDOLL_WITH_FALL` (hash 0x13E4042D).
///
/// Unlike the other handlers, this one reads no script arguments at all:
/// it passes its own call-context pointer together with a fixed engine
/// callback address to the engine worker. The original pushes the callback
/// as an immediate that the loader relocates (Verified: a HIGHLOW fixup
/// covers it, and the checker shows the original pushing the base-adjusted
/// value), so the rewrite derives the mapped address with `relocated`
/// rather than repeating the file address. No return slot is written.
export!(cdecl, rw_00ba29b0(ctx: *const u8) -> u32 {
    /// Engine callback, file address as pushed by the original's immediate.
    const RAGDOLL_FALLBACK_FILE_VA: u32 = 0x00BAE2F0;
    callee_cdecl!(1, u32, relocated(RAGDOLL_FALLBACK_FILE_VA), ctx as u32)
});
