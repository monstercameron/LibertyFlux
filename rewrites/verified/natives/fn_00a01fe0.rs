// original: 0x00a01fe0 SET_PLAYERS_DROP_MONEY_IN_NETWORK_GAME
/// SET_PLAYERS_DROP_MONEY_IN_NETWORK_GAME: forward 1 script argument to the engine implementation.
/// Argument 0 is coerced to 0/1; the pushed word keeps the context
/// pointer's high bytes (the original reuses its incoming stack slot).
lf_rn101_rt::export!(cdecl, rw_fn_00a01fe0(ctx: u32) -> u32 {
    let args = unsafe { *((ctx + 8) as *const u32) };
    let a0 = unsafe { *((args + 0) as *const u32) };
    let q0 = (ctx & 0xffff_ff00) | u32::from(a0 != 0);
    lf_rn101_rt::callee_cdecl!(1, u32, q0,)
});
