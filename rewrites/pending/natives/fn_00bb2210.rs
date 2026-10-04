// original: 0x00bb2210 HAS_ACHIEVEMENT_BEEN_PASSED
/// Native handler `HAS_ACHIEVEMENT_BEEN_PASSED`.
///
/// Report whether an achievement id has been passed.
/// Forwards 1 argument(s) to the engine and stores the low byte
/// of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bb2210(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let arg0 = *args.add(0);
        let _ = arg0;
        // The original keeps only the low byte of the answer.
        let answer = callee_cdecl!(1, u32, arg0);
        *(*ctx as *mut u32) = answer & 0xFF;
        0
    }
});
