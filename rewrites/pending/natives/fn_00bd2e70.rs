// original: 0x00bd2e70 SPECIFY_SCRIPT_POPULATION_ZONE_GROUPS
/// Script native `SPECIFY_SCRIPT_POPULATION_ZONE_GROUPS` (hash 0x70F0538F).
///
/// Forwards five script arguments (zone group ids) to the engine. No return slot is written.
export!(cdecl, rw_00bd2e70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2), *args.add(3), *args.add(4))
    }
});
