// original: 0x005e74e0 SET_USE_POOL_GAME_PHYSICS_SETTINGS
lf_rn22_rt::export!(cdecl,
    /// Script native `SET_USE_POOL_GAME_PHYSICS_SETTINGS`.
    /// Selects one of two physics-settings pairs by the script flag: a nonzero
    /// flag stores mode 4 and applies one constant, a zero flag stores mode 2
    /// and applies another. Each path makes the same two engine calls.
    rw_005e74e0(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let slot = lf_rn22_rt::global::<u32>(0x0103B694);
        if *args == 0 {
            *slot = 2;
            let _: u32 = lf_rn22_rt::callee_cdecl!(3, u32, 0);
            lf_rn22_rt::callee_cdecl!(4, u32, 0x3BE56042)
        } else {
            *slot = 4;
            let _: u32 = lf_rn22_rt::callee_cdecl!(1, u32, 1);
            lf_rn22_rt::callee_cdecl!(2, u32, 0x3A83126F)
        }
    }
});
