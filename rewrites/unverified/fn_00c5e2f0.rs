// original: 0x00C5E2F0 CTaskComplexDestroyCar::vf19

/// Clears the task's mode byte, then selects a task type from its current
/// state and the character's current task flags. The selected constructor is
/// called with this task as its object and the original ordered arguments.
/// If the character state permits vehicle inspection, the current vehicle
/// task is queried twice and its subtype is checked before choosing between
/// the completion task types. The method takes the character as its stack
/// argument and returns the selected helper's value.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00c5e2f0(this: u32, ped: u32) -> u32 {
    unsafe {
        const TASK_SUBTASK_OFFSET: u32 = 0x18;
        const TASK_MODE_OFFSET: u32 = 0x14;
        const PED_CURRENT_TASK_OFFSET: u32 = 0x0b30;
        const PED_TASK_FLAGS_OFFSET: u32 = 0x26c;
        const PED_VEHICLE_STATE_OFFSET: u32 = 0x2b0;
        const TASK_TYPE_INITIAL: u32 = 0x516;
        const TASK_TYPE_GATED: u32 = 0x2e2;
        const TASK_TYPE_RETRY: u32 = 0x3ed;
        const TASK_TYPE_DONE: u32 = 0x3ec;
        const VEHICLE_STATUS_READY: u32 = 1;

        (this.wrapping_add(TASK_MODE_OFFSET) as *mut u8).write(0);
        let current_subtask = (this.wrapping_add(TASK_SUBTASK_OFFSET) as *const u32)
            .read_unaligned();
        if current_subtask == 0 {
            return lf_checker_rt::callee_thiscall!(1, u32, this, TASK_TYPE_INITIAL, 0);
        }

        let ped_current_task = (ped.wrapping_add(PED_CURRENT_TASK_OFFSET) as *const u32)
            .read_unaligned();
        let ped_task_flags = (ped.wrapping_add(PED_TASK_FLAGS_OFFSET) as *const u8).read();
        if ped_current_task != 0 && ped_task_flags & 0x04 != 0 {
            return lf_checker_rt::callee_thiscall!(1, u32, this, TASK_TYPE_GATED, ped);
        }

        let vehicle_state = ped.wrapping_add(PED_VEHICLE_STATE_OFFSET);
        let vehicle_task: u32 = lf_checker_rt::callee_thiscall!(2, u32, vehicle_state);
        if vehicle_task == 0 {
            return lf_checker_rt::callee_thiscall!(1, u32, this, TASK_TYPE_RETRY, 0);
        }
        let vehicle_task_again: u32 =
            lf_checker_rt::callee_thiscall!(2, u32, vehicle_state);
        let vehicle_task_type = (vehicle_task_again.wrapping_add(0x18) as *const u32)
            .read_unaligned();
        let status_info: u32 = lf_checker_rt::callee_cdecl!(3, u32, vehicle_task_type);
        let vehicle_status = (status_info.wrapping_add(0x0c) as *const u32)
            .read_unaligned();
        let task_type = if vehicle_status == VEHICLE_STATUS_READY {
            TASK_TYPE_DONE
        } else {
            TASK_TYPE_RETRY
        };
        lf_checker_rt::callee_thiscall!(1, u32, this, task_type, 0)
    }
}
