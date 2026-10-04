// original: 0x00b8cfd0 PRINT_HELP_WITH_NUMBER
/// Script native `PRINT_HELP_WITH_NUMBER` (hash 0x4475789E).
///
/// Forwards two script arguments (a text label and a number) to the engine
/// text queue. No return slot is written.
export!(cdecl, rw_00b8cfd0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
