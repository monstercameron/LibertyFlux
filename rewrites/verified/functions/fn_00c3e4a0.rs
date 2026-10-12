// original: 0x00C3E4A0 train_double_pair_call
/// Update the stored train heading from the vehicle's forward and position vectors.
///
/// The routine exits when its active byte is clear or its speed field is zero.
/// It computes the displacement from the stored position using the scale at
/// offset `0x54`, asks the angle helper
/// for the heading using two promoted single-precision values, converts the
/// answer back to `f32`, scales it, clamps it to the inclusive range 8..=95,
/// and stores the result at offset `0x50`. The helper's scripted return is read
/// as raw double bits so every NaN payload and signed zero is preserved.
///
/// The function reads the `0x10..0x18`, `0x30..0x38`, `0x40..0x48`, `0x54`,
/// `0x68` and `0x70` fields of `this`; only the word at `0x50` is written.
/// Its single outgoing call is the angle helper, whose two double inputs are
/// transported as four stack words for the checker.
///
/// Original: `0x00C3E4A0` (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00c3e4a0(this: u32) -> () {
    unsafe {
        const ACTIVE_OFFSET: u32 = 0x70;
        const SPEED_OFFSET: u32 = 0x68;
        const POSITION_SCALE_OFFSET: u32 = 0x54;
        const OUTPUT_OFFSET: u32 = 0x50;

        let active = ((this + ACTIVE_OFFSET) as *const u8).read();
        let speed = ((this + SPEED_OFFSET) as *const f32).read_unaligned();
        let position_scale = ((this + POSITION_SCALE_OFFSET) as *const f32).read_unaligned();
        if active == 0 || speed == 0.0 {
            return;
        }

        let forward_x = ((this + 0x10) as *const f32).read_unaligned();
        let forward_y = ((this + 0x14) as *const f32).read_unaligned();
        let forward_z = ((this + 0x18) as *const f32).read_unaligned();
        let position_x = ((this + 0x30) as *const f32).read_unaligned();
        let position_y = ((this + 0x34) as *const f32).read_unaligned();
        let position_z = ((this + 0x38) as *const f32).read_unaligned();
        let origin_x = ((this + 0x40) as *const f32).read_unaligned();
        let origin_y = ((this + 0x44) as *const f32).read_unaligned();
        let origin_z = ((this + 0x48) as *const f32).read_unaligned();

        let dx = sub32(add32(position_x, mul32(forward_x, position_scale)), origin_x);
        let dy = sub32(add32(position_y, mul32(forward_y, position_scale)), origin_y);
        let dz = sub32(add32(position_z, mul32(forward_z, position_scale)), origin_z);
        let dx_squared = mul32(dx, dx);
        let dy_squared = mul32(dy, dy);
        let dz_squared = mul32(dz, dz);
        let distance_squared = add32(add32(dy_squared, dx_squared), dz_squared);
        let distance = core::hint::black_box(distance_squared).sqrt();

        let speed_as_double = f64::from(speed).to_bits();
        let distance_as_double = f64::from(distance).to_bits();
        let answer_bits: u64 = lf_checker_rt::callee_cdecl!(
            1,
            u64,
            speed_as_double as u32,
            (speed_as_double >> 32) as u32,
            distance_as_double as u32,
            (distance_as_double >> 32) as u32,
        );
        let angle = f64::from_bits(answer_bits) as f32;
        const DEGREES_PER_RADIAN: f32 = f32::from_bits(0x4265_2EE0);
        const DOUBLE_ANGLE: f32 = f32::from_bits(0x4000_0000);
        const MAX_ANGLE: f32 = f32::from_bits(0x42BE_0000);
        const MIN_ANGLE: f32 = f32::from_bits(0x4100_0000);
        let scaled = mul32(mul32(angle, DEGREES_PER_RADIAN), DOUBLE_ANGLE);

        // COMISS followed by JBE takes the upper clamp for unordered input too.
        let output = if jbe32(MAX_ANGLE, scaled) {
            MAX_ANGLE
        } else if ja32(scaled, MIN_ANGLE) {
            scaled
        } else {
            MIN_ANGLE
        };
        ((this + OUTPUT_OFFSET) as *mut f32).write_unaligned(output);
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

#[inline(always)]
fn ja32(left: f32, right: f32) -> bool {
    !left.is_nan() && !right.is_nan() && left > right
}
