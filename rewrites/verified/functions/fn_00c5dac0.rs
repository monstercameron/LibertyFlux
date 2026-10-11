// original: 0x00C5DAC0 ped_heading_atan2_pair

/// Compute the normalized horizontal heading and vertical angle from an
/// object's frame to a target point. Store the normalized heading and
/// elevation through the two output pointers, and return the heading output
/// pointer. The horizontal normalization preserves the source's zero-distance
/// behavior, and the frame-relative difference passes through its normalizer.
const ATAN_ELEVATION: u32 = 1;
const ATAN_HEADING: u32 = 2;
const ATAN_REFERENCE: u32 = 3;
const NORMALIZE_HEADING: u32 = 4;
const SIGN_BIT: u32 = 0x8000_0000;

#[inline(always)]
fn add_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(core::hint::black_box(left) + core::hint::black_box(right))
}

#[inline(always)]
fn sub_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(core::hint::black_box(left) - core::hint::black_box(right))
}

#[inline(always)]
fn mul_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(core::hint::black_box(left) * core::hint::black_box(right))
}

#[inline(always)]
fn neg_f32(value: f32) -> f32 {
    f32::from_bits(value.to_bits() ^ SIGN_BIT)
}

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn read_f32(address: u32) -> f32 {
    f32::from_bits(unsafe { read_u32(address) })
}

#[inline(always)]
unsafe fn call_atan2(callee: u32, y: f32, x: f32) -> f64 {
    let y_bits = (core::hint::black_box(y) as f64).to_bits();
    let x_bits = (core::hint::black_box(x) as f64).to_bits();
    let result: u64 = lf_checker_rt::callee_cdecl!(
        callee,
        u64,
        y_bits as u32,
        (y_bits >> 32) as u32,
        x_bits as u32,
        (x_bits >> 32) as u32,
    );
    f64::from_bits(result)
}

unsafe fn run_heading(object: u32, target: u32, heading_out: u32, elevation_out: u32) -> u32 {
    const POSITION_OFFSET: u32 = 0x30;
    const OBJECT_FRAME_OFFSET: u32 = 0x20;
    const FRAME_YAW_OFFSET: u32 = 0x10;
    const FRAME_PITCH_OFFSET: u32 = 0x14;

    let frame = unsafe { read_u32(object.wrapping_add(OBJECT_FRAME_OFFSET)) };
    let delta_x = sub_f32(
        unsafe { read_f32(target) },
        unsafe { read_f32(frame.wrapping_add(POSITION_OFFSET)) },
    );
    let delta_y = sub_f32(
        unsafe { read_f32(target.wrapping_add(4)) },
        unsafe { read_f32(frame.wrapping_add(POSITION_OFFSET + 4)) },
    );
    let delta_z = sub_f32(
        unsafe { read_f32(target.wrapping_add(8)) },
        unsafe { read_f32(frame.wrapping_add(POSITION_OFFSET + 8)) },
    );

    let squared_x = mul_f32(delta_x, delta_x);
    let squared_y = mul_f32(delta_y, delta_y);
    let horizontal_squared = add_f32(squared_x, squared_y);
    let horizontal_distance = core::hint::black_box(horizontal_squared).sqrt();

    let elevation = unsafe { call_atan2(ATAN_ELEVATION, delta_z, horizontal_distance) };
    unsafe { write_u32(elevation_out, (elevation as f32).to_bits()) };

    let inverse_distance = if horizontal_squared == 0.0 {
        0.0
    } else {
        1.0 / core::hint::black_box(horizontal_squared).sqrt()
    };
    let normalized_x = mul_f32(inverse_distance, delta_x);
    let normalized_y = mul_f32(inverse_distance, delta_y);
    let heading = unsafe { call_atan2(ATAN_HEADING, neg_f32(normalized_x), normalized_y) };

    let frame_yaw = unsafe { read_f32(frame.wrapping_add(FRAME_YAW_OFFSET)) };
    let frame_pitch = unsafe { read_f32(frame.wrapping_add(FRAME_PITCH_OFFSET)) };
    let reference = unsafe { call_atan2(ATAN_REFERENCE, neg_f32(frame_yaw), frame_pitch) };
    let difference = sub_f32(heading as f32, reference as f32);
    let normalized: f32 = lf_checker_rt::callee_cdecl!(
        NORMALIZE_HEADING,
        f32,
        difference.to_bits(),
    );
    unsafe { write_u32(heading_out, normalized.to_bits()) };

    heading_out
}

lf_checker_rt::export!(cdecl, rw_00c5dac0(object: u32, target: u32, heading_out: u32, elevation_out: u32) -> u32 {
    unsafe { run_heading(object, target, heading_out, elevation_out) }
});
