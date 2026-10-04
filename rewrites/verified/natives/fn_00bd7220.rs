// original: 0x00bd7220 GET_MINUTES_TO_TIME_OF_DAY
/// Script native `GET_MINUTES_TO_TIME_OF_DAY` (hash 0x740C4C84).
///
/// Hour + minute; stores full answer.
///
/// Stores the engine's full 32-bit answer into the return slot.
export!(cdecl, rw_00bd7220(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
