// original: 0x00BC7620 SET_CAR_HEALTH
// Rewrite of the SET_CAR_HEALTH native handler.

/// Script native `SET_CAR_HEALTH(car, health)`.
///
/// Forwards the two script arguments to the engine health routine. No return
/// slot is written; the engine's answer is left in the return register.
export!(cdecl, rw_bc7620(ctx: *const u8) -> u32 {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
