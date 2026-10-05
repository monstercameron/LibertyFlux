// original: 0x00bc7d60 SET_TRAIN_IS_STOPPED_AT_STATION
/// Script native `SET_TRAIN_IS_STOPPED_AT_STATION` (hash 0x270C7AB3).
///
/// Forwards one script argument (a train handle) to the engine. No return
/// slot is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00bc7d60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
