// original: 0x00B44290 line_box_query

/// Expand a three-dimensional segment into two parallel XY-offset segments,
/// then query each through the spatial box helper. The offset is formed from
/// the normalized direction as `(dy, -dx, 0)`, with the original sequence of
/// multiply-by-zero operations retained for unordered inputs. The first
/// query uses the negative offset; the second runs only when the first returns
/// nonzero and uses the positive offset. Both helper calls receive four-word
/// point records, with the fourth word supplied by the defined zero stack
/// fill. The return value is the low byte from the last query reached.
///
/// The squared length is accumulated as `dx² + dy² + dz²`; nonzero values,
/// including unordered values, follow the original square-root and reciprocal
/// path. All floating-point operations keep their source operand order. The
/// function uses stdcall, takes two point pointers, a radius and a mode word,
/// and compares only the meaningful low-byte result.
lf_checker_rt::export!(stdcall, rw_00b44290(point_a: u32, point_b: u32, radius: f32, mode: u32) -> u32 {
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
    fn div_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) / core::hint::black_box(right)
    }

    unsafe {
        const SPATIAL_QUERY_FIRST: u32 = 1;
        const SPATIAL_QUERY_SECOND: u32 = 2;
        const ZERO: f32 = 0.0;
        const ONE: f32 = 1.0;

        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            f32::from_bits((address as *const u32).read_unaligned())
        }

        let start = [
            read_f32(point_a),
            read_f32(point_a + 4),
            read_f32(point_a + 8),
        ];
        let end = [
            read_f32(point_b),
            read_f32(point_b + 4),
            read_f32(point_b + 8),
        ];
        let dx = sub_f32(end[0], start[0]);
        let dy = sub_f32(end[1], start[1]);
        let dz = sub_f32(end[2], start[2]);

        let dx_squared = mul_f32(dx, dx);
        let dy_squared = mul_f32(dy, dy);
        let dz_squared = mul_f32(dz, dz);
        let horizontal_sum = add_f32(dx_squared, dy_squared);
        let length_squared = add_f32(horizontal_sum, dz_squared);
        let inverse_length = if length_squared == ZERO {
            ZERO
        } else {
            let length = core::hint::black_box(length_squared).sqrt();
            div_f32(ONE, length)
        };

        let unit_y = mul_f32(inverse_length, dy);
        let unit_z = mul_f32(inverse_length, dz);
        let unit_x = mul_f32(inverse_length, dx);
        let zero_y = mul_f32(unit_y, ZERO);
        let zero_z = mul_f32(unit_z, ZERO);
        let offset_x_unscaled = sub_f32(unit_y, zero_z);
        let offset_y_unscaled = sub_f32(zero_z, unit_x);
        let zero_x = mul_f32(unit_x, ZERO);
        let offset_z_unscaled = sub_f32(zero_x, zero_y);
        let offset_y = mul_f32(offset_y_unscaled, radius);
        let offset_x = mul_f32(offset_x_unscaled, radius);
        let offset_z = mul_f32(offset_z_unscaled, radius);

        let mut start_minus = [
            sub_f32(start[0], offset_x),
            sub_f32(start[1], offset_y),
            sub_f32(start[2], offset_z),
            ZERO,
        ];
        let mut end_minus = [
            sub_f32(end[0], offset_x),
            sub_f32(end[1], offset_y),
            sub_f32(end[2], offset_z),
            ZERO,
        ];
        // Keep the source points and computed offsets in memory across the
        // intercepted helper call. The original stores these locals before
        // the call and reloads them to construct the second query.
        let mut preserved = [
            start[0], start[1], start[2],
            end[0], end[1], end[2],
            offset_x, offset_y, offset_z,
        ];
        let preserved_ptr = preserved.as_mut_ptr();
        for index in 0..preserved.len() {
            core::ptr::write_volatile(preserved_ptr.add(index), preserved[index]);
        }
        let first_hit = lf_checker_rt::callee_stdcall!(
            SPATIAL_QUERY_FIRST,
            u8,
            start_minus.as_mut_ptr() as u32,
            end_minus.as_mut_ptr() as u32,
            mode,
        );
        if first_hit == 0 {
            return 0;
        }

        let saved_start = [
            core::ptr::read_volatile(preserved_ptr.add(0)),
            core::ptr::read_volatile(preserved_ptr.add(1)),
            core::ptr::read_volatile(preserved_ptr.add(2)),
        ];
        let saved_end = [
            core::ptr::read_volatile(preserved_ptr.add(3)),
            core::ptr::read_volatile(preserved_ptr.add(4)),
            core::ptr::read_volatile(preserved_ptr.add(5)),
        ];
        let saved_offset = [
            core::ptr::read_volatile(preserved_ptr.add(6)),
            core::ptr::read_volatile(preserved_ptr.add(7)),
            core::ptr::read_volatile(preserved_ptr.add(8)),
        ];

        let mut start_plus = [
            add_f32(saved_start[0], saved_offset[0]),
            add_f32(saved_start[1], saved_offset[1]),
            add_f32(saved_start[2], saved_offset[2]),
            ZERO,
        ];
        let mut end_plus = [
            add_f32(saved_end[0], saved_offset[0]),
            add_f32(saved_end[1], saved_offset[1]),
            add_f32(saved_end[2], saved_offset[2]),
            ZERO,
        ];
        let second_hit = lf_checker_rt::callee_stdcall!(
            SPATIAL_QUERY_SECOND,
            u8,
            start_plus.as_mut_ptr() as u32,
            end_plus.as_mut_ptr() as u32,
            mode,
        );
        u32::from(second_hit)
    }
});
