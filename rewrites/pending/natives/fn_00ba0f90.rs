// original: 0x00ba0f90 SET_CHAR_ANIM_BLEND_OUT_DELTA
/// Script native `SET_CHAR_ANIM_BLEND_OUT_DELTA` (hash 0x000A1FCE).
///
/// Forwards four script arguments to the engine: three integers (character
/// handle, animation slot, blend flag) and one float bit-pattern (the
/// blend-out delta). The float is copied as raw bits, so the forward is
/// bit-exact. No return slot is written.
export!(cdecl, rw_00ba0f90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
