// original: 0x00932330 table_vector_normalized_copy

/// Copy one table vector to `output`, normalize it in place, and return the
/// same pointer. `record_index` selects a 16-byte row containing three `f32`
/// components at offsets `0`, `4`, and `8`; the output is a three-component
/// vector. The squared length is accumulated as `y*y + x*x`, then that sum
/// plus `z*z`, matching the original operand order. A zero vector is written
/// as signed component zeros scaled by positive zero; other vectors are
/// multiplied by the image's unit length divided by the square root of the
/// accumulated length. The cdecl function takes the output pointer followed
/// by the row index and returns the pointer in EAX.
#[inline(always)]
fn mul_in_order(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) * core::hint::black_box(right)
}

#[inline(always)]
fn add_in_order(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) + core::hint::black_box(right)
}

#[inline(always)]
fn div_in_order(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) / core::hint::black_box(right)
}

lf_checker_rt::export!(cdecl, rw_00932330(output: *mut f32, record_index: u32) -> *mut f32 {
    const VECTOR_TABLE_VA: u32 = 0x0103_6D10;
    const VECTOR_STRIDE: u32 = 16;
    const Y_OFFSET: u32 = 4;
    const Z_OFFSET: u32 = 8;
    const UNIT_LENGTH_VA: u32 = 0x00FE_88E8;

    let row = lf_checker_rt::relocated(VECTOR_TABLE_VA)
        .wrapping_add(record_index.wrapping_mul(VECTOR_STRIDE));
    let x = unsafe { (row as *const f32).read_unaligned() };
    let y = unsafe { (row.wrapping_add(Y_OFFSET) as *const f32).read_unaligned() };
    let z = unsafe { (row.wrapping_add(Z_OFFSET) as *const f32).read_unaligned() };

    unsafe {
        output.write_unaligned(x);
        output.wrapping_add(1).write_unaligned(y);
        output.wrapping_add(2).write_unaligned(z);
    }

    let x_squared = mul_in_order(x, x);
    let y_squared = mul_in_order(y, y);
    let z_squared = mul_in_order(z, z);
    let xy_squared = add_in_order(y_squared, x_squared);
    let length_squared = add_in_order(xy_squared, z_squared);
    let scale = if length_squared == 0.0 {
        0.0
    } else {
        let unit_length = unsafe { lf_checker_rt::global::<f32>(UNIT_LENGTH_VA).read_unaligned() };
        let length = core::hint::black_box(length_squared).sqrt();
        div_in_order(unit_length, length)
    };

    let normalized_x = mul_in_order(x, scale);
    let normalized_y = mul_in_order(y, scale);
    let normalized_z = mul_in_order(z, scale);
    unsafe {
        output.write_unaligned(normalized_x);
        output.wrapping_add(1).write_unaligned(normalized_y);
        output.wrapping_add(2).write_unaligned(normalized_z);
    }
    output
});
