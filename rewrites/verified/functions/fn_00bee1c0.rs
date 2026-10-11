// original: 0x00BEE1C0 interpolate_task_visual_targets

/// Interpolate the task's horizontal and vertical target coordinates into the
/// output record. `this` supplies the current coordinates at `+0x70/+0x74`,
/// `source` supplies the next coordinates at the same offsets, and `target`
/// receives the result at `+0x149c/+0x1498`. Horizontal interpolation uses
/// source-minus-current, times the factor, then adds current. Vertical values
/// use the same order for short differences and wrap across the signed angle
/// range when their absolute difference exceeds the global half-turn threshold.
/// The method is thiscall and cleans three stack arguments; EAX returns source.
lf_checker_rt::export!(thiscall, rw_00BEE1C0(this: u32, target: u32, source: u32, factor_bits: u32) -> u32 {
    const CURRENT_X: u32 = 0x70;
    const CURRENT_Y: u32 = 0x74;
    const OUTPUT_X: u32 = 0x149C;
    const OUTPUT_Y: u32 = 0x1498;
    const ANGLE_THRESHOLD: u32 = 0x00FE8978;
    const TAU: u32 = 0x00FE8AEC;
    const PI: u32 = 0x00FE8AA0;
    const NEGATIVE_PI: u32 = 0x00FE8DC4;
    const ABS_MASK: u32 = 0x00FE8F80;

    #[inline(always)]
    fn add_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }
    #[inline(always)]
    fn sub_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    }
    #[inline(always)]
    fn mul_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }
    #[inline(always)]
    fn comiss_jbe(left: f32, right: f32) -> bool {
        left.is_nan() || right.is_nan() || left <= right
    }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }
    #[inline(always)]
    unsafe fn read_global_f32(address: u32) -> f32 {
        unsafe { f32::from_bits(lf_checker_rt::global::<u32>(address).read_unaligned()) }
    }

    unsafe {
        let factor = f32::from_bits(factor_bits);
        let current_x = f32::from_bits(read_u32(this + CURRENT_X));
        let source_x = f32::from_bits(read_u32(source + CURRENT_X));
        let horizontal_delta = sub_f32(source_x, current_x);
        let horizontal_step = mul_f32(horizontal_delta, factor);
        let horizontal = add_f32(horizontal_step, current_x);
        write_u32(target + OUTPUT_X, horizontal.to_bits());

        let current_y = f32::from_bits(read_u32(this + CURRENT_Y));
        let source_y = f32::from_bits(read_u32(source + CURRENT_Y));
        let abs_mask = read_u32(lf_checker_rt::relocated(ABS_MASK));
        let signed_y_delta = sub_f32(current_y, source_y);
        let absolute_y_delta = f32::from_bits(signed_y_delta.to_bits() & abs_mask);
        let threshold = read_global_f32(ANGLE_THRESHOLD);
        let tau = read_global_f32(TAU);
        let pi = read_global_f32(PI);
        let negative_pi = read_global_f32(NEGATIVE_PI);

        let vertical_result = if comiss_jbe(absolute_y_delta, threshold) {
            let delta = sub_f32(source_y, current_y);
            let step = mul_f32(delta, factor);
            Some(add_f32(step, current_y))
        } else if !comiss_jbe(current_y, 0.0) {
            let wrapped_source = add_f32(source_y, tau);
            let delta = sub_f32(wrapped_source, current_y);
            let step = mul_f32(delta, factor);
            let candidate = add_f32(step, current_y);
            if comiss_jbe(candidate, pi) {
                Some(candidate)
            } else {
                Some(sub_f32(candidate, tau))
            }
        } else if comiss_jbe(0.0, current_y) {
            None
        } else {
            let wrapped_source = sub_f32(source_y, tau);
            let delta = sub_f32(wrapped_source, current_y);
            let step = mul_f32(delta, factor);
            let candidate = add_f32(step, current_y);
            if comiss_jbe(negative_pi, candidate) {
                Some(candidate)
            } else {
                Some(add_f32(candidate, tau))
            }
        };

        if let Some(vertical) = vertical_result {
            write_u32(target + OUTPUT_Y, vertical.to_bits());
        }
        source
    }
});
