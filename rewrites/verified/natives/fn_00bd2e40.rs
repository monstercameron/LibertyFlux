// original: 0x00bd2e40 SPECIFY_SCRIPT_POPULATION_ZONE_AREA
/// Script native `SPECIFY_SCRIPT_POPULATION_ZONE_AREA` (hash 0x5A07394A).
///
/// Forwards six script arguments (a zone name hash, coordinates and related
/// values) to the engine. No return slot is written.
export!(cdecl, rw_00bd2e40(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5),
        )
    }
});
