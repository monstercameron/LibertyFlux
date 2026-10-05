// original: 0x00b1e610 run_subinit_sequence (proposed)

/// Runs the 26-step sub-initialisation sequence.
///
/// Cdecl with no arguments. Issues 25 direct calls with no arguments in fixed order and tail-calls the last step, returning its result.
lf_checker_rt::export!(cdecl, rw_00b1e610() -> u32 {
    unsafe {
        lf_checker_rt::callee_cdecl!(1, u32,);
        lf_checker_rt::callee_cdecl!(2, u32,);
        lf_checker_rt::callee_cdecl!(3, u32,);
        lf_checker_rt::callee_cdecl!(4, u32,);
        lf_checker_rt::callee_cdecl!(5, u32,);
        lf_checker_rt::callee_cdecl!(6, u32,);
        lf_checker_rt::callee_cdecl!(7, u32,);
        lf_checker_rt::callee_cdecl!(8, u32,);
        lf_checker_rt::callee_cdecl!(9, u32,);
        lf_checker_rt::callee_cdecl!(10, u32,);
        lf_checker_rt::callee_cdecl!(11, u32,);
        lf_checker_rt::callee_cdecl!(12, u32,);
        lf_checker_rt::callee_cdecl!(13, u32,);
        lf_checker_rt::callee_cdecl!(14, u32,);
        lf_checker_rt::callee_cdecl!(15, u32,);
        lf_checker_rt::callee_cdecl!(16, u32,);
        lf_checker_rt::callee_cdecl!(17, u32,);
        lf_checker_rt::callee_cdecl!(18, u32,);
        lf_checker_rt::callee_cdecl!(19, u32,);
        lf_checker_rt::callee_cdecl!(20, u32,);
        lf_checker_rt::callee_cdecl!(21, u32,);
        lf_checker_rt::callee_cdecl!(22, u32,);
        lf_checker_rt::callee_cdecl!(23, u32,);
        lf_checker_rt::callee_cdecl!(24, u32,);
        lf_checker_rt::callee_cdecl!(25, u32,);
        lf_checker_rt::callee_cdecl!(26, u32,)
    }
});
