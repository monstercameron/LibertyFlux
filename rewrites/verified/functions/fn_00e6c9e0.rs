// original: 0x00e6c9e0 veh_lookup_call_e0 (proposed)

/// Look up one fixed vehicle key: `return CALLEE(ARG)`.
///
/// The original pushes a constant address, calls the shared lookup routine
/// (cdecl, caller cleans up with `(an instruction of the original)`) and returns its answer. Takes no
/// arguments.
///
/// Original: 0x00e6c9e0 (cdecl, no arguments, returns eax).
lf_checker_rt::export!(cdecl, rw_00e6c9e0() -> u32 {
    unsafe {
        const ARG: u32 = 0x00e72d80;
        lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(ARG))
    }
});
