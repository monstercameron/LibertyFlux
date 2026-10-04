// original: 0x00ba0cb0 REMOVE_ADDITIONAL_POPULATION_MODEL
lf_rn22_rt::export!(cdecl,
    /// Script native `REMOVE_ADDITIONAL_POPULATION_MODEL`.
    /// Forwards the model id to the population engine function. No script
    /// return value.
    rw_00ba0cb0(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        // No script return: EAX keeps the engine answer.
        lf_rn22_rt::callee_cdecl!(1, u32, a0)
    }
});
