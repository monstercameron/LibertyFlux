// original: 0x00bb6600 REGISTER_NETWORK_BEST_GAME_SCORES
/// REGISTER_NETWORK_BEST_GAME_SCORES: Registers the network best-game scores; forward 3 script arguments to the engine implementation.
lf_rn109_rt::export!(cdecl, rw_fn_00bb6600(ctx: u32) -> u32 {
    let args = unsafe { *((ctx + 8) as *const u32) };
    let a0 = unsafe { *((args + 0) as *const u32) };
    let a1 = unsafe { *((args + 4) as *const u32) };
    let a2 = unsafe { *((args + 8) as *const u32) };
    lf_rn109_rt::callee_cdecl!(1, u32, a0, a1, a2,)
});
