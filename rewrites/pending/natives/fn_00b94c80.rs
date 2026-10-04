// original: 0x00b94c80 REGISTER_SAVE_HOUSE
/// Script native `REGISTER_SAVE_HOUSE` (hash 0x7DF45001).
///
/// Forwards four float words (passed as raw bits) and two integers to the
/// engine and stores all 32 bits of its answer into the return slot.
export!(cdecl, rw_00b94c80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
