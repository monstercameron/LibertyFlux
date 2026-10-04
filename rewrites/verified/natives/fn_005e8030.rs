// original: 0x005e8030 GET_NUMBER_OF_WEB_PAGE_LINKS
/// Script native `GET_NUMBER_OF_WEB_PAGE_LINKS` (hash 0x18A22AE4).
///
/// Forwards one script argument (a web page handle) to the engine and
/// stores its full 32-bit answer into the return slot.
export!(cdecl, rw_005e8030(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
