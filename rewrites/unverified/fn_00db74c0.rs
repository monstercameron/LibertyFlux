// original: 0x00DB74C0 UIFontString::vf120

/// When the flag byte is set, refresh through the parent slot (thiscall, no
/// stack args); when the text pointer is non-null, hand the buffer at
/// `+0x20E` to the copier (cdecl: buffer, text, 0x100) and force a NUL at
/// `+0x30D`.
///
/// `thiscall`, two stack words (text pointer, flag). Both calls are
/// conditional and independent. Void.
lf_checker_rt::export!(thiscall, rw_00db74c0(this_ptr: u32, text_ptr: u32, flag: u32) -> u32 {
    if (flag as u8) != 0 {
        lf_checker_rt::callee_thiscall!(1, u32, this_ptr);
    }
    if text_ptr != 0 {
        lf_checker_rt::callee_cdecl!(2, u32, this_ptr + 0x20E, text_ptr, 0x100);
        unsafe { ((this_ptr + 0x30D) as *mut u8).write(0) };
    }
    0
});
