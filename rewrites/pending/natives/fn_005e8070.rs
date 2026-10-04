// original: 0x005e8070 GET_WEB_PAGE_LINK_POSN
/// Script native `GET_WEB_PAGE_LINK_POSN` (hash 0x717B5EFB).
///
/// Forwards four script arguments to the engine. No return slot is written.
export!(cdecl, rw_005e8070(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
