// original: 0x00b8cab0 GET_NUMBER_LINES_WITH_SUBSTRINGS
/// Script native `GET_NUMBER_LINES_WITH_SUBSTRINGS` (hash 0x00541084).
///
/// Forwards five script arguments to the engine: two floats copied as raw
/// bits, then three integers. Stores the engine's full 32-bit answer (the
/// line count) into the return slot.
export!(cdecl, rw_00b8cab0(ctx: *const u8) -> u32 {
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
