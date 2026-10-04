// original: 0x00bd21e0 GET_NUMBER_OF_FIRES_IN_AREA
/// Script native `GET_NUMBER_OF_FIRES_IN_AREA` (hash 0x1E144C8B).
///
/// Forwards 6 script argument(s) to the engine: 6 float bit-pattern(s).
/// Floats are copied as raw bits, so the forward is bit-exact.
/// Stores the full engine answer into the return slot.
export!(cdecl, rw_00bd21e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
