// original: 0x00dcd930 CTaskSimpleReloadGun::UpdatePedState

/// Refresh a reload task's destination vector from its current vector, the
/// ped's weapon transform, or a saved source vector. The routine first asks
/// the ped's weapon manager and requires the weapon-info pointer. Depending
/// on the ped's two reload flags and whether the task's first two vector
/// coordinates are zero, it either copies the current four-word vector,
/// asks the geometry helper for an adjusted point, refreshes the animation
/// subobject, or combines ped and weapon translations. The destination is
/// the four-word field at task offset +0x30. The contract fills the final
/// temporary word with zero where the original reads stack residue.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00DCD930(this_ptr: u32, ped_ptr: u32) -> u32 {
    const WEAPON_MANAGER_FIELD: u32 = 0x2B0;
    const WEAPON_INFO_FIELD: u32 = 0x2C4;
    const PED_RELOAD_FLAG_A: u32 = 0x218;
    const PED_RELOAD_FLAG_B: u32 = 0x219;
    const PED_ANIMATION_MIXER: u32 = 0xBB0;
    const PED_TRANSFORM: u32 = 0x20;
    const PED_WORLD_POSITION: u32 = 0xAA0;
    const TASK_SOURCE_VECTOR: u32 = 0x14;
    const TASK_VECTOR: u32 = 0x20;
    const TASK_DESTINATION_VECTOR: u32 = 0x30;
    const TASK_ANIMATION: u32 = 0x64;
    const ANIMATION_PHASE: u32 = 0x58;
    const TRANSFORM_X: u32 = 0x30;
    const TRANSFORM_Y: u32 = 0x34;
    const TRANSFORM_Z: u32 = 0x38;
    const MANAGER_LOOKUP: u32 = 1;
    const VECTOR_REFRESH: u32 = 2;
    const GEOMETRY_POINT: u32 = 3;
    const ANIMATION_RESET: u32 = 4;

    const OFFSET_A: u32 = 0x01B4_B2A0;
    const OFFSET_B: u32 = 0x01B4_B2A4;
    const OFFSET_C: u32 = 0x01B4_B2A8;
    const NORMALIZE_LIMIT: u32 = 0x00FE_88E8;

    #[inline(always)]
    fn read_word(address: u32) -> u32 {
        // SAFETY: the checker maps every input object and helper output range.
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    fn read_byte(address: u32) -> u8 {
        // SAFETY: the checker maps the ped object containing its flag bytes.
        unsafe { (address as *const u8).read_unaligned() }
    }

    #[inline(always)]
    fn write_word(address: u32, value: u32) {
        // SAFETY: all destinations are fields of the mapped task object.
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    #[inline(always)]
    fn read_float(address: u32) -> f32 {
        f32::from_bits(read_word(address))
    }

    #[inline(always)]
    fn global_float(file_va: u32) -> f32 {
        let relocated = lf_checker_rt::global::<u32>(file_va) as u32;
        read_float(relocated)
    }

    #[inline(always)]
    fn write_float(address: u32, value: f32) {
        write_word(address, value.to_bits());
    }

    #[inline(always)]
    fn add(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }

    #[inline(always)]
    fn subtract(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    }

    #[inline(always)]
    fn multiply(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }

    #[inline(always)]
    fn divide(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) / core::hint::black_box(right)
    }

    #[inline(always)]
    fn square_root(value: f32) -> f32 {
        core::hint::black_box(value).sqrt()
    }

    #[inline(always)]
    fn copy_vector(source: u32, destination: u32) -> u32 {
        let x = read_word(source);
        let y = read_word(source.wrapping_add(4));
        let z = read_word(source.wrapping_add(8));
        let w = read_word(source.wrapping_add(12));
        write_word(destination, x);
        write_word(destination.wrapping_add(4), y);
        write_word(destination.wrapping_add(8), z);
        write_word(destination.wrapping_add(12), w);
        w
    }

    #[inline(always)]
    fn animation_phase(animation: u32) -> u32 {
        read_word(animation.wrapping_add(ANIMATION_PHASE))
    }

    #[inline(always)]
    fn refresh_vector(ped_ptr: u32, vector_ptr: u32, animation: u32) -> u32 {
        lf_checker_rt::callee_thiscall!(
            VECTOR_REFRESH,
            u32,
            ped_ptr.wrapping_add(PED_ANIMATION_MIXER),
            vector_ptr,
            animation_phase(animation),
        )
    }

    let weapon_manager = lf_checker_rt::callee_thiscall!(
        MANAGER_LOOKUP,
        u32,
        ped_ptr.wrapping_add(WEAPON_MANAGER_FIELD),
    );
    let weapon_info = read_word(ped_ptr.wrapping_add(WEAPON_INFO_FIELD));
    if weapon_manager == 0 || weapon_info == 0 {
        return weapon_manager;
    }

    let use_geometry = read_byte(ped_ptr.wrapping_add(PED_RELOAD_FLAG_A)) == 0
        && read_byte(ped_ptr.wrapping_add(PED_RELOAD_FLAG_B)) != 0;
    let current_vector = this_ptr.wrapping_add(TASK_VECTOR);
    let current_x = read_float(current_vector);
    let current_y = read_float(current_vector.wrapping_add(4));
    let current_has_nonzero_xy = current_x != 0.0 && current_y != 0.0;

    if use_geometry && !current_has_nonzero_xy {
        let mut temporary_vectors = [0_u32; 8];
        let base_values = [
            global_float(OFFSET_A).to_bits(),
            global_float(OFFSET_B).to_bits(),
            global_float(OFFSET_C).to_bits(),
        ];
        temporary_vectors[0..3].copy_from_slice(&base_values);
        temporary_vectors[4..7].copy_from_slice(&base_values);
        let output_vector = temporary_vectors.as_mut_ptr() as u32;
        let secondary_vector = output_vector.wrapping_add(16);
        let weapon_transform = read_word(weapon_info.wrapping_add(0x20));

        let mut result = lf_checker_rt::callee_thiscall!(
            GEOMETRY_POINT,
            u32,
            weapon_manager,
            ped_ptr,
            weapon_transform,
            secondary_vector,
            output_vector,
        );

        let transform = read_word(ped_ptr.wrapping_add(PED_TRANSFORM));
        let difference_y = subtract(
            f32::from_bits(temporary_vectors[1]),
            read_float(transform.wrapping_add(TRANSFORM_Y)),
        );
        let difference_z = subtract(
            f32::from_bits(temporary_vectors[2]),
            read_float(transform.wrapping_add(TRANSFORM_Z)),
        );
        let difference_x = subtract(
            f32::from_bits(temporary_vectors[0]),
            read_float(transform.wrapping_add(TRANSFORM_X)),
        );

        let y_squared = multiply(difference_y, difference_y);
        let x_squared = multiply(difference_x, difference_x);
        let xy_squared = add(y_squared, x_squared);
        let z_squared = multiply(difference_z, difference_z);
        let xy_length = square_root(xy_squared);
        let xyz_squared = add(z_squared, xy_squared);
        let xyz_length = square_root(xyz_squared);
        let planar_ratio = divide(xy_length, xyz_length);
        let normalized_remainder = subtract(global_float(NORMALIZE_LIMIT), planar_ratio);

        let weighted_y = multiply(read_float(transform.wrapping_add(4)), difference_y);
        let weighted_x = multiply(difference_x, read_float(transform));
        let weighted_z = multiply(read_float(transform.wrapping_add(8)), difference_z);
        let weighted_xy = add(weighted_y, weighted_x);
        let weighted_xyz = add(weighted_xy, weighted_z);
        let correction_scale = multiply(normalized_remainder, weighted_xyz);

        let corrected_x = subtract(
            f32::from_bits(temporary_vectors[0]),
            multiply(read_float(transform), correction_scale),
        );
        let corrected_y = subtract(
            f32::from_bits(temporary_vectors[1]),
            multiply(read_float(transform.wrapping_add(4)), correction_scale),
        );
        let corrected_z = subtract(
            f32::from_bits(temporary_vectors[2]),
            multiply(read_float(transform.wrapping_add(8)), correction_scale),
        );
        temporary_vectors[0] = corrected_x.to_bits();
        temporary_vectors[1] = corrected_y.to_bits();
        temporary_vectors[2] = corrected_z.to_bits();

        if result & 0xFF != 0 {
            let animation = read_word(this_ptr.wrapping_add(TASK_ANIMATION));
            result = refresh_vector(
                ped_ptr,
                output_vector,
                animation,
            );
        }

        write_word(this_ptr.wrapping_add(TASK_DESTINATION_VECTOR), temporary_vectors[0]);
        write_word(
            this_ptr.wrapping_add(TASK_DESTINATION_VECTOR + 4),
            temporary_vectors[1],
        );
        write_word(
            this_ptr.wrapping_add(TASK_DESTINATION_VECTOR + 8),
            temporary_vectors[2],
        );
        write_word(
            this_ptr.wrapping_add(TASK_DESTINATION_VECTOR + 12),
            temporary_vectors[3],
        );
        return result;
    }

    let animation = read_word(this_ptr.wrapping_add(TASK_ANIMATION));
    if use_geometry {
        if animation != 0 {
            let _ = refresh_vector(ped_ptr, current_vector, animation);
        }
        return copy_vector(current_vector, this_ptr.wrapping_add(TASK_DESTINATION_VECTOR));
    }

    if animation == 0 {
        return weapon_manager;
    }

    if current_has_nonzero_xy {
        let _ = refresh_vector(ped_ptr, current_vector, animation);
        return copy_vector(current_vector, this_ptr.wrapping_add(TASK_DESTINATION_VECTOR));
    }

    let saved_source = read_word(this_ptr.wrapping_add(TASK_SOURCE_VECTOR));
    if saved_source != 0 {
        let component = read_word(saved_source.wrapping_add(0x20));
        let vector_source = if component != 0 {
            component.wrapping_add(0x30)
        } else {
            saved_source.wrapping_add(0x10)
        };
        let _ = refresh_vector(ped_ptr, vector_source, animation);

        let refreshed_component = read_word(saved_source.wrapping_add(0x20));
        let refreshed_source = if refreshed_component != 0 {
            refreshed_component.wrapping_add(0x30)
        } else {
            saved_source.wrapping_add(0x10)
        };
        return copy_vector(
            refreshed_source,
            this_ptr.wrapping_add(TASK_DESTINATION_VECTOR),
        );
    }

    let animation_time = animation_phase(animation);
    let world_position = read_word(ped_ptr.wrapping_add(PED_WORLD_POSITION));
    let _ = lf_checker_rt::callee_thiscall!(
        ANIMATION_RESET,
        u32,
        ped_ptr.wrapping_add(PED_ANIMATION_MIXER),
        world_position,
        0_u32,
        0_u32,
        animation_time,
    );

    let ped_transform = read_word(ped_ptr.wrapping_add(PED_TRANSFORM));
    let weapon_transform = read_word(weapon_info.wrapping_add(0x20));
    let world_x = add(
        read_float(ped_transform.wrapping_add(0x10)),
        read_float(weapon_transform.wrapping_add(TRANSFORM_X)),
    );
    let world_y = add(
        read_float(ped_transform.wrapping_add(0x14)),
        read_float(weapon_transform.wrapping_add(TRANSFORM_Y)),
    );
    let world_z = add(
        read_float(ped_transform.wrapping_add(0x18)),
        read_float(weapon_transform.wrapping_add(TRANSFORM_Z)),
    );
    write_float(this_ptr.wrapping_add(TASK_DESTINATION_VECTOR), world_x);
    write_float(this_ptr.wrapping_add(TASK_DESTINATION_VECTOR + 4), world_y);
    write_float(this_ptr.wrapping_add(TASK_DESTINATION_VECTOR + 8), world_z);
    write_word(this_ptr.wrapping_add(TASK_DESTINATION_VECTOR + 12), 0);
    weapon_transform
}
