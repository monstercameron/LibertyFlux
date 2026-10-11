// original: 0x009EA990 orthonormalize_basis_matrix
#![no_std]
#![allow(unsafe_code)]

use lf_checker_rt::{callee_thiscall, export};

const BASIS_0: u32 = 0x00;
const BASIS_1: u32 = 0x04;
const BASIS_2: u32 = 0x08;
const BASIS_3: u32 = 0x10;
const BASIS_4: u32 = 0x14;
const BASIS_5: u32 = 0x18;
const BASIS_6: u32 = 0x20;
const BASIS_7: u32 = 0x24;
const BASIS_8: u32 = 0x28;

const BASIS_SCALE: u32 = 0x017A_D148;
const BASIS_EPSILON: u32 = 0x0110_DAD8;
const BASIS_EPSILON_Y: u32 = 0x0110_DAD4;
const BASIS_EPSILON_Z: u32 = 0x0110_DAD0;
const CROSS_EPSILON: u32 = 0x00FE_8670;
const ONE: u32 = 0x00FE_88E8;
const CROSS_AXIS_X: u32 = 0x0110_DB54;
const CROSS_AXIS_Y: u32 = 0x0110_DB50;
const CROSS_AXIS_Z: u32 = 0x0110_DB58;
const CROSS_AXIS_FALLBACK: u32 = 0x0110_DB70;
const FALLBACK_SIGN_MASK: u32 = 0x0110_DB00;
const HELPER_ID: u32 = 1;

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
    unsafe {
        let input = core::arch::x86::_mm_set_ss(core::hint::black_box(value));
        core::arch::x86::_mm_cvtss_f32(core::arch::x86::_mm_sqrt_ss(input))
    }
}

