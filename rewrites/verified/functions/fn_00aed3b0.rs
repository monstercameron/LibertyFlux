// original: 0x00AED3B0 kv_sort_forward (proposed)

/// Sort a key/value range forward: forward three words and two zeros.
///
/// Calls the heap-pass callee with (`a`, `b`, `c`, 0, 0) and returns its
/// result. A thin adapter over the five-argument sort pass.
///
/// Original: 0x00AED3B0 (cdecl, three stack words, one direct callee).
lf_checker_rt::export!(cdecl, rw_00aed3b0(a: u32, b: u32, c: u32) -> u32 {
    unsafe {
        const SORT_CALLEE: u32 = 1;
        lf_checker_rt::callee_cdecl!(SORT_CALLEE, u32, a, b, c, 0, 0)
    }
});
