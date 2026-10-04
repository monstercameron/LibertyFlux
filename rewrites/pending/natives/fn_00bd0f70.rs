// original: 0x00bd0f70 GET_NUMBER_OF_STICKY_BOMBS_STUCK_TO_OBJECT
/// Script native `GET_NUMBER_OF_STICKY_BOMBS_STUCK_TO_OBJECT`
/// (hash 0x4AD026EE).
///
/// Forwards one script argument (an object handle) to the engine and stores
/// its full 32-bit answer into the return slot.
export!(cdecl, rw_00bd0f70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
