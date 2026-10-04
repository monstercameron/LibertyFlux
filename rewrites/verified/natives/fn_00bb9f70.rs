// original: 0x00bb9f70 TASK_LEAVE_GROUP
/// Script native `TASK_LEAVE_GROUP` (hash 0x1905109F).
///
/// Forwards one script argument (a character handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bb9f70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});

