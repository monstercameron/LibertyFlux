// original: 0x00BA2920 SUPPRESS_PED_MODEL
/// Suppresses a ped model.
///
/// Forwards the single argument word to the engine function.
lf_rn26_rt::export!(cdecl, rw_00BA2920(ctx: *const u8) -> u32 {
    unsafe {
        let a = *(ctx.add(8) as *const *const u32);
        lf_rn26_rt::callee_cdecl!(1, u32, *a)
    }
});
