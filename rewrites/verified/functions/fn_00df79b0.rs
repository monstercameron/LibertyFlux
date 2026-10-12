// original: 0x00DF79B0 UIGalleryFileViewer::vf107

/// Refresh the gallery viewer and, when present, its first active child.
///
/// Resolve the first active child, call the viewer's state update with flag
/// one, then call two child methods with the same flag when a child exists.
/// Finally call the viewer's `+0x24` and `+0x18` methods with flag one. The
/// final callee leaves a value in EAX, but this is a void-style method and
/// that register is not compared.
lf_checker_rt::export!(thiscall, rw_df79b0(this: u32) -> u32 {
    let child = lf_checker_rt::callee_thiscall!(1, u32, this);
    let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, this, 1u32);

    if child != 0 {
        let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, child);
        let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, child, 1u32);
    }

    let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, this, 1u32);
    lf_checker_rt::callee_thiscall!(6, u32, this, 1u32)
});
