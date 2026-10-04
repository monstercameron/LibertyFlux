// original: 0x00E60960 critsec_init_pair_a (proposed)

/// Critical-section initialisation plus callback registration.
///
/// Behaviour: calls the imported `InitializeCriticalSection` through its
/// import slot with the section address `SECTION`, then calls the cdecl
/// registrar with the callback address `CALLBACK`, and returns the
/// registrar's answer. The import is invoked exactly as the original
/// does: loaded from its slot and called with one stack argument.
/// Call order is fixed; no memory is read or written besides the calls.
///
/// Original: no stack arguments; stdcall import (callee cleans its one
/// argument). Return value is the registrar's answer.
lf_checker_rt::export!(cdecl, rw_00e60960() -> u32 {
    unsafe {
        let slot = lf_checker_rt::relocated(0x00E731C4) as *const u32;
        let init: extern "stdcall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(slot.read() as usize) };
        init(lf_checker_rt::relocated(0x0019F3A40));
        lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(0x00E6FB20))
    }
});

