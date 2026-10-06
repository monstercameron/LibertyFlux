// original: 0x00B54CB0 crmt_channel_open_b (proposed)

/// Resolve a channel handle, then open it with ten arguments.
///
/// Same shape as the sibling at 0x00B54C60 with a different resolver
/// (callee 1) and different constant placement: on a non-zero handle the
/// opener (callee 2, thiscall on the passed-through `this`) is called
/// with (`handle`, `a2`, `a3`, `a4`, -1, -1, `a0`, `a1`, -1, -1).
/// Returns zero when resolving fails, else the opener's answer.
///
/// Original: 0x00B54CB0 (thiscall, five stack words).
lf_checker_rt::export!(thiscall, rw_00b54cb0(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const RESOLVE: u32 = 1;
        const OPEN: u32 = 2;
        let handle: u32 = lf_checker_rt::callee_cdecl!(RESOLVE, u32, a0, a1);
        if handle == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(
            OPEN, u32, this, handle, a2, a3, a4, 0xffff_ffff, 0xffff_ffff, a0, a1,
            0xffff_ffff, 0xffff_ffff
        )
    }
});
