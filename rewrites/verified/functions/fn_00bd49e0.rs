// original: 0x00bd49e0 NativeImpl_DRAW_CORONA

/// Build a point vector from three raw float words, scale the radius, and convert the center-adjusted horizontal coordinates with x86 truncation semantics. Forward those derived values, fixed corona dimensions, and the remaining draw words to the corona renderer. All arithmetic follows the original single-precision operand order.
#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn rw_00bd49e0(
    point_x_bits: u32,
    point_y_bits: u32,
    point_z_bits: u32,
    radius_bits: u32,
    unused_5: u32,
    unused_6: u32,
    draw_word_7: u32,
    draw_word_8: u32,
    draw_word_9: u32,
) -> u32 {
    const DRAW_CORONA: u32 = 1;
    const CENTER_GLOBAL: u32 = 0x00fe_8e08;
    const RADIUS_SCALE_GLOBAL: u32 = 0x00fe_8b08;
    const PIXEL_FACTOR_GLOBAL: u32 = 0x00eb_7e94;
    const TWENTY: u32 = 0x41a0_0000;
    const FOUR_HUNDRED_FIFTY: u32 = 0x43e1_0000;
    const ONE_AND_A_HALF: u32 = 0x3fc0_0000;
    const FOUR: u32 = 0x4080_0000;

    let center_bits = unsafe { lf_checker_rt::global::<u32>(CENTER_GLOBAL).read_unaligned() };
    let radius_scale_bits = unsafe { lf_checker_rt::global::<u32>(RADIUS_SCALE_GLOBAL).read_unaligned() };
    let pixel_factor_bits = unsafe { lf_checker_rt::global::<u32>(PIXEL_FACTOR_GLOBAL).read_unaligned() };
    let center_y = sub_f32_rw(point_y_bits, center_bits);
    let pixel_y = mul_f32_rw(center_y, pixel_factor_bits);
    let truncated_y = cvttss2si_rw(pixel_y);
    let center_x = sub_f32_rw(point_x_bits, center_bits);
    let truncated_x = cvttss2si_rw(center_x);
    let x_delta = truncated_x.wrapping_sub(truncated_y) as u32;
    let scaled_radius = mul_f32_rw(radius_bits, radius_scale_bits);
    let position = [point_x_bits, point_y_bits, point_z_bits];

    unsafe {
        lf_checker_rt::callee_cdecl!(
            DRAW_CORONA,
            u32,
            x_delta,
            draw_word_7,
            draw_word_8,
            draw_word_9,
            TWENTY,
            position.as_ptr() as usize as u32,
            scaled_radius,
            FOUR_HUNDRED_FIFTY,
            ONE_AND_A_HALF,
            0,
            FOUR,
            0,
            0,
            0,
            0,
        )
    }
}

#[inline(never)]
fn sub_f32_rw(left_bits: u32, right_bits: u32) -> u32 {
    (core::hint::black_box(f32::from_bits(left_bits)) - core::hint::black_box(f32::from_bits(right_bits))).to_bits()
}

#[inline(never)]
fn mul_f32_rw(left_bits: u32, right_bits: u32) -> u32 {
    (core::hint::black_box(f32::from_bits(left_bits)) * core::hint::black_box(f32::from_bits(right_bits))).to_bits()
}

#[inline(never)]
fn cvttss2si_rw(bits: u32) -> i32 {
    let value = f32::from_bits(core::hint::black_box(bits));
    if !value.is_finite() || value < -2_147_483_648.0 || value >= 2_147_483_648.0 {
        i32::MIN
    } else {
        value.trunc() as i32
    }
}
