// original: 0x00bb96b0 TASK_DUCK
/// Script native `TASK_DUCK` (hash 0x72BF79F1).
///
/// Forwards two script arguments to the engine. No return slot is written.
export!(cdecl, rw_00bb96b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
