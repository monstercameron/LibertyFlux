// original: 0x00A3E3A0 vehicle_add_linear_displacement

/// Add a three-component `f32` vector to the receiver's linear accumulator
/// at offsets `0x1050`, `0x1054`, and `0x1058`, in that order. `delta`
/// points to three consecutive `f32` values. Each result uses the existing
/// supplied component as the left operand and the existing accumulator as
/// the right operand, matching the original SSE instruction order exactly.
///
/// The original is a thiscall method with one stack pointer argument.
lf_checker_rt::export!(thiscall, rw_00a3e3a0(this: u32, delta: u32) -> () {
    const LINEAR_ACCUMULATOR: u32 = 0x1050;
    const COMPONENT_STRIDE: u32 = 4;

    #[inline(always)]
    unsafe fn read_f32(address: u32) -> f32 {
        unsafe { f32::from_bits((address as *const u32).read_unaligned()) }
    }
    #[inline(always)]
    unsafe fn write_f32(address: u32, value: f32) {
        unsafe { (address as *mut u32).write_unaligned(value.to_bits()) }
    }
    #[inline(always)]
    fn add(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }

    unsafe {
        for component in 0..3 {
            let field = this.wrapping_add(LINEAR_ACCUMULATOR + component * COMPONENT_STRIDE);
            let current = read_f32(field);
            let delta_value = read_f32(delta.wrapping_add(component * COMPONENT_STRIDE));
            write_f32(field, add(delta_value, current));
        }
    }
});
