// original: 0x00B05D80 angle_between_vectors
/// Return the scalar angle helper's result for two three-component vectors.
///
/// The routine forms each squared length and their product, normalizes the dot
/// product when that product is nonzero, clamps the normalized value to the
/// inclusive interval `[-1, 1]`, and asks the scalar angle helper for the
/// result. Products below the image's stored epsilon return positive zero.
/// The reciprocal and clamp bounds also come from the original image's
/// read-only data. The helper
/// takes its scalar `f32` in XMM0 and returns an `f32` in XMM0; only the low
/// four logged bytes are meaningful to that helper. Its answer is returned
/// unchanged through the function's x87 float return channel.
///
/// Inputs are the two vector pointers (`this` and the sole stack argument).
/// The function writes no memory and makes one helper call on the non-small
/// product path.
///
/// Original: `0x00B05D80` (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b05d80(this: u32, other: u32) -> f32 {
    unsafe {
        let ax = ((this + 0) as *const f32).read_unaligned();
        let ay = ((this + 4) as *const f32).read_unaligned();
        let az = ((this + 8) as *const f32).read_unaligned();
        let bx = ((other + 0) as *const f32).read_unaligned();
        let by = ((other + 4) as *const f32).read_unaligned();
        let bz = ((other + 8) as *const f32).read_unaligned();

        let ax_squared = mul32(ay, ay);
        let ax_x_squared = mul32(ax, ax);
        let length_a_squared = add32(add32(ax_squared, ax_x_squared), mul32(az, az));
        let bx_squared = mul32(by, by);
        let bx_x_squared = mul32(bx, bx);
        let length_b_squared = add32(add32(bx_squared, bx_x_squared), mul32(bz, bz));
        let length_product = mul32(length_a_squared, length_b_squared);

        let dot_x = mul32(bx, ax);
        let dot_y = mul32(by, ay);
        let dot_z = mul32(bz, az);
        let dot = add32(add32(dot_x, dot_y), dot_z);
        const ONE_VA: u32 = 0x00FE_88E8;
        const NEGATIVE_ONE_VA: u32 = 0x00FE_8D94;
        const LENGTH_EPSILON_VA: u32 = 0x00FE_863C;
        let one = lf_checker_rt::global::<f32>(ONE_VA).read_unaligned();
        let reciprocal_length = if length_product == 0.0 {
            0.0
        } else {
            let root = core::hint::black_box(length_product).sqrt();
            div32(one, root)
        };
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

        let length_epsilon = lf_checker_rt::global::<f32>(LENGTH_EPSILON_VA).read_unaligned();
        let product_minus_epsilon = sub32(length_product, length_epsilon);
        if jb32(product_minus_epsilon, 0.0) {
            return 0.0;
        }

        let helper_arg = clamped_dot;
        let angle_bits: u32 = lf_checker_rt::callee_cdecl!(1, u32, helper_arg.to_bits());
        f32::from_bits(angle_bits)
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
