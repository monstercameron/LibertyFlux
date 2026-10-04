// original: 0x00ba02c0 KNOCK_PED_OFF_BIKE
/// Native handler `KNOCK_PED_OFF_BIKE`.
///
/// Forward a ped handle to the engine.
///
/// `ctx` is the native call context: word 0 points at the return
/// slot, word 2 points at the script argument array.
lf_rn14_rt::export!(cdecl, rw_00ba02c0(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        lf_rn14_rt::callee_cdecl!(1, u32, *args.add(0))
    }
});
