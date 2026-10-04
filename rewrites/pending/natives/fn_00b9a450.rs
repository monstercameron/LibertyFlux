// original: 0x00b9a450 FIND_STREET_NAME_AT_POSITION
/// Script native `FIND_STREET_NAME_AT_POSITION` (hash 0x49763A4F).
///
/// Forwards five script arguments to the engine: three float bit-patterns
/// (the x, y, z position) followed by two out-pointers for the street names.
/// Floats are copied as raw bits, so the forward is bit-exact. The original
/// assembles the position through scratch stack slots; only the five pushed
/// words are observable. No return slot is written.
export!(cdecl, rw_00b9a450(ctx: *const u8) -> u32 {
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
        )
    }
});
