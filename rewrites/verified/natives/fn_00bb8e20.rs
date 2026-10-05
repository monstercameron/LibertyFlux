// original: 0x00bb8e20 OPEN_SEQUENCE_TASK
/// Script native `OPEN_SEQUENCE_TASK` (hash 0x14A67125).
///
/// Forwards one script argument (a task-sequence slot index) to the engine.
/// No return slot is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00bb8e20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
