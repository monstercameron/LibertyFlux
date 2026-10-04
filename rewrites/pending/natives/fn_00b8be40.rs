// original: 0x00b8be40 CHANGE_TERRITORY_BLIP_SCALE
/// Native handler `CHANGE_TERRITORY_BLIP_SCALE`.
///
/// Forward a blip id and two scale floats to the engine.
///
/// Floats pass through as raw words.
///
/// `ctx` is the native call context: word 0 points at the return
/// slot, word 2 points at the script argument array.
lf_rn14_rt::export!(cdecl, rw_00b8be40(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        lf_rn14_rt::callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2))
    }
});
