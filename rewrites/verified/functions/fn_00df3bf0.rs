// original: 0x00DF3BF0 UIFileViewer::UIFileViewer_2

/// Install the secondary file-viewer vtable, run its base initializer, and
/// forward control and the result to the frame handoff method.
lf_checker_rt::export!(thiscall, rw_df3bf0(this: u32) -> u32 {
    unsafe {
        const VIEWER_VTABLE: u32 = 0xF01F24;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VIEWER_VTABLE));
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
        lf_checker_rt::callee_thiscall!(2, u32, this)
    }
});
