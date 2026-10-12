// original: 0x00C0D800 UILayoutFrame::vf58
/// Return whether the optional value referenced by `this + 0xec` differs
/// from the image's threshold. A null value pointer returns false; unordered
/// floating-point comparisons (including NaN) count as different. The
/// method takes `this` in ECX, returns one boolean byte in AL, and reads the
/// threshold from the mapped image data.
lf_checker_rt::export!(thiscall, rw_c0d800(this: u32) -> u32 {
    unsafe {
        const VALUE_POINTER: u32 = 0x0ec;
        const THRESHOLD_VA: u32 = 0x00fe_8628;
        let value_address = (this.wrapping_add(VALUE_POINTER) as *const u32).read_unaligned();
        if value_address == 0 {
            return 0;
        }
        let value_bits = (value_address as *const u32).read_unaligned();
        let threshold_bits = lf_checker_rt::global::<u32>(THRESHOLD_VA).read_unaligned();
        let value = f32::from_bits(value_bits);
        let threshold = f32::from_bits(threshold_bits);
        u32::from(value != threshold)
    }
});
