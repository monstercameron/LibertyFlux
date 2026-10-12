// original: 0x00DF7B30 UIGalleryFileViewer::vf108

/// Reset the gallery viewer and its first active child when present.
///
/// Resolve the first active child, call the viewer's state update with flag
/// zero, then call the child's `+0x1b0` and `+0x18` methods with flag zero
/// when the child exists. Finally call the viewer's `+0x24` method with zero.
/// The final callee leaves EAX in the void-style return path; EAX is not
/// compared.
lf_checker_rt::export!(thiscall, rw_df7b30(this: u32) -> u32 {
    let child = lf_checker_rt::callee_thiscall!(1, u32, this);
    let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, this, 0u32);

    if child != 0 {
        let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, child);
        let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, child, 0u32);
    }

    lf_checker_rt::callee_thiscall!(5, u32, this, 0u32)
});
