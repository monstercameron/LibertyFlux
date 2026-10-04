// original: 0x00ba0e60 SET_BLOCKING_OF_NON_TEMPORARY_EVENTS
/// Set blocking of non-temporary events: pass the character handle plus
/// the second script argument coerced to 0/1. As in the original, the
/// coerced flag travels in the low byte of a dword holding the context
/// pointer's high bytes; only the low byte is significant.
export!(cdecl, rw_00ba0e60(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        callee_cdecl!(1, u32, *args, (ctx as u32 & 0xFFFFFF00) | flag);
        0
    }
});
