// original: 0x00a01bf0 SET_OBJECT_ANIM_SPEED
/// Script native `SET_OBJECT_ANIM_SPEED` (hash 0x168B18ED).
///
/// Forwards four script arguments to the engine: an object handle, two
/// integers (animation and clip references) and a float speed as raw bits.
/// No return slot is written.
export!(cdecl, rw_00a01bf0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
