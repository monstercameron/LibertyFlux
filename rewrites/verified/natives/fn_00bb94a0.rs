// original: 0x00bb94a0 TASK_COMBAT
/// Script native `TASK_COMBAT` (hash 0x1F157FD3).
///
/// Forwards two script arguments (two character handles) to the engine
/// to assign a combat task. No return slot is written.
export!(cdecl, rw_00bb94a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
