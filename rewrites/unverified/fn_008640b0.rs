// original: 0x008640B0 physics_scale_vector_above_threshold

/// Measures the three component vector at `this + 0`, `+4` and `+8` using
/// the original single precision sum order (y squared, then x squared, then
/// z squared). When that sum is ordered and above 0.01, the vector is scaled
/// by `0.1 / sqrt(sum)` and written back. The square root is an intercepted
/// cdecl helper that returns its f32 value on the x87 stack. For finite
/// inputs the second, negative-sum helper path is unreachable because a sum
/// of squares cannot be negative; unordered sums leave the vector unchanged.
lf_checker_rt::export!(thiscall, rw_008640b0(this: u32) -> u32 {
    unsafe {
        const X: u32 = 0x00;
        const Y: u32 = 0x04;
        const Z: u32 = 0x08;
        const NORMALIZE_THRESHOLD_BITS: u32 = 0x3c23_d70b;
        const RECIPROCAL_NUMERATOR_BITS: u32 = 0x3f80_0000;
        const SCALE_FACTOR_BITS: u32 = 0x3dcc_cccd;
        const SQRT_CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn read_float(base: u32, offset: u32) -> f32 {
            let bits = unsafe { ((base.wrapping_add(offset)) as *const u32).read_unaligned() };
            f32::from_bits(bits)
        }
        #[inline(always)]
        unsafe fn write_float(base: u32, offset: u32, value: f32) {
            unsafe { ((base.wrapping_add(offset)) as *mut u32).write_unaligned(value.to_bits()) }
        }
        #[inline(always)]
        fn multiply(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }
        #[inline(always)]
        fn add(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }
        #[inline(always)]
        fn divide(numerator: f32, denominator: f32) -> f32 {
            core::hint::black_box(numerator) / core::hint::black_box(denominator)
        }

        let x = read_float(this, X);
        let y = read_float(this, Y);
        let z = read_float(this, Z);
        let y_squared = multiply(y, y);
        let x_squared = multiply(x, x);
        let mut length_squared = add(y_squared, x_squared);
        let z_squared = multiply(z, z);
        length_squared = add(length_squared, z_squared);

        if length_squared > f32::from_bits(NORMALIZE_THRESHOLD_BITS) {
            let root: f32 = lf_checker_rt::callee_cdecl!(
                SQRT_CALLEE,
                f32,
                length_squared.to_bits()
            );
            let reciprocal = divide(f32::from_bits(RECIPROCAL_NUMERATOR_BITS), root);
            let scale = multiply(reciprocal, f32::from_bits(SCALE_FACTOR_BITS));
            write_float(this, X, multiply(x, scale));
            write_float(this, Y, multiply(y, scale));
            write_float(this, Z, multiply(scale, z));
        }
        0
    }
});
