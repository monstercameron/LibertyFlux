// original: 0x00b8cf20 PRINT_HELP
/// Script native `PRINT_HELP` (hash 0x71076BBA).
///
/// Forwards one script argument (a text label key) to the engine. No
/// return slot is written.
export!(cdecl, rw_00b8cf20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
