// original: 0x00b1ee80 run_shutdown_sequence (proposed)

/// Runs the 11-step shutdown sequence.
///
/// Cdecl with no arguments. Calls the first step with a zero word, issues nine more direct calls with no arguments in fixed order and tail-calls the last step, returning its result.
lf_checker_rt::export!(cdecl, rw_00b1ee80() -> u32 {
    unsafe {
        lf_checker_rt::callee_cdecl!(1, u32, 0);
        lf_checker_rt::callee_cdecl!(2, u32,);
        lf_checker_rt::callee_cdecl!(3, u32,);
        lf_checker_rt::callee_cdecl!(4, u32,);
        lf_checker_rt::callee_cdecl!(5, u32,);
        lf_checker_rt::callee_cdecl!(6, u32,);
        lf_checker_rt::callee_cdecl!(7, u32,);
        lf_checker_rt::callee_cdecl!(8, u32,);
        lf_checker_rt::callee_cdecl!(9, u32,);
        lf_checker_rt::callee_cdecl!(10, u32,);
        lf_checker_rt::callee_cdecl!(11, u32,)
    }
});
