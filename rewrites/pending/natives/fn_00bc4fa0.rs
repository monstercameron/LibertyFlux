// original: 0x00bc4fa0 ACTIVATE_HELI_SPEED_CHEAT
lf_rn22_rt::export!(cdecl,
    /// Script native `ACTIVATE_HELI_SPEED_CHEAT`.
    /// Forwards two script arguments (cheat id, enable flag) to the cheat
    /// engine function and returns nothing to the script.
    rw_00bc4fa0(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        // No script return: EAX keeps the engine answer.
        lf_rn22_rt::callee_cdecl!(1, u32, a0, a1)
    }
});
