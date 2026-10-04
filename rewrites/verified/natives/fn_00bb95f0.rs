// original: 0x00bb95f0 TASK_DIE
/// Script native `TASK_DIE` (hash 0x7EED364B).
///
/// Forwards one script argument (a character handle) to the engine task
/// system. No return slot is written.
export!(cdecl, rw_00bb95f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
