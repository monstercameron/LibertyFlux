// original: 0x00DCB9B0 vehicle_collision_entities_are_close

/// Returns whether two entities are separated by less than the squared
/// distance threshold.
///
/// This cdecl predicate takes two entity pointers. Null on either side returns
/// false. Otherwise it reads each position vector at entity offset `+0x20`,
/// computes the second position minus the first in X, Y, Z order, squares the
/// components, and sums Y then X then Z. The squared distance is compared
/// strictly against the global threshold of 900.0; unordered float comparisons
/// return false. The boolean result is returned in AL.
lf_checker_rt::export!(cdecl, rw_00dcb9b0(first_entity: u32, second_entity: u32) -> u8 {
    unsafe {
        const POSITION_POINTER: u32 = 0x20;
        const POSITION_X: u32 = 0x00;
        const POSITION_Y: u32 = 0x04;
        const POSITION_Z: u32 = 0x08;
        const SQUARED_DISTANCE_THRESHOLD_FILE_VA: u32 = 0x00E9CAE0;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            f32::from_bits(unsafe { read_u32(address) })
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
        fn add(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }

        if first_entity == 0 || second_entity == 0 {
            return 0;
        }

        let first_position = read_u32(first_entity.wrapping_add(POSITION_POINTER));
        let second_position = read_u32(second_entity.wrapping_add(POSITION_POINTER));
        let delta_x = subtract(
            read_f32(second_position.wrapping_add(POSITION_X)),
            read_f32(first_position.wrapping_add(POSITION_X)),
        );
        let delta_y = subtract(
            read_f32(second_position.wrapping_add(POSITION_Y)),
            read_f32(first_position.wrapping_add(POSITION_Y)),
        );
        let delta_z = subtract(
            read_f32(second_position.wrapping_add(POSITION_Z)),
            read_f32(first_position.wrapping_add(POSITION_Z)),
        );
        let delta_y_squared = multiply(delta_y, delta_y);
        let delta_x_squared = multiply(delta_x, delta_x);
        let delta_z_squared = multiply(delta_z, delta_z);
        let distance_squared = add(add(delta_y_squared, delta_x_squared), delta_z_squared);
        let threshold = lf_checker_rt::global::<f32>(SQUARED_DISTANCE_THRESHOLD_FILE_VA)
            .read_unaligned();
        u8::from(distance_squared < threshold)
    }
});
