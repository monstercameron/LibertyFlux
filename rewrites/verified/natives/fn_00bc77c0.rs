// original: 0x00bc77c0 SET_CAR_RANDOM_ROUTE_SEED
/// Script native `SET_CAR_RANDOM_ROUTE_SEED` (hash 0x19D302AE).
///
/// Forwards two script arguments (a vehicle handle and a seed) to the engine. No return slot is written.
export!(cdecl, rw_00bc77c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args.add(0), *args.add(1))
    }
});
