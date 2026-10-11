// original: 0x00D7C230 ui_update_pass
/// Update pass for one object (original name `ui_update_pass`, 32-bit x86 image).
///
/// Arguments: `outer` is the object; `arg2_slot` is the address of the caller's second
/// float argument, which the flag path may overwrite in place; `arg3_bits` is the third
/// float argument; `state` is the state object that holds the mode byte, the flag byte and
/// the stored floats.
///
/// Order of work:
/// 1. The signed 16-bit heading at the object is stored as a float before any decision.
/// 2. Two remainders of two global counters (each offset by the object's 16-bit field)
///    choose the path: the prefix path when the second is below the first, otherwise the
///    late path directly. Both end in the shared tail.
/// 3. Prefix path: clears one flag bit, calls the outer vector method twice, the sine and
///    cosine helpers, and a block helper. A zero answer clears a second flag bit and goes to
///    the tail. A nonzero answer runs eight flag-site helpers in order (each may set a flag
///    bit and leaves for the tail at once); with all answers clear, the best answer is
///    compared with a search limit: above it the state's float is rewritten and the flag set,
///    at or below it the state word takes the smaller of two values plus ten.
/// 4. Tail: a speed capped when flag 0x40 is set; the heading and target angle from the inner
///    vector's x and y; a turn clamped to [-1, 1]; a phase-dependent width (none beyond a right
///    angle); a local adjustment clamped to [-0.8, 0.8]; a wobble clamped to [-1, 1]; a clamped
///    offset; a trailing factor clamped to [0, 1]; a tail slope clamped to [-1, 1]; the next
///    heading; the trailing output as one plus the clamped offset; and a dot-product output
///    clamped to a symmetric limit (blend floored at 0.5 when flag 0x20 is set).
/// 5. The optional final angle helper runs when the third argument is below 30 and the mode
///    byte is 0x04, 0x0E or 0x1A; the cookie helper closes the function.
///
/// Scope of the proof (see the lane's results): helper answers are scripted per contract;
/// pointer arguments passed to the vector method and the flag-site helpers are not compared.

use lf_checker_rt::{callee_cdecl, callee_thiscall, global};

// Callee ids as the contract declares them.
const C_ATAN2: u32 = 12;
const C_LENGTH_A: u32 = 13;
const C_LENGTH_B: u32 = 14;
const C_COOKIE: u32 = 16;
const C_OUTER_VECTOR: u32 = 17;
const C_INNER_VECTOR: u32 = 17;
const C_SINE: u32 = 18;
const C_COSINE: u32 = 19;
const C_BLOCK: u32 = 20;
const FLAG_SITE_IDS: [u32; 8] = [21, 22, 23, 24, 25, 26, 27, 28];
const FLAG_SITE_CONSTANTS: [u32; 8] = [
    0x3E80_ADFD, 0xBE80_ADFD, 0x3F00_ADFD, 0xBF00_ADFD,
    0x3F49_0FDB, 0xBF49_0FDB, 0x3FA0_D97C, 0xBFA0_D97C,
];

// Offsets in the outer object (the first argument).
const OUTER_SIGNED_HEADING: u32 = 0xE68;
const OUTER_OFFSET_FIELD: u32 = 0x2C;
const STATE_MODE: u32 = 0x26;
const STATE_FINAL_ANGLE: u32 = 0x18;
const STATE_ARG2_VALUE: u32 = 0x1C;
const STATE_SPEED_WORD: u32 = 0x20;
const FLAG_ARG2_OVERRIDE: u8 = 0x20;
const OUTER_SPEED_BYTE: u32 = 0xE6F;
const OUTER_INNER_PTR: u32 = 0x20;
const OUTER_TARGET_REF: u32 = 0x1F74;
const OUTER_DRIFT_REF: u32 = 0x1F78;
const OUTER_STORED_HEADING: u32 = 0x1F00;
const OUTER_TURN: u32 = 0x1EB4;
const OUTER_WOBBLE: u32 = 0x1EB8;
const OUTER_TURN_OUTPUT: u32 = 0x1EBC;
const OUTER_TRAILING_OUTPUT: u32 = 0x1EC0;

// Offsets in the inner object.
const INNER_X: u32 = 0x10;
const INNER_Y: u32 = 0x14;
const INNER_Z: u32 = 0x18;
const INNER_FOLLOW: u32 = 0x38;

