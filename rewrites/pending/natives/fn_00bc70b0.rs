// original: 0x00bc70b0 OVERRIDE_NUMBER_OF_PARKED_CARS
/// Script native `OVERRIDE_NUMBER_OF_PARKED_CARS` (hash 0x7F483739).
///
/// Forwards one script argument (the count) to the engine. No return slot
/// is written.
export!(cdecl, rw_00bc70b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
