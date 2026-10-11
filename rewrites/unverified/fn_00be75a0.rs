// original: 0x00BE75A0 CTaskSimpleSitDown::vf17
/// Copies the position when the task state permits it, optionally issues a
/// movement update, and reports completion in the low return byte. The final
/// interpolation path passes two widened floats to a scripted double helper
/// and forwards its single-precision result to the entity update callback.
use core::hint::black_box;
use lf_checker_rt::{callee_cdecl, callee_thiscall, export};

const TASK_FLAGS: usize = 0x0C;
const TASK_POSITION_X: usize = 0x20;
const TASK_POSITION_Y: usize = 0x24;
const TASK_POSITION_Z: usize = 0x28;
const TASK_SOURCE_X: usize = 0x30;
const TASK_SOURCE_Y: usize = 0x34;
const TASK_SOURCE_Z: usize = 0x38;
const TASK_STATUS: usize = 0x3C;
const TASK_TARGET_X: usize = 0x40;
const TASK_TARGET_Y: usize = 0x44;
const TASK_TARGET_Z: usize = 0x48;
const TASK_VELOCITY_X: usize = 0x50;
const TASK_VELOCITY_Y: usize = 0x54;
const TASK_VELOCITY_Z: usize = 0x58;
const TASK_STATE: usize = 0x60;
const TASK_AUXILIARY: usize = 0x64;
const TASK_DURATION: usize = 0x70;
const TASK_UPDATE_GATE: usize = 0x74;
const TASK_INTERPOLATION_GATE: usize = 0x75;

const ENTITY_FALLBACK: usize = 0x1C;
const ENTITY_POSITION: usize = 0x20;
const ENTITY_VTABLE: usize = 0;
const VTABLE_UPDATE_SLOT: usize = 8;
const POSITION_HELPER_X: usize = 0x10;
const POSITION_HELPER_Y: usize = 0x14;
const POSITION_COPY_START: usize = 0x30;

const CALLEE_EARLY_MOVE: u32 = 1;
const CALLEE_DIRECT_MOVE: u32 = 2;
const CALLEE_CREATE_STATE: u32 = 3;
const CALLEE_DOUBLE_HELPER: u32 = 5;
const CALLEE_FINAL_MOVE: u32 = 6;

const MOVE_MODE: u32 = 0x3505;
const FINAL_MOVE_MODE: u32 = 0x1505;
const SIGN_BIT_F32: u32 = 0x8000_0000;
const LOW_F32_FRAC_MASK: u32 = 0x007F_FFFF;
const LOW_F64_FRAC_MASK: u64 = 0x000F_FFFF_FFFF_FFFF;
const F32_QUIET_BIT: u32 = 0x0040_0000;
const F64_QUIET_BIT: u64 = 0x0008_0000_0000_0000;
const F32_INFINITY: u32 = 0x7F80_0000;
const F64_INFINITY: u64 = 0x7FF0_0000_0000_0000;

#[inline(always)]
unsafe fn read_word(base: *mut u8, offset: usize) -> u32 {
    // SAFETY: the contract supplies mapped objects with the declared fields.
    unsafe { base.add(offset).cast::<u32>().read_unaligned() }
}

#[inline(always)]
unsafe fn read_byte(base: *mut u8, offset: usize) -> u8 {
    // SAFETY: the contract supplies mapped objects with the declared fields.
    unsafe { base.add(offset).read() }
}

#[inline(always)]
unsafe fn read_pointer(base: *mut u8, offset: usize) -> *mut u8 {
    unsafe { read_word(base, offset) as usize as *mut u8 }
}

#[inline(always)]
unsafe fn write_word(base: *mut u8, offset: usize, value: u32) {
    // SAFETY: the contract supplies mapped writable objects with these fields.
    unsafe { base.add(offset).cast::<u32>().write_unaligned(value) }
}

#[inline(always)]
unsafe fn read_float(base: *mut u8, offset: usize) -> f32 {
    f32::from_bits(unsafe { read_word(base, offset) })
}

#[inline(never)]
fn subtract_f32(left: f32, right: f32) -> f32 {
    let left = black_box(left);
    let right = black_box(right);
    black_box(left - right)
}

#[inline(never)]
fn multiply_f32(left: f32, right: f32) -> f32 {
    let left = black_box(left);
    let right = black_box(right);
    black_box(left * right)
}

