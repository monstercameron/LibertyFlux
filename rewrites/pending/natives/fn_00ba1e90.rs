// original: 0x00ba1e90 SET_DECISION_MAKER_ATTRIBUTE_MOVEMENT_STYLE
use lf_k2_rt::{callee_cdecl, export};
/// Script native `SET_DECISION_MAKER_ATTRIBUTE_MOVEMENT_STYLE`.
/// Forwards the decision-maker handle and the style id to the ped
/// engine function. No script return value.
export!(cdecl, rw_00ba1e90(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        // No script return: EAX keeps the engine answer.
        callee_cdecl!(1, u32, a0, a1)
    }
});
