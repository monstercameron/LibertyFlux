// original: 0x00BC3890 try_update_nearest_ped_candidate

/// Update the nearest eligible candidate when the helper accepts the entity
/// and its signed 16-bit area at +0x2e matches the signed current area, or
/// the current area is -1. Position comes from the pointer at +0x20 plus
/// +0x30 when present, otherwise from +0x10; the reference is three global
/// float words. Compare dy² + dx² + dz² in that operand order with the
/// previous distance, update candidate and distance globals when nearer,
/// and return ready status 1. The cdecl function takes one entity pointer.
lf_checker_rt::export!(cdecl, rw_00BC3890(entity: u32) -> u32 {
    unsafe fn update_nearest_candidate(entity: u32, include_vertical: bool) -> u32 {
        const ENTITY_POSITION: u32 = 0x10;
        const ENTITY_POSITION_POINTER: u32 = 0x20;
        const ENTITY_AREA: u32 = 0x2e;
        const POINTER_POSITION: u32 = 0x30;
        const CURRENT_AREA: u32 = 0x0104_7380;
        const CURRENT_CANDIDATE: u32 = 0x0167_f114;
        const CURRENT_DISTANCE: u32 = 0x0167_f118;
        const REFERENCE_POSITION: u32 = 0x0167_f120;
        const AREA_WILDCARD: i32 = -1;
        const DISTANCE_HELPER: u32 = 1;
        const HELPER_KIND: u32 = 0x11;
    
        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_i16(address: u32) -> i16 {
            unsafe { (address as *const i16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_float(address: u32) -> f32 {
            f32::from_bits(unsafe { read_word(address) })
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
        #[inline(always)]
        fn comiss_jbe(left: f32, right: f32) -> bool {
            left.is_nan() || right.is_nan() || left <= right
        }
    
        let position_pointer = unsafe { read_word(entity.wrapping_add(ENTITY_POSITION_POINTER)) };
        let position = if position_pointer != 0 {
            position_pointer.wrapping_add(POINTER_POSITION)
        } else {
            entity.wrapping_add(ENTITY_POSITION)
        };
        let reference_x = unsafe {
            f32::from_bits(lf_checker_rt::global::<u32>(REFERENCE_POSITION).read_unaligned())
        };
        let reference_y = unsafe {
            f32::from_bits(lf_checker_rt::global::<u32>(REFERENCE_POSITION + 4).read_unaligned())
        };
        let reference_z = unsafe {
            f32::from_bits(lf_checker_rt::global::<u32>(REFERENCE_POSITION + 8).read_unaligned())
        };
        let dx = subtract(unsafe { read_float(position) }, reference_x);
        let dy = subtract(unsafe { read_float(position + 4) }, reference_y);
        let dz = subtract(unsafe { read_float(position + 8) }, reference_z);
        let dy_squared = multiply(dy, dy);
        let dx_squared = multiply(dx, dx);
        let dz_squared = multiply(dz, dz);
        let horizontal_squared = add(dy_squared, dx_squared);
        let squared_distance = if include_vertical {
            add(horizontal_squared, dz_squared)
        } else {
            horizontal_squared
        };
    
        let helper_result = lf_checker_rt::callee_thiscall!(
            DISTANCE_HELPER,
            u32,
            entity,
            HELPER_KIND,
            lf_checker_rt::relocated(REFERENCE_POSITION),
        );
        if helper_result != 0 {
            let entity_area = unsafe { read_i16(entity.wrapping_add(ENTITY_AREA)) };
            let current_area = unsafe { lf_checker_rt::global::<i32>(CURRENT_AREA).read_unaligned() };
            if i32::from(entity_area) == current_area || current_area == AREA_WILDCARD {
                let previous_candidate = unsafe {
                    lf_checker_rt::global::<u32>(CURRENT_CANDIDATE).read_unaligned()
                };
                let should_replace = if previous_candidate == 0 {
                    true
                } else {
                    let previous_distance = unsafe {
                        f32::from_bits(
                            lf_checker_rt::global::<u32>(CURRENT_DISTANCE).read_unaligned(),
                        )
                    };
                    !comiss_jbe(previous_distance, squared_distance)
                };
                if should_replace {
                    unsafe {
                        lf_checker_rt::global::<u32>(CURRENT_CANDIDATE).write_unaligned(entity);
                        lf_checker_rt::global::<f32>(CURRENT_DISTANCE).write_unaligned(squared_distance);
                    }
                }
            }
        }
        1
    }
    unsafe { update_nearest_candidate(entity, true) }
});
