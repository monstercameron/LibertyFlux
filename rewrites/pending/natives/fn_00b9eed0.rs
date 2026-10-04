// original: 0x00b9eed0 GET_CHAR_MAX_MOVE_BLEND_RATIO
lf_rn22_rt::export!(cdecl,
    /// Script native `GET_CHAR_MAX_MOVE_BLEND_RATIO`.
    /// Passes the character handle to the ped engine function and stores
    /// the single-precision result it returns to the script return slot.
    rw_00b9eed0(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        let ret_slot = *ctx_words as *mut f32;
        // The engine answer arrives as a float and is stored verbatim.
        *ret_slot = lf_rn22_rt::callee_cdecl!(1, f32, a0);
        ret_slot as u32
    }
});
