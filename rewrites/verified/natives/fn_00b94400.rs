// original: 0x00b94400 GET_DISTANCE_BETWEEN_COORDS_2D
/// Script native `GET_DISTANCE_BETWEEN_COORDS_2D` (hash 0x687107CA).
///
/// Forwards five script arguments to the engine: four float coordinates (copied as raw bits) and one integer. No return slot is written.
export!(cdecl, rw_00b94400(ctx: *const u8) -> u32 {
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
