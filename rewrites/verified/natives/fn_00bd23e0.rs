// original: 0x00bd23e0 START_CHAR_FIRE
/// Script native `START_CHAR_FIRE` (hash 0x5FB31295).
///
/// Forwards one script argument (a ped handle) to the engine and stores its full 32-bit answer into the return slot.
export!(cdecl, rw_00bd23e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
