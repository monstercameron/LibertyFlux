// original: 0x00a00d10 GET_OBJECT_ANIM_TOTAL_TIME
/// Script native `GET_OBJECT_ANIM_TOTAL_TIME` (hash 0x26E66DF3).
///
/// Forwards four script arguments (an object handle, animation identifiers
/// and an out-pointer) to the engine. No return slot is written by the
/// handler itself.
export!(cdecl, rw_00a00d10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
