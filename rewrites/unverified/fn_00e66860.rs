// original: 0x00E66860 zero_state_5_and_register (proposed)

/// Zero a small global state block, then register its handler routine.
///
/// Writes 0 to 5 consecutive dwords at the state base, then passes
/// the handler routine's code address to the registrar callee (cdecl,
/// one argument). No arguments; returns the registrar's answer (cdecl).
lf_checker_rt::export!(cdecl, rw_00e66860() -> u32 {
    unsafe {
        const ROUTINE: u32 = 0x00E71FD0;
        const ZERO_BASE: u32 = 0x01295780;
        const NWORDS: u32 = 5;
        const REGISTRAR_CALLEE: u32 = 1;
        let z = lf_checker_rt::relocated(ZERO_BASE);
        let mut k = 0u32;
        while k != NWORDS {
            (z.wrapping_add(k.wrapping_mul(4)) as *mut u32).write_unaligned(0);
            k += 1;
        }
        lf_checker_rt::callee_cdecl!(REGISTRAR_CALLEE, u32, lf_checker_rt::relocated(ROUTINE))
    }
});
