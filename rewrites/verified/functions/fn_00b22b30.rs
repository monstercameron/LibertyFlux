// original: 0x00B22B30 portal_plane_side_test
/// Form the normal of a triangle stored as three packed XYZ points at offsets
/// 0x00, 0x10 and 0x20, then return whether the query point at `point` is on
/// the non-negative side of that plane. Each subtraction, multiplication and
/// addition keeps the original single-precision operand order. The final
/// COMISS/SETBE result is true for a non-negative dot product or an unordered
/// NaN. Thiscall with one point pointer; only AL is the declared result.
lf_checker_rt::export!(thiscall, rw_00b22b30(this: u32, point: u32) -> u32 {
    unsafe {
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }

        #[inline(always)]
        fn sub(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }
        #[inline(always)]
        fn mul(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }
        #[inline(always)]
        fn add(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }
        #[inline(always)]
        unsafe fn f32_at(address: u32) -> f32 {
            unsafe { f32::from_bits(read_u32(address)) }
        }

        const POINT_A: u32 = 0x00;
        const POINT_B: u32 = 0x10;
        const POINT_C: u32 = 0x20;

        let ax = f32_at(this.wrapping_add(POINT_A));
        let ay = f32_at(this.wrapping_add(POINT_A + 4));
        let az = f32_at(this.wrapping_add(POINT_A + 8));
        let bx = f32_at(this.wrapping_add(POINT_B));
        let by = f32_at(this.wrapping_add(POINT_B + 4));
        let bz = f32_at(this.wrapping_add(POINT_B + 8));
        let cx = f32_at(this.wrapping_add(POINT_C));
        let cy = f32_at(this.wrapping_add(POINT_C + 4));
        let cz = f32_at(this.wrapping_add(POINT_C + 8));

        let a_minus_b_x = sub(ax, bx);
        let a_minus_b_y = sub(ay, by);
        let a_minus_b_z = sub(az, bz);
        let b_minus_c_x = sub(bx, cx);
        let b_minus_c_y = sub(by, cy);
        let b_minus_c_z = sub(bz, cz);

        let normal_x = sub(mul(b_minus_c_z, a_minus_b_y), mul(a_minus_b_z, b_minus_c_y));
        let normal_y = sub(mul(a_minus_b_z, b_minus_c_x), mul(b_minus_c_z, a_minus_b_x));
        let normal_z = sub(mul(b_minus_c_y, a_minus_b_x), mul(b_minus_c_x, a_minus_b_y));

        let point_x = sub(f32_at(point), bx);
        let point_y = sub(f32_at(point.wrapping_add(4)), by);
        let point_z = sub(f32_at(point.wrapping_add(8)), bz);
        let x_term = mul(point_x, normal_x);
        let y_term = mul(point_y, normal_y);
        let z_term = mul(point_z, normal_z);
        let partial = add(x_term, y_term);
        let dot = add(partial, z_term);
        u32::from(dot.is_nan() || dot >= 0.0)
    }
});
