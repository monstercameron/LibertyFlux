// original: 0x00DF3B60 UIFileViewer::UIFileViewer

/// Construct the file viewer by initializing its frame base and local state.
///
/// The frame constructor receives the two incoming words, then this method
/// installs the viewer's vtable pointer and clears the listed child, cursor,
/// and file-list fields. It returns the constructed object pointer.
lf_checker_rt::export!(thiscall, rw_df3b60(this: u32, first_arg: u32, second_arg: u32) -> u32 {
    unsafe {
        const VIEWER_VTABLE: u32 = 0xF01F24;
        const CLEARED_WORDS: [u32; 10] = [
            0x1E8, 0x1E0, 0x1E4, 0x1EC, 0x1F0,
            0x1F4, 0x1F8, 0x1FC, 0x308, 0x310,
        ];
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, first_arg, second_arg);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VIEWER_VTABLE));
        for field_offset in CLEARED_WORDS {
            ((this + field_offset) as *mut u32).write_unaligned(0);
        }
        ((this + 0x314) as *mut u8).write(0);
        this
    }
});
