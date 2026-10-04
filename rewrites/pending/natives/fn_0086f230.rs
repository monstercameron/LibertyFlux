// original: 0x0086f230 SQRT
lf_rn22_rt::export!(cdecl,
    /// Script native `SQRT`.
    /// The only leaf in this batch: takes the single script float argument,
    /// computes its square root, and stores it in the script return slot.
    rw_0086f230(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const f32;
        let ret_slot = *ctx_words as *mut f32;
        *ret_slot = (*args).sqrt();
        ret_slot as u32
    }
});
