// original: 0x00b94340 GET_ANGLE_BETWEEN_2D_VECTORS
/// Script native `GET_ANGLE_BETWEEN_2D_VECTORS` (hash 0x5BC4602D).
///
/// Forwards five script arguments to the engine: four float bit-patterns (two 2D vectors) and an integer. No return slot is written.
export!(cdecl, rw_00b94340(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4))
    }
});
