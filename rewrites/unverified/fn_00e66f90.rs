// original: 0x00e66f90 push_const_call_4 (proposed)
/// Call the shared cdecl/1 callee with one fixed image address argument.
///
/// Same shape as 0x00e66f60 with a different constant. Returns the callee's
/// result. No arguments (cdecl/0). Calling convention: cdecl.
lf_checker_rt::export!(cdecl, rw_00e66f90() -> u32 {
    unsafe {
        const ARG: u32 = 0x00E72190;
        lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(ARG))
    }
});
