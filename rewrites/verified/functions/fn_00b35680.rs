// original: 0x00B35680 heap_wrap_b

/// Forward `(a0, a1, a2, 0, 0)` to the second five-argument heap callee.
///
/// Twin of `rw_00B35660` calling the neighbouring callee.
///
/// Original: 0x00B35680.

lf_checker_rt::export!(cdecl, rw_00B35680(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const CALLEE: u32 = 1;
        lf_checker_rt::callee_cdecl!(CALLEE, u32, a0, a1, a2, 0, 0)
    }
});
