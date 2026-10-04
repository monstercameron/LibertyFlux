// original: 0x00bba080 TASK_PAUSE
/// Script native `TASK_PAUSE` (hash 0x5E702E2C).
///
/// Forwards two script arguments to the engine. No return slot is written.
export!(cdecl, rw_00bba080(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
