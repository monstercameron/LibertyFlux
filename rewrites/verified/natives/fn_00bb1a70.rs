// original: 0x00bb1a70 ADD_SCORE
use lf_k2_rt::{callee_cdecl, export};
/// Add an amount to a player's money: forward both script arguments
/// (player index, amount) to the engine. No return value.
export!(cdecl, rw_00bb1a70(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1));
        0
    }
});
