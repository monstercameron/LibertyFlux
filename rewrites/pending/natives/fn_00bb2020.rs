// original: 0x00bb2020 GET_REMOTE_CONTROLLED_CAR
use lf_k2_rt::{callee_cdecl, export};
/// Script native `GET_REMOTE_CONTROLLED_CAR`.
/// Forwards two script arguments to the vehicle engine function. No
/// script return value at handler level.
export!(cdecl, rw_00bb2020(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        // No script return: EAX keeps the engine answer.
        callee_cdecl!(1, u32, a0, a1)
    }
});
