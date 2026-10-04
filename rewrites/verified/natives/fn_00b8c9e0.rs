// original: 0x00b8c9e0 GET_MENU_POSITION
/// Script native `GET_MENU_POSITION` (hash 0x5B576767).
///
/// Reads the current menu position into script out-pointers. Forwards three
/// script arguments (out-pointers) to the engine. No return slot is written
/// by the handler itself.
export!(cdecl, rw_00b8c9e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
