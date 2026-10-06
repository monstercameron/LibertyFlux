// original: 0x00dcf7d0 ui_view_set_child (proposed)

/// Replace the view's child at `+0x230` with `child`, releasing the old one
/// unless it is already the new child or null.
///
/// `this` points to the view. When the old child differs from `child` and
/// is nonzero, the lookup callee (object in ECX plus five stack words: old
/// child, 0, two key pointers, 0; pops nothing) resolves it, the release
/// callee (stdcall, one word) converts the answer, and a nonzero release
/// answer is passed in ECX to the detach callee (thiscall, no stack words).
/// The new child is then stored unconditionally. No result.
///
/// Original: 0x00DCF7D0 (thiscall, one stack word, no result).
lf_checker_rt::export!(thiscall, rw_00dcf7d0(this: u32, child: u32) -> u32 {
    unsafe {
        /// Old-child lookup callee id.
        const LOOKUP: u32 = 1;
        /// Child-release callee id.
        const RELEASE: u32 = 2;
        /// Child-detach callee id.
        const DETACH: u32 = 3;
        const CHILD: u32 = 0x230;
        const OWNER: u32 = 0x1981a4c;
        const KEY_A: u32 = 0x114e384;
        const KEY_B: u32 = 0x1057b04;
        let old = ((this + CHILD) as *const u32).read_unaligned();
        if old != child && old != 0 {
            let found: u32 = lf_checker_rt::callee_thiscall!(
                LOOKUP,
                u32,
                lf_checker_rt::relocated(OWNER),
                old,
                0,
                lf_checker_rt::relocated(KEY_A),
                lf_checker_rt::relocated(KEY_B),
                0
            );
            let rel: u32 = lf_checker_rt::callee_stdcall!(RELEASE, u32, found);
            if rel != 0 {
                lf_checker_rt::callee_thiscall!(DETACH, u32, rel);
            }
        }
        ((this + CHILD) as *mut u32).write_unaligned(child);
        0
    }
});
