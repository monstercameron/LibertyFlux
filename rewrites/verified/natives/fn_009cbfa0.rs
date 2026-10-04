// original: 0x009cbfa0 NEW_MOBILE_PHONE_CALL
/// Script native `NEW_MOBILE_PHONE_CALL` (hash 0x720E7EA6).
///
/// Tail-jumps to a shared implementation; the rewrite forwards the call context and returns its answer.
export!(cdecl, rw_009cbfa0(ctx: *const u8) -> u32 {
    unsafe { callee_cdecl!(1, u32, ctx as u32) }
});
