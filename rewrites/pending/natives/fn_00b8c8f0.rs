// original: 0x00b8c8f0 GET_HUD_COLOUR
/// Native handler `GET_HUD_COLOUR`.
///
/// Forward a colour id and four out-pointers to the engine.
///
/// The engine writes RGBA through the out-pointers; not observed under interception.
///
/// `ctx` is the native call context: word 0 points at the return
/// slot, word 2 points at the script argument array.
lf_rn14_rt::export!(cdecl, rw_00b8c8f0(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        lf_rn14_rt::callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2), *args.add(3), *args.add(4))
    }
});
