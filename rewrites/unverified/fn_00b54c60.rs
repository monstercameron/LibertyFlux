// original: 0x00B54C60 crmt_channel_open_a (proposed)

/// Resolve a channel handle, then open it with ten arguments.
///
/// Calls the resolver (callee 1, cdecl) with (`a0`, `a1`) and returns
/// zero when it answers zero. Otherwise forwards to the opener (callee 2,
/// thiscall on `this`, which is only passed through) with the handle, the
/// remaining keys, the float bits of `a4`, four constant words and the
/// original keys: (`handle`, `a2`, `a3`, `a4`, -1, -1, 0, 0, `a0`, `a1`),
/// returning the opener's answer.
///
/// Original: 0x00B54C60 (thiscall, five stack words).
lf_checker_rt::export!(thiscall, rw_00b54c60(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const RESOLVE: u32 = 1;
        const OPEN: u32 = 2;
        let handle: u32 = lf_checker_rt::callee_cdecl!(RESOLVE, u32, a0, a1);
        if handle == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(
            OPEN, u32, this, handle, a2, a3, a4, 0xffff_ffff, 0xffff_ffff, 0, 0, a0, a1
        )
    }
});
