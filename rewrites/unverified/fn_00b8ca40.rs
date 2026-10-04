// original: 0x00b8ca40 GET_NUMBER_LINES
/// Script native `GET_NUMBER_LINES` (hash 0x67B725B2).
///
/// Forwards three script arguments (two float bit-patterns and an integer)
/// to the engine and stores its full 32-bit answer into the return slot.
export!(cdecl, rw_00b8ca40(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
