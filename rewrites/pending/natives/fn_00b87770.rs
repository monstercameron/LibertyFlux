// original: 0x00b87770 SET_CAR_FOV_FADE_MULT
/// Script native `SET_CAR_FOV_FADE_MULT` (hash 0x5EEE6ADB).
///
/// Forwards one float bit-pattern (the field-of-view fade multiplier)
/// to the engine. No return slot is written.
export!(cdecl, rw_00b87770(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
