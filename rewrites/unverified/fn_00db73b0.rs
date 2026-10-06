// original: 0x00DB73B0 UIFontString::vf128

/// Store the low byte of the argument into the flag at `+209`.
///
/// `thiscall`, one stack word. Leaf: no calls, no globals. Void.
lf_checker_rt::export!(thiscall, rw_00db73b0(this_ptr: u32, value: u32) -> u32 {
    const FLAG: u32 = 0x209;
    unsafe { ((this_ptr + FLAG) as *mut u8).write(value as u8) };
    0
});
