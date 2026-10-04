// original: 0x00E60A70 timer_callback_register_a (proposed)

/// Callback registration: hands one callback address to the registrar.
///
/// Behaviour: calls the cdecl registrar once with the callback address
/// `CALLBACK` and returns its answer. Nothing else is read or written.
///
/// Original: cdecl, no stack arguments (`push`/`call`/`pop` balanced).
/// Return value is the registrar's answer.
lf_checker_rt::export!(cdecl, rw_00e60a70() -> u32 {
    unsafe { lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(0x00E6FB70)) }
});