// Constants, by the address the original reads them from.
const G_DIVISOR: u32 = 0x0117_35BC;
const G_SCALE_A: u32 = 0x0105_6D90;
const G_TWO: u32 = 0x0105_6D94;
const G_SCALE_B: u32 = 0x0105_6D98;
const G_TURN_SCALE: u32 = 0x00FE_8DB0;
const G_TAU: u32 = 0x00FE_8AEC;
const G_PI: u32 = 0x00FE_8AA0;
const G_NEG_PI: u32 = 0x00FE_8DC4;
const G_BIG: u32 = 0x00FE_879C;
const G_SMALL: u32 = 0x00FE_876C;
const G_ONE: u32 = 0x00FE_88E8;
const G_NEG_ONE: u32 = 0x00FE_8D94;
const G_FIVE: u32 = 0x0105_6D9C;
const G_SPEED_FACTOR: u32 = 0x00FE_8A24;
const G_SCALE_C: u32 = 0x0105_6DA0;
const G_MIN_A: u32 = 0x0105_6DA4;
const G_MAX_A: u32 = 0x0105_6DA8;
const G_RATE: u32 = 0x0105_6DBC;
const G_MIN_B: u32 = 0x0105_6DAC;
const G_WOBBLE_SCALE: u32 = 0x0105_6DB0;
const G_WOBBLE_CAP: u32 = 0x0105_6DB4;
const G_MAX_B: u32 = 0x0105_6DB8;
const G_TAIL_STEP: u32 = 0x0105_6DC4;
const G_TAIL_SCALE: u32 = 0x0105_6DC0;
const G_NORM_SCALE: u32 = 0x00E9_BFC8;
const G_DOT_SCALE: u32 = 0x0105_6DC8;
const G_DOT_LIMIT: u32 = 0x0105_6DCC;
const G_HALF_PI: u32 = 0x00FE_8978;
const G_PHASE: u32 = 0x00EE_C914;
const G_SPEED_CAP: u32 = 0x00FE_8B08;
const G_OPTIONAL_LIMIT: u32 = 0x00FE_8B48;
const G_SEARCH_LIMIT: u32 = 0x00FE_8B5C;
const G_LENGTH_OFFSET: u32 = 0x0105_6D8C;
const G_MOD_A: u32 = 0x0117_35B4;
const G_MOD_B: u32 = 0x0117_35B8;
const G_OBJECT: u32 = 0x012B_9C78;
const MOD_DIVISOR: u32 = 500;

#[inline(always)]
fn fadd(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        a
    } else if b.is_nan() {
        b
    } else {
        core::hint::black_box(a) + core::hint::black_box(b)
    }
}

#[inline(always)]
fn fsub(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        a
    } else if b.is_nan() {
        b
    } else {
        core::hint::black_box(a) - core::hint::black_box(b)
    }
}

#[inline(always)]
fn fmul(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        a
    } else if b.is_nan() {
        b
    } else {
        core::hint::black_box(a) * core::hint::black_box(b)
    }
}

#[inline(always)]
fn fdiv(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        a
    } else if b.is_nan() {
        b
    } else {
        core::hint::black_box(a) / core::hint::black_box(b)
    }
}

#[inline(always)]
fn read_u16(at: u32) -> u16 {
    unsafe { (at as *const u16).read() }
}

#[inline(always)]
fn read_u8(at: u32) -> u8 {
    unsafe { (at as *const u8).read() }
}

#[inline(always)]
fn read_u32(at: u32) -> u32 {
    unsafe { (at as *const u32).read() }
}

#[inline(always)]
fn read_f32(at: u32) -> f32 {
    unsafe { (at as *const f32).read() }
}

#[inline(always)]
fn write_f32(at: u32, value: f32) {
    unsafe { (at as *mut f32).write(value) }
}

#[inline(always)]
fn g32(at: u32) -> f32 {
    unsafe { global::<f32>(at).read() }
}

#[inline(always)]
fn gu32(at: u32) -> u32 {
    unsafe { global::<u32>(at).read() }
}

#[inline(always)]
fn slot_f32(at: u32) -> f32 {
    unsafe { core::ptr::read_volatile(at as *const f32) }
}

#[inline(always)]
fn slot_u32(at: u32) -> u32 {
    unsafe { core::ptr::read_volatile(at as *const u32) }
}

