// original: 0x00d401e0 jump_query_nonnull (proposed)

/// Ask a helper whether this task object is active, as a 0/1 byte.
///
/// `this` is passed straight through to the helper callee (cdecl, one
/// argument); the result is 1 when the helper returns nonzero, else 0.
/// Only al carries the result; the upper bytes of eax are the helper's
/// leftovers on the original side and are not compared.
///
/// Original: 0x00d401e0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00d401e0(this: u32) -> u32 {
    unsafe {
        const HELPER: u32 = 1;
        let answer: u32 = lf_checker_rt::callee_cdecl!(HELPER, u32, this);
        (answer != 0) as u32
    }
});
