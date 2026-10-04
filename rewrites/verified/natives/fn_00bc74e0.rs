// original: 0x00bc74e0 SET_CAR_DENSITY_MULTIPLIER
/// Script native `SET_CAR_DENSITY_MULTIPLIER` (hash 0x0AA73A12).
///
/// Forwards one script argument (a float bit-pattern) to the engine
/// worker. No return slot is written.
export!(cdecl, rw_00bc74e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
