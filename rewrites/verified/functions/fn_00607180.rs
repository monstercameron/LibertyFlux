// original: 0x00607180 rage_ptxSprite_set_vector_scalars

/// Stores three scalar bit patterns in consecutive sprite vector fields.
/// Inputs are raw words so NaN payloads and signed zero are copied unchanged.
lf_checker_rt::export!(thiscall, rw_00607180(this: u32, x_bits: u32, y_bits: u32, z_bits: u32) -> () {
    const VALUE_X: u32 = 0x150;
    const VALUE_Y: u32 = 0x154;
    const VALUE_Z: u32 = 0x158;
    unsafe {
        (this.wrapping_add(VALUE_X) as *mut u32).write_unaligned(x_bits);
        (this.wrapping_add(VALUE_Y) as *mut u32).write_unaligned(y_bits);
        (this.wrapping_add(VALUE_Z) as *mut u32).write_unaligned(z_bits);
    }
});
