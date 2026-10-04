// original: 0x00bd0f50 GET_NUMBER_OF_ACTIVE_STICKY_BOMBS_OWNED_BY_PED
/// Script native `GET_NUMBER_OF_ACTIVE_STICKY_BOMBS_OWNED_BY_PED` (hash 0x21B85DA9).
///
/// Forwards one script argument to the engine and stores its full 32-bit answer into the return slot.
export!(cdecl, rw_00bd0f50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
