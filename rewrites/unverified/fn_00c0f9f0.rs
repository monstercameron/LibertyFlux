// original: 0x00C0F9F0 UILayoutFrame::vf37
/// Replace the float bits at `this + 0xa4` and reset the two following
/// dword fields to zero. The method takes `this` in ECX and cleans one stack
/// word containing the float's raw bits.
lf_checker_rt::export!(thiscall, rw_c0f9f0(this: u32, value_bits: u32) -> u32 {
    unsafe {
        const VALUE: u32 = 0x0a4;
        const FIRST_RESET: u32 = 0x0a8;
        const SECOND_RESET: u32 = 0x0ac;
        (this.wrapping_add(VALUE) as *mut u32).write_unaligned(value_bits);
        (this.wrapping_add(FIRST_RESET) as *mut u32).write_unaligned(0);
        (this.wrapping_add(SECOND_RESET) as *mut u32).write_unaligned(0);
        0
    }
});
