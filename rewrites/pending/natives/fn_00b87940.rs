// original: 0x00b87940 SET_FOLLOW_PED_PITCH_LIMIT_DOWN
/// Script native `SET_FOLLOW_PED_PITCH_LIMIT_DOWN` (hash 0x31DB4020).
///
/// Forwards one float bit-pattern (the pitch limit) to the engine. No
/// return slot is written.
export!(cdecl, rw_00b87940(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
        )
    }
});
