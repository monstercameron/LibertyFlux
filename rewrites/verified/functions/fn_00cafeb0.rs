// original: 0x00CAFEB0 clamp_distance_to_task_radius

/// Return the stored fallback value when the point pointer is null. Otherwise
/// compute the planar distance from the point to the target's position, scale
/// it by the game's distance conversion factor, and cap it at this task's
/// radius. The target pointer is supplied through the first stack argument;
/// the optional point pointer is the second. Unordered distance/radius
/// comparisons retain the computed distance, matching the original branch.
lf_checker_rt::export!(thiscall, rw_00cafeb0(this: u32, target: u32, point: u32) -> f32 {
    #[inline(never)]
    fn ordered_sub(left: f32, right: f32) -> f32 {
        let left = core::hint::black_box(left);
        let right = core::hint::black_box(right);
        core::hint::black_box(left - right)
    }
    #[inline(never)]
    fn ordered_mul(left: f32, right: f32) -> f32 {
        let left = core::hint::black_box(left);
        let right = core::hint::black_box(right);
        core::hint::black_box(left * right)
    }
    #[inline(never)]
    fn ordered_add(left: f32, right: f32) -> f32 {
        let left = core::hint::black_box(left);
        let right = core::hint::black_box(right);
        core::hint::black_box(left + right)
    }

    const FALLBACK_VALUE: u32 = 0x18;
    const TASK_RADIUS: u32 = 0x24;
    const TARGET_STATE: u32 = 0x20;
    const TARGET_X: u32 = 0x30;
    const TARGET_Y: u32 = 0x34;
    const DISTANCE_SCALE: u32 = 0x00FE_8A24;

    unsafe {
        if point == 0 {
            return ((this + FALLBACK_VALUE) as *const f32).read_unaligned();
        }

        let point_x = (point as *const f32).read_unaligned();
        let point_y = ((point + 4) as *const f32).read_unaligned();
        let target_state = ((target + TARGET_STATE) as *const u32).read_unaligned();
        let target_x = ((target_state + TARGET_X) as *const f32).read_unaligned();
        let target_y = ((target_state + TARGET_Y) as *const f32).read_unaligned();

        let delta_y = ordered_sub(point_y, target_y);
        let delta_x = ordered_sub(point_x, target_x);
        let squared_y = ordered_mul(delta_y, delta_y);
        let squared_x = ordered_mul(delta_x, delta_x);
        let squared_distance = ordered_add(squared_y, squared_x);
        let planar_distance = squared_distance.sqrt();
        let scale = (lf_checker_rt::relocated(DISTANCE_SCALE) as *const f32).read_unaligned();
        let scaled_distance = ordered_mul(planar_distance, scale);
        let radius = ((this + TASK_RADIUS) as *const f32).read_unaligned();
        if scaled_distance > radius { radius } else { scaled_distance }
    }
});
