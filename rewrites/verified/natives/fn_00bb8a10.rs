// original: 0x00bb8a10 CLEAR_CHAR_TASKS_IMMEDIATELY
/// Script native handler `CLEAR_CHAR_TASKS_IMMEDIATELY` (hash 0x3C116620).
///
/// Forwards script argument 0 to the engine worker and returns its answer.
export!(cdecl, rw_00bb8a10(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let answer: u32 = callee_cdecl!(1, u32, *args);
        answer
    }
});
