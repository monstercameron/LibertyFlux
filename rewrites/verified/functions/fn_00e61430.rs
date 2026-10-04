// original: 0x00e61430 init_then_submit_11
/// init_then_submit_11: run one initializer, then submit one stub.
///
/// Runs this pair's initializer, then hands this pair's code stub to the
/// shared submit routine. Returns the submit routine's answer; the
/// initializer's answer is discarded.
lf_checker_rt::export!(cdecl, rw_00e61430() -> u32 {
    unsafe {
        lf_checker_rt::callee_cdecl!(1, u32,);
        lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(0x00E700B0))
    }
});
