// original: 0x0096CD10 interpolate_timing_value

/// Interpolates from `d` to `e` using `(a-b)/(c-b)` clamped to the global
/// upper limit and zero. Every arithmetic operation keeps the original SSE
/// operand order. The single-precision result is returned in ST0.
lf_checker_rt::export!(cdecl, rw_0096cd10(a: f32, b: f32, c: f32, d: f32, e: f32) -> f32 {
    #[inline(always)]
    fn add(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }
    #[inline(always)]
    fn subtract(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    }
    #[inline(always)]
    fn multiply(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }
    #[inline(always)]
    fn divide(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) / core::hint::black_box(right)
    }

    const MAX_GLOBAL: u32 = 0x00fe88e8;
    let numerator = subtract(a, b);
    let denominator = subtract(c, b);
    let ratio = divide(numerator, denominator);
    let maximum = unsafe { lf_checker_rt::global::<f32>(MAX_GLOBAL).read_unaligned() };
    let factor = if ratio < 0.0 {
        0.0
    } else if ratio > maximum {
        maximum
    } else {
        ratio
    };
    let delta = subtract(e, d);
    let scaled = multiply(delta, factor);
    add(scaled, d)
});
