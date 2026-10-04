// original: 0x00b8c700 GET_BLIP_INFO_ID_DISPLAY
/// Script native `GET_BLIP_INFO_ID_DISPLAY` (hash 0x1B731C3F).
///
/// Forwards one script argument (a blip handle) to the engine and stores
/// its full 32-bit answer into the return slot.
export!(cdecl, rw_00b8c700(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
