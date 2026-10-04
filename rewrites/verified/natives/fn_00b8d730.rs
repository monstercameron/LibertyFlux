// original: 0x00b8d730 SET_MENU_ITEM_WITH_NUMBER
use lf_k2_rt::{callee_cdecl, export};
/// Script native `SET_MENU_ITEM_WITH_NUMBER`.
/// Forwards five menu arguments to the frontend engine function. No
/// script return value.
export!(cdecl, rw_00b8d730(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let a2 = *args.add(2);
        let a3 = *args.add(3);
        let a4 = *args.add(4);
        // No script return: EAX keeps the engine answer.
        callee_cdecl!(1, u32, a0, a1, a2, a3, a4)
    }
});
