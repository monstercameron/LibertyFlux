// original: 0x009B5BE0 script_distance_fade_factor

/// Compute a distance-based fade factor from a point and scripted parameters.
///
/// The stdcall arguments are a pointer to a three-float point and a float
/// scale. A scripted parameter object supplies the reference point at
/// `+0x40..+0x48`. The distance is formed from Y, X, then Z squared deltas,
/// summed in that order and square-rooted. Distances above 50 return positive
/// zero; distances below 5 are clamped to 5 and receive an extra factor of
/// 0.5. The remaining factor is `(1 - (distance - 5) * rate) * 0.9`, then
/// multiplied by the scale argument, except the exact sentinel `-1.0` selects
/// `0.5`. The float return is in ST0. The original also writes that result
/// over the first incoming stack argument; the proof observes it through ST0.
lf_checker_rt::export!(stdcall, rw_009b5be0(point: u32, scale_bits: u32) -> f32 {
    unsafe {
        const PARAMETER_MANAGER: u32 = 0x0103_e498;
        const REFERENCE_X: u32 = 0x40;
        const REFERENCE_Y: u32 = 0x44;
        const REFERENCE_Z: u32 = 0x48;
        const MAX_DISTANCE: f32 = f32::from_bits(0x4248_0000);
        const MIN_DISTANCE: f32 = f32::from_bits(0x40a0_0000);
        const HALF: f32 = f32::from_bits(0x3f00_0000);
        const ONE: f32 = f32::from_bits(0x3f80_0000);
        const DISTANCE_RATE: f32 = f32::from_bits(0x3cb6_0b61);
        const FADE_SCALE: f32 = f32::from_bits(0x3f66_6666);
        const SCALE_SENTINEL: f32 = f32::from_bits(0xbf80_0000);

        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            unsafe { f32::from_bits((address as *const u32).read_unaligned()) }
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
        fn mul(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }

        let parameter_record = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            lf_checker_rt::relocated(PARAMETER_MANAGER)
        );
        let point_x = read_f32(point);
        let point_y = read_f32(point.wrapping_add(4));
        let point_z = read_f32(point.wrapping_add(8));
        let delta_y = sub(read_f32(parameter_record.wrapping_add(REFERENCE_Y)), point_y);
        let delta_x = sub(read_f32(parameter_record.wrapping_add(REFERENCE_X)), point_x);
        let delta_z = sub(read_f32(parameter_record.wrapping_add(REFERENCE_Z)), point_z);
        let square_y = mul(delta_y, delta_y);
        let square_x = mul(delta_x, delta_x);
        let square_z = mul(delta_z, delta_z);
        let sum_yx = add(square_y, square_x);
        let squared_distance = add(sum_yx, square_z);
        let mut distance = core::hint::black_box(squared_distance).sqrt();

        if distance > MAX_DISTANCE {
            return 0.0;
        }

        let clamped = MIN_DISTANCE > distance;
        if clamped {
            distance = MIN_DISTANCE;
        }
        let distance_from_minimum = sub(distance, MIN_DISTANCE);
        let rate_adjustment = mul(distance_from_minimum, DISTANCE_RATE);
        let mut fade = mul(sub(ONE, rate_adjustment), FADE_SCALE);
        if clamped {
            fade = mul(fade, HALF);
        }

        let requested_scale = f32::from_bits(scale_bits);
        let scale = if requested_scale == SCALE_SENTINEL { HALF } else { requested_scale };
        mul(fade, scale)
    }
});
