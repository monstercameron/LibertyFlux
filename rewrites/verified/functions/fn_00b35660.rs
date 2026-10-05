// original: 0x00B35660 heap_wrap_a

/// Forward `(a0, a1, a2, 0, 0)` to the five-argument heap callee.
///
/// Thin wrappers: the three incoming words plus two zero words. Cdecl, three
/// stack words, returns the callee's answer.
///
/// Originals: 0x00B35660 (callee 1), 0x00B35680 (callee 2).

lf_checker_rt::export!(cdecl, rw_00B35660(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const CALLEE: u32 = 1;
        lf_checker_rt::callee_cdecl!(CALLEE, u32, a0, a1, a2, 0, 0)
    }
});
