// original: 0x00bd2420 START_SCRIPT_FIRE
/// Script native `START_SCRIPT_FIRE` (hash 0x24742BB9).
///
/// Forwards five script arguments to the engine: three float bit-patterns
/// (a position) followed by two integers. Floats are copied as raw bits, so
/// the forward is bit-exact. The handler then stores the engine's full
/// 32-bit answer (a fire handle) into the return slot.
export!(cdecl, rw_00bd2420(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
