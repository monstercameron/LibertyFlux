// original: 0x00DF3C90 UIFileViewer::clear_file_list_state

/// Clear the file-list activity byte and its current item word.
lf_checker_rt::export!(thiscall, rw_df3c90(this: u32) -> u32 {
    unsafe {
        ((this + 0x314) as *mut u8).write(0);
        ((this + 0x308) as *mut u32).write_unaligned(0);
        0
    }
});
