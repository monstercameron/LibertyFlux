// original: 0x00E608E0 timer_init_pair_scratch (proposed)

/// Subsystem initialisation that reserves scratch space, stamps a global,
/// then registers a callback.
///
/// Behaviour: reserves 12 bytes of stack scratch (restored by the first
/// callee, which pops 12 bytes although none were pushed for it), calls
/// callee 1 (no usable arguments, return ignored), writes the constant
/// `STAMP` to the global `FLAG_CELL`, then calls the cdecl registrar
/// with the callback address `CALLBACK`, and returns its answer.
/// Call order and the single global write are the whole behaviour.
///
/// Original: cdecl, no stack arguments. The 12 scratch bytes are never
/// read; only the stack-pointer round-trip matters, which the stub
/// reproduces on both sides.
lf_checker_rt::export!(cdecl, rw_00e608e0() -> u32 {
    unsafe {
        lf_checker_rt::callee_stdcall!(1, u32, 0, 0, 0);
        (lf_checker_rt::relocated(0x0019F3984) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(0x00FE4B68));
        lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(0x00E6FAE0))
    }
});

