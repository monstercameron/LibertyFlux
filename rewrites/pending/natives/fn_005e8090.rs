// original: 0x005e8090 GET_WEB_PAGE_LINK_AT_POSN
/// Script native `GET_WEB_PAGE_LINK_AT_POSN` (hash 0x0C1C5B1B).
///
/// Forwards a handle and two float bit-patterns (a position) to the
/// engine and stores the full 32-bit engine answer (the link id) into the
/// return slot.
export!(cdecl, rw_005e8090(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
