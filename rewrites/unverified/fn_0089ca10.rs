// original: 0x0089ca10 audio_hash_then_dispatch (proposed)

/// Hashes the first argument, then dispatches on the hash plus all four inputs.
///
/// Calls the hash helper (callee 1, cdecl/2) with (`a0`, 0), then the
/// dispatch routine (callee 2, cdecl/5) with the hash followed by the four
/// original arguments in order, returning the dispatch result.
///
/// Original: 0x0089ca10 (cdecl, four stack words).
lf_checker_rt::export!(cdecl, rw_0089ca10(a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const HASH: u32 = 1;
        const DISPATCH: u32 = 2;
        let h: u32 = lf_checker_rt::callee_cdecl!(HASH, u32, a0, 0);
        lf_checker_rt::callee_cdecl!(DISPATCH, u32, h, a0, a1, a2, a3)
    }
});
