// original: 0x00bb9450 TASK_CLEAR_LOOK_AT
/// Script native `TASK_CLEAR_LOOK_AT` (hash 0x05745ACA).
///
/// Forwards one script word (a character handle) to the engine. No return
/// slot is written.
export!(cdecl, rw_00bb9450(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
