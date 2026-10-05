// original: 0x0093e200 heap_build_wrap (proposed)

/// Forward to the heap-build callee with two trailing zero words.
///
/// Calls the callee with (`a0`, `a1`, `a2`, 0, 0); the callee reads only
/// the first three. Returns the callee's answer.
///
/// Original: 0x0093e200 (cdecl, three stack words; one direct callee).
lf_checker_rt::export!(cdecl, rw_0093e200(a0: u32, a1: u32, a2: u32) -> u32 {
    const HEAP_BUILD: u32 = 1;
    unsafe { lf_checker_rt::callee_cdecl!(HEAP_BUILD, u32, a0, a1, a2, 0u32, 0u32) }
});
