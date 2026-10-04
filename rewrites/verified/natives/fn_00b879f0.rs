// original: 0x00b879f0 SET_FOLLOW_VEHICLE_PITCH_LIMIT_UP
/// Script native `SET_FOLLOW_VEHICLE_PITCH_LIMIT_UP` (hash 0x5567728E).
///
/// Forwards one float script argument (an angle limit) to the engine as raw bits, so the forward is bit-exact.
/// No return slot is written.
export!(cdecl, rw_00b879f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args,)
    }
});
