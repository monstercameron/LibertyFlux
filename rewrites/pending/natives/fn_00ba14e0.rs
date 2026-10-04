// original: 0x00ba14e0 SET_CHAR_DUCKING_TIMED
lf_rn22_rt::export!(cdecl,
    /// Script native `SET_CHAR_DUCKING_TIMED`.
    /// Forwards the character handle and the duration to the ped engine
    /// function. No script return value.
    rw_00ba14e0(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        // No script return: EAX keeps the engine answer.
        lf_rn22_rt::callee_cdecl!(1, u32, a0, a1)
    }
});
