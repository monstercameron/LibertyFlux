// original: 0x00e60e20 submit_stub_00
/// submit_stub_00: submit one stub with no initializer.
///
/// Hands this stub to the shared submit routine with no prior initializer.
/// Returns the submit routine's answer.
lf_checker_rt::export!(cdecl, rw_00e60e20() -> u32 {
    unsafe {
        lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(0x00E6FD10))
    }
});
