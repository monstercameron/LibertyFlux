// original: 0x00b8c720 GET_BLIP_INFO_ID_CAR_INDEX
/// Script native `GET_BLIP_INFO_ID_CAR_INDEX` (hash 0x566D04C2).
///
/// Forwards one script argument (a blip handle) to the engine and stores
/// the full 32-bit answer into the return slot. The answer is the exit value.
export!(cdecl, rw_00b8c720(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
