// original: 0x00A3CBA0 vehicle_accumulate_directional_samples

/// Read `count` samples from the receiver's array at `+0x0c`, with a `0x20`
/// byte stride. The signed count is clamped by the loop's positive-only test.
/// Each sample contains four single-precision words. The first component is
/// accumulated into one of four receiver fields at `+0x94`, `+0x98`, `+0x9c`
/// or `+0xa0`, according to the sign and configured threshold of the computed
/// vertical and lateral components. COMISS unordered comparisons follow the
/// original flag branches: unordered vertical values take the sign-mask path,
/// while an unordered lateral value selects the positive lateral bucket.
/// Float operations preserve the original operand order and bitwise sign-mask
/// operation. The original is a thiscall method with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00a3cba0(this: u32) -> () {
    const SAMPLE_COUNT: u32 = 0x88;
    const SAMPLES: u32 = 0x0c;
    const SAMPLE_STRIDE: u32 = 0x20;
    const COMPONENT_X: u32 = 0x00;
    const COMPONENT_Y: u32 = 0x04;
    const COMPONENT_Z: u32 = 0x08;
    const COMPONENT_W: u32 = 0x0c;
    const ACCUMULATOR_POSITIVE_VERTICAL: u32 = 0x94;
    const ACCUMULATOR_NEGATIVE_VERTICAL: u32 = 0x98;
    const ACCUMULATOR_NEGATIVE_LATERAL: u32 = 0x9c;
    const ACCUMULATOR_POSITIVE_LATERAL: u32 = 0xa0;
    const VERTICAL_SIGN_MASK_VA: u32 = 0x00fe_8fa0;
    const DIRECTION_THRESHOLD_VA: u32 = 0x00fe_88b4;
    const ZERO: f32 = 0.0;

    #[inline(always)]
    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn read_float(address: u32) -> f32 {
        f32::from_bits(unsafe { read_word(address) })
    }
    #[inline(never)]
    fn game_mul(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }
    #[inline(never)]
    fn game_add(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }
    #[inline(always)]
    unsafe fn accumulate(receiver: u32, offset: u32, sample: f32) {
        let field = receiver.wrapping_add(offset);
        let previous = unsafe { read_float(field) };
        let updated = game_add(previous, sample);
        unsafe { (field as *mut u32).write_unaligned(updated.to_bits()) };
    }

    unsafe {
        let sign_mask = lf_checker_rt::global::<u32>(VERTICAL_SIGN_MASK_VA).read_unaligned();
        let threshold_bits = lf_checker_rt::global::<u32>(DIRECTION_THRESHOLD_VA).read_unaligned();
        let threshold = f32::from_bits(threshold_bits);
        let mut remaining = read_word(this.wrapping_add(SAMPLE_COUNT)) as i32;
        let mut sample_address = this.wrapping_add(SAMPLES);

        while remaining > 0 {
            let sample_x = read_float(sample_address.wrapping_add(COMPONENT_X));
            let sample_y = read_float(sample_address.wrapping_add(COMPONENT_Y));
            let sample_z = read_float(sample_address.wrapping_add(COMPONENT_Z));
            let sample_w = read_float(sample_address.wrapping_add(COMPONENT_W));

            let y_zero = game_mul(sample_y, ZERO);
            let w_zero = game_mul(sample_w, ZERO);
            let z_zero = game_mul(sample_z, ZERO);
            let vertical = game_add(game_add(y_zero, sample_z), w_zero);
            let lateral = game_add(game_add(z_zero, sample_y), w_zero);

            let vertical_bucketed = if vertical.is_nan() || vertical < ZERO {
                let magnitude = f32::from_bits(vertical.to_bits() ^ sign_mask);
                if magnitude.partial_cmp(&threshold).is_some_and(|order| order.is_ge()) {
                    accumulate(this, ACCUMULATOR_NEGATIVE_VERTICAL, sample_x);
                    true
                } else {
                    false
                }
            } else if vertical.partial_cmp(&threshold).is_some_and(|order| order.is_ge()) {
                accumulate(this, ACCUMULATOR_POSITIVE_VERTICAL, sample_x);
                true
            } else {
                false
            };
            if !vertical_bucketed {
                if lateral.partial_cmp(&ZERO) == Some(core::cmp::Ordering::Less) {
                    accumulate(this, ACCUMULATOR_NEGATIVE_LATERAL, sample_x);
                } else {
                    accumulate(this, ACCUMULATOR_POSITIVE_LATERAL, sample_x);
                }
            }

            sample_address = sample_address.wrapping_add(SAMPLE_STRIDE);
            remaining -= 1;
        }
    }
});
