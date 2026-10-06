// original: 0x00B54D00 crmt_channel_open_c (proposed)

/// Resolve a channel handle with a fallback key, then open it.
///
/// Calls the resolver (callee 1, cdecl) with (`a0`, `a1`). When that
/// answers zero and `a5` is not -1, retries as callee 2 with (`a5`,
/// `a1`); when that also answers zero, or `a5` was -1, returns zero.
/// Otherwise the opener (callee 3, thiscall on the passed-through `this`)
/// runs with (`handle`, `a2`, `a3`, `a4`, `key`, `a1`, 0, 0, -1, -1)
/// where `key` is whichever of `a0`/`a5` resolved, returning its answer.
///
/// Original: 0x00B54D00 (thiscall, six stack words).
lf_checker_rt::export!(thiscall, rw_00b54d00(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    unsafe {
        const RESOLVE1: u32 = 1;
        const RESOLVE2: u32 = 2;
        const OPEN: u32 = 3;
        const NONE: u32 = 0xffff_ffff;
        let mut key = a0;
        let mut handle: u32 = lf_checker_rt::callee_cdecl!(RESOLVE1, u32, a0, a1);
        if handle == 0 {
            if a5 == NONE {
                return 0;
            }
            key = a5;
            handle = lf_checker_rt::callee_cdecl!(RESOLVE2, u32, a5, a1);
            if handle == 0 {
                return 0;
            }
        }
        lf_checker_rt::callee_thiscall!(
            OPEN, u32, this, handle, a2, a3, a4, key, a1, 0, 0, NONE, NONE
        )
    }
});
