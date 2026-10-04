// original: 0x00bb8990 BEGIN_PED_QUEUE_MEMBERSHIP_LIST
/// Script native `BEGIN_PED_QUEUE_MEMBERSHIP_LIST` (hash 0x28CA3430).
///
/// Tail-jumps to a shared implementation.
/// The rewrite forwards the context pointer through the intercepted callee and returns its answer.
export!(cdecl, rw_00bb8990(ctx: *const u8) -> u32 {
    // Tail call in the original: forward the whole context.
    callee_cdecl!(1, u32, ctx as u32)
});