#[inline(never)]
fn add_f32(left: f32, right: f32) -> f32 {
    let left = black_box(left);
    let right = black_box(right);
    black_box(left + right)
}

/// Converts a binary32 value to the exact binary64 result of scalar widening.
fn widen_f32_bits(bits: u32) -> u64 {
    let sign = u64::from(bits >> 31) << 63;
    let exponent = (bits >> 23) & 0xFF;
    let fraction = bits & LOW_F32_FRAC_MASK;
    if exponent == 0xFF {
        if fraction == 0 {
            return sign | F64_INFINITY;
        }
        return sign
            | F64_INFINITY
            | (u64::from(fraction) << 29)
            | F64_QUIET_BIT;
    }
    if exponent == 0 {
        if fraction == 0 {
            return sign;
        }
        let leading = 31 - fraction.leading_zeros();
        let shift = 23 - leading;
        let normalized = fraction << shift;
        let unbiased = leading as i32 - 149;
        let biased = (unbiased + 1023) as u64;
        return sign | (biased << 52) | (u64::from(normalized & LOW_F32_FRAC_MASK) << 29);
    }
    let widened_exponent = u64::from(exponent + 896);
    sign | (widened_exponent << 52) | (u64::from(fraction) << 29)
}

fn round_shift_right(value: u64, shift: u32) -> u64 {
    if shift == 0 {
        return value;
    }
    if shift >= 64 {
        return 0;
    }
    let quotient = value >> shift;
    let remainder = value & ((1u64 << shift) - 1);
    let halfway = 1u64 << (shift - 1);
    if remainder > halfway || (remainder == halfway && quotient & 1 != 0) {
        quotient + 1
    } else {
        quotient
    }
}

/// Narrows a binary64 result as a round-to-nearest-even scalar conversion.
fn narrow_f64_bits(bits: u64) -> u32 {
    let sign = (bits >> 32) as u32 & SIGN_BIT_F32;
    let exponent = ((bits >> 52) & 0x7FF) as u32;
    let fraction = bits & LOW_F64_FRAC_MASK;
    if exponent == 0x7FF {
        if fraction == 0 {
            return sign | F32_INFINITY;
        }
        let payload = ((fraction >> 29) as u32) | F32_QUIET_BIT;
        return sign | F32_INFINITY | payload;
    }
    if exponent == 0 {
        return sign;
    }

    let mut f32_exponent = exponent as i32 - 896;
    let significand = (1u64 << 52) | fraction;
    if f32_exponent >= 255 {
        return sign | F32_INFINITY;
    }
    if f32_exponent > 0 {
        let mut rounded = round_shift_right(significand, 29);
        if rounded >= (1 << 24) {
            rounded >>= 1;
            f32_exponent += 1;
        }
        if f32_exponent >= 255 {
            return sign | F32_INFINITY;
        }
        return sign | ((f32_exponent as u32) << 23) | (rounded as u32 & LOW_F32_FRAC_MASK);
    }

    let shift = (30 - f32_exponent) as u32;
    let rounded = round_shift_right(significand, shift);
    sign | rounded as u32
}

fn low_high_words(value: u64) -> (u32, u32) {
    (value as u32, (value >> 32) as u32)
}

