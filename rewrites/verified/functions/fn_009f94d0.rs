// original: 0x009F94D0 stat_gauge_update_f64

/// Update the gauge's current difference and accumulated amount from its
/// linked sample, then emit an integer bucket when the change exceeds the
/// configured threshold.
///
/// The linked sample supplies two single-precision inputs that are converted
/// to double precision for the interpolation callee. A null link uses the
/// object's fallback sample. The function updates two process-wide float
/// words and may call the gauge predicate and the two statistic sinks.
///
/// Original: cdecl with one object pointer; return value is unused.
/// The original also writes its local difference into the incoming argument
/// slot; the contract documents the stack comparison exception.
fn update_gauge(obj: u32, wrong_second_sample: bool) -> u32 {
    unsafe {
        const LINK: u32 = 0x20;
        const FALLBACK_SAMPLE: u32 = 0x1c;
        const FIRST_INPUT: u32 = 0x10;
        const SECOND_INPUT: u32 = 0x14;
        const SAMPLE_HEIGHT: u32 = 0x18;

        const SIGN_MASK: u32 = 0x00fe_8fa0;
        const ABS_MASK: u32 = 0x00fe_8f80;
        const HEIGHT_LIMIT: u32 = 0x00fe_88bc;
        const DELTA_LIMIT: u32 = 0x00fe_8aa0;
        const DELTA_WRAP: u32 = 0x00fe_8aec;
        const BUCKET_SCALE: u32 = 0x00fe_87bc;

        const CURRENT_SAMPLE: u32 = 0x012b_6280;
        const ACCUMULATED_AMOUNT: u32 = 0x012b_6284;
        const INTERPOLATE_CALLEE: u32 = 0;
        const PREDICATE_CALLEE: u32 = 1;
        const BUCKET_CALLEE: u32 = 2;
        const EMIT_CALLEE: u32 = 3;
        const STAT_ID: u32 = 0x109;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_float(address: u32) -> f32 {
            f32::from_bits(unsafe { read_word(address) })
        }

        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        #[inline(always)]
        fn add(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }

        #[inline(always)]
        fn sub(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }

        #[inline(always)]
        fn truncate_i32(value: f32) -> i32 {
            if value.is_nan() || value >= 2_147_483_648.0 || value < -2_147_483_648.0 {
                i32::MIN
            } else {
                value as i32
            }
        }

        #[inline(always)]
        unsafe fn interpolate(point: u32, reverse_result: bool) -> f32 {
            let sign_mask = read_word(lf_checker_rt::relocated(SIGN_MASK));
            let first = f32::from_bits(read_word(point + FIRST_INPUT) ^ sign_mask);
            let second = read_float(point + SECOND_INPUT);
            let first_bits = (core::hint::black_box(first) as f64).to_bits();
            let second_bits = (core::hint::black_box(second) as f64).to_bits();
            let answer_bits: u64 = lf_checker_rt::callee_cdecl!(
                INTERPOLATE_CALLEE,
                u64,
                first_bits as u32,
                (first_bits >> 32) as u32,
                second_bits as u32,
                (second_bits >> 32) as u32
            );
            let answer = f64::from_bits(answer_bits) as f32;
            if reverse_result {
                f32::from_bits(answer.to_bits() ^ 0x8000_0000)
            } else {
                answer
            }
        }

        let linked_point = read_word(obj + LINK);
        let base_sample = if linked_point != 0 {
            interpolate(linked_point, false)
        } else {
            read_float(obj + FALLBACK_SAMPLE)
        };
        let absolute_mask = read_word(lf_checker_rt::relocated(ABS_MASK));
        let old_sample_address = lf_checker_rt::relocated(CURRENT_SAMPLE);
        let old_sample = read_float(old_sample_address);
        let mut delta = f32::from_bits(sub(base_sample, old_sample).to_bits() & absolute_mask);

        let current_sample = if linked_point != 0 {
            interpolate(linked_point, wrong_second_sample)
        } else {
            read_float(obj + FALLBACK_SAMPLE)
        };
        write_word(
            lf_checker_rt::relocated(CURRENT_SAMPLE),
            current_sample.to_bits(),
        );

        let predicate: u32 = lf_checker_rt::callee_thiscall!(PREDICATE_CALLEE, u32, obj);
        if (predicate as i32) >= 1 {
            write_word(lf_checker_rt::relocated(ACCUMULATED_AMOUNT), 0);
            return 0;
        }

        let sample_height = f32::from_bits(read_word(linked_point + SAMPLE_HEIGHT) & absolute_mask);
        if sample_height > read_float(lf_checker_rt::relocated(HEIGHT_LIMIT)) {
            return 0;
        }

        let delta_limit = read_float(lf_checker_rt::relocated(DELTA_LIMIT));
        if delta > delta_limit {
            delta = sub(read_float(lf_checker_rt::relocated(DELTA_WRAP)), delta);
        }

        let amount_address = lf_checker_rt::relocated(ACCUMULATED_AMOUNT);
        let amount = add(read_float(amount_address), delta);
        write_word(amount_address, amount.to_bits());

        let bucket_scale = read_float(lf_checker_rt::relocated(BUCKET_SCALE));
        let bucket = truncate_i32(core::hint::black_box(amount) * core::hint::black_box(bucket_scale));
        let _: u32 = lf_checker_rt::callee_cdecl!(BUCKET_CALLEE, u32, STAT_ID);
        let rounded_bucket = bucket as f32;
        if rounded_bucket > delta {
            let _: u32 = lf_checker_rt::callee_cdecl!(
                EMIT_CALLEE,
                u32,
                STAT_ID,
                rounded_bucket.to_bits()
            );
        }
        0
    }
}

lf_checker_rt::export!(cdecl, rw_009F94D0(obj: u32) -> u32 {
    update_gauge(obj, false)
});

lf_checker_rt::export!(cdecl, mut_009F94D0(obj: u32) -> u32 {
    update_gauge(obj, true)
});
