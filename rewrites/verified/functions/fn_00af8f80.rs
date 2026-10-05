// original: 0x00AF8F80 veh_init_forward (proposed)

/// Run the first initialisation step, then tail-jump into the second.
///
/// Calls callee 1, discards its result, and forwards to callee 2 as a tail
/// call, returning callee 2's result to the caller.
///
/// Original: 0x00AF8F80 (cdecl, no arguments, result in EAX).
lf_checker_rt::export!(cdecl, rw_00AF8F80() -> u32 {
    unsafe {
        const FIRST: u32 = 1;
        const SECOND: u32 = 2;
        lf_checker_rt::callee_cdecl!(FIRST, u32,);
        lf_checker_rt::callee_cdecl!(SECOND, u32,)
    }
});
