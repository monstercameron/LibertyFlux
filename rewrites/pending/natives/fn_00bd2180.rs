// original: 0x00bd2180 EXTINGUISH_CHAR_FIRE
/// Put out a character's fire: forward the script argument (the
/// character handle) to the engine. No return value.
export!(cdecl, rw_00bd2180(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        callee_cdecl!(1, u32, *args);
        0
    }
});
