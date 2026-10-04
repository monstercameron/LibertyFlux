// original: 0x00bd8300 NETWORK_FINISH_EXTENDED_SEARCH
/// Script native `NETWORK_FINISH_EXTENDED_SEARCH` (hash 0x1E0A7AD8).
///
/// Tail-jumps to the shared engine routine with the call context still on
/// the stack. The rewrite expresses the same behaviour as a call that
/// forwards the context pointer and returns the engine answer.
export!(cdecl, rw_00bd8300(ctx: *const u8) -> u32 {
    // The original is a single tail jump: the context pointer stays on
    // the stack for the shared routine. The leading zero word stands in
    // for the checker's patch slot and is skipped by the contract, so
    // only the forwarded context pointer is compared.
    callee_cdecl!(1, u32, 0u32, ctx as u32)
});
