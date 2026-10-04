// original: 0x00bb62e0 DECREMENT_FLOAT_STAT
/// Script native `DECREMENT_FLOAT_STAT` (hash 0x0754000C).
///
/// Forwards two script arguments (a statistic index and a float bit-pattern) to the engine. No return slot is written.
export!(cdecl, rw_00bb62e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
