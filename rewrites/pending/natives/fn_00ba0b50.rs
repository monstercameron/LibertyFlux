// original: 0x00ba0b50 MP_GET_AMOUNT_OF_ANCHOR_POINTS
/// Script native `MP_GET_AMOUNT_OF_ANCHOR_POINTS` (hash 0x6C7566F3).
///
/// Forwards two script arguments to the engine and stores the engine's
/// full 32-bit answer into the return slot.
export!(cdecl, rw_00ba0b50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
