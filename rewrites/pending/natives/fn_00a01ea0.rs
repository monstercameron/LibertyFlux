// original: 0x00a01ea0 SET_OBJECT_QUATERNION
/// Script native `SET_OBJECT_QUATERNION` (hash 0x71270D73).
///
/// Forwards five script arguments (an object handle and four float words,
/// bit-for-bit) to the engine. No return slot is written.
export!(cdecl, rw_00a01ea0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4)
        )
    }
});
