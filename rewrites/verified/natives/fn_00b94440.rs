// original: 0x00b94440 GET_DISTANCE_BETWEEN_COORDS_3D
/// Script native `GET_DISTANCE_BETWEEN_COORDS_3D` (hash 0x23F772E7).
///
/// Forwards seven script arguments to the engine: six float bit-patterns
/// (two 3D points) and an out-pointer the engine writes the distance
/// through. No return slot is written by the handler itself.
export!(cdecl, rw_00b94440(ctx: *const u8) -> u32 {
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
            *args.add(6),
        )
    }
});
