// original: 0x009cbf10 MISSION_AUDIO_BANK_NO_LONGER_NEEDED
/// Script native `MISSION_AUDIO_BANK_NO_LONGER_NEEDED` (hash 0x12C42F66).
///
/// This handler is a single tail jump to a shared engine routine: it makes
/// no call of its own and reads no arguments. The rewrite expresses the
/// same transfer as a call that forwards the context pointer to the
/// shared routine and returns its answer. (The checker intercepts the
/// original's jump with its tail-jump patch, so both sides are observed
/// firing the same callee and returning the same answer.)
export!(cdecl, rw_009cbf10(ctx: *const u8) -> u32 {
    callee_cdecl!(1, u32, ctx as u32)
});
