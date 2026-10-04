// original: 0x00b8ca70 GET_NUMBER_LINES_WITH_LITERAL_STRINGS
/// Script native `GET_NUMBER_LINES_WITH_LITERAL_STRINGS` (hash 0x71DE26A3).
///
/// Forwards five script arguments to the engine (two float bit-patterns, a
/// literal-string reference and two integers) and stores its full 32-bit
/// answer into the return slot.
export!(cdecl, rw_00b8ca70(ctx: *const u8) -> u32 {
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
