// original: 0x00c8d770 fn_00c8d770_point_within_distance_limit

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn read_global_f32(file_va: u32) -> f32 {
    let bits = unsafe { read_u32(lf_checker_rt::relocated(file_va)) };
    f32::from_bits(bits)
}

/// Return true when any signed-positive record lies strictly within the
/// squared-distance limit of the input point. The object count is signed; the
/// records begin at offset zero with stride 0x30, and the input point is a
/// three-float stack argument. Distance terms are accumulated in the original
/// Y, X, Z order before comparison with the read-only threshold.
const DISTANCE_COUNT_OFFSET: u32 = 0x240;
const DISTANCE_RECORD_STRIDE: u32 = 0x30;
const DISTANCE_LIMIT_SOURCE: u32 = 0x00FE86D4;

#[inline(always)]
fn subtract_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) - core::hint::black_box(right)
}

#[inline(always)]
fn multiply_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) * core::hint::black_box(right)
}

#[inline(always)]
fn add_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) + core::hint::black_box(right)
}

unsafe fn point_within_distance_limit(object: u32, point: u32, inclusive: bool) -> u8 {
    let signed_count = unsafe { read_u32(object + DISTANCE_COUNT_OFFSET) as i32 };
    if signed_count <= 0 {
        return 0;
    }
    let threshold = f32::from_bits(unsafe { read_u32(lf_checker_rt::relocated(DISTANCE_LIMIT_SOURCE)) });
    let point_x = f32::from_bits(unsafe { read_u32(point) });
    let point_y = f32::from_bits(unsafe { read_u32(point + 4) });
    let point_z = f32::from_bits(unsafe { read_u32(point + 8) });
    let mut index = 0u32;
    while index < signed_count as u32 {
        let record = object + index * DISTANCE_RECORD_STRIDE;
        let y_delta = subtract_f32(f32::from_bits(unsafe { read_u32(record + 4) }), point_y);
        let x_delta = subtract_f32(f32::from_bits(unsafe { read_u32(record) }), point_x);
        let z_delta = subtract_f32(f32::from_bits(unsafe { read_u32(record + 8) }), point_z);
        let y_square = multiply_f32(y_delta, y_delta);
        let x_square = multiply_f32(x_delta, x_delta);
        let z_square = multiply_f32(z_delta, z_delta);
        let xy_square = add_f32(y_square, x_square);
        let distance_squared = add_f32(xy_square, z_square);
        if if inclusive { threshold >= distance_squared } else { threshold > distance_squared } {
            return 1;
        }
        index += 1;
    }
    0
}

lf_checker_rt::export!(thiscall, rw_00c8d770_point_within_distance_limit(object: u32, point: u32) -> u8 {
    unsafe { point_within_distance_limit(object, point, false) }
});
