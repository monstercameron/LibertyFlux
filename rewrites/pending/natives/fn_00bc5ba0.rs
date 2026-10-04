// original: 0x00bc5ba0 GET_CAR_MODEL
/// Native handler `GET_CAR_MODEL`.
///
/// Forward a vehicle handle and an out-pointer to the engine.
///
/// The engine writes the model through the out-pointer; stubbed callees have no memory effects so that write is not observed.
///
/// `ctx` is the native call context: word 0 points at the return
/// slot, word 2 points at the script argument array.
lf_rn14_rt::export!(cdecl, rw_00bc5ba0(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        lf_rn14_rt::callee_cdecl!(1, u32, *args.add(0), *args.add(1))
    }
});
