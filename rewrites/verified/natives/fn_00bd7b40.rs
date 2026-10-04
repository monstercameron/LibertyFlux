// original: 0x00bd7b40 GET_NUM_KILLS_FOR_RANK_POINTS
/// Script native `GET_NUM_KILLS_FOR_RANK_POINTS` (hash 0x239C0EEC).
///
/// Forwards one integer script argument to the engine and stores the
/// engine's full 32-bit answer into the return slot.
export!(cdecl, rw_00bd7b40(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
