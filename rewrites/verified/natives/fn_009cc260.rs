// original: 0x009cc260 RELEASE_SCRIPT_CONTROLLED_MICROPHONE
/// Script native handler `RELEASE_SCRIPT_CONTROLLED_MICROPHONE` (hash 0x2F907FF2).
///
/// The original body is a single jump to a shared implementation at 0x009CD530;
/// the rewrite forwards the call context and returns the answer.
export!(cdecl, rw_009cc260(ctx: u32) -> u32 {
    unsafe { callee_cdecl!(1, u32, ctx) }
});
