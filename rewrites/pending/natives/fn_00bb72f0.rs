// original: 0x00bb72f0 ATTACH_ANIMS_TO_MODEL
lf_rn22_rt::export!(cdecl,
    /// Script native `ATTACH_ANIMS_TO_MODEL`.
    /// Forwards the model handle and the animation-set id to the streaming
    /// engine function. No script return value.
    rw_00bb72f0(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        // No script return: EAX keeps the engine answer.
        lf_rn22_rt::callee_cdecl!(1, u32, a0, a1)
    }
});
