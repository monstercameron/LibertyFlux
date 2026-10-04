// original: 0x00bb8a40 CLEAR_SEQUENCE_TASK
use lf_k2_rt::{callee_cdecl, export};
/// Script native `CLEAR_SEQUENCE_TASK`.
/// Forwards the sequence id to the task engine function. No script
/// return value.
export!(cdecl, rw_00bb8a40(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        // No script return: EAX keeps the engine answer.
        callee_cdecl!(1, u32, a0)
    }
});
