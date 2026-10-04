// original: 0x00b8d090 PRINT_STRING_IN_STRING_NOW
/// Script native `PRINT_STRING_IN_STRING_NOW` (hash 0x2BB65467).
///
/// Forwards four script arguments to the engine. No return slot is written.
export!(cdecl, rw_00b8d090(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
