// original: 0x00bd83f0 NETWORK_GET_HOST_NAME
/// Script native `NETWORK_GET_HOST_NAME` (hash 0x4A8048F8).
///
/// Forwards one script argument to the engine and stores its full
/// /// 32-bit answer into the return slot.
export!(cdecl, rw_00bd83f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
