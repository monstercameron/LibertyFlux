// original: 0x00a35a90 vehicle_wrapped_lerp (proposed)

/// Blend `a` toward a wrapped delta and wrap the result into `[-π, π]`.
///
/// Wraps `d = b - a` into `[-π, π]` in single precision, forms
/// `t = d * c + a`, then wraps `t` into `[-π, π]` again, this time adding
/// or subtracting 2π in double precision (converting through `f64` each
/// pass). Returns `t` on x87 ST0. Cdecl/3 (float bits). The contract
/// bounds inputs so every wrap loop terminates quickly.
lf_checker_rt::export!(cdecl, rw_00a35a90(a_bits: u32, b_bits: u32, c_bits: u32) -> f32 {
    unsafe {
        const PI: f32 = f32::from_bits(0x4049_0FDB);
        const TWO_PI: f32 = f32::from_bits(0x40C9_0FDB);
        const NEG_PI: f32 = f32::from_bits(0xC049_0FDB);
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let a = f32::from_bits(a_bits);
        let b = f32::from_bits(b_bits);
        let c = f32::from_bits(c_bits);
        let mut d = sub(b, a);
        while d > PI {
            d = sub(d, TWO_PI);
        }
        while NEG_PI > d {
            d = add(d, TWO_PI);
        }
        let mut t = add(mul(d, c), a);
        // The double constant is exactly the single-precision 2π widened.
        let wide: f64 = TWO_PI as f64;
        while t > PI {
            t = ((t as f64) - wide) as f32;
        }
        while NEG_PI > t {
            t = ((t as f64) + wide) as f32;
        }
        t
    }
});
