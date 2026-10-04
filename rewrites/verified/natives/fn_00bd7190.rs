// original: 0x00bd7190 FORCE_TIME_OF_DAY
/// Script native `FORCE_TIME_OF_DAY` (hash 0x0B9B5070).
///
/// Forwards an on/off flag and an hour value to the engine. No return slot
/// is written.
export!(cdecl, rw_00bd7190(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
