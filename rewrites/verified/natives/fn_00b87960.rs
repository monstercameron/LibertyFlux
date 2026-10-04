// original: 0x00b87960 SET_FOLLOW_PED_PITCH_LIMIT_UP
/// Script native `SET_FOLLOW_PED_PITCH_LIMIT_UP` (hash 0x360E2977).
///
/// Forwards one script argument, a float bit-pattern (pitch limit), to the engine. No return slot is written.
export!(cdecl, rw_00b87960(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
