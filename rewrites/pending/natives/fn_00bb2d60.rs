// original: 0x00bb2d60 SET_USE_LEG_IK
/// Native handler `SET_USE_LEG_IK`.
///
/// Forward a ped handle and a coerced flag to the engine.
///
/// `ctx` is the native call context: word 0 points at the return
/// slot, word 2 points at the script argument array.
lf_rn14_rt::export!(cdecl, rw_00bb2d60(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let flag1 = u32::from(*args.add(1) != 0);
        lf_rn14_rt::callee_cdecl!(1, u32, *args.add(0), (ctx & 0xFFFF_FF00) | flag1)
    }
});
