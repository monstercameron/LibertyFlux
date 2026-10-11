// original: 0x00D7D600 input_stick_update

/// Updates the stick object's smoothed position, wrapped phase, and bounded
/// response values. It samples the object's substate through its virtual
/// interface, combines that sample with the prior frame, and returns the
/// object's stored result word.
const OBJECT_VTABLE: u32 = 0x0;
const OBJECT_SUBSTATE: u32 = 0x20;
const VTABLE_QUERY_SLOT: u32 = 0xEC;
const SUBSTATE_FIRST_X: u32 = 0x10;
const SUBSTATE_FIRST_Y: u32 = 0x14;
const SUBSTATE_CURRENT: u32 = 0x18;
const SUBSTATE_SIGN: u32 = 0x28;
const SUBSTATE_DELTA_X: u32 = 0x30;
const SUBSTATE_DELTA_Y: u32 = 0x34;
const SUBSTATE_TURN_X: u32 = 0x0;
const SUBSTATE_TURN_Y: u32 = 0x4;
const SUBSTATE_BASE: u32 = 0x38;
const SUBSTATE_SECOND: u32 = 0x8;
const OBJECT_PHASE_SOURCE: u32 = 0x1EDC;
const OBJECT_PHASE_COPY: u32 = 0x1EE0;
const OBJECT_BASE_SOURCE: u32 = 0xE54;
const OBJECT_BASE_COPY: u32 = 0x1ED4;
const OBJECT_OLD_POSITION: u32 = 0x1EE4;
const OBJECT_POSITION: u32 = 0x1EC4;
const OBJECT_POSITION_X: u32 = 0xE4C;
const OBJECT_POSITION_Y: u32 = 0xE50;
const OBJECT_RESULT: u32 = 0x1EC0;
const OBJECT_PHASE_OUTPUT: u32 = 0x1EB0;
const OBJECT_RESPONSE: u32 = 0x1EB4;
const OBJECT_ANGLE: u32 = 0x1EB8;
const OBJECT_RETURN_COPY: u32 = 0x1EBC;


