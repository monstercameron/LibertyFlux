// original: 0x00B41190 task_geometry_record_build

/// Build the axis-aligned task geometry record from a point descriptor.
///
/// The receiver is a writable record and the sole stack argument is a
/// descriptor. When the descriptor's point field is present, its nested
/// point supplies three single-precision coordinates; a null nested point
/// selects the descriptor's inline coordinates. A null descriptor point
/// asks the vector helper to fill a four-word local vector. The routine
/// copies those coordinates and derives the record's fixed axes and six
/// plane coefficients. All float operations follow the original order and
/// round as `f32`; values are compared bit for bit.
///
/// The record fields are addressed by byte offsets because the source type
/// is not yet identified. The descriptor and point references are 32-bit
/// game pointers; the arithmetic itself is pointer-width independent.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00B41190(this_ptr: u32, descriptor_ptr: u32) {
    const DESCRIPTOR_POINT: u32 = 0x48;
    const NESTED_POINT: u32 = 0x20;
    const INLINE_VECTOR: u32 = 0x10;
    const NESTED_VECTOR: u32 = 0x30;
    const VECTOR_HELPER: u32 = 1;
    const HALF: u32 = 0x00FE8830;
    const ONE: u32 = 0x00FE88E8;
    const THREE_HALVES: u32 = 0x00FE8960;
    const NEGATIVE_ZERO_A: u32 = 0x00FE8D1C;
    const SIGN_BIT: u32 = 0x00FE8FA0;
    const VECTOR_WORDS: usize = 4;

    let read_word = |base: u32, offset: u32| -> u32 {
        // SAFETY: contract inputs provide the receiver, descriptor and
        // selected point storage for every accessed field.
        unsafe { (base.wrapping_add(offset) as *const u32).read_unaligned() }
    };
    let write_word = |base: u32, offset: u32, value: u32| {
        // SAFETY: every output offset is a field within the receiver record.
        unsafe { (base.wrapping_add(offset) as *mut u32).write_unaligned(value) }
    };
    let read_float = |base: u32, offset: u32| -> f32 {
        f32::from_bits(read_word(base, offset))
    };
    let write_float = |base: u32, offset: u32, value: f32| {
        write_word(base, offset, value.to_bits());
    };
    let add = |left: f32, right: f32| -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    };
    let sub = |left: f32, right: f32| -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    };
    let mul = |left: f32, right: f32| -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    };
    let global_float = |address: u32| -> f32 {
        let pointer = lf_checker_rt::global::<u32>(address) as u32;
        f32::from_bits(read_word(pointer, 0))
    };

    let mut local_vector = [0_u32; VECTOR_WORDS];
    let source_pointer = read_word(descriptor_ptr, DESCRIPTOR_POINT);
    let (point_x, point_y, point_z, point_w) = if source_pointer == 0 {
        let local_pointer = local_vector.as_mut_ptr() as u32;
        let _returned_pointer = lf_checker_rt::callee_stdcall!(
            VECTOR_HELPER,
            u32,
            local_pointer,
        );
        (
            f32::from_bits(local_vector[0]),
            f32::from_bits(local_vector[1]),
            f32::from_bits(local_vector[2]),
            f32::from_bits(local_vector[3]),
        )
    } else {
        let nested_pointer = read_word(source_pointer, NESTED_POINT);
        let vector_pointer = if nested_pointer != 0 {
            nested_pointer.wrapping_add(NESTED_VECTOR)
        } else {
            source_pointer.wrapping_add(INLINE_VECTOR)
        };
        (
            read_float(vector_pointer, 0),
            read_float(vector_pointer, 4),
            read_float(vector_pointer, 8),
            0.0,
        )
    };

    write_float(this_ptr, 0x10, point_x);
    write_float(this_ptr, 0x14, point_y);
    write_float(this_ptr, 0x18, point_z);
    write_float(this_ptr, 0x1C, point_w);

    let negative_zero = global_float(NEGATIVE_ZERO_A);
    let half = global_float(HALF);
    let x_minus_half = sub(point_x, half);
    let x_plus_half = add(point_x, half);
    let y_minus_half = sub(point_y, half);
    let y_plus_half = add(point_y, half);

    write_word(this_ptr, 0x108, 0x3F000000);
    let z_plus_three_halves = add(point_z, global_float(THREE_HALVES));
    let z_minus_one = sub(point_z, global_float(ONE));
    write_float(this_ptr, 0x110, z_plus_three_halves);
    write_float(this_ptr, 0x10C, z_minus_one);

    write_word(this_ptr, 0x40, 0);
    write_word(this_ptr, 0x44, 0x3F800000);
    write_word(this_ptr, 0x48, 0);
    write_float(this_ptr, 0x4C, point_w);
    write_word(this_ptr, 0x50, 0xBF800000);
    write_word(this_ptr, 0x54, 0);
    write_word(this_ptr, 0x58, 0);
    write_float(this_ptr, 0x5C, point_w);
    write_word(this_ptr, 0x60, 0);
    write_word(this_ptr, 0x64, 0xBF800000);
    write_word(this_ptr, 0x68, 0);
    write_float(this_ptr, 0x6C, point_w);
    write_word(this_ptr, 0x70, 0x3F800000);
    write_word(this_ptr, 0x74, 0);
    write_word(this_ptr, 0x78, 0);
    write_float(this_ptr, 0x7C, point_w);

    let positive_axis = read_float(this_ptr, 0x48);
    let unit_axis = read_float(this_ptr, 0x44);
    let zero_axis = read_float(this_ptr, 0x40);
    let first_axis_plane = sub(
        mul(positive_axis, negative_zero),
        add(mul(y_plus_half, unit_axis), mul(x_minus_half, zero_axis)),
    );
    write_float(this_ptr, 0xE0, first_axis_plane);

    let negative_x_axis = read_float(this_ptr, 0x50);
    let zero_y_axis = read_float(this_ptr, 0x54);
    let zero_z_axis = read_float(this_ptr, 0x58);
    let second_axis_plane = sub(
        mul(zero_z_axis, negative_zero),
        add(mul(y_minus_half, zero_y_axis), mul(x_minus_half, negative_x_axis)),
    );
    write_float(this_ptr, 0xE4, second_axis_plane);

    let negative_y_axis = read_float(this_ptr, 0x64);
    let zero_axis_y = read_float(this_ptr, 0x60);
    let third_axis_plane = sub(
        mul(read_float(this_ptr, 0x68), negative_zero),
        add(mul(y_minus_half, negative_y_axis), mul(x_plus_half, zero_axis_y)),
    );
    write_float(this_ptr, 0xE8, third_axis_plane);

    let positive_x_axis = read_float(this_ptr, 0x70);
    let zero_axis_w = read_float(this_ptr, 0x74);
    let fourth_axis_plane = sub(
        mul(read_float(this_ptr, 0x78), negative_zero),
        add(mul(y_plus_half, zero_axis_w), mul(x_plus_half, positive_x_axis)),
    );
    write_float(this_ptr, 0xEC, fourth_axis_plane);

    write_word(this_ptr, 0x80, 0);
    write_word(this_ptr, 0x84, 0);
    write_word(this_ptr, 0x88, 0x3F800000);
    write_float(this_ptr, 0x8C, point_w);

    let coordinate_sum = add(read_float(this_ptr, 0x14), read_float(this_ptr, 0x10));
    let near_plane = sub(mul(coordinate_sum, negative_zero), read_float(this_ptr, 0x110));
    write_float(this_ptr, 0xF0, near_plane);
    write_word(this_ptr, 0x90, 0);
    write_word(this_ptr, 0x94, 0);
    write_word(this_ptr, 0x98, 0xBF800000);
    write_float(this_ptr, 0x9C, point_w);
    let far_plane = add(mul(coordinate_sum, negative_zero), read_float(this_ptr, 0x10C));
    write_float(this_ptr, 0xF4, far_plane);

    let sign_bit = global_float(SIGN_BIT).to_bits();
    let axis_x_delta = sub(x_minus_half, read_float(this_ptr, 0x10));
    let axis_y_delta = sub(y_plus_half, read_float(this_ptr, 0x14));
    let flipped_y_delta = f32::from_bits(axis_y_delta.to_bits() ^ sign_bit);
    write_word(this_ptr, 0xA8, 0);
    write_float(this_ptr, 0xAC, point_w);
    write_float(this_ptr, 0xA4, axis_x_delta);
    write_float(this_ptr, 0xA0, flipped_y_delta);
    let corner_mix_sum = add(mul(axis_x_delta, y_plus_half), mul(flipped_y_delta, x_minus_half));
    let corner_mix = f32::from_bits(corner_mix_sum.to_bits() ^ sign_bit);
    write_float(this_ptr, 0xF8, corner_mix);

    let lower_y_delta = sub(y_minus_half, read_float(this_ptr, 0x14));
    let flipped_lower_y = f32::from_bits(lower_y_delta.to_bits() ^ sign_bit);
    let lower_x_delta = sub(x_minus_half, read_float(this_ptr, 0x10));
    write_float(this_ptr, 0xB0, flipped_lower_y);
    write_float(this_ptr, 0xB4, lower_x_delta);
    write_word(this_ptr, 0xB8, 0);
    write_float(this_ptr, 0xBC, point_w);
    let lower_corner_sum = add(
        mul(y_minus_half, read_float(this_ptr, 0xB4)),
        mul(x_minus_half, read_float(this_ptr, 0xB0)),
    );
    let lower_corner = sub(mul(read_float(this_ptr, 0xB8), negative_zero), lower_corner_sum);
    write_float(this_ptr, 0xFC, lower_corner);

    let upper_y_delta = sub(y_minus_half, read_float(this_ptr, 0x14));
    let flipped_upper_y = f32::from_bits(upper_y_delta.to_bits() ^ sign_bit);
    let upper_x_delta = sub(x_plus_half, read_float(this_ptr, 0x10));
    write_float(this_ptr, 0xC0, flipped_upper_y);
    write_float(this_ptr, 0xC4, upper_x_delta);
    write_word(this_ptr, 0xC8, 0);
    write_float(this_ptr, 0xCC, point_w);
    let upper_corner_sum = add(
        mul(y_minus_half, read_float(this_ptr, 0xC4)),
        mul(x_plus_half, read_float(this_ptr, 0xC0)),
    );
    let upper_corner = sub(mul(read_float(this_ptr, 0xC8), negative_zero), upper_corner_sum);
    write_float(this_ptr, 0x100, upper_corner);

    let final_y_delta = sub(y_plus_half, read_float(this_ptr, 0x14));
    let flipped_final_y = f32::from_bits(final_y_delta.to_bits() ^ sign_bit);
    let final_x_delta = sub(x_plus_half, read_float(this_ptr, 0x10));
    write_float(this_ptr, 0xD0, flipped_final_y);
    write_float(this_ptr, 0xD4, final_x_delta);
    write_word(this_ptr, 0xD8, 0);
    write_float(this_ptr, 0xDC, point_w);
    let final_corner_sum = add(
        mul(y_plus_half, read_float(this_ptr, 0xD4)),
        mul(x_plus_half, read_float(this_ptr, 0xD0)),
    );
    let final_corner = sub(mul(read_float(this_ptr, 0xD8), negative_zero), final_corner_sum);
    write_float(this_ptr, 0x104, final_corner);
}
