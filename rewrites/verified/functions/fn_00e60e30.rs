// original: 0x00e60e30 submit_this_init
/// submit_this_init: initialize one object, then submit one stub.
///
/// Runs the object initializer with ECX pointing at its static
/// instance, then hands this pair's code stub to the shared submit
/// routine. Returns the submit routine's answer.
lf_checker_rt::export!(cdecl, rw_00e60e30() -> u32 {
    unsafe {
        lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(0x019FBAF0));
        lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(0x00E6FD30))
    }
});
