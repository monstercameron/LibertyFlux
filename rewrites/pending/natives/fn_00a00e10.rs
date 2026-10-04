// original: 0x00a00e10 GET_OBJECT_QUATERNION
/// Script native `GET_OBJECT_QUATERNION` (hash 0x0F731898).
///
/// Forwards five script arguments (an object handle and four out-pointers)
/// to the engine. No return slot is written by the handler itself.
export!(cdecl, rw_00a00e10(ctx: *const u8) -> u32 {
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
