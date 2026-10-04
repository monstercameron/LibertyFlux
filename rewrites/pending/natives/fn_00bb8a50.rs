// original: 0x00bb8a50 CLOSE_SEQUENCE_TASK
/// Script native `CLOSE_SEQUENCE_TASK` (hash 0x016C1B04).
///
/// Forwards one script argument (a task-sequence handle) to the engine. No
/// return slot is written; the engine answer is the exit value.
export!(cdecl, rw_00bb8a50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
