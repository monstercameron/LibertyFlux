// original: 0x00a01b90 SET_OBJECT_ANIM_CURRENT_TIME
/// Script native `SET_OBJECT_ANIM_CURRENT_TIME` (hash 0x368274DA).
///
/// Forwards four script arguments to the engine: an object handle, an animation-set id, a clip-name id and a float bit-pattern (the new current time). The engine clamps the time to [0,1]. No return slot is written.
export!(cdecl, rw_00a01b90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
