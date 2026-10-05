// original: 0x0093e220 ptr_sort_wrap (proposed)

/// Forward to the pointer-sort callee with a zero fourth word.
///
/// Calls the callee with (`a0`, `a1`, `a2`, 0, `a3`); the callee's sixth
/// input word is whatever the caller left above the arguments. Returns
/// the callee's answer.
///
/// Original: 0x0093e220 (cdecl, four stack words; one direct callee).
lf_checker_rt::export!(cdecl, rw_0093e220(a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    const PTR_SORT: u32 = 1;
    unsafe { lf_checker_rt::callee_cdecl!(PTR_SORT, u32, a0, a1, a2, 0u32, a3) }
});
