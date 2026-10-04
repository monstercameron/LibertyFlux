// original: 0x00b8c280 DISPLAY_TEXT_WITH_FLOAT
/// DISPLAY_TEXT_WITH_FLOAT: forward 5 script arguments to the engine implementation.
/// Float arguments are forwarded as raw bits (bit-exact by construction).
lf_rn101_rt::export!(cdecl, rw_fn_00b8c280(ctx: u32) -> u32 {
    let args = unsafe { *((ctx + 8) as *const u32) };
    let a0 = unsafe { *((args + 0) as *const u32) }; // float bits
    let a1 = unsafe { *((args + 4) as *const u32) }; // float bits
    let a2 = unsafe { *((args + 8) as *const u32) };
    let a3 = unsafe { *((args + 12) as *const u32) }; // float bits
    let a4 = unsafe { *((args + 16) as *const u32) };
    lf_rn101_rt::callee_cdecl!(1, u32, a0, a1, a2, a3, a4,)
});
