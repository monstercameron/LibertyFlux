// original: 0x00B43E00 spatial_box_query

/// Build a grid-cell query for the axis-aligned bounds of two four-float
/// points, then ask the spatial enumerator to test each overlapping cell.
/// The bounds are stored as six low-word signed conversions in the order
/// X-min, X-max, Y-min, Y-max, Z-min, Z-max; the Z range expands by one
/// unit in both directions. The callback context carries the two points,
/// their adjusted Z values, and the caller's byte mode. The return is one
/// exactly when the callback leaves its result byte clear.
///
/// Each float-to-integer conversion reproduces CVTTSS2SI: NaN, positive
/// infinity, values at least 2^31, and values below -2^31 yield i32::MIN;
/// finite in-range values truncate toward zero before the low word is kept.
/// COMISS/JBE ordering treats unordered comparisons as JBE, matching the
/// original selection of each endpoint. Arithmetic is pinned in source
/// order. The function uses stdcall and removes its three argument words.
lf_checker_rt::export!(stdcall, rw_00b43e00(point_a: u32, point_b: u32, mode: u32) -> u32 {
    #[repr(C)]
    struct QueryContext {
        first: [f32; 4],
        second: [f32; 4],
        high_z: f32,
        low_z: f32,
        mode: u8,
        result: u8,
        padding: [u8; 2],
        tail: u32,
    }

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
    fn jbe(left: f32, right: f32) -> bool {
        left <= right || left.is_nan() || right.is_nan()
    }

    #[inline(always)]
    fn cvttss2si(value: f32) -> i32 {
        if value.is_nan() || value >= 2_147_483_648.0 || value < -2_147_483_648.0 {
            i32::MIN
        } else {
            // The explicit range check above bounds this cast to CVTTSS2SI's
            // defined conversion range, including its valid -2^31 endpoint.
            value as i32
        }
    }

    unsafe extern "C" fn local_hit_callback(_physical: u32, _context: u32) -> u32 {
        0
    }

    unsafe {
        const HIT_FLAG_VA: u32 = 0x0179_fd48;
        const SPATIAL_ENUM_CALL: u32 = 1;
        const CELL_SCALE: f32 = 8.0;
        const Z_MARGIN: f32 = 1.0;
        const POINT_Z_BIAS: f32 = f32::from_bits(0x3ecc_cccd);

        let first = [
            (point_a as *const f32).read_unaligned(),
            ((point_a + 4) as *const f32).read_unaligned(),
            ((point_a + 8) as *const f32).read_unaligned(),
            ((point_a + 12) as *const f32).read_unaligned(),
        ];
        let second = [
            (point_b as *const f32).read_unaligned(),
            ((point_b + 4) as *const f32).read_unaligned(),
            ((point_b + 8) as *const f32).read_unaligned(),
            ((point_b + 12) as *const f32).read_unaligned(),
        ];

        core::ptr::write_volatile(lf_checker_rt::global::<u8>(HIT_FLAG_VA), 0);

        let (min_x, max_x) = if jbe(second[0], first[0]) {
            (second[0], first[0])
        } else {
            (first[0], second[0])
        };
        let (min_y, max_y) = if jbe(second[1], first[1]) {
            (second[1], first[1])
        } else {
            (first[1], second[1])
        };
        let (min_z, max_z) = if jbe(second[2], first[2]) {
            (second[2], first[2])
        } else {
            (first[2], second[2])
        };

        let mut bounds = [
            cvttss2si(mul_f32(min_x, CELL_SCALE)) as u16,
            cvttss2si(mul_f32(max_x, CELL_SCALE)) as u16,
            cvttss2si(mul_f32(min_y, CELL_SCALE)) as u16,
            cvttss2si(mul_f32(max_y, CELL_SCALE)) as u16,
            cvttss2si(mul_f32(sub_f32(min_z, Z_MARGIN), CELL_SCALE)) as u16,
            cvttss2si(mul_f32(add_f32(max_z, Z_MARGIN), CELL_SCALE)) as u16,
        ];

        let first_z = add_f32(POINT_Z_BIAS, first[2]);
        let second_z = add_f32(POINT_Z_BIAS, second[2]);
        let (high_z, low_z) = if jbe(second_z, first_z) {
            (first_z, second_z)
        } else {
            (second_z, first_z)
        };
        let mut context = QueryContext {
            first: [first[0], first[1], first_z, first[3]],
            second: [second[0], second[1], second_z, second[3]],
            high_z,
            low_z,
            mode: mode as u8,
            result: 0,
            padding: [0; 2],
            tail: 0,
        };

        let bounds_ptr = bounds.as_mut_ptr() as u32;
        let context_ptr = &mut context as *mut QueryContext as u32;
        let callback_ptr = local_hit_callback as usize as u32;
        let _ = lf_checker_rt::callee_cdecl!(
            SPATIAL_ENUM_CALL,
            u32,
            bounds_ptr,
            callback_ptr,
            context_ptr,
        );

        let result = ((context_ptr + 0x29) as *const u8).read_volatile();
        u32::from(result == 0)
    }
});
