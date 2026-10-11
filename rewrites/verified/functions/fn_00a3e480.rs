// original: 0x00A3E480 vehicle_add_angular_delta

/// Add the three `f32` components pointed to by `delta` to the receiver's
/// angular accumulator at offsets `0x1060`, `0x1064`, and `0x1068`. The
/// accumulator is the left operand for each addition, matching the
/// original's exact floating-point operand order.
///
/// The original is a thiscall method with one stack pointer argument.
lf_checker_rt::export!(thiscall, rw_00a3e480(this: u32, delta: u32) -> () {
    const ANGULAR_ACCUMULATOR: u32 = 0x1060;
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
            let field = this.wrapping_add(ANGULAR_ACCUMULATOR + component * COMPONENT_STRIDE);
            let current = read_f32(field);
            let delta_value = read_f32(delta.wrapping_add(component * COMPONENT_STRIDE));
            write_f32(field, add(current, delta_value));
        }
    }
});
