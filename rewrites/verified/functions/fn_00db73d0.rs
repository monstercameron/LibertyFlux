// original: 0x00DB73D0 UIFontString::vf119

/// Store two float words at `+0x200`/`+0x204` and, when the flag byte is
/// clear, notify the text backend (cdecl, four words: 2, 0, pointer, 0).
///
/// `thiscall` with three stack words (x bits, y bits, flag). The backend
/// pointer argument is heap-relative and identical on both sides in value
/// class only, so it is skipped and its first four words snapshotted. Void.
lf_checker_rt::export!(thiscall, rw_00db73d0(this_ptr: u32, x_bits: u32, y_bits: u32, flag: u32) -> u32 {
    unsafe {
        ((this_ptr + 0x200) as *mut u32).write_unaligned(x_bits);
        ((this_ptr + 0x204) as *mut u32).write_unaligned(y_bits);
    }
    if (flag as u8) == 0 {
        lf_checker_rt::callee_cdecl!(1, u32, 2, 0, this_ptr + 0x200, 0);
    }
    0
});