const VOICE_RATE_GLOBAL: u32 = 0x011735BC;
const RATE_MULTIPLIER: u32 = 0x00FE8B68;
const PHASE_STEP_NUMERATOR: u32 = 0x00FE8A24;
const POSITIVE_LIMIT: u32 = 0x00FE891C;
const NEGATIVE_LIMIT: u32 = 0x00EB9504;
const ANGLE_LOW: u32 = 0x00FE8DC4;
const ANGLE_HIGH: u32 = 0x00FE8AA0;
const SIGN_MASK: u32 = 0x00FE8FA0;
const RESPONSE_CAP: u32 = 0x00FE88BC;
const RESPONSE_SCALE: u32 = 0x00FE8960;
const RESPONSE_DEADZONE: u32 = 0x00FE879C;
const RESPONSE_GAIN: u32 = 0x00FE8AB8;
const RESPONSE_FLOOR: u32 = 0x00E9D14C;
const NEGATIVE_ONE: u32 = 0x00FE8D94;
const ONE: u32 = 0x00FE88E8;
const FULL_TURN: u32 = 0x00FE8AEC;
const POSITION_RATE: u32 = 0x01056DF0;
const ANGLE_SIGN: u32 = 0x01056DF4;
const POSITION_WEIGHT: u32 = 0x01056DF8;
const HALF: u32 = 0x00FE8830;
const ZERO_BITS: u32 = 0;

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
unsafe fn write_f32(address: u32, value: f32) {
    unsafe { (address as *mut f32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn global_f32(virtual_address: u32) -> f32 {
    unsafe { read_f32(lf_checker_rt::relocated(virtual_address)) }
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
fn div(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) / core::hint::black_box(right)
}

#[inline(always)]
fn sqrt(value: f32) -> f32 {
    core::hint::black_box(value).sqrt()
}

#[inline(always)]
fn flip_sign(value: f32, mask: f32) -> f32 {
    f32::from_bits(value.to_bits() ^ mask.to_bits())
}

#[inline(always)]
fn wrap_angle(mut angle: f32, low: f32, high: f32, step: f32) -> f32 {
    while low > angle {
        angle = add(angle, step);
    }
    while angle > high {
        angle = sub(angle, step);
    }
    angle
}

#[inline(always)]
unsafe fn query_virtual_state(object: u32, buffer: u32) -> u32 {
    let vtable = unsafe { read_u32(object + OBJECT_VTABLE) };
    let target = unsafe { read_u32(vtable + VTABLE_QUERY_SLOT) };
    let query: extern "thiscall" fn(u32, u32) -> u32 =
        unsafe { core::mem::transmute(target as usize) };
    query(object, buffer)
}

unsafe fn update_stick(object: u32) -> u32 {
    let phase_source = unsafe { read_f32(object + OBJECT_PHASE_SOURCE) };
    let phase_base = unsafe { read_f32(object + OBJECT_BASE_SOURCE) };
    unsafe {
        write_f32(object + OBJECT_PHASE_COPY, phase_source);
        write_f32(object + OBJECT_BASE_COPY, phase_base);
    }

    let substate = unsafe { read_u32(object + OBJECT_SUBSTATE) };
    let initial_sample: f32 = lf_checker_rt::callee_cdecl!(
        2,
        f32,
        unsafe { read_f32(substate + SUBSTATE_FIRST_X) }.to_bits(),
        unsafe { read_f32(substate + SUBSTATE_FIRST_Y) }.to_bits(),
    );

    let mut virtual_buffer = [0u32; 4];
    let virtual_result = unsafe {
        query_virtual_state(object, virtual_buffer.as_mut_ptr() as u32)
    };
    let virtual_base = unsafe { read_f32(virtual_result + 8) };
    let prior_target = mul(virtual_base, unsafe { global_f32(PHASE_STEP_NUMERATOR) });
    let local_phase = add(prior_target, unsafe { read_f32(substate + SUBSTATE_BASE) });

    let current_input = unsafe { read_f32(substate + SUBSTATE_CURRENT) };
    let current_bits: u32 = lf_checker_rt::callee_cdecl!(3, u32, current_input.to_bits());


    let current_value = f32::from_bits(current_bits);
    let runtime_rate = unsafe { global_f32(VOICE_RATE_GLOBAL) };
    let rate_scale = unsafe { global_f32(RATE_MULTIPLIER) };
    let rate_denominator = mul(runtime_rate, rate_scale);
    let phase_rate = div(unsafe { global_f32(PHASE_STEP_NUMERATOR) }, rate_denominator);
    let phase_change = sub(current_value, unsafe { read_f32(object + OBJECT_OLD_POSITION) });
    unsafe { write_f32(object + OBJECT_OLD_POSITION, current_value) };
    let smoothed_value = add(mul(phase_rate, phase_change), current_value);

    let delta_y = sub(
        unsafe { read_f32(substate + SUBSTATE_DELTA_Y) },
        unsafe { read_f32(object + OBJECT_POSITION_Y) },
    );
    let delta_x = sub(
        unsafe { read_f32(substate + SUBSTATE_DELTA_X) },
        unsafe { read_f32(object + OBJECT_POSITION_X) },
    );
    let distance_sq = add(mul(delta_x, delta_x), mul(delta_y, delta_y));
    let distance = sqrt(distance_sq);
    let direction_numerator = sub(unsafe { read_f32(object + OBJECT_BASE_COPY) }, local_phase);
    let mut direction = div(direction_numerator, distance);
    let positive_limit = unsafe { global_f32(POSITIVE_LIMIT) };
    let negative_limit = unsafe { global_f32(NEGATIVE_LIMIT) };
    direction = if !(positive_limit > direction) {
        positive_limit
    } else if direction > negative_limit {
        direction
    } else {
        negative_limit
    };

    let step_scale = unsafe { global_f32(HALF) };
    let response_step = mul(sub(direction, smoothed_value), step_scale);

    let phase_start = sub(unsafe { read_f32(object + OBJECT_PHASE_COPY) }, initial_sample);
    let turn_size = unsafe { global_f32(FULL_TURN) };
    let wrapped_phase = wrap_angle(
        phase_start,
        unsafe { global_f32(ANGLE_LOW) },
        unsafe { global_f32(ANGLE_HIGH) },
        turn_size,
    );
    let response_origin = unsafe { global_f32(RESPONSE_CAP) };
    let response_raw = mul(
        flip_sign(wrapped_phase, unsafe { global_f32(SIGN_MASK) }),
        unsafe { global_f32(RESPONSE_SCALE) },
    );
    let zero = f32::from_bits(ZERO_BITS);
    let negative_zero = unsafe { global_f32(SIGN_MASK) };
    let mut signed_response = response_raw;
    let mut response_limit = response_origin;
    if signed_response > response_limit {
        signed_response = response_limit;
    } else {
        response_limit = unsafe { global_f32(RESPONSE_FLOOR) };
        if !(response_limit > signed_response) {
            if !(zero > signed_response) {
                response_limit = signed_response;
            } else {
                response_limit = flip_sign(signed_response, negative_zero);
            }
        } else {
            signed_response = response_limit;
            response_limit = flip_sign(response_limit, negative_zero);
        }
    }

    let deadzone = unsafe { global_f32(RESPONSE_DEADZONE) };
    let phase_offset;
    if !(deadzone > response_limit) {
        phase_offset = flip_sign(signed_response, negative_zero);
        unsafe { write_f32(object + OBJECT_PHASE_OUTPUT, signed_response) };
    } else {
        signed_response = mul(signed_response, unsafe { global_f32(RESPONSE_GAIN) });
        phase_offset = zero;
        unsafe { write_f32(object + OBJECT_PHASE_OUTPUT, signed_response) };
    }

    let local_turn_seed = if substate != 0 {
        let sign_control = unsafe { read_f32(substate + SUBSTATE_SIGN) };
        let x_component = unsafe { read_f32(substate + SUBSTATE_TURN_X) };
        let y_component = unsafe { read_f32(substate + SUBSTATE_TURN_Y) };
        let x2 = mul(x_component, x_component);
        let y2 = mul(y_component, y_component);
        let mut length = sqrt(add(x2, y2));
        if zero > sign_control {
            length = mul(length, unsafe { global_f32(NEGATIVE_ONE) });
        }
        let first_double = f64::from(unsafe { read_f32(substate + SUBSTATE_SECOND) }).to_bits();
        let second_double = f64::from(length).to_bits();
        let answer: u64 = lf_checker_rt::callee_cdecl!(
            4,
            u64,
            first_double as u32,
            (first_double >> 32) as u32,
            second_double as u32,
            (second_double >> 32) as u32,
        );
        core::hint::black_box(f64::from_bits(answer)) as f32
    } else {
        zero
    };

    let mut position_denominator = mul(runtime_rate, rate_scale);
    let one = unsafe { global_f32(ONE) };
    if one > position_denominator {
        position_denominator = one;
    }
    let position_rate = div(unsafe { global_f32(POSITION_RATE) }, position_denominator);
    let position_change = sub(local_turn_seed, unsafe { read_f32(object + OBJECT_POSITION) });
    let predicted_position = add(mul(position_rate, position_change), local_turn_seed);
    let raw_angle = sub(phase_offset, predicted_position);
    let wrapped_angle = wrap_angle(
        raw_angle,
        unsafe { global_f32(ANGLE_LOW) },
        unsafe { global_f32(ANGLE_HIGH) },
        turn_size,
    );
    unsafe { write_f32(object + OBJECT_POSITION, local_turn_seed) };

    let scaled_angle = mul(unsafe { global_f32(ANGLE_SIGN) }, wrapped_angle);
    let mut bounded_angle = scaled_angle;
    if !(one > bounded_angle) {
        bounded_angle = one;
    }
    if !(bounded_angle > unsafe { global_f32(NEGATIVE_ONE) }) {
        bounded_angle = unsafe { global_f32(NEGATIVE_ONE) };
    }
    unsafe { write_f32(object + OBJECT_ANGLE, bounded_angle) };

    let weighted_position = mul(unsafe { global_f32(POSITION_WEIGHT) }, local_turn_seed);
    let nonnegative_position = if zero > weighted_position {
        flip_sign(weighted_position, negative_zero)
    } else {
        weighted_position
    };
    let mut response = add(response_step, nonnegative_position);
    if zero > response_step {
        let response_floor = mul(response_step, unsafe { global_f32(HALF) });
        response = if response_floor > response { response } else { response_floor };
    }
    let mut bounded_response = response;
    if !(one > bounded_response) {
        bounded_response = one;
    }
    let negative_one = unsafe { global_f32(NEGATIVE_ONE) };
    if !(bounded_response > negative_one) {
        bounded_response = negative_one;
    }
    unsafe { write_f32(object + OBJECT_RESPONSE, bounded_response) };

    let result = unsafe { read_u32(object + OBJECT_RESULT) };
    unsafe { write_u32(object + OBJECT_RETURN_COPY, result) };
    result
}

lf_checker_rt::export!(cdecl, rw_input_stick_update(object: u32) -> u32 {
    unsafe { update_stick(object) }
});
