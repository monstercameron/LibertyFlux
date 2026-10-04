// original: 0x00bd2ec0 SPECIFY_SCRIPT_POPULATION_ZONE_NUM_SCENARIO_PEDS
/// Script native `SPECIFY_SCRIPT_POPULATION_ZONE_NUM_SCENARIO_PEDS` (hash 0x6A733E6C).
///
/// Forwards one script argument (the scenario-ped count) to the engine. No
/// return slot is written.
export!(cdecl, rw_00bd2ec0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
