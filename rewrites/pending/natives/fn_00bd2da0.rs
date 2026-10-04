// original: 0x00bd2da0 GET_NAME_OF_ZONE
/// Script native `GET_NAME_OF_ZONE` (hash 0x25442DF7).
///
/// Forwards 3 script argument(s) to the engine: 3 float bit-pattern(s).
/// Floats are copied as raw bits, so the forward is bit-exact.
/// Stores the full engine answer into the return slot.
export!(cdecl, rw_00bd2da0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
