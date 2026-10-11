// original: 0x00C2F7C0 vehicle_dynamics_sample_clamped_factor

/// Produce two scalar factors from the signed speed-like value at `this +
/// 0x1c`, and cache the second factor at `this + 0x20`. The first output is
/// one for nonpositive or unordered input; for positive input it is one minus
/// the input multiplied by the fixed response coefficient. The second output
/// is zero for nonnegative or unordered input, follows the original negative
/// ranges and thresholds, and is written both through its output pointer and
/// to the cache field. This is a thiscall method with three stack arguments;
/// only the first two stack arguments are output pointers.
lf_checker_rt::export!(thiscall, rw_00c2f7c0(
    this: *mut u8,
    positive_factor_out: *mut u8,
    negative_factor_out: *mut u8,
    unused_stack_word: u32,
) -> () {
    const SPEED_OFFSET: usize = 0x1c;
    const CACHED_FACTOR_OFFSET: usize = 0x20;
    const ONE_BITS: u32 = 0x3f80_0000;
    const POSITIVE_RESPONSE_BITS: u32 = 0x3b23_d70a;
    const NEGATIVE_RESPONSE_BITS: u32 = 0xbabb_3ee7;
    const LOWER_CLAMP_BITS: u32 = 0xc42f_0000;
    const ZERO_TO_LOWER_BITS: u32 = 0xc561_0000;
    const UPPER_FADE_START_BITS: u32 = 0xc548_0000;
    const UPPER_FADE_SCALE_BITS: u32 = 0x3aa3_d70a;

    #[inline(never)]
    fn mul_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }
    #[inline(never)]
    fn sub_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    }

    let _ = unused_stack_word;
    unsafe {
        let speed = this.add(SPEED_OFFSET).cast::<f32>().read_unaligned();
        let mut positive_factor = f32::from_bits(ONE_BITS);
        if speed > 0.0 {
            let response = mul_f32(speed, f32::from_bits(POSITIVE_RESPONSE_BITS));
            positive_factor = sub_f32(f32::from_bits(ONE_BITS), response);
        }
        positive_factor_out.cast::<f32>().write_unaligned(positive_factor);

        let mut negative_factor = 0.0f32;
        if speed < 0.0 {
            if speed <= f32::from_bits(LOWER_CLAMP_BITS) {
                if speed >= f32::from_bits(ZERO_TO_LOWER_BITS) {
                    if speed >= f32::from_bits(UPPER_FADE_START_BITS) {
                        negative_factor = f32::from_bits(ONE_BITS);
                    } else {
                        let distance = sub_f32(speed, f32::from_bits(ZERO_TO_LOWER_BITS));
                        let fade = mul_f32(distance, f32::from_bits(UPPER_FADE_SCALE_BITS));
                        negative_factor = sub_f32(f32::from_bits(ONE_BITS), fade);
                    }
                }
            } else {
                negative_factor = mul_f32(speed, f32::from_bits(NEGATIVE_RESPONSE_BITS));
            }
        }
        negative_factor_out.cast::<f32>().write_unaligned(negative_factor);
        this.add(CACHED_FACTOR_OFFSET).cast::<f32>().write_unaligned(negative_factor);
    }
});
