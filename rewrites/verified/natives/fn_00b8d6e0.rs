// original: 0x00b8d6e0 SET_MENU_COLUMN_WIDTH
/// Script native `SET_MENU_COLUMN_WIDTH` (hash 0x0DBF663C).
///
/// Forwards three script arguments to the engine: two integers and
/// /// one float bit-pattern (the column width). The float is copied
/// /// as raw bits, so the forward is bit-exact. No return slot is
/// /// written.
export!(cdecl, rw_00b8d6e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
