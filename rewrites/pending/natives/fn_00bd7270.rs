// original: 0x00bd7270 SET_TIME_OF_DAY
use lf_k2_rt::{callee_cdecl, export};
/// Script native `SET_TIME_OF_DAY`.
/// Forwards the hour and minute to the clock engine function. No
/// script return value.
export!(cdecl, rw_00bd7270(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        // No script return: EAX keeps the engine answer.
        callee_cdecl!(1, u32, a0, a1)
    }
});
