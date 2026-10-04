// original: 0x00bc8060 SMASH_CAR_WINDOW
/// Script native `SMASH_CAR_WINDOW` (hash 0x2CDF628C).
///
/// Forwards two script arguments (a vehicle handle and a window index) to
/// the engine. No return slot is written.
export!(cdecl, rw_00bc8060(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