#[inline(always)]
fn set_slot_u32(at: u32, value: u32) {
    unsafe { core::ptr::write_volatile(at as *mut u32, value) }
}

#[inline(always)]
fn write_u16(at: u32, value: u16) {
    unsafe { (at as *mut u16).write(value) }
}

#[inline(always)]
fn write_u8(at: u32, value: u8) {
    unsafe { (at as *mut u8).write(value) }
}

#[inline(always)]
fn sqrt_f32(value: f32) -> f32 {
    unsafe {
        use core::arch::x86::{_mm_cvtss_f32, _mm_set_ss, _mm_sqrt_ss};
        _mm_cvtss_f32(_mm_sqrt_ss(_mm_set_ss(value)))
    }
}

/// Sign flip by bit pattern, as the original does it with a sign mask.
#[inline(always)]
fn flip_sign(value: f32) -> f32 {
    f32::from_bits(value.to_bits() ^ 0x8000_0000)
}

/// Wraps an angle into [-pi, pi]: the upper bound is applied first.
#[inline(always)]
fn wrap_angle_upper_first(mut value: f32) -> f32 {
    let pi = g32(G_PI);
    let tau = g32(G_TAU);
    let neg_pi = g32(G_NEG_PI);
    if value > pi {
        while value > pi {
            value = fsub(value, tau);
        }
    }
    if neg_pi > value {
        while neg_pi > value {
            value = fadd(value, tau);
        }
    }
    value
}

/// Wraps an angle into [-pi, pi]: the lower bound is applied first.
#[inline(always)]
fn wrap_angle_lower_first(mut value: f32) -> f32 {
    let pi = g32(G_PI);
    let tau = g32(G_TAU);
    let neg_pi = g32(G_NEG_PI);
    if neg_pi > value {
        while neg_pi > value {
            value = fadd(value, tau);
        }
    }
    if value > pi {
        while value > pi {
            value = fsub(value, tau);
        }
    }
    value
}

#[inline(always)]
fn as_double_words(value: f32) -> (u32, u32) {
    let bits = (value as f64).to_bits();
    (bits as u32, (bits >> 32) as u32)
}

/// Late branch of the update pass, the one entered when the initial modulo
/// decision selects the tail of the function.
///
/// Stores the signed heading from the outer object as a float, takes the
/// heading from the inner vector's x and y through the angle helper, and
/// works out the wrapped target and turn. It writes the clamped turn, then
/// the normalised heading error with its phase, the width and the local
/// adjustment, the wobble (raw, then floored), the trailing factor and the
/// next heading, the trailing output as one plus the clamped offset, and
/// finally the dot-product output clamped to a symmetric limit. The cookie
/// helper closes the path.
///
/// The contract fixes the modulo decision to this branch, the flag byte
/// clear (so the optional final angle helper, which needs the threshold
/// above the third argument, is absent), finite vector data, a zero speed
/// byte and a zero second argument. It is one branch shape of the function;
/// it does not claim the prefix sine and cosine, the earlier thiscall, the
/// eight other sites, the other flag and mode paths or the optional helper.
/// Whole-function entry: the heading store, the modulo decision, then either
/// the prefix path (taken when the second remainder is below the first) or
/// the late branch directly. Both paths end in the shared tail.
fn run_function(outer: u32, arg2_slot: u32, arg3_bits: u32, state: u32) {
    // Frame output area: the vector method's buffer is its first three words; the
    // flag-site helpers write words 0 to 7 of the same area.
    let mut frame = [0.0_f32; 8];
    let frame_ptr = frame.as_mut_ptr() as u32;
    let arg2_bits = slot_u32(arg2_slot);
    // The heading is the signed slot, stored as a float before the decision.
    let signed_slot = read_u16(outer.wrapping_add(OUTER_SIGNED_HEADING)) as i16;
    let initial_heading = signed_slot as f32;
    let stored_heading = initial_heading;
    write_f32(outer.wrapping_add(OUTER_STORED_HEADING), stored_heading);

    // Modulo decision: two remainders of two global counters, each offset by
    // the object's 16-bit field at +0x2C. The late branch is taken when the
    // second is at least the first.
    let offset_field = u32::from(read_u16(outer.wrapping_add(OUTER_OFFSET_FIELD)));
    let first_rem = gu32(G_MOD_B).wrapping_add(offset_field) % MOD_DIVISOR;
    let second_rem = gu32(G_MOD_A).wrapping_add(offset_field) % MOD_DIVISOR;

    let flags_entry = read_u8(state.wrapping_add(0x2B));
    let flags = if second_rem < first_rem {
        prefix_path(outer, arg2_bits, arg3_bits, state, flags_entry, frame_ptr)
    } else {
        flags_entry
    };
    run_tail(outer, arg2_slot, arg3_bits, state, flags, frame_ptr)
}

