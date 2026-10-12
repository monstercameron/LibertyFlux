// original: 0x00DF68B0 UIFileViewer::vf1

/// Ask the file viewer for its first active child, then test that child.
///
/// If the child lookup returns null, return false. Otherwise query the
/// child's readiness through vtable slot `+0x1d4`; a zero answer returns
/// false. A ready child is queried through slot `+0x1c`, whose low byte is
/// returned. The method uses the thiscall convention.
lf_checker_rt::export!(thiscall, rw_df68b0(this: u32) -> u32 {
    let child = lf_checker_rt::callee_thiscall!(1, u32, this);
    if child == 0 { return 0; }

    let ready = lf_checker_rt::callee_thiscall!(2, u32, child);
    if ready == 0 { return 0; }

    lf_checker_rt::callee_thiscall!(3, u32, child) & 0xff
});
