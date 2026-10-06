// original: 0x00b9de40 ADD_SCENARIO_BLOCKING_AREA
/// Add a scenario blocking area between two corners.
///
/// Builds a six-word block holding two corner vectors (script arguments 0-2
/// and 3-5) and forwards its address in ECX to the engine implementation.
///
/// The contract skips the ECX address and snapshots all six pointed-to words.
export!(cdecl, rw_00b9de40(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let pair = [*args, *args.add(1), *args.add(2),
                    *args.add(3), *args.add(4), *args.add(5)];
        callee_thiscall!(1, u32, pair.as_ptr() as u32)
    }
});
