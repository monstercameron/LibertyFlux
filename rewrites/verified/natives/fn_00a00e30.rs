// original: 0x00a00e30 GET_OBJECT_ROTATION_VELOCITY
/// Script native `GET_OBJECT_ROTATION_VELOCITY` (hash 0x492A71E2).
///
/// Forwards four script arguments (an object handle and output slots) to
/// the engine. No return slot is written by the handler itself.
export!(cdecl, rw_00a00e30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
