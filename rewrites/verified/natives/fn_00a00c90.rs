// original: 0x00a00c90 GET_INTERIOR_AT_COORDS
/// Script native `GET_INTERIOR_AT_COORDS` (hash 0x29216610).
///
/// Forwards four script arguments to the engine: three float bit-patterns
/// (world coordinates) and an integer. Floats are copied as raw bits, so
/// the forward is bit-exact. No return slot is written.
export!(cdecl, rw_00a00c90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
