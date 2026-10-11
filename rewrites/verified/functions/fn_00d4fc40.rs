// original: 0x00D4FC40 check_entity_task_distance_and_state
/// Reject a missing entity, task, or task component. Unless either entity flag
/// at `+0x218`/`+0x219` bypasses the distance check, compare the squared
/// distance between the entity target at `+0x20` (position at `+0x30`) and the
/// task reference position (either the linked object at `+0x30` or task `+0x10`)
/// with the global squared-distance limit. The x, y, and z deltas are squared
/// and accumulated in that order. A distance rejection returns the low byte
/// of the target pointer; otherwise a helper may prepare the task before a
/// tail-called final predicate returns its low-byte result. This cdecl
/// function takes three stack pointers.
lf_checker_rt::export!(cdecl, rw_00d4fc40(entity: u32, task: u32, active_task: u32) -> u8 {
    const ENTITY_FLAGS_A: u32 = 0x218;
    const ENTITY_FLAGS_B: u32 = 0x219;
    const ENTITY_TARGET: u32 = 0x20;
    const TARGET_POSITION: u32 = 0x30;
    const TASK_COMPONENT: u32 = 0x20;
    const TASK_FALLBACK_POSITION: u32 = 0x10;
    const TASK_COMPONENT_POSITION: u32 = 0x30;
    const TASK_COMPONENT_LINK: u32 = 0x6c;
    const DISTANCE_LIMIT_GLOBAL: u32 = 0x00fe_8b00;
    const CAN_SKIP: u32 = 1;
    const PREPARE_TASK: u32 = 2;
    const FINAL_PREDICATE: u32 = 3;

    if entity == 0 || task == 0 {
        return 0;
    }
    let component_link = unsafe {
        ((task + TASK_COMPONENT_LINK) as *const u32).read_unaligned()
    };
    if component_link == 0 {
        return 0;
    }

    let flags_a = unsafe { ((entity + ENTITY_FLAGS_A) as *const u8).read() };
    let flags_b = unsafe { ((entity + ENTITY_FLAGS_B) as *const u8).read() };
    if flags_a == 0 && flags_b == 0 {
        let target = unsafe { ((entity + ENTITY_TARGET) as *const u32).read_unaligned() };
        let linked_component =
            unsafe { ((task + TASK_COMPONENT) as *const u32).read_unaligned() };
        let reference = if linked_component != 0 {
            linked_component + TASK_COMPONENT_POSITION
        } else {
            task + TASK_FALLBACK_POSITION
        };
        let target_position = target + TARGET_POSITION;
        let target_x = f32::from_bits(unsafe {
            (target_position as *const u32).read_unaligned()
        });
        let target_y = f32::from_bits(unsafe {
            ((target_position + 4) as *const u32).read_unaligned()
        });
        let target_z = f32::from_bits(unsafe {
            ((target_position + 8) as *const u32).read_unaligned()
        });
        let reference_x = f32::from_bits(unsafe { (reference as *const u32).read_unaligned() });
        let reference_y = f32::from_bits(unsafe {
            ((reference + 4) as *const u32).read_unaligned()
        });
        let reference_z = f32::from_bits(unsafe {
            ((reference + 8) as *const u32).read_unaligned()
        });

        let dx = core::hint::black_box(target_x) - core::hint::black_box(reference_x);
        let dy = core::hint::black_box(target_y) - core::hint::black_box(reference_y);
        let dz = core::hint::black_box(target_z) - core::hint::black_box(reference_z);
        let dx_squared = core::hint::black_box(dx) * core::hint::black_box(dx);
        let dy_squared = core::hint::black_box(dy) * core::hint::black_box(dy);
        let dz_squared = core::hint::black_box(dz) * core::hint::black_box(dz);
        let xy_squared = core::hint::black_box(dx_squared) + core::hint::black_box(dy_squared);
        let distance_squared = core::hint::black_box(xy_squared) + core::hint::black_box(dz_squared);
        let maximum = f32::from_bits(unsafe {
            lf_checker_rt::global::<u32>(DISTANCE_LIMIT_GLOBAL).read_unaligned()
        });
        if core::hint::black_box(distance_squared) > core::hint::black_box(maximum) {
            return target as u8;
        }
    }

    let can_skip: u8 = lf_checker_rt::callee_thiscall!(CAN_SKIP, u8, active_task);
    if can_skip == 0 {
        let _: u32 = lf_checker_rt::callee_thiscall!(
            PREPARE_TASK, u32, active_task, component_link, 0
        );
    }
    lf_checker_rt::callee_thiscall!(FINAL_PREDICATE, u8, active_task)
});
