// original: 0x005e8050 GET_WEB_PAGE_LINK_HREF
/// Script native `GET_WEB_PAGE_LINK_HREF` (hash 0x750C1CD7).
///
/// Forwards two script arguments to the engine and stores its full 32-bit
/// answer into the return slot. Unlike the boolean natives, this handler
/// keeps the whole answer (`mov`, not `movzx`).
export!(cdecl, rw_005e8050(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
