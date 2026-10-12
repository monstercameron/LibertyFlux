// original: 0x00C0F9E0 UILayoutFrame::vf72
/// Store the low byte of the stack argument in the byte field at
/// `this + 0xc5`. The method takes `this` in ECX, has one callee-cleaned
/// stack word, and does not return a value used by its callers.
lf_checker_rt::export!(thiscall, rw_c0f9e0(this: u32, value: u32) -> u32 {
    unsafe {
        const VALUE_BYTE: u32 = 0x0c5;
        (this.wrapping_add(VALUE_BYTE) as *mut u8).write(value as u8);
        0
    }
});
