// original: 0x00bb9e90 TASK_LEAVE_ANY_CAR
use lf_k2_rt::{callee_cdecl, export};
/// Script native `TASK_LEAVE_ANY_CAR`.
/// Forwards the character handle to the task engine function. No
/// script return value.
export!(cdecl, rw_00bb9e90(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        // No script return: EAX keeps the engine answer.
        callee_cdecl!(1, u32, a0)
    }
});
