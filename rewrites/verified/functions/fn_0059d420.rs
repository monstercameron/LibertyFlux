// original: 0x0059D420 randomized_counter_adjustment

/// Run the count-limited callback and adjust the shared timing counter.
///
/// This no-argument routine returns immediately when the signed counter is
/// zero. A positive counter loads the callback from its image data slot and
/// calls it with `1` until the signed loop index reaches the current counter;
/// a negative counter skips that loop. The routine then masks a scripted random
/// result to 16 bits, converts it exactly to `f32`, multiplies by the image
/// scale and the first image adjustment factor, truncates toward zero, and adds
/// `2 - delta` to the counter. If that result is signed-negative, a second
/// random result is scaled with the second image factor and the counter is
/// replaced with `1 - delta`. The float-to-integer inputs are code-bounded to
/// the interval `(-18, 0]`, so each truncating `as i32` stays in range. The
/// global counter is a single 32-bit word; this routine has no semantic return.
lf_checker_rt::export!(cdecl, rw_0059d420() -> () {
    const COUNTER_GLOBAL: u32 = 0x018B6C80;
    const CALLBACK_SLOT: u32 = 0x00E73150;
    const RANDOM_SCALE: u32 = 0x00FE8680;
    const FIRST_ADJUSTMENT_SCALE: u32 = 0x00FE8DC8;
    const SECOND_ADJUSTMENT_SCALE: u32 = 0x00FE8DDC;
    const CALLBACK_ARGUMENT: u32 = 1;
    const FIRST_BASE_ADJUSTMENT: i32 = 2;
    const SECOND_BASE_ADJUSTMENT: i32 = 1;
    const RANDOM_MASK: u32 = 0xFFFF;

    unsafe {
        let counter = lf_checker_rt::global::<i32>(COUNTER_GLOBAL);
        let starting_count = counter.read_unaligned();
        if starting_count == 0 {
            return;
        }

        if starting_count > 0 {
            let callback = lf_checker_rt::global::<u32>(CALLBACK_SLOT).read_unaligned();
            let _ = core::hint::black_box(callback);
            let mut loop_index = 0i32;
            while loop_index < counter.read_unaligned() {
                let _ = lf_checker_rt::callee_stdcall!(1, u32, CALLBACK_ARGUMENT);
                loop_index = loop_index.wrapping_add(1);
            }
        }

        let first_random = lf_checker_rt::callee_cdecl!(2, u32,);
        let first_random16 = first_random & RANDOM_MASK;
        let random_scale = lf_checker_rt::global::<f32>(RANDOM_SCALE).read_unaligned();
        let first_scale = lf_checker_rt::global::<f32>(FIRST_ADJUSTMENT_SCALE).read_unaligned();
        let first_samples = core::hint::black_box(first_random16 as f32)
            * core::hint::black_box(random_scale);
        let first_delta_float = core::hint::black_box(first_samples)
            * core::hint::black_box(first_scale);
        // first_random16 <= 65535 and the factors are 2^-15 and -4, so the
        // product is in [-8, 0]; the truncating conversion is safely in range.
        let first_delta = first_delta_float as i32;
        let updated_count = counter.read_unaligned().wrapping_add(
            FIRST_BASE_ADJUSTMENT.wrapping_sub(first_delta),
        );
        counter.write_unaligned(updated_count);

        if updated_count < 0 {
            let second_random = lf_checker_rt::callee_cdecl!(2, u32,);
            let second_random16 = second_random & RANDOM_MASK;
            let second_scale = lf_checker_rt::global::<f32>(SECOND_ADJUSTMENT_SCALE)
                .read_unaligned();
            let second_samples = core::hint::black_box(second_random16 as f32)
                * core::hint::black_box(random_scale);
            let second_delta_float = core::hint::black_box(second_samples)
                * core::hint::black_box(second_scale);
            // first_random16 <= 65535 and the factors are 2^-15 and -9, so
            // this truncating conversion is also bounded within i32.
            let second_delta = second_delta_float as i32;
            counter.write_unaligned(SECOND_BASE_ADJUSTMENT.wrapping_sub(second_delta));
        }
    }
});
