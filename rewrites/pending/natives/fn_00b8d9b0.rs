// original: 0x00b8d9b0 SET_TEXT_LINE_DISPLAY
/// Script native `SET_TEXT_LINE_DISPLAY` (hash 0x1F6A54B6).
///
/// Forwards two script arguments (a text-line handle and a flag) to the
/// engine. No return slot is written.
export!(cdecl, rw_00b8d9b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
