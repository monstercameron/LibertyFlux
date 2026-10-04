// original: 0x00e66f60 push_const_call_1 (proposed)
/// Call the shared cdecl/1 callee with one fixed image address argument.
///
/// Pushes the constant, calls, pops the word back (caller cleanup) and
/// returns the callee's result in eax. No arguments (cdecl/0).
/// Calling convention: cdecl.
lf_checker_rt::export!(cdecl, rw_00e66f60() -> u32 {
    unsafe {
        const ARG: u32 = 0x00E72160;
        lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(ARG))
    }
});
