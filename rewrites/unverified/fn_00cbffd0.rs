// original: 0x00CBFFD0 interpolate_boundary_position
/// Given four ascending f32 boundaries and an f32 input, return the interval
/// index plus the input's fractional position inside that interval. Clamp
/// below the first boundary to zero and beyond the last boundary to three.
/// Subtraction, division and addition retain the original x86 operand order.
lf_checker_rt::export!(stdcall, rw_00cbffd0(boundaries: *const u32, input_bits: u32) -> f32 {
    use core::arch::x86::{_mm_add_ss,_mm_cvtss_f32,_mm_div_ss,_mm_set_ss,_mm_sub_ss};
    const BOUNDARY_COUNT: usize = 4;
    const LAST_INTERVAL: usize = BOUNDARY_COUNT - 1;
    const CLAMPED_LOW: f32 = 0.0;
    const FINAL_BOUNDARY_INDEX: f32 = 3.0;
    const FULL_INTERVAL: f32 = 1.0;
    let input = f32::from_bits(input_bits);
    let first = unsafe { f32::from_bits(boundaries.read_unaligned()) };
    if first > input {
        return CLAMPED_LOW;
    }
    for interval in 0..LAST_INTERVAL {
        let current = unsafe { f32::from_bits(boundaries.add(interval).read_unaligned()) };
        let next = unsafe { f32::from_bits(boundaries.add(interval + 1).read_unaligned()) };
        if input.is_nan() || current.is_nan() || input < current || next >= input {
            let fraction = if current == next {
                FULL_INTERVAL
            } else {
                let left = core::hint::black_box(input);
                let right = core::hint::black_box(current);
                let numerator = unsafe { _mm_cvtss_f32(_mm_sub_ss(_mm_set_ss(left),_mm_set_ss(right))) };
                let left = core::hint::black_box(next);
                let right = core::hint::black_box(current);
                let denominator = unsafe { _mm_cvtss_f32(_mm_sub_ss(_mm_set_ss(left),_mm_set_ss(right))) };
                let left = core::hint::black_box(numerator);
                let right = core::hint::black_box(denominator);
                unsafe { _mm_cvtss_f32(_mm_div_ss(_mm_set_ss(left),_mm_set_ss(right))) }
            };
            let left = core::hint::black_box(interval as f32);
            let right = core::hint::black_box(fraction);
            return unsafe { _mm_cvtss_f32(_mm_add_ss(_mm_set_ss(left),_mm_set_ss(right))) };
        }
    }
    FINAL_BOUNDARY_INDEX
});
