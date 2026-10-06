// original: 0x00DB4E00 uimouse_set_mode

/// Store the cursor mode word of a mouse-cursor object.
///
/// `this` points to the object; `mode` is written to the word at `+0x1f8`
/// and also returned in `eax`, as the original leaves the loaded argument
/// in the return register.
///
/// Original: 0x00DB4E00 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00db4e00(this: u32, mode: u32) -> u32 {
    unsafe {
        const MODE: u32 = 0x1f8;
        ((this + MODE) as *mut u32).write_unaligned(mode);
        mode
    }
});
