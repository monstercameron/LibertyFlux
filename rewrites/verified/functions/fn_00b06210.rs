// original: 0x00B06210 timed_falloff
/// Computes a normalized progress value from three signed counters and a
/// float argument. At or above the duration threshold it converts the value
/// to an angle, calls the sine helper, and applies the image scale and bias.
lf_checker_rt::export!(thiscall, rw_00b06210(this_: u32, arg_bits: u32) -> f32 {
    unsafe {
        #[inline(always)]
        fn fadd(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }
        #[inline(always)]
        fn fsub(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }
        #[inline(always)]
        fn fmul(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }
        #[inline(always)]
        fn fdiv(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) / core::hint::black_box(right)
        }
        #[inline(always)]
        unsafe fn read_i32(address: u32) -> i32 {
            unsafe { (address as *const i32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn image_f32(file_va: u32) -> f32 {
            unsafe {
                let bits = (lf_checker_rt::relocated(file_va) as *const u32).read_unaligned();
                f32::from_bits(bits)
            }
        }

        const DURATION_OFFSET: u32 = 0x184;
        const NUMERATOR_OFFSET: u32 = 0x194;
        const DENOMINATOR_OFFSET: u32 = 0x198;
        const ONE_VA: u32 = 0x00FE88E8;
        const ANGLE_FACTOR_VA: u32 = 0x00E81218;
        const ANGLE_OFFSET_VA: u32 = 0x00E8121C;
        const DEGREE_SCALE_VA: u32 = 0x00FE8728;
        const OUTPUT_SCALE_VA: u32 = 0x00FE8830;
        const CALL_THRESHOLD: i32 = 10_000;

        let duration = read_i32(this_ + DURATION_OFFSET);
        let numerator = read_i32(this_ + NUMERATOR_OFFSET);
        let denominator = read_i32(this_ + DENOMINATOR_OFFSET);
        let one = image_f32(ONE_VA);
        let reciprocal = fdiv(one, duration as f32);
        let numerator_rate = fmul(numerator as f32, reciprocal);
        let remainder = duration.wrapping_sub(denominator);
        let remainder_rate = fmul(remainder as f32, reciprocal);
        let argument = f32::from_bits(arg_bits);

        let progress = if numerator_rate > argument {
            fdiv(argument, numerator_rate)
        } else if remainder_rate > argument {
            one
        } else {
            let remaining = fsub(argument, remainder_rate);
            let denominator_rate = fmul(denominator as f32, reciprocal);
            let fraction = fdiv(remaining, denominator_rate);
            fsub(one, fraction)
        };

        if duration < CALL_THRESHOLD {
            progress
        } else {
            let angle_factor = image_f32(ANGLE_FACTOR_VA);
            let angle_offset = image_f32(ANGLE_OFFSET_VA);
            let degree_scale = image_f32(DEGREE_SCALE_VA);
            let output_scale = image_f32(OUTPUT_SCALE_VA);
            let scaled_progress = fmul(progress, angle_factor);
            let complement = fsub(angle_offset, scaled_progress);
            let radians = fmul(complement, degree_scale);
            let answer_bits = lf_checker_rt::callee_cdecl!(1, u32, radians.to_bits());
            let answer = f32::from_bits(answer_bits);
            let biased = fadd(answer, one);
            fmul(biased, output_scale)
        }
    }
});
