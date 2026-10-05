// original: 0x00AED3D0 kv_sort_with_ctx (proposed)

/// Sort a key/value range with a context word: forward four words.
///
/// Calls the partition-sort callee with (`a`, `b`, `c`, 0, `ctx`) and
/// returns its result. A thin adapter over the five-argument sort.
///
/// Original: 0x00AED3D0 (cdecl, four stack words, one direct callee).
lf_checker_rt::export!(cdecl, rw_00aed3d0(a: u32, b: u32, c: u32, ctx: u32) -> u32 {
    unsafe {
        const SORT_CALLEE: u32 = 1;
        lf_checker_rt::callee_cdecl!(SORT_CALLEE, u32, a, b, c, 0, ctx)
    }
});
