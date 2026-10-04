// original: 0x00a00e90 GET_OBJECT_VELOCITY
/// Script native `GET_OBJECT_VELOCITY` (hash 0x06D651A7).
///
/// Forwards four script arguments (an object handle and three out-pointers)
/// to the engine. No return slot is written by the handler itself.
export!(cdecl, rw_00a00e90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
