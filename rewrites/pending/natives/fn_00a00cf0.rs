// original: 0x00a00cf0 GET_OBJECT_ANIM_CURRENT_TIME
/// Script native `GET_OBJECT_ANIM_CURRENT_TIME` (hash 0x29F02CB1).
///
/// Forwards 4 script arguments to the engine. No return slot is written.
export!(cdecl, rw_00a00cf0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
