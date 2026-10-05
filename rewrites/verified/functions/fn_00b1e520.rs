// original: 0x00b1e520 run_init_sequence (proposed)

/// Runs the 30-step subsystem initialisation sequence.
///
/// Cdecl with no arguments. Issues 29 direct calls in fixed order (two carry an object in ECX, one takes two string pointers, one takes five words) and tail-calls the last step, returning its result.
lf_checker_rt::export!(cdecl, rw_00b1e520() -> u32 {
    unsafe {
        lf_checker_rt::callee_cdecl!(1, u32,);
        lf_checker_rt::callee_cdecl!(2, u32,);
        lf_checker_rt::callee_cdecl!(3, u32,);
        lf_checker_rt::callee_thiscall!(4, u32, lf_checker_rt::relocated(0x015F8B48));
        lf_checker_rt::callee_cdecl!(5, u32,);
        lf_checker_rt::callee_cdecl!(6, u32,);
        lf_checker_rt::callee_cdecl!(3, u32,);
        lf_checker_rt::callee_cdecl!(7, u32,);
        lf_checker_rt::callee_cdecl!(8, u32,);
        lf_checker_rt::callee_cdecl!(3, u32,);
        lf_checker_rt::callee_cdecl!(9, u32,);
        lf_checker_rt::callee_cdecl!(10, u32,);
        lf_checker_rt::callee_cdecl!(11, u32,);
        lf_checker_rt::callee_cdecl!(3, u32,);
        lf_checker_rt::callee_thiscall!(12, u32, lf_checker_rt::relocated(0x01177A80));
        lf_checker_rt::callee_cdecl!(13, u32, lf_checker_rt::relocated(0xEABB70), lf_checker_rt::relocated(0xEABB68));
        lf_checker_rt::callee_cdecl!(14, u32,);
        lf_checker_rt::callee_cdecl!(3, u32,);
        lf_checker_rt::callee_cdecl!(15, u32,);
        lf_checker_rt::callee_cdecl!(16, u32, 2, 0, 0, 0, 0);
        lf_checker_rt::callee_cdecl!(17, u32,);
        lf_checker_rt::callee_cdecl!(3, u32,);
        lf_checker_rt::callee_cdecl!(18, u32,);
        lf_checker_rt::callee_cdecl!(19, u32,);
        lf_checker_rt::callee_cdecl!(3, u32,);
        lf_checker_rt::callee_cdecl!(20, u32,);
        lf_checker_rt::callee_cdecl!(21, u32,);
        lf_checker_rt::callee_cdecl!(3, u32,);
        lf_checker_rt::callee_cdecl!(17, u32,);
        lf_checker_rt::callee_cdecl!(22, u32,)
    }
});
