// original: 0x00C8BED0 audio_voice_render_setup

/// On the tested modes 1-3 path, transforms the supplied position with one
/// of the manager's two angle-input groups, delegates the update, and returns the failure
/// marker when the delegate leaves the local record unset.
const OWNER_MANAGER: u32 = 0x20;
const MANAGER_TEST_A: u32 = 0x28;
const MANAGER_TEST_B: u32 = 0x18;
const MANAGER_TEST_C: u32 = 0x08;
const MANAGER_ANGLE: u32 = 0x20;
const MANAGER_ANGLE_PAIR: u32 = 0x24;
const MANAGER_POSITION_X: u32 = 0x30;
const MANAGER_POSITION_Y: u32 = 0x34;
const MANAGER_POSITION_Z: u32 = 0x38;
const LIVE_COUNT: u32 = 0x016FC640;
const ANGLE_LIMIT: u32 = 0x00FE88BC;
const SIGN_MASK: u32 = 0x00FE8FA0;

#[inline(always)]
unsafe fn read_word(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_word(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn read_float(address: u32) -> f32 {
    f32::from_bits(unsafe { read_word(address) })
}

#[inline(always)]
unsafe fn global_float(address: u32) -> f32 {
    unsafe { read_float(lf_checker_rt::relocated(address)) }
}

#[inline(always)]
fn add(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) + core::hint::black_box(right)
}

#[inline(always)]
fn sub(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) - core::hint::black_box(right)
}

#[inline(always)]
fn mul(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) * core::hint::black_box(right)
}

#[inline(always)]
unsafe fn abs_like_original(value: f32) -> f32 {
    let sign = unsafe { global_float(SIGN_MASK) }.to_bits();
    if 0.0f32 > value {
        f32::from_bits(value.to_bits() ^ sign)
    } else {
        value
    }
}

#[inline(always)]
fn double_helper(id: u32, angle: u64, pair_low: u64, pair_high: u64) -> f32 {
    let a0 = angle as u32;
    let a1 = (angle >> 32) as u32;
    let p0 = pair_low as u32;
    let p1 = (pair_low >> 32) as u32;
    let p2 = pair_high as u32;
    let p3 = (pair_high >> 32) as u32;
    let result: u64 = lf_checker_rt::callee_cdecl!(id, u64, a0, a1, p0, p1, p2, p3);
    f64::from_bits(result) as f32
}

#[inline(always)]
fn scalar_helper(id: u32, input: u32) -> f32 {
    let answer: u32 = lf_checker_rt::callee_cdecl!(id, u32, input);
    f32::from_bits(answer)
}

unsafe fn run(object: u32, mode: u32, owner: u32, source: u32) -> u32 {
    let mut first_query = 0xA40u32;
    let mut second_query = 0u32;
    let ready: u32 = lf_checker_rt::callee_cdecl!(
        1,
        u32,
        object,
        (&mut first_query as *mut u32) as u32,
        (&mut second_query as *mut u32) as u32,
    );
    if ready & 0xff == 0 {
        return u32::MAX;
    }

    let count = unsafe { read_word(lf_checker_rt::relocated(LIVE_COUNT)) };
    if count >= 0xA40 {
        return u32::MAX;
    }

    if !matches!(mode, 1 | 2 | 3) {
        return 0;
    }

    let manager = unsafe { read_word(owner + OWNER_MANAGER) };
    if manager == 0 {
        return 0;
    }
    let limit = unsafe { global_float(ANGLE_LIMIT) };
    let a = unsafe { abs_like_original(read_float(manager + MANAGER_TEST_A)) };
    let b = unsafe { abs_like_original(read_float(manager + MANAGER_TEST_B)) };
    let c = unsafe { abs_like_original(read_float(manager + MANAGER_TEST_C)) };
    let primary = !(a > limit) && (b > limit || c > limit);
    let (angle_offset, pair_offset, double_id, sine_id, cosine_id) = if primary {
        (MANAGER_ANGLE, MANAGER_ANGLE_PAIR, 3, 4, 5)
    } else {
        (0x10, 0x14, 6, 7, 8)
    };
    let sign = unsafe { global_float(SIGN_MASK) }.to_bits();
    let first_angle = f32::from_bits(unsafe { read_word(manager + angle_offset) } ^ sign);
    let second_angle = unsafe { read_float(manager + pair_offset) };
    let third_angle = unsafe { read_float(manager + pair_offset + 4) };
    let angle = f64::from(first_angle).to_bits();
    let pair_low = f64::from(second_angle).to_bits();
    let pair_high = f64::from(third_angle).to_bits();
    let angle_as_float = double_helper(double_id, angle, pair_low, pair_high);
    let angle_bits = angle_as_float.to_bits();

    let sine = scalar_helper(sine_id, angle_bits);
    let cosine = scalar_helper(cosine_id, angle_bits);

    let source_x = unsafe { read_float(source) };
    let source_y = unsafe { read_float(source + 4) };
    let source_z = unsafe { read_float(source + 8) };
    let source_w = unsafe { read_word(source + 12) };
    let target_x = unsafe { read_float(manager + MANAGER_POSITION_X) };
    let target_y = unsafe { read_float(manager + MANAGER_POSITION_Y) };
    let target_z = unsafe { read_float(manager + MANAGER_POSITION_Z) };

    let x = add(target_x, sub(mul(source_x, cosine), mul(source_y, sine)));
    let y = add(target_y, add(mul(source_y, cosine), mul(source_x, sine)));
    let z = add(target_z, source_z);
    let mut missing_record = 0u32;
    let mut result_position = [x.to_bits(), y.to_bits(), z.to_bits(), source_w];
    let delegated: u32 = lf_checker_rt::callee_cdecl!(
        2,
        u32,
        (&mut missing_record as *mut u32) as u32,
        mode,
        owner,
        result_position.as_mut_ptr() as u32,
    );
    if delegated != 0 || missing_record != 0 {
        return 0;
    }
    u32::MAX
}

lf_checker_rt::export!(cdecl, rw_audio_voice_render_setup(
    object: u32, mode: u32, owner: u32, source: u32,
    arg4: u32, arg5: u32, arg6: u32, arg7: u32
) -> u32 {
    let _ = (arg4, arg5, arg6, arg7);
    unsafe { run(object, mode, owner, source) }
});
