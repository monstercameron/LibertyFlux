// original: 0x00a00ef0 GET_PED_OBJECT_IS_ATTACHED_TO
lf_rn22_rt::export!(cdecl,
    /// Script native `GET_PED_OBJECT_IS_ATTACHED_TO`.
    /// Passes the ped handle to the ped engine function and stores the
    /// returned handle in the script return slot.
    rw_00a00ef0(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        let ret_slot = *ctx_words as *mut u32;
        let answer: u32 = lf_rn22_rt::callee_cdecl!(1, u32, a0);
        *ret_slot = answer;
        answer
    }
});
