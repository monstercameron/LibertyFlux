// original: 0x00DF3C10 UIFileViewer::vf2

/// Destroy the file-viewer child state and optionally send a closing message.
///
/// The incoming flags word is tested for bit zero. The frame cleanup helper
/// always runs; the message helper runs only when that bit is set. The method
/// returns the viewer pointer.
lf_checker_rt::export!(thiscall, rw_df3c10(this: u32, flags: u32) -> u32 {
    let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
    if flags & 1 != 0 {
        let _: u32 = lf_checker_rt::callee_cdecl!(2, u32, this);
    }
    this
});
