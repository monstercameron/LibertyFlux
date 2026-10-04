// original: 0x00bc82f0 TRAIN_LEAVE_STATION
/// Script native `TRAIN_LEAVE_STATION` (hash 0x37890B14).
///
/// Forwards one script argument (a train handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bc82f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
