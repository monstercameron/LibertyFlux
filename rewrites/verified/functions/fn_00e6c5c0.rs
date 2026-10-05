// original: 0x00e6c5c0 veh_init_thunk_c0 (proposed)

/// Tail-forward to a shared vehicle initializer with a fixed object: `return CALLEE(this=THIS_PTR)`.
///
/// The original is two instructions (`(an instruction of the original); jmp CALLEE`): it takes
/// no arguments of its own and returns whatever the shared routine returns.
/// The rewrite makes the same intercepted thiscall with the same object
/// pointer and returns its answer.
///
/// Original: 0x00e6c5c0 (cdecl shape, no arguments, tail jump; returns eax).
lf_checker_rt::export!(cdecl, rw_00e6c5c0() -> u32 {
    unsafe {
        const THIS_PTR: u32 = 0x0171cdd0;
        lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(THIS_PTR))
    }
});
