// original: 0x00bd2e00 SET_ZONE_POPULATION_TYPE
/// Script native `SET_ZONE_POPULATION_TYPE` (hash 0x70582D53).
///
/// Forwards two script arguments (zone and population type ids) to the
/// engine. No return slot is written.
export!(cdecl, rw_00bd2e00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
