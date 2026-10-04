// original: 0x00abc270 forward_with_two_zeros

/// Forward three arguments to the five-argument engine worker, padding zeros.
///
/// Pushes the three arguments plus two trailing zero words and returns the
/// worker answer (the callee is stubbed by the checker).
export!(cdecl, rs64_abc270(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe { callee_cdecl!(0, u32, a0, a1, a2, 0, 0) }
});
