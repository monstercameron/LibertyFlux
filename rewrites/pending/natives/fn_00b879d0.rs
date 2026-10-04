// original: 0x00b879d0 SET_FOLLOW_VEHICLE_PITCH_LIMIT_DOWN
/// Script native `SET_FOLLOW_VEHICLE_PITCH_LIMIT_DOWN` (hash 0x02F65CB2).
///
/// Forwards one float script argument (the pitch limit) to the engine as raw bits. Writes no return slot.
export!(cdecl, rw_00b879d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args.add(0))
    }
});
