// original: 0x00bd2e90 SPECIFY_SCRIPT_POPULATION_ZONE_NUM_CARS
/// Script native `SPECIFY_SCRIPT_POPULATION_ZONE_NUM_CARS` (hash 0x1B886584).
///
/// Forwards one script argument (a car count) to the engine.
/// No return slot is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00bd2e90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
