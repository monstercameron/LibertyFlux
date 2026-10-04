// original: 0x00B9E690 COPY_SHARED_COMBAT_DECISION_MAKER
/// Native handler `COPY_SHARED_COMBAT_DECISION_MAKER`.
///
/// Forward source and destination decision-maker handles to the engine.
///
/// `ctx` is the native call context: word 0 points at the return
/// slot, word 2 points at the script argument array.
lf_k2_rt::export!(cdecl, rw_00b9e690(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        lf_k2_rt::callee_cdecl!(1, u32, *args.add(0), *args.add(1))
    }
});
