// original: 0x00B060E0 scaled_angle_between
/// Measure the angle between the object's position offset and a point offset,
/// then return the scalar angle helper's result scaled to degrees.
///
/// The first vector subtracts the shared reference point from the position
/// stored on `this`; the second subtracts that same point from the first stack
/// pointer. Their squared lengths form a normalization factor, their dot
/// product is clamped to the image's `[-1, 1]` bounds, and the scalar helper
/// receives the normalized value in XMM0. A product below the image epsilon
/// returns positive zero. The reciprocal, clamp bounds, epsilon and degree
/// scale are loaded from the original image's read-only data.
///
/// The function reads three coordinates from each vector and makes at most
/// one scripted scalar angle call. The helper's XMM0 comparison is limited to
/// its low four bytes because it consumes one `f32`; the result is returned in
/// the x87 float channel.
///
/// Original: `0x00B060E0` (thiscall, two stack pointers).
lf_checker_rt::export!(thiscall, rw_00b060e0(this: u32, first: u32, reference: u32) -> f32 {
    unsafe {
        const ONE_VA: u32 = 0x00FE_88E8;
        const NEGATIVE_ONE_VA: u32 = 0x00FE_8D94;
        const LENGTH_EPSILON_VA: u32 = 0x00FE_863C;
        const DEGREE_SCALE_VA: u32 = 0x00E7_C2A8;

        let this_x = ((this + 0x170) as *const f32).read_unaligned();
        let this_y = ((this + 0x174) as *const f32).read_unaligned();
        let this_z = ((this + 0x178) as *const f32).read_unaligned();
        let reference_x = (reference as *const f32).read_unaligned();
        let reference_y = ((reference + 4) as *const f32).read_unaligned();
        let reference_z = ((reference + 8) as *const f32).read_unaligned();
        let point_x = (first as *const f32).read_unaligned();
        let point_y = ((first + 4) as *const f32).read_unaligned();
        let point_z = ((first + 8) as *const f32).read_unaligned();

        let ax = sub32(this_x, reference_x);
        let ay = sub32(this_y, reference_y);
        let az = sub32(this_z, reference_z);
        let bx = sub32(point_x, reference_x);
        let by = sub32(point_y, reference_y);
        let bz = sub32(point_z, reference_z);

        let ax_squared = mul32(ax, ax);
        let ay_squared = mul32(ay, ay);
        let az_squared = mul32(az, az);
        let length_a_squared = add32(add32(ax_squared, ay_squared), az_squared);
        let by_squared = mul32(by, by);
        let bx_squared = mul32(bx, bx);
        let bz_squared = mul32(bz, bz);
        let length_b_squared = add32(add32(by_squared, bx_squared), bz_squared);
        let length_product = mul32(length_a_squared, length_b_squared);

        let one = lf_checker_rt::global::<f32>(ONE_VA).read_unaligned();
        let reciprocal_length = if length_product == 0.0 || length_product.is_nan() {
            0.0
        } else {
            let root = core::hint::black_box(length_product).sqrt();
            div32(one, root)
        };

        let dot_x = mul32(ax, bx);
        let dot_y = mul32(ay, by);
        let dot_z = mul32(bz, az);
        let dot = add32(add32(dot_x, dot_y), dot_z);
        let normalized_dot = mul32(dot, reciprocal_length);
        let lower = lf_checker_rt::global::<f32>(NEGATIVE_ONE_VA).read_unaligned();
        let upper = one;
        let clamped_dot = if ja32(lower, normalized_dot) {
            lower
        } else if jbe32(normalized_dot, upper) {
            normalized_dot
        } else {
            upper
        };

        let epsilon = lf_checker_rt::global::<f32>(LENGTH_EPSILON_VA).read_unaligned();
        let product_minus_epsilon = sub32(length_product, epsilon);
        if jb32(product_minus_epsilon, 0.0) {
            0.0
        } else {
            let helper_arg = clamped_dot;
            let angle_bits: u32 = lf_checker_rt::callee_cdecl!(1, u32, helper_arg.to_bits());
            let angle = f32::from_bits(angle_bits);
            let degree_scale = lf_checker_rt::global::<f32>(DEGREE_SCALE_VA).read_unaligned();
            mul32(angle, degree_scale)
        }
    }
});

#[inline(never)]
fn add32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) + core::hint::black_box(right)
}

#[inline(never)]
fn sub32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) - core::hint::black_box(right)
}

#[inline(never)]
fn mul32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) * core::hint::black_box(right)
}

#[inline(never)]
fn div32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) / core::hint::black_box(right)
}

#[inline(always)]
fn jbe32(left: f32, right: f32) -> bool {
    left.is_nan() || right.is_nan() || left <= right
}

#[inline(always)]
fn ja32(left: f32, right: f32) -> bool {
    !left.is_nan() && !right.is_nan() && left > right
}

#[inline(always)]
fn jb32(left: f32, right: f32) -> bool {
    left.is_nan() || right.is_nan() || left < right
}
