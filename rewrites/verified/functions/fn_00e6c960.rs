// original: 0x00e6c960 veh_dual_init_60 (proposed)

/// Initialize one fixed object, then look up one fixed key and return its answer.
///
/// `CALLEE1(this=THIS_PTR)` (thiscall/0) followed by `return CALLEE2(ARG2)`
/// (cdecl/1, caller cleans up with `(an instruction of the original)`). Takes no arguments.
///
/// Original: 0x00e6c960 (cdecl, no arguments, returns eax).
lf_checker_rt::export!(cdecl, rw_00e6c960() -> u32 {
    unsafe {
        const THIS_PTR: u32 = 0x017205a0;
        const ARG2: u32 = 0x00e72d30;
        lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(THIS_PTR));
        lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(ARG2))
    }
});
