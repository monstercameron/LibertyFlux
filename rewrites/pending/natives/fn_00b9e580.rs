// original: 0x00b9e580 CLEAR_RELATIONSHIP
/// Script native `CLEAR_RELATIONSHIP` (hash 0x3FF16CBC).
///
/// Forwards three script arguments (two relationship-group handles and a
/// flags word) to the engine. No return slot is written.
export!(cdecl, rw_00b9e580(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
