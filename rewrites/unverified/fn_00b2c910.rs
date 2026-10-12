// original: 0x00B2C910 squared_distance_from_blend_limits

/// Return the sum of squared signed deviations from two inclusive intervals.
/// The x bounds are words at +0x28 and +0x2c; the y bounds are at +0x30 and
/// +0x34. Each component contributes zero inside its interval and the ordered
/// endpoint difference outside. Unordered comparisons follow the original
/// COMISS branches. SSE scalar helpers preserve single precision and operand
/// order. The receiver is thiscall with two stack float words; the result is
/// returned on the x87 stack.
lf_checker_rt::export!(thiscall, rw_b2c910(this: u32, x_bits: u32, y_bits: u32) -> f32 {
    unsafe {
        use core::arch::x86::{_mm_add_ss, _mm_cvtss_f32, _mm_mul_ss, _mm_set_ss, _mm_sub_ss};
        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        fn sub(left: f32, right: f32) -> f32 {
            unsafe { _mm_cvtss_f32(_mm_sub_ss(_mm_set_ss(core::hint::black_box(left)), _mm_set_ss(core::hint::black_box(right)))) }
        }
        #[inline(always)]
        fn mul(left: f32, right: f32) -> f32 {
            unsafe { _mm_cvtss_f32(_mm_mul_ss(_mm_set_ss(core::hint::black_box(left)), _mm_set_ss(core::hint::black_box(right)))) }
        }
        #[inline(always)]
        fn add(left: f32, right: f32) -> f32 {
            unsafe { _mm_cvtss_f32(_mm_add_ss(_mm_set_ss(core::hint::black_box(left)), _mm_set_ss(core::hint::black_box(right)))) }
        }
        let x = f32::from_bits(x_bits);
        let y = f32::from_bits(y_bits);
        let x_low = f32::from_bits(read_word(this.wrapping_add(0x28)));
        let x_high = f32::from_bits(read_word(this.wrapping_add(0x2c)));
        let y_low = f32::from_bits(read_word(this.wrapping_add(0x30)));
        let y_high = f32::from_bits(read_word(this.wrapping_add(0x34)));
        let x_delta = if x_low > x { sub(x, x_low) } else if x > x_high { sub(x, x_high) } else { 0.0 };
        let y_delta = if y_low > y { sub(y, y_low) } else if y > y_high { sub(y, y_high) } else { 0.0 };
        let x_square = mul(x_delta, x_delta);
        let y_square = mul(y_delta, y_delta);
        add(x_square, y_square)
    }
});
