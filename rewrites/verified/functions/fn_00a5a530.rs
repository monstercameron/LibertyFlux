// original: 0x00A5A530 entity_within_range_in_front

/// Tests whether the first entity's position is within the squared radius
/// stored by the game and lies in front of the second entity. The displacement
/// is first minus second. Distance is accumulated as Y² + X² + Z²; the forward
/// dot product is Y·dy + X·dx + Z·dz. Both comparisons are ordered and strict,
/// so equal thresholds and NaNs fail. The two entity pointers are cdecl stack
/// arguments; AL returns the Boolean result.
lf_checker_rt::export!(cdecl, rw_00A5A530(first_entity: u32, second_entity: u32) -> u32 {
    const POSITION_POINTER: u32 = 0x20;
    const POSITION: u32 = 0x30;
    const DIRECTION: u32 = 0x10;
    const RANGE_SQUARED_VA: u32 = 0x00e9d650;
    const FRONT_THRESHOLD_VA: u32 = 0x00fe8628;

    #[inline(always)]
    fn subtract_f32(left: f32, right: f32) -> f32 {
        let left = core::hint::black_box(left);
        let right = core::hint::black_box(right);
        left - right
    }

    #[inline(always)]
    fn multiply_f32(left: f32, right: f32) -> f32 {
        let left = core::hint::black_box(left);
        let right = core::hint::black_box(right);
        left * right
    }

    #[inline(always)]
    fn add_f32(left: f32, right: f32) -> f32 {
        let left = core::hint::black_box(left);
        let right = core::hint::black_box(right);
        left + right
    }

    #[inline(always)]
    fn ordered_greater(left: f32, right: f32) -> bool {
        let left = core::hint::black_box(left);
        let right = core::hint::black_box(right);
        left > right
    }

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_f32(address: u32) -> f32 {
        f32::from_bits(unsafe { read_u32(address) })
    }

    let first_position = unsafe { read_u32(first_entity.wrapping_add(POSITION_POINTER)) };
    let second_position = unsafe { read_u32(second_entity.wrapping_add(POSITION_POINTER)) };
    let delta_x = subtract_f32(
        unsafe { read_f32(first_position.wrapping_add(POSITION)) },
        unsafe { read_f32(second_position.wrapping_add(POSITION)) },
    );
    let delta_y = subtract_f32(
        unsafe { read_f32(first_position.wrapping_add(POSITION + 4)) },
        unsafe { read_f32(second_position.wrapping_add(POSITION + 4)) },
    );
    let delta_z = subtract_f32(
        unsafe { read_f32(first_position.wrapping_add(POSITION + 8)) },
        unsafe { read_f32(second_position.wrapping_add(POSITION + 8)) },
    );

    let x_squared = multiply_f32(delta_x, delta_x);
    let y_squared = multiply_f32(delta_y, delta_y);
    let xy_squared = add_f32(y_squared, x_squared);
    let z_squared = multiply_f32(delta_z, delta_z);
    let distance_squared = add_f32(xy_squared, z_squared);
    let maximum_distance_squared = unsafe {
        lf_checker_rt::global::<f32>(RANGE_SQUARED_VA).read_unaligned()
    };
    let is_near = ordered_greater(maximum_distance_squared, distance_squared);
    if !is_near {
        return 0;
    }

    let direction_x = unsafe { read_f32(second_position.wrapping_add(DIRECTION)) };
    let direction_y = unsafe { read_f32(second_position.wrapping_add(DIRECTION + 4)) };
    let direction_z = unsafe { read_f32(second_position.wrapping_add(DIRECTION + 8)) };
    let x_projection = multiply_f32(direction_x, delta_x);
    let y_projection = multiply_f32(direction_y, delta_y);
    let xy_projection = add_f32(y_projection, x_projection);
    let z_projection = multiply_f32(direction_z, delta_z);
    let forward_projection = add_f32(xy_projection, z_projection);
    let front_threshold = unsafe {
        lf_checker_rt::global::<f32>(FRONT_THRESHOLD_VA).read_unaligned()
    };
    u32::from(ordered_greater(forward_projection, front_threshold))
});