#[inline(always)]
fn read_word(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
fn write_word(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
fn read_float(address: u32) -> f32 {
    f32::from_bits(read_word(address))
}

#[inline(always)]
fn write_float(address: u32, value: f32) {
    write_word(address, value.to_bits())
}

#[inline(always)]
unsafe fn read_basis(this: u32, offset: u32) -> f32 {
    unsafe { read_float(this.wrapping_add(offset)) }
}

#[inline(always)]
unsafe fn write_basis(this: u32, offset: u32, value: f32) {
    unsafe { write_float(this.wrapping_add(offset), value) }
}

#[inline(always)]
fn read_global_float(file_va: u32) -> f32 {
    let address = lf_checker_rt::relocated(file_va);
    read_float(address)
}

#[inline(always)]
fn masked_component(value: f32, mask: f32, fallback: f32) -> f32 {
    f32::from_bits((value.to_bits() & mask.to_bits()) | (!mask.to_bits() & fallback.to_bits()))
}

/// Rebuild an orthonormal three-row basis in place.
///
/// The receiver stores nine `f32` components at offsets `0x00` through
/// `0x28`, with rows at `0x00`, `0x10`, and `0x20`. The last row is normalized
/// first. The first row is formed from the cross product of the middle and
/// last rows; when that product is too short, the routine tries the two
/// fixed basis axes and delegates the final degenerate case to its vector
/// helper. The middle row is then rebuilt from the first and last rows.
///
/// The squared-length and cross-product tests use strict ordered greater-than
/// comparisons; an unordered NaN takes the fallback path. Arithmetic keeps
/// the original operand and accumulation order so stored float bits, including
/// signed zeros and NaNs, remain observable. The helper receives a pointer to
/// a temporary three-word vector and one shared sign-mask address.
unsafe fn orthonormalize_basis(this: u32) {
    let last_x = unsafe { read_basis(this, BASIS_6) };
    let last_y = unsafe { read_basis(this, BASIS_7) };
    let last_z = unsafe { read_basis(this, BASIS_8) };

    let last_y_squared = mul(last_y, last_y);
    let last_x_squared = mul(last_x, last_x);
    let first_two_squares = add(last_x_squared, last_y_squared);
    let last_z_squared = mul(last_z, last_z);
    let last_length_squared = add(first_two_squares, last_z_squared);
    let normalization_epsilon = read_global_float(BASIS_EPSILON);
    let one = read_global_float(ONE);
    let inverse_last_length = div(one, sqrt(last_length_squared));

    let basis_scale = read_global_float(BASIS_SCALE);
    let selected_scale_x = if last_length_squared > read_global_float(BASIS_EPSILON_Z) { basis_scale } else { 0.0 };
    let selected_scale_y = if last_length_squared > read_global_float(BASIS_EPSILON_Y) { basis_scale } else { 0.0 };
    let selected_scale_z = if last_length_squared > normalization_epsilon { basis_scale } else { 0.0 };
    let normalized_last_x = mul(inverse_last_length, last_x);
    let normalized_last_y = mul(inverse_last_length, last_y);
    let normalized_last_z = mul(inverse_last_length, last_z);
    let fallback_axis = lf_checker_rt::relocated(CROSS_AXIS_FALLBACK);
    let normalized_row_x = masked_component(normalized_last_x, selected_scale_x, 0.0);
    let normalized_row_y = masked_component(normalized_last_y, selected_scale_y, 0.0);
    let normalized_row_z = masked_component(
        normalized_last_z,
        selected_scale_z,
        f32::from_bits(read_word(fallback_axis.wrapping_add(8))),
    );
    unsafe {
        write_basis(this, BASIS_6, normalized_row_x);
        write_basis(this, BASIS_7, normalized_row_y);
        write_basis(this, BASIS_8, normalized_row_z);
    }

    let middle_x = unsafe { read_basis(this, BASIS_3) };
    let middle_y = unsafe { read_basis(this, BASIS_4) };
    let middle_z = unsafe { read_basis(this, BASIS_5) };
    let cross_x = sub(mul(middle_y, normalized_row_z), mul(middle_z, normalized_row_y));
    let cross_y = sub(mul(middle_z, normalized_row_x), mul(normalized_row_z, middle_x));
    let cross_z = sub(mul(normalized_row_y, middle_x), mul(middle_y, normalized_row_x));
    unsafe {
        write_basis(this, BASIS_0, cross_x);
        write_basis(this, BASIS_1, cross_y);
        write_basis(this, BASIS_2, cross_z);
    }

    let cross_x_squared = mul(cross_x, cross_x);
    let cross_y_squared = mul(cross_y, cross_y);
    let cross_partial = add(cross_x_squared, cross_y_squared);
    let cross_z_squared = mul(cross_z, cross_z);
    let cross_length_squared = add(cross_partial, cross_z_squared);
    let cross_epsilon = read_global_float(CROSS_EPSILON);

    let (first_x, first_y, first_z) = if cross_length_squared > cross_epsilon {
        let inverse_cross_length = div(one, sqrt(cross_length_squared));
        (
            mul(cross_x, inverse_cross_length),
            mul(cross_y, inverse_cross_length),
            mul(cross_z, inverse_cross_length),
        )
    } else {
        let axis_x = read_global_float(CROSS_AXIS_X);
        let axis_y = read_global_float(CROSS_AXIS_Y);
        let axis_z = read_global_float(CROSS_AXIS_Z);
        let candidate_x = sub(mul(normalized_row_z, axis_x), mul(normalized_row_y, axis_z));
        let candidate_y = sub(mul(normalized_row_x, axis_z), mul(normalized_row_z, axis_y));
        let candidate_z = sub(mul(normalized_row_y, axis_y), mul(normalized_row_x, axis_x));

        let candidate_y_squared = mul(candidate_y, candidate_y);
        let candidate_x_squared = mul(candidate_x, candidate_x);
        let candidate_partial = add(candidate_x_squared, candidate_y_squared);
        let candidate_z_squared = mul(candidate_z, candidate_z);
        let candidate_length_squared = add(candidate_partial, candidate_z_squared);

        if candidate_length_squared > cross_epsilon {
            let inverse_candidate_length = div(one, sqrt(candidate_length_squared));
            (
                mul(candidate_x, inverse_candidate_length),
                mul(candidate_y, inverse_candidate_length),
                mul(candidate_z, inverse_candidate_length),
            )
        } else {
            let middle_x = unsafe { read_basis(this, BASIS_3) };
            let middle_y = unsafe { read_basis(this, BASIS_4) };
            let middle_z = unsafe { read_basis(this, BASIS_5) };
            let helper_x = sub(mul(middle_y, read_global_float(0x0110_DB78)), mul(middle_z, read_global_float(0x0110_DB74)));
            let helper_y = sub(mul(middle_z, read_global_float(0x0110_DB70)), mul(middle_x, read_global_float(0x0110_DB78)));
            let helper_z = sub(mul(middle_x, read_global_float(0x0110_DB74)), mul(middle_y, read_global_float(0x0110_DB70)));
            let mut helper_vector = [helper_x.to_bits(), helper_y.to_bits(), helper_z.to_bits()];
            let helper_pointer = helper_vector.as_mut_ptr() as u32;
            let sign_mask_pointer = lf_checker_rt::relocated(FALLBACK_SIGN_MASK);
            unsafe {
                let _ = callee_thiscall!(HELPER_ID, u32, helper_pointer, sign_mask_pointer);
            }
            (
                f32::from_bits(helper_vector[0]),
                f32::from_bits(helper_vector[1]),
                f32::from_bits(helper_vector[2]),
            )
        }
    };

    unsafe {
        write_basis(this, BASIS_0, first_x);
        write_basis(this, BASIS_1, first_y);
        write_basis(this, BASIS_2, first_z);
    }

    let second_x = sub(mul(first_z, normalized_row_y), mul(first_y, normalized_row_z));
    let second_y = sub(mul(first_x, normalized_row_z), mul(first_z, normalized_row_x));
    let second_z = sub(mul(first_y, normalized_row_x), mul(first_x, normalized_row_y));
    unsafe {
        write_basis(this, BASIS_3, second_x);
        write_basis(this, BASIS_4, second_y);
        write_basis(this, BASIS_5, second_z);
    }

}

export!(thiscall, rw_009ea990(this: u32) -> () {
    unsafe { orthonormalize_basis(this) }
});


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
