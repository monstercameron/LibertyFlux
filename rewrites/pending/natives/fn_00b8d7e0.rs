// original: 0x00b8d7e0 SET_MULTIPLAYER_HUD_TIME
/// Script native handler `SET_MULTIPLAYER_HUD_TIME`.
///
/// Forwards the time string to the HUD engine; no script return value.
lf_rn94_rt::export!(cdecl, rw_fn_00b8d7e0(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32);
        let a0 = *((args + 0) as *const u32);
        let answer = lf_rn94_rt::callee_cdecl!(1, u32, a0);
        answer
    }
});
