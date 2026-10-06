// original: 0x00DB74B0 UIFontString::vf125

/// Store the low byte of the argument into the flag at `+20D`.
///
/// `thiscall`, one stack word. Leaf: no calls, no globals. Void.
lf_checker_rt::export!(thiscall, rw_00db74b0(this_ptr: u32, value: u32) -> u32 {
    const FLAG: u32 = 0x20D;
    unsafe { ((this_ptr + FLAG) as *mut u8).write(value as u8) };
    0
});
