// original: 0x009CBFB0 ON_FIRE_SCREAM
/// Makes a character scream while on fire.
///
/// Forwards the single argument word to the engine function.
lf_rn26_rt::export!(cdecl, rw_009CBFB0(ctx: *const u8) -> u32 {
    unsafe {
        let a = *(ctx.add(8) as *const *const u32);
        lf_rn26_rt::callee_cdecl!(1, u32, *a)
    }
});
