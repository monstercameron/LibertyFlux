// original: 0x00b8cee0 PRINT_BIG
/// Script native `PRINT_BIG` (hash 0x2C8A5404).
///
/// Forwards three script arguments (a text label, a display time and a style)
/// to the engine text queue. No return slot is written.
export!(cdecl, rw_00b8cee0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
