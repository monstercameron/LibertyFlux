// original: 0x00ba2360 SET_PED_DENSITY_MULTIPLIER
/// Script native `SET_PED_DENSITY_MULTIPLIER` (hash 0x540F2DF7).
///
/// Forwards one script argument (a float multiplier) to the engine as raw
/// bits, so the forward is bit-exact. No return slot is written.
export!(cdecl, rw_00ba2360(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