/// Prefix path (not the late branch). Clears flag 0x40 on entry, then calls the
/// outer vector method twice with one output buffer, the sine and cosine helpers
/// with the second argument, and the block helper. The clamped length of the
/// second vector is the first argument of the flag-site helpers.
///
/// When the block helper answers zero, the path clears flag 0x20 and reaches the
/// shared tail. When it answers nonzero, the eight flag-site helpers run in
/// order with their own constants; a site whose answer sets bit 0x20 leaves the
/// chain for the tail at once. With all eight answers clear, the best answer is
/// compared with the search limit: above it, the state's second float is
/// rewritten (every candidate word holds that same float) and flag 0x20 is set
/// for the tail; at or below it, the state word at +0x20 takes the smaller of
/// the inner object's truncated +0x38 plus ten and its own value plus ten, and
/// flag 0x20 is cleared. Returns the flag byte the tail starts with.
fn prefix_path(outer: u32, arg2_bits: u32, arg3_bits: u32, state: u32, flags_entry: u8, frame_ptr: u32) -> u8 {
    let flags_on_entry = flags_entry & 0xBF;
    write_u8(state.wrapping_add(0x2B), flags_on_entry);

    let out_ptr = frame_ptr;
    let _first_vector = callee_thiscall!(C_OUTER_VECTOR, u32, outer, out_ptr);
    let second_vector = callee_thiscall!(C_OUTER_VECTOR, u32, outer, out_ptr);

    // Length of the second vector plus a constant, clamped by the third argument.
    let vx = read_f32(second_vector);
    let vy = read_f32(second_vector.wrapping_add(4));
    let vz = read_f32(second_vector.wrapping_add(8));
    let length_sq = fadd(fadd(fmul(vx, vx), fmul(vy, vy)), fmul(vz, vz));
    let length = fadd(sqrt_f32(length_sq), g32(G_LENGTH_OFFSET));
    let arg3 = f32::from_bits(arg3_bits);
    let clamped_length = if arg3 > length { length } else { arg3 };
    let clamped_bits = clamped_length.to_bits();

    let _sine = callee_cdecl!(C_SINE, u32, arg2_bits);
    let _cosine = callee_cdecl!(C_COSINE, u32, arg2_bits);

    let mut mode_word: u32 = 0;
    let mode_word_ptr = core::ptr::addr_of_mut!(mode_word) as u32;
    let block = [0_u32; 3];
    let block_ptr = block.as_ptr() as u32;
    let object = gu32(G_OBJECT);
    let _block_result = callee_thiscall!(
        C_BLOCK,
        u32,
        object,
        mode_word_ptr,
        block_ptr,
        0,
        6,
        0xFFFF_FFFF,
        7,
        1,
        0
    );
    let mode_word_now = unsafe { core::ptr::read_volatile(block_ptr as *const u32) };
    if mode_word_now == 0 {
        let flags_after = flags_on_entry & 0xDF;
        write_u8(state.wrapping_add(0x2B), flags_after);
        return flags_after;
    }

    // Flag-site chain: flag 0x40 is set, then each site writes bit 0x20 from its answer.
    let state_float_at_entry = read_f32(state.wrapping_add(STATE_ARG2_VALUE));
    let mut flags = flags_on_entry | 0x40;
    write_u8(state.wrapping_add(0x2B), flags);
    for site in 0..8 {
        let mut vector = [0.0_f32; 3];
        let vector_ptr = vector.as_mut_ptr() as u32;
        let answer_ptr = frame_ptr.wrapping_add(4 * site as u32);
        let constant = FLAG_SITE_CONSTANTS[site];
        let answer = callee_cdecl!(
            FLAG_SITE_IDS[site],
            u8,
            vector_ptr,
            arg2_bits,
            clamped_bits,
            constant,
            constant,
            state.wrapping_add(STATE_ARG2_VALUE),
            answer_ptr
        );
        flags = (flags & !FLAG_ARG2_OVERRIDE) | ((answer & 1) << 5);
        write_u8(state.wrapping_add(0x2B), flags);
        if flags & FLAG_ARG2_OVERRIDE != 0 {
            return flags;
        }
    }

    // Candidate search over the eight answers, starting from -1.
    let mut best = -1.0_f32;
    for site in 0..8 {
        let answer = slot_f32(frame_ptr.wrapping_add(4 * site as u32));
        if answer > best {
            best = answer;
        }
    }
    if best > g32(G_SEARCH_LIMIT) {
        write_f32(state.wrapping_add(STATE_ARG2_VALUE), state_float_at_entry);
        flags |= FLAG_ARG2_OVERRIDE;
        write_u8(state.wrapping_add(0x2B), flags);
        return flags;
    }

    let inner = read_u32(outer.wrapping_add(OUTER_INNER_PTR));
    let inner_limit = (read_f32(inner.wrapping_add(INNER_FOLLOW)) as i32) + 10;
    let state_word = i32::from(read_u16(state.wrapping_add(STATE_SPEED_WORD)) as i16) + 10;
    let word_value = if state_word < inner_limit { state_word } else { inner_limit };
    write_u16(state.wrapping_add(STATE_SPEED_WORD), word_value as u16);
    flags &= !FLAG_ARG2_OVERRIDE;
    write_u8(state.wrapping_add(0x2B), flags);
    flags
}

