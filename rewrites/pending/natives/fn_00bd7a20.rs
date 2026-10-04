// original: 0x00bd7a20 GET_KILL_TRACKING_RESULTS
/// Script native `GET_KILL_TRACKING_RESULTS` (hash 0x095932D8).
///
/// Forwards two script arguments to the engine and stores all 32 bits of
/// its answer into the return slot.
export!(cdecl, rw_00bd7a20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
