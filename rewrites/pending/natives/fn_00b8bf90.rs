// original: 0x00b8bf90 DELETE_MENU
/// Script native `DELETE_MENU` (hash 0x252138B3).
///
/// Forwards one script argument (a menu handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b8bf90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
