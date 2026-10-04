// original: 0x00a01c80 SET_OBJECT_COORDINATES
/// Script native `SET_OBJECT_COORDINATES` (hash 0x52FD30EB).
///
/// Forwards an object handle and three float coordinates to the engine. Floats are copied as raw bits. No return slot is written.
export!(cdecl, rw_00a01c80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
        )
    }
});
