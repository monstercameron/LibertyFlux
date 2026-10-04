// original: 0x00bb1e30 GET_NEEDED_PLAYER_CASH_FOR_LEVEL
/// Script native `GET_NEEDED_PLAYER_CASH_FOR_LEVEL` (hash 0x3C14367C).
///
/// Forwards one script argument (a level index) to the engine and stores
/// its full 32-bit answer into the return slot.
export!(cdecl, rw_00bb1e30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
