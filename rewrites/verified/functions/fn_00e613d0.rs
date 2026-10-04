// original: 0x00e613d0 init_then_submit_08
/// init_then_submit_08: run one initializer, then submit one stub.
///
/// Runs this pair's initializer, then hands this pair's code stub to the
/// shared submit routine. Returns the submit routine's answer; the
/// initializer's answer is discarded.
lf_checker_rt::export!(cdecl, rw_00e613d0() -> u32 {
    unsafe {
        lf_checker_rt::callee_cdecl!(1, u32,);
        lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(0x00E70080))
    }
});