unsafe fn run(this: *mut u8, entity: *mut u8, mutate_double: bool) -> u32 {
    let state_flags = unsafe { read_word(this, TASK_FLAGS) };
    if state_flags & 4 == 0 {
        let source = unsafe { read_pointer(entity, ENTITY_POSITION) };
        for offset in [TASK_SOURCE_X, TASK_SOURCE_Y, TASK_SOURCE_Z, TASK_STATUS] {
            let value = unsafe { read_word(source, POSITION_COPY_START + offset - TASK_SOURCE_X) };
            unsafe { write_word(this, offset, value) };
        }
    }

    if unsafe { read_byte(this, TASK_UPDATE_GATE) } != 0 {
        let auxiliary = unsafe { read_pointer(this, TASK_AUXILIARY) };
        if !auxiliary.is_null() {
            let duration = unsafe { read_word(this, TASK_DURATION) };
            let vector = unsafe { this.add(TASK_TARGET_X) } as u32;
            let _: u32 = callee_thiscall!(
                CALLEE_EARLY_MOVE,
                u32,
                entity as u32,
                auxiliary as u32,
                0,
                MOVE_MODE,
                vector,
                duration,
                0
            );
        }
        return 1;
    }

    if unsafe { read_pointer(this, TASK_STATE) }.is_null() {
        let _: u32 = callee_thiscall!(CALLEE_CREATE_STATE, u32, this as u32, entity as u32);
    }
    let state = unsafe { read_pointer(this, TASK_STATE) };
    if state.is_null() {
        return 0;
    }

    let auxiliary = unsafe { read_pointer(this, TASK_AUXILIARY) };
    if !auxiliary.is_null() {
        let scale = subtract_f32(1.0, unsafe { read_float(state, 0x4C) });
        let x_step = multiply_f32(unsafe { read_float(this, TASK_VELOCITY_X) }, scale);
        let y_step = multiply_f32(unsafe { read_float(this, TASK_VELOCITY_Y) }, scale);
        let z_step = multiply_f32(unsafe { read_float(this, TASK_VELOCITY_Z) }, scale);
        let mut target = [
            add_f32(x_step, unsafe { read_float(this, TASK_TARGET_X) }),
            add_f32(y_step, unsafe { read_float(this, TASK_TARGET_Y) }),
            add_f32(z_step, unsafe { read_float(this, TASK_TARGET_Z) }),
        ];
        let _: u32 = callee_thiscall!(
            CALLEE_DIRECT_MOVE,
            u32,
            entity as u32,
            auxiliary as u32,
            0,
            MOVE_MODE,
            target.as_mut_ptr() as u32,
            unsafe { read_word(this, TASK_DURATION) },
            0
        );
        return 0;
    }

    if unsafe { read_byte(this, TASK_INTERPOLATION_GATE) } == 0 {
        return 0;
    }

    let fraction = unsafe { read_float(state, 0x4C) };
    let mut interpolated = [0.0f32; 3];
    for (index, (target_offset, source_offset)) in [
        (TASK_POSITION_X, TASK_SOURCE_X),
        (TASK_POSITION_Y, TASK_SOURCE_Y),
        (TASK_POSITION_Z, TASK_SOURCE_Z),
    ]
    .into_iter()
    .enumerate()
    {
        let destination = add_f32(
            unsafe { read_float(this, target_offset) },
            unsafe { read_float(this, [TASK_TARGET_X, TASK_TARGET_Y, TASK_TARGET_Z][index]) },
        );
        let distance = subtract_f32(destination, unsafe { read_float(this, source_offset) });
        let movement = multiply_f32(distance, fraction);
        interpolated[index] = add_f32(unsafe { read_float(this, source_offset) }, movement);
    }

    let vtable = unsafe { read_pointer(entity, ENTITY_VTABLE) };
    let callback = unsafe { read_word(vtable, VTABLE_UPDATE_SLOT) };
    let callback: extern "thiscall" fn(u32, u32, u32, u32) -> u32 = unsafe {
        core::mem::transmute(callback as usize)
    };
    let _: u32 = callback(entity as u32, interpolated.as_mut_ptr() as u32, 0, 1);

    let position = unsafe { read_pointer(entity, ENTITY_POSITION) };
    let result_bits = if position.is_null() {
        unsafe { read_word(entity, ENTITY_FALLBACK) }
    } else {
        let x_single = unsafe { read_word(position, POSITION_HELPER_X) } ^ SIGN_BIT_F32;
        let y_single = unsafe { read_word(position, POSITION_HELPER_Y) };
        let mut x_double = widen_f32_bits(x_single);
        let y_double = widen_f32_bits(y_single);
        if mutate_double {
            x_double ^= 1u64 << 63;
        }
        let (x_low, x_high) = low_high_words(x_double);
        let (y_low, y_high) = low_high_words(y_double);
        let answer: u64 = callee_cdecl!(
            CALLEE_DOUBLE_HELPER,
            u64,
            x_low,
            x_high,
            y_low,
            y_high
        );
        narrow_f64_bits(answer)
    };

    let update_data = (position as u32).wrapping_add(POSITION_COPY_START as u32);
    let _: u32 = callee_thiscall!(
        CALLEE_FINAL_MOVE,
        u32,
        entity as u32,
        FINAL_MOVE_MODE,
        update_data,
        result_bits,
        0
    );
    0
}

export!(thiscall, rw_fn_00be75a0(this: *mut u8, entity: *mut u8) -> u32 {
    unsafe { run(this, entity, false) }
});
