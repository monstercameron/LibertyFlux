// original: 0x00b877f0 SET_CAR_FOV_START_SPEED
/// Script native `SET_CAR_FOV_START_SPEED` (hash 0x3CF41D47).
///
/// Forwards one float script argument to the engine as a raw bit-pattern.
/// No return slot is written.
export!(cdecl, rw_00b877f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
