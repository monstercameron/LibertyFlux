// original: 0x00bb95b0 TASK_COWER
/// Script native `TASK_COWER` (hash 0x29103E08).
///
/// Forwards one script argument (a ped handle) to the engine. No return slot is written.
export!(cdecl, rw_00bb95b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
