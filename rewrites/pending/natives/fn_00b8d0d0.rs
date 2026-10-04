// original: 0x00b8d0d0 PRINT_STRING_WITH_LITERAL_STRING_NOW
/// Script native `PRINT_STRING_WITH_LITERAL_STRING_NOW` (hash 0x0CA539D6).
///
/// Forwards four print-job arguments (durations, flags and string slots)
/// to the engine's text queue. No return slot is written.
export!(cdecl, rw_00b8d0d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
