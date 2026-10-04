// original: 0x00b8ca00 GET_NEXT_BLIP_INFO_ID
/// Script native `GET_NEXT_BLIP_INFO_ID` (hash 0x154932F0).
///
/// Forwards one script argument (a blip info id) to the engine and stores
/// its full 32-bit answer into the return slot.
export!(cdecl, rw_00b8ca00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
