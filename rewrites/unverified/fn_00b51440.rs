// original: 0x00B51440 smooth_heading_rate
/// Move the stored heading toward the sum of a vector's horizontal angle and
/// an offset, limiting the change by a frame-scaled rate.
///
/// The horizontal angle helper receives the vector's Z component and the
/// square root of its X/Y squared sum as two promoted doubles. Its scripted
/// result is narrowed to `f32` before the offset is added. The absolute
/// difference from the current heading is compared with the global rate
/// multiplied by the frame value. A difference above the positive limit adds
/// that limit; one below the negative limit subtracts it; otherwise the target
/// heading is stored directly.
///
/// The function reads the three vector words, the current heading, the frame
/// value, the offset and one writable image scalar. It writes only the current
/// heading field. The helper's two vector-register doubles use the v8 stack
/// transport, with all 16 logged bytes compared.
///
/// Original: `0x00B51440` (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00b51440(this: u32, vector: u32, frame_bits: u32, offset_bits: u32) -> () {
    unsafe {
        const CURRENT_HEADING_OFFSET: u32 = 0x149C;
        const RATE_SCALE_VA: u32 = 0x0117_35BC;

        let vector_x = ((vector + 0) as *const f32).read_unaligned();
        let vector_y = ((vector + 4) as *const f32).read_unaligned();
        let vector_z = ((vector + 8) as *const f32).read_unaligned();
        let x_squared = mul32(vector_x, vector_x);
        let y_squared = mul32(vector_y, vector_y);
        let horizontal_squared = add32(x_squared, y_squared);
        let horizontal_length = core::hint::black_box(horizontal_squared).sqrt();

        let z_as_double = f64::from(vector_z).to_bits();
        let length_as_double = f64::from(horizontal_length).to_bits();
        let target_bits: u64 = lf_checker_rt::callee_cdecl!(
            1,
            u64,
            z_as_double as u32,
            (z_as_double >> 32) as u32,
            length_as_double as u32,
            (length_as_double >> 32) as u32,
        );
        let angle = f64::from_bits(target_bits) as f32;
        let offset = f32::from_bits(offset_bits);
        let target_heading = add32(angle, offset);
        let current_heading = ((this + CURRENT_HEADING_OFFSET) as *const f32).read_unaligned();
        let difference = sub32(target_heading, current_heading);
        let frame = f32::from_bits(frame_bits);
        let rate_scale = lf_checker_rt::global::<f32>(RATE_SCALE_VA).read();
        let limit = mul32(rate_scale, frame);
        let negative_limit = f32::from_bits(limit.to_bits() ^ 0x8000_0000);

        let next_heading = if jbe32(difference, limit) {
            if jbe32(negative_limit, difference) {
                target_heading
            } else {
                sub32(current_heading, limit)
            }
        } else {
            add32(current_heading, limit)
        };
        ((this + CURRENT_HEADING_OFFSET) as *mut f32).write_unaligned(next_heading);
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

#[inline(always)]
fn jbe32(left: f32, right: f32) -> bool {
    left.is_nan() || right.is_nan() || left <= right
}
