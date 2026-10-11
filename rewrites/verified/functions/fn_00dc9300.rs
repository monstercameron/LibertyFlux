// original: 0x00DC9300 CTaskComplexEvasiveStep::vf19

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, x87_f32};

/// Normalize the task's relative direction and pass its two planar angles to
/// the angle helper. The strict proof exercises the non-null type-1 object,
/// scripts the x87 result, runs the random and supply calls, and uses the
/// null-supply return. It does not cover the vector-prologue branch, the
/// classifier cases, the indirect basis call, or the non-null builder path.
/// The entry's uninitialized stack scratch is fixed to zero by the contract;
/// that value is copied to `this+0x2c`.
///
/// The direction source is `[this+0x40]+0x20+0x30`; the reference source is
/// `[arg1+0x20]+0x30`. Their three float differences are normalized in the
/// original sum and operand order, then stored at `this+0x20..0x28`. The
/// first double call receives the sign-flipped normalized X and normalized
/// Y; the second receives the sign-flipped reference X and reference Y.
/// The x87-returning callee receives a zero stack word after the original
/// clears ECX at its basis gate. The random answer and scripted null supply
/// lead to a zero return.
///
/// Original: 0x00DC9300 (thiscall, one stack word).

const OBJECT_OFF: u32 = 0x40;
const OBJECT_VECTOR_OFF: u32 = 0x20;
const VECTOR_OFF: u32 = 0x30;
const ARG_VECTOR_PTR_OFF: u32 = 0x20;
const ANGLE_X_OFF: u32 = 0x10;
const ANGLE_Y_OFF: u32 = 0x14;
const TASK_MANAGER_GLOBAL: u32 = 0x0167_E2A0;
const SIGN_BIT: u32 = 0x8000_0000;
const ANGLE_CALLEE: u32 = 1;
const X87_CALLEE: u32 = 2;
const RANDOM_CALLEE: u32 = 3;
const SUPPLY_CALLEE: u32 = 4;

#[inline(always)]
fn sub32(left: f32, right: f32) -> f32 {
    core::hint::black_box(core::hint::black_box(left) - core::hint::black_box(right))
}

#[inline(always)]
fn mul32(left: f32, right: f32) -> f32 {
    core::hint::black_box(core::hint::black_box(left) * core::hint::black_box(right))
}

#[inline(always)]
fn add32(left: f32, right: f32) -> f32 {
    core::hint::black_box(core::hint::black_box(left) + core::hint::black_box(right))
}

#[inline(always)]
fn div32(left: f32, right: f32) -> f32 {
    core::hint::black_box(core::hint::black_box(left) / core::hint::black_box(right))
}

#[inline(always)]
fn double_words(value: f32) -> (u32, u32) {
    let widened = core::hint::black_box(core::hint::black_box(value) as f64);
    let bits = widened.to_bits();
    (bits as u32, (bits >> 32) as u32)
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

unsafe fn run(this: u32, arg1: u32, mutant_zero_x: bool) -> u32 {
    let object = unsafe { read_u32(this.wrapping_add(OBJECT_OFF)) };
    let object_vector = unsafe { read_u32(object.wrapping_add(OBJECT_VECTOR_OFF)) };
    let reference = unsafe { read_u32(arg1.wrapping_add(ARG_VECTOR_PTR_OFF)) };

    let dx = sub32(
        unsafe { read_f32(object_vector.wrapping_add(VECTOR_OFF)) },
        unsafe { read_f32(reference.wrapping_add(VECTOR_OFF)) },
    );
    let dy = sub32(
        unsafe { read_f32(object_vector.wrapping_add(VECTOR_OFF + 4)) },
        unsafe { read_f32(reference.wrapping_add(VECTOR_OFF + 4)) },
    );
    let dz = sub32(
        unsafe { read_f32(object_vector.wrapping_add(VECTOR_OFF + 8)) },
        unsafe { read_f32(reference.wrapping_add(VECTOR_OFF + 8)) },
    );

    // The original reads an uninitialized stack word here. The contract
    // defines that word as zero, and the same value is written to this field.
    unsafe { write_u32(this.wrapping_add(0x2c), 0) };

    // Preserve the original SSE order: Y squared plus X squared, then Z.
    let y_squared = mul32(dy, dy);
    let x_squared = mul32(dx, dx);
    let xy_squared = add32(y_squared, x_squared);
    let z_squared = mul32(dz, dz);
    let squared_length = add32(xy_squared, z_squared);
    let reciprocal_length = if squared_length > 0.0 {
        div32(1.0, core::hint::black_box(squared_length).sqrt())
    } else {
        0.0
    };

    let mut normalized_x = mul32(dx, reciprocal_length);
    let normalized_y = mul32(dy, reciprocal_length);
    let normalized_z = mul32(dz, reciprocal_length);
    if mutant_zero_x {
        normalized_x = 0.0;
    }
    unsafe {
        write_u32(this.wrapping_add(0x20), normalized_x.to_bits());
        write_u32(this.wrapping_add(0x24), normalized_y.to_bits());
        write_u32(this.wrapping_add(0x28), normalized_z.to_bits());
    }

    // XOR with the sign mask matches the original bitwise sign flip before
    // the single-precision to double-precision conversions.
    let first_x = f32::from_bits(normalized_x.to_bits() ^ SIGN_BIT);
    let first_y = normalized_y;
    let (first_x_lo, first_x_hi) = double_words(first_x);
    let (first_y_lo, first_y_hi) = double_words(first_y);
    let _first_angle: u64 = callee_cdecl!(
        ANGLE_CALLEE,
        u64,
        first_x_lo,
        first_x_hi,
        first_y_lo,
        first_y_hi,
    );

    let second_x = f32::from_bits(
        unsafe { read_u32(reference.wrapping_add(ANGLE_X_OFF)) } ^ SIGN_BIT,
    );
    let second_y = unsafe { read_f32(reference.wrapping_add(ANGLE_Y_OFF)) };
    let (second_x_lo, second_x_hi) = double_words(second_x);
    let (second_y_lo, second_y_hi) = double_words(second_y);
    let _second_angle: u64 = callee_cdecl!(
        ANGLE_CALLEE,
        u64,
        second_x_lo,
        second_x_hi,
        second_y_lo,
        second_y_hi,
    );

    let x87_argument = x87_f32(0).to_bits();
    let _x87_value: f32 = callee_cdecl!(X87_CALLEE, f32, x87_argument);
    let _random_value: u32 = callee_cdecl!(RANDOM_CALLEE, u32,);
    let task_manager = unsafe { global::<u32>(TASK_MANAGER_GLOBAL).read_unaligned() };
    let supply: u32 = callee_thiscall!(SUPPLY_CALLEE, u32, task_manager);

    // The proof scripts the supply call to return null; the task-building
    // branch is excluded and documented above.
    let _null_supply = supply;
    0
}

export!(thiscall, rw_dc9300(this: u32, arg1: u32) -> u32 {
    unsafe { run(this, arg1, false) }
});

export!(thiscall, mut_dc9300(this: u32, arg1: u32) -> u32 {
    unsafe { run(this, arg1, true) }
});
