// original: 0x00bd2cb0 DEACTIVATE_SCRIPT_POPULATION_ZONE
/// Script native `DEACTIVATE_SCRIPT_POPULATION_ZONE` (hash 0x66BB737D).
///
/// Body is a single tail jump to a shared implementation taking the call
/// context. The rewrite forwards the context through the checker's
/// tail-call interception and returns the shared routine's answer.
export!(cdecl, rw_00bd2cb0(ctx: *const u8) -> u32 {
    unsafe { callee_cdecl!(1, u32, ctx as u32) }
});
