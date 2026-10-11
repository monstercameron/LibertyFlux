// original: 0x00d173b0 ped_position_outside_radius
/// Returns AL when the task's position lies strictly outside the combined
/// radii. Coordinates are subtracted in X, Y, Z order; the squared distance
/// accumulates Y², X², then Z², matching the single precision evaluation.
/// NaN comparisons return false, and equality is not outside.

const TASK_POS_X: u32 = 0x13E0;
const TASK_POS_Y: u32 = 0x13E4;
const TASK_POS_Z: u32 = 0x13E8;
const TASK_RADIUS: u32 = 0x1440;
const ENTITY_POS_X: u32 = 0x30;
const ENTITY_POS_Y: u32 = 0x34;
const ENTITY_POS_Z: u32 = 0x38;
const ENTITY_RADIUS: u32 = 0x08;

lf_checker_rt::export!(thiscall, rw_00d173b0(task: u32, entity: u32) -> u32 {
    unsafe {
        let task_x = ((task + TASK_POS_X) as *const f32).read_unaligned();
        let task_y = ((task + TASK_POS_Y) as *const f32).read_unaligned();
        let task_z = ((task + TASK_POS_Z) as *const f32).read_unaligned();
        let entity_x = ((entity + ENTITY_POS_X) as *const f32).read_unaligned();
        let entity_y = ((entity + ENTITY_POS_Y) as *const f32).read_unaligned();
        let entity_z = ((entity + ENTITY_POS_Z) as *const f32).read_unaligned();
        let task_radius = ((task + TASK_RADIUS) as *const f32).read_unaligned();
        let entity_radius = ((entity + ENTITY_RADIUS) as *const f32).read_unaligned();

        let dx = core::hint::black_box(task_x) - core::hint::black_box(entity_x);
        let dy = core::hint::black_box(task_y) - core::hint::black_box(entity_y);
        let dz = core::hint::black_box(task_z) - core::hint::black_box(entity_z);
        let combined_radius = core::hint::black_box(task_radius)
            + core::hint::black_box(entity_radius);

        let dy_squared = core::hint::black_box(dy) * core::hint::black_box(dy);
        let dx_squared = core::hint::black_box(dx) * core::hint::black_box(dx);
        let dz_squared = core::hint::black_box(dz) * core::hint::black_box(dz);
        let radius_squared = core::hint::black_box(combined_radius)
            * core::hint::black_box(combined_radius);
        let xy_squared = core::hint::black_box(dy_squared)
            + core::hint::black_box(dx_squared);
        let distance_squared = core::hint::black_box(xy_squared)
            + core::hint::black_box(dz_squared);

        u32::from(distance_squared > radius_squared)
    }
});
