// original: 0x00bc7d20 SET_TRAIN_CRUISE_SPEED
/// Script native `SET_TRAIN_CRUISE_SPEED` (hash 0x02E93A3E).
///
/// Forwards a train handle and one float bit-pattern (cruise speed) to
/// the engine. No return slot is written.
export!(cdecl, rw_00bc7d20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
