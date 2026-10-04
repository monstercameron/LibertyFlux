// original: 0x00a00e70 GET_OBJECT_TURN_MASS
/// Script native `GET_OBJECT_TURN_MASS` (hash 0x3C85109F).
///
/// Forwards an object handle and an out-pointer argument to the engine.
/// No return slot is written.
export!(cdecl, rw_00a00e70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
