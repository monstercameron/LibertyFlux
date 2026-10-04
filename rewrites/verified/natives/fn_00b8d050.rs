// original: 0x00b8d050 PRINT_NOW
/// Script native `PRINT_NOW` (hash 0x73B01573).
///
/// Forwards 3 script arguments to the engine. No return slot is written.
export!(cdecl, rw_00b8d050(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
