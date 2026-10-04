// original: 0x00bd8760 NETWORK_IS_FIND_RESULT_UPDATED
lf_rn22_rt::export!(cdecl,
    /// Script native `NETWORK_IS_FIND_RESULT_UPDATED`.
    /// Passes the find index to the network engine function and writes its
    /// low result byte to the script return slot.
    rw_00bd8760(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        let ret_slot = *ctx_words as *mut u32;
        let answer: u32 = lf_rn22_rt::callee_cdecl!(1, u32, a0);
        // The handler keeps only the low byte (movzx).
        *ret_slot = answer & 0xFF;
        ret_slot as u32
    }
});
