// original: 0x00C5E6A0 CTaskComplexDestroyCar::create_next_sub_task

/// Selects the next destroy-car child from the current child task's type.
/// The type comparisons are signed. Vehicle task types 0x2e2 and 0x2bf
/// consult the character's vehicle state and return either child type 0x3ec
/// or 0x3ed. Existing child types 0xd0, 0x3ec, 0x3ed, and 0x3f0 request
/// factory type 0x516. Other types return null. Direct helper arguments
/// retain their original order, and the child type is dispatched through
/// the child's virtual slot +0x0c.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00c5e6a0(this: u32, ped: u32) -> u32 {
    unsafe {
        const CHILD_OFFSET: u32 = 0x08;
        const CHILD_TYPE_SLOT: u32 = 0x0c;
        const PED_VEHICLE_STATE_OFFSET: u32 = 0x2b0;
        const VEHICLE_TASK_TYPE: u32 = 0x2e2;
        const VEHICLE_TASK_TYPE_ARMED: u32 = 0x2bf;
        const LOW_TASK_TYPE: u32 = 0xd0;
        const CHILD_TYPE_LOW: i32 = 0x3ec;
        const CHILD_TYPE_HIGH: i32 = 0x3ed;
        const CHILD_TYPE_SPECIAL: u32 = 0x3f0;
        const DEFAULT_FACTORY_TYPE: u32 = 0x516;
        const RESULT_TYPE_OFFSET: u32 = 0x18;
        const STATUS_OFFSET: u32 = 0x0c;
        const READY_STATUS: u32 = 1;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        fn make_child(this: u32, child_type: u32) -> u32 {
            unsafe { lf_checker_rt::callee_thiscall!(5, u32, this, child_type, 0) }
        }

        let child = read_u32(this.wrapping_add(CHILD_OFFSET));
        let child_vtable = read_u32(child);
        type TypeCall = extern "thiscall" fn(u32) -> u32;
        let type_call: TypeCall = core::mem::transmute(
            read_u32(child_vtable.wrapping_add(CHILD_TYPE_SLOT)) as usize,
        );
        let task_type = type_call(child);
        let signed_type = task_type as i32;

        if signed_type > VEHICLE_TASK_TYPE as i32 {
            if signed_type < CHILD_TYPE_LOW {
                return 0;
            }
            if signed_type <= CHILD_TYPE_HIGH || task_type == CHILD_TYPE_SPECIAL {
                return make_child(this, DEFAULT_FACTORY_TYPE);
            }
            return 0;
        }

        if task_type == LOW_TASK_TYPE {
            return make_child(this, DEFAULT_FACTORY_TYPE);
        }
        if task_type == VEHICLE_TASK_TYPE_ARMED {
            let vehicle_state = ped.wrapping_add(PED_VEHICLE_STATE_OFFSET);
            let first_task: u32 = lf_checker_rt::callee_thiscall!(2, u32, vehicle_state);
            if first_task == 0 {
                return make_child(this, 0x3ec);
            }
            let second_task: u32 = lf_checker_rt::callee_thiscall!(2, u32, vehicle_state);
            let can_continue: u8 =
                lf_checker_rt::callee_thiscall!(4, u8, second_task);
            if can_continue != 0 {
                return make_child(this, 0x3ec);
            }
            return make_child(this, 0x3ed);
        }
        if task_type == VEHICLE_TASK_TYPE {
            let vehicle_state = ped.wrapping_add(PED_VEHICLE_STATE_OFFSET);
            let first_task: u32 = lf_checker_rt::callee_thiscall!(2, u32, vehicle_state);
            if first_task != 0 {
                return make_child(this, 0x3ec);
            }
            let second_task: u32 = lf_checker_rt::callee_thiscall!(2, u32, vehicle_state);
            let vehicle_task_type = read_u32(second_task.wrapping_add(RESULT_TYPE_OFFSET));
            let status_info: u32 = lf_checker_rt::callee_cdecl!(3, u32, vehicle_task_type);
            let status = read_u32(status_info.wrapping_add(STATUS_OFFSET));
            if status == READY_STATUS {
                return make_child(this, 0x3ec);
            }
            return make_child(this, 0x3ed);
        }
        0
    }
}
