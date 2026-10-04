// original: 0x00ba15a0 SET_CHAR_HEADING
/// SET_CHAR_HEADING: forward 2 script arguments to the engine implementation.
/// Float arguments are forwarded as raw bits (bit-exact by construction).
lf_rn101_rt::export!(cdecl, rw_fn_00ba15a0(ctx: u32) -> u32 {
    let args = unsafe { *((ctx + 8) as *const u32) };
    let a0 = unsafe { *((args + 0) as *const u32) };
    let a1 = unsafe { *((args + 4) as *const u32) }; // float bits
    lf_rn101_rt::callee_cdecl!(1, u32, a0, a1,)
});
