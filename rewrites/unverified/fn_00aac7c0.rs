// original: 0x00AAC7C0 stream_pose_point_transform

/// Test whether a transformed point lies inside the object's interpolated radius.
///
/// The stdcall arguments are pointers to a transform record, an object record,
/// and a three-float point. The transform uses matrix fields at offsets
/// `0x30..0x68`; the object supplies basis weights, translation, a radius
/// interval, interpolation limits, and a final scale. The function computes
/// the transformed point in the original single-precision operation order,
/// subtracts the input point, and sums squared distance as Y then X then Z.
/// It selects `numerator / denominator` only when the denominator is ordered
/// greater than the numerator; equality and unordered comparisons use the
/// read-only image threshold constant. The interpolated radius is scaled and
/// squared, then compared with ordered `>` semantics. It returns the result in
/// AL and removes all three stack arguments.
lf_checker_rt::export!(stdcall, rw_00aac7c0(transform: u32, object: u32, point: u32) -> u8 {
    const X_ROW_FIRST: u32 = 0x40;
    const X_ROW_SECOND: u32 = 0x30;
    const X_ROW_THIRD: u32 = 0x50;
    const X_TRANSLATION: u32 = 0x60;
    const Y_ROW_FIRST: u32 = 0x44;
    const Y_ROW_SECOND: u32 = 0x34;
    const Y_ROW_THIRD: u32 = 0x54;
    const Y_TRANSLATION: u32 = 0x64;
    const Z_ROW_FIRST: u32 = 0x48;
    const Z_ROW_SECOND: u32 = 0x38;
    const Z_ROW_THIRD: u32 = 0x58;
    const Z_TRANSLATION: u32 = 0x68;
    const BASIS_X: u32 = 0x60;
    const BASIS_Y: u32 = 0x64;
    const BASIS_Z: u32 = 0x68;
    const RADIUS_MIN: u32 = 0x14;
    const RADIUS_MAX: u32 = 0x18;
    const RATIO_DENOMINATOR: u32 = 0x1c;
    const RATIO_NUMERATOR: u32 = 0x48;
    const RADIUS_SCALE: u32 = 0x70;
    const IMAGE_THRESHOLD_RVA: u32 = 0x00be_88e8;

    #[inline(always)]
    unsafe fn read_f32(address: u32) -> f32 {
        unsafe { f32::from_bits((address as *const u32).read_unaligned()) }
    }
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

    unsafe {
        let x_axis_weight = read_f32(object.wrapping_add(BASIS_X));
        let y_axis_weight = read_f32(object.wrapping_add(BASIS_Y));
        let z_axis_weight = read_f32(object.wrapping_add(BASIS_Z));

        let x_first_term = multiply(
            read_f32(transform.wrapping_add(X_ROW_FIRST)),
            y_axis_weight,
        );
        let x_second_term = multiply(
            read_f32(transform.wrapping_add(X_ROW_SECOND)),
            x_axis_weight,
        );
        let mut transformed_x = add(x_first_term, x_second_term);
        transformed_x = add(
            transformed_x,
            multiply(read_f32(transform.wrapping_add(X_ROW_THIRD)), z_axis_weight),
        );
        transformed_x = add(
            transformed_x,
            read_f32(transform.wrapping_add(X_TRANSLATION)),
        );

        let y_first_term = multiply(
            read_f32(transform.wrapping_add(Y_ROW_FIRST)),
            y_axis_weight,
        );
        let y_second_term = multiply(
            read_f32(transform.wrapping_add(Y_ROW_SECOND)),
            x_axis_weight,
        );
        let mut transformed_y = add(y_first_term, y_second_term);
        transformed_y = add(
            transformed_y,
            multiply(read_f32(transform.wrapping_add(Y_ROW_THIRD)), z_axis_weight),
        );
        transformed_y = add(
            transformed_y,
            read_f32(transform.wrapping_add(Y_TRANSLATION)),
        );

        let z_first_term = multiply(
            read_f32(transform.wrapping_add(Z_ROW_FIRST)),
            y_axis_weight,
        );
        let z_second_term = multiply(
            read_f32(transform.wrapping_add(Z_ROW_SECOND)),
            x_axis_weight,
        );
        let mut transformed_z = add(z_first_term, z_second_term);
        transformed_z = add(
            transformed_z,
            multiply(read_f32(transform.wrapping_add(Z_ROW_THIRD)), z_axis_weight),
        );
        transformed_z = add(
            transformed_z,
            read_f32(transform.wrapping_add(Z_TRANSLATION)),
        );

        let ratio_denominator = read_f32(object.wrapping_add(RATIO_DENOMINATOR));
        let ratio_numerator = read_f32(object.wrapping_add(RATIO_NUMERATOR));

        let delta_y = subtract(transformed_y, read_f32(point.wrapping_add(4)));
        let delta_x = subtract(transformed_x, read_f32(point));
        let delta_z = subtract(transformed_z, read_f32(point.wrapping_add(8)));
        let delta_y_squared = multiply(delta_y, delta_y);
        let delta_x_squared = multiply(delta_x, delta_x);
        let delta_z_squared = multiply(delta_z, delta_z);
        let distance_squared = add(add(delta_y_squared, delta_x_squared), delta_z_squared);

        let image_threshold = read_f32(
            lf_checker_rt::xbase().wrapping_add(IMAGE_THRESHOLD_RVA),
        );
        let interpolation_factor = if ratio_denominator > ratio_numerator {
            divide(ratio_numerator, ratio_denominator)
        } else {
            image_threshold
        };
        let radius_min = read_f32(object.wrapping_add(RADIUS_MIN));
        let radius_delta = subtract(read_f32(object.wrapping_add(RADIUS_MAX)), radius_min);
        let interpolated_radius = add(multiply(radius_delta, interpolation_factor), radius_min);
        let scaled_radius = multiply(
            interpolated_radius,
            read_f32(object.wrapping_add(RADIUS_SCALE)),
        );
        let radius_squared = multiply(scaled_radius, scaled_radius);
        u8::from(radius_squared > distance_squared)
    }
});
