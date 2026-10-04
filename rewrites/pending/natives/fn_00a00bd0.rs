// original: 0x00a00bd0 GET_CLOSEST_STEALABLE_OBJECT
/// Script native `GET_CLOSEST_STEALABLE_OBJECT` (hash 0x27045521).
///
/// Forwards five script arguments to the engine: four float bit-patterns
/// and an integer. No return slot is written.
export!(cdecl, rw_00a00bd0(ctx: *const u8) -> u32 {
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
