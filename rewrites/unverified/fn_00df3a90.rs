// original: 0x00DF3A90 UITransitionViewer::vf0

/// Return the class-name hash through the shared string-hash routine.
///
/// The function passes the class-name string in read-only image data to a
/// cdecl helper and returns that helper's 32-bit answer unchanged.
lf_checker_rt::export!(cdecl, rw_df3a90() -> u32 {
    unsafe {
        lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(0xF01A1C))
    }
});
