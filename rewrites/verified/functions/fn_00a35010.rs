// original: 0x00a35010 vehicle_deref_dispatch (proposed)

/// Dispatch through a pointer held behind one indirection, or answer 1.
///
/// Loads `p = *pp`; a null `p` answers 1 with no call. Otherwise tail-calls
/// the handler (id 1, one argument) with `p` and returns its answer.
/// Cdecl/1. Only AL is compared: the early path defines nothing else.
lf_checker_rt::export!(cdecl, rw_00a35010(pp: u32) -> u32 {
    unsafe {
        const HANDLER: u32 = 1;
        let p = core::ptr::read_unaligned(pp as *const u32);
        if p == 0 {
            1
        } else {
            lf_checker_rt::callee_cdecl!(HANDLER, u32, p)
        }
    }
});
