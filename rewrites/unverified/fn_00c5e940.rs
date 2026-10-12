// original: 0x00C5E940 create_destroy_car_sub_task

/// Creates the requested destroy-car child task. Signed task types above
/// 0x3ed return null; type 0x2e2 creates an exit-vehicle task for the
/// character's current vehicle, and type 0x3ed creates an armed destroy-car
/// task using four words from this task. Other types return null. Both
/// constructors are reached only after the task-pool allocation succeeds.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00c5e940(this: u32, task_type: u32, ped: u32) -> u32 {
    unsafe {
        const TASK_POOL_GLOBAL: u32 = 0x0167e2a0;
        const TYPE_EXIT_VEHICLE: u32 = 0x2e2;
        const TYPE_ARMED_DESTROY_CAR: u32 = 0x3ed;
        const SIGNED_UPPER_BOUND: i32 = 0x3ed;
        const PED_VEHICLE_OFFSET: u32 = 0x0b30;
        const TASK_ARG_A: u32 = 0x18;
        const TASK_ARG_B: u32 = 0x1c;
        const TASK_ARG_C: u32 = 0x20;
        const TASK_ARG_D: u32 = 0x24;

        let signed_type = task_type as i32;
        if signed_type > SIGNED_UPPER_BOUND {
            return 0;
        }
        if task_type != TYPE_EXIT_VEHICLE && task_type != TYPE_ARMED_DESTROY_CAR {
            return 0;
        }

        let pool = lf_checker_rt::global::<u32>(TASK_POOL_GLOBAL).read_unaligned();
        let allocated = lf_checker_rt::callee_thiscall!(1, u32, pool);
        if allocated == 0 {
            return 0;
        }

        if task_type == TYPE_EXIT_VEHICLE {
            let vehicle = (ped.wrapping_add(PED_VEHICLE_OFFSET) as *const u32)
                .read_unaligned();
            return lf_checker_rt::callee_thiscall!(2, u32, allocated, vehicle, 0, 0, 0);
        }

        lf_checker_rt::callee_thiscall!(
            3,
            u32,
            allocated,
            (this.wrapping_add(TASK_ARG_A) as *const u32).read_unaligned(),
            (this.wrapping_add(TASK_ARG_B) as *const u32).read_unaligned(),
            (this.wrapping_add(TASK_ARG_C) as *const u32).read_unaligned(),
            (this.wrapping_add(TASK_ARG_D) as *const u32).read_unaligned()
        )
    }
}