/// Shared tail from the late branch: speed, angle and projection, wobble,
/// trailing factor, next heading, the trailing output and the dot output.
fn run_tail(outer: u32, arg2_slot: u32, arg3_bits: u32, state: u32, flags: u8, frame_ptr: u32) {
    if flags & FLAG_ARG2_OVERRIDE != 0 {
        set_slot_u32(arg2_slot, read_u32(state.wrapping_add(STATE_ARG2_VALUE)));
    }
    let arg2_bits = slot_u32(arg2_slot);
    let arg2 = f32::from_bits(arg2_bits);
    let arg3 = f32::from_bits(arg3_bits);

    let inner = read_u32(outer.wrapping_add(OUTER_INNER_PTR));
    let mut speed = read_u8(outer.wrapping_add(OUTER_SPEED_BYTE)) as f32;
    // Flag 0x40 caps the speed at 10.0 before it is used.
    if flags & 0x40 != 0 && speed > g32(G_SPEED_CAP) {
        speed = g32(G_SPEED_CAP);
    }

    // Heading from the inner vector's x and y, and the target offset. The
    // target offset is read from the outer object.
    let x = read_f32(inner.wrapping_add(INNER_X));
    let y = read_f32(inner.wrapping_add(INNER_Y));
    let z = read_f32(inner.wrapping_add(INNER_Z));
    let atan = callee_cdecl!(C_ATAN2, f32, x.to_bits(), y.to_bits());
    let target_angle = wrap_angle_upper_first(fsub(
        atan,
        read_f32(outer.wrapping_add(OUTER_TARGET_REF)),
    ));
    let divisor = g32(G_DIVISOR);
    let scale = fdiv(g32(G_SCALE_A), divisor);
    let scaled_angle = fadd(fmul(scale, target_angle), atan);

    let out_ptr = frame_ptr;
    let outer_vector = callee_thiscall!(C_OUTER_VECTOR, u32, outer, out_ptr);
    let follow = fadd(
        fmul(read_f32(outer_vector.wrapping_add(8)), g32(G_TWO)),
        read_f32(inner.wrapping_add(INNER_FOLLOW)),
    );
    let stored_after_write = read_f32(outer.wrapping_add(OUTER_STORED_HEADING));
    let outer_error = fsub(stored_after_write, follow);
    let rate = fadd(fmul(outer_error, g32(G_BIG)), g32(G_SMALL));
    let bounded_rate = if g32(G_SCALE_B) > rate {
        g32(G_SCALE_B)
    } else {
        rate
    };

    // Turn toward the requested heading, clamped to [-1, 1].
    let delta = wrap_angle_upper_first(fsub(arg2, scaled_angle));
    let turn_unclamped = fmul(delta, g32(G_TURN_SCALE));
    write_f32(outer.wrapping_add(OUTER_TURN), turn_unclamped);
    let turn = if !(g32(G_ONE) > turn_unclamped) {
        g32(G_ONE)
    } else if g32(G_NEG_ONE) > turn_unclamped {
        g32(G_NEG_ONE)
    } else {
        turn_unclamped
    };
    write_f32(outer.wrapping_add(OUTER_TURN), turn);

    // Normalised heading error, its magnitude (the phase angle) and the width.
    let normalized_error = wrap_angle_lower_first(fsub(scaled_angle, arg2));
    let phase_angle = if normalized_error < 0.0 {
        flip_sign(normalized_error)
    } else {
        normalized_error
    };
    let phase = fmul(phase_angle, g32(G_PHASE));
    let mut width = fsub(g32(G_ONE), phase);
    if g32(G_SMALL) > width {
        width = g32(G_SMALL);
    }
    let mut turn_scale = fdiv(arg3, g32(G_FIVE));
    let speed_scale = fmul(speed, g32(G_SPEED_FACTOR));
    if !(speed_scale > turn_scale) {
        turn_scale = speed_scale;
    }
    let scaled_width = fmul(width, turn_scale);
    // A heading error beyond a right angle leaves no width.
    let local_width = if phase_angle > g32(G_HALF_PI) {
        0.0
    } else {
        scaled_width
    };

    // Projection of the inner vector's x and y onto their unit direction.
    let norm_sq = fadd(fmul(x, x), fmul(y, y));
    let unit_scale = if norm_sq == 0.0 {
        0.0
    } else {
        fdiv(g32(G_ONE), sqrt_f32(norm_sq))
    };
    let x_unit = fmul(x, unit_scale);
    let y_unit = fmul(y, unit_scale);
    let z_unit = fmul(unit_scale, 0.0);

    let inner_vector = callee_thiscall!(C_INNER_VECTOR, u32, outer, out_ptr);
    let blend_x = fmul(x_unit, read_f32(inner_vector));
    let blend_y = fmul(read_f32(inner_vector.wrapping_add(4)), y_unit);
    let blend_z = fmul(read_f32(inner_vector.wrapping_add(8)), z_unit);
    let projected = fadd(fadd(blend_y, blend_x), blend_z);

    // Local adjustment: the edge difference scaled and clamped.
    let edge_difference = fsub(local_width, projected);
    let mut local_adjust = fmul(edge_difference, g32(G_SCALE_C));
    if !(local_adjust > g32(G_MIN_A)) {
        local_adjust = g32(G_MIN_A);
    }
    if !(g32(G_MAX_A) > local_adjust) {
        local_adjust = g32(G_MAX_A);
    }

    // Wobble from the first length helper, stored raw and then floored.
    let (z_lo, z_hi) = as_double_words(z);
    let (len_lo, len_hi) = as_double_words(sqrt_f32(norm_sq));
    // The double answer comes back in edx:eax (the checker's f64 answer channel),
    // not in the x87 stack a Rust f64 return would use.
    let length_a = f64::from_bits(callee_cdecl!(C_LENGTH_A, u64, z_lo, z_hi, len_lo, len_hi)) as f32;
    let drift = fsub(length_a, read_f32(outer.wrapping_add(OUTER_DRIFT_REF)));
    let drift_scale = fdiv(g32(G_MIN_B), divisor);
    let raw_wobble = fmul(
        fsub(fadd(fmul(drift, drift_scale), length_a), local_adjust),
        g32(G_WOBBLE_SCALE),
    );
    let capped_wobble = if raw_wobble > g32(G_WOBBLE_CAP) {
        g32(G_WOBBLE_CAP)
    } else {
        raw_wobble
    };
    let wobble_floor = if g32(G_MAX_B) > capped_wobble {
        g32(G_MAX_B)
    } else {
        capped_wobble
    };
    write_f32(outer.wrapping_add(OUTER_WOBBLE), raw_wobble);
    write_f32(outer.wrapping_add(OUTER_WOBBLE), wobble_floor);

    // Clamped offset: the edge difference ramped, added to the rate, clamped.
    let mut offset = fmul(edge_difference, g32(G_RATE));
    if offset > g32(G_ONE) {
        offset = g32(G_ONE);
    } else if 0.0 > offset {
        offset = 0.0;
    }
    let offset_sum = fadd(bounded_rate, offset);
    let clamped_offset = if offset_sum > g32(G_ONE) {
        g32(G_ONE)
    } else if g32(G_NEG_ONE) > offset_sum {
        g32(G_NEG_ONE)
    } else {
        offset_sum
    };

    // Trailing factor from the outer error, clamped to [0, 1].
    let trailing_raw = fsub(g32(G_ONE), fmul(outer_error, g32(G_SMALL)));
    let trailing = if 0.0 > trailing_raw {
        0.0
    } else if trailing_raw > g32(G_ONE) {
        g32(G_ONE)
    } else {
        trailing_raw
    };
    let next_value = fmul(trailing, wobble_floor);
    write_f32(outer.wrapping_add(OUTER_WOBBLE), next_value);

    // Tail slope from the second length helper, clamped to [-1, 1].
    let (z_lo_b, z_hi_b) = as_double_words(z);
    let (len_lo_b, len_hi_b) = as_double_words(sqrt_f32(norm_sq));
    let length_b = f64::from_bits(callee_cdecl!(C_LENGTH_B, u64, z_lo_b, z_hi_b, len_lo_b, len_hi_b)) as f32;
    let tail_drift = fsub(length_b, read_f32(outer.wrapping_add(OUTER_DRIFT_REF)));
    let tail_step = fmul(tail_drift, fdiv(g32(G_TAIL_STEP), divisor));
    let tail_value = fmul(fadd(tail_step, length_b), g32(G_TAIL_SCALE));
    let tail_slope = if tail_value > g32(G_ONE) {
        g32(G_ONE)
    } else if g32(G_NEG_ONE) > tail_value {
        g32(G_NEG_ONE)
    } else {
        tail_value
    };

    // Next heading: the trailing complement times the tail slope, plus the
    // stored wobble product.
    let next_heading = fadd(fmul(fsub(g32(G_ONE), trailing), tail_slope), next_value);
    write_f32(outer.wrapping_add(OUTER_WOBBLE), next_heading);
    write_f32(
        outer.wrapping_add(OUTER_TRAILING_OUTPUT),
        fadd(clamped_offset, g32(G_ONE)),
    );

    // Dot product of the inner vector with the second vector, weighted by
    // the blend, clamped to a symmetric limit.
    let result_vector = callee_thiscall!(C_INNER_VECTOR, u32, outer, out_ptr);
    let dot_x = fmul(read_f32(inner), read_f32(result_vector));
    let dot_y = fmul(read_f32(inner.wrapping_add(4)), read_f32(result_vector.wrapping_add(4)));
    let dot_z = fmul(read_f32(inner.wrapping_add(8)), read_f32(result_vector.wrapping_add(8)));
    let dot = fadd(fadd(dot_y, dot_x), dot_z);
    let tail_blend_raw = fsub(g32(G_ONE), fmul(arg3, g32(G_NORM_SCALE)));
    let mut tail_blend = if 0.0 > tail_blend_raw {
        0.0
    } else {
        tail_blend_raw
    };
    if flags & FLAG_ARG2_OVERRIDE != 0 && !(tail_blend > 0.5) {
        tail_blend = 0.5;
    }
    let mut tail_output = fmul(fmul(dot, g32(G_DOT_SCALE)), tail_blend);
    if tail_output > g32(G_DOT_LIMIT) {
        tail_output = g32(G_DOT_LIMIT);
    }
    let negative_limit = flip_sign(g32(G_DOT_LIMIT));
    let final_output = if negative_limit > tail_output {
        negative_limit
    } else {
        tail_output
    };
    write_f32(outer.wrapping_add(OUTER_TURN_OUTPUT), final_output);

    // Optional final angle helper: it runs only when the third argument is
    // below the threshold (30.0) and the mode byte is one of three values. It
    // stores the angle of the inner object's x and y into the state object at +0x18.
    let mode = read_u8(state.wrapping_add(STATE_MODE));
    if g32(G_OPTIONAL_LIMIT) > arg3 && (mode == 0x04 || mode == 0x0E || mode == 0x1A) {
        let final_angle = callee_cdecl!(C_ATAN2, f32, x.to_bits(), y.to_bits());
        write_f32(state.wrapping_add(STATE_FINAL_ANGLE), final_angle);
    }

    callee_thiscall!(C_COOKIE, u32, 0);
}
