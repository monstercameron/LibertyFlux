// original: 0x00bd2cc0 FIND_PRIMARY_POPULATION_ZONE_GROUP
/// Script native `FIND_PRIMARY_POPULATION_ZONE_GROUP` (hash 0x36601178).
///
/// Forwards two script arguments (a zone hash and an out-variable) to the engine zone-group lookup.
/// No return slot is written.
export!(cdecl, rw_00bd2cc0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
