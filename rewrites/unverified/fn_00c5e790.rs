// original: 0x00C5E790 CTaskComplexDestroyCarArmed::create_next_sub_task

/// Chooses the next armed destroy-car child from its current child task's
/// type. A missing current child returns null. Signed type ranges select a
/// small set of factory cases, one of which inspects the child's next task
/// through virtual slot +0x0c. The geometry helper runs only for types
/// 0x3fc, 0xca, and 0x40e; those paths then create a 0x38b child. Other
/// factory paths use type 0x3fc. A current type of 0x516 and all unhandled
/// values return null. Callee argument order follows the original stack.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00c5e790(this: u32, ped: u32) -> u32 {
    unsafe {
        const CURRENT_CHILD_OFFSET: u32 = 0x08;
        const CURRENT_SUBTASK_OFFSET: u32 = 0x18;
        const CHILD_NEXT_TASK_OFFSET: u32 = 0x14;
        const CHILD_TYPE_SLOT: u32 = 0x0c;
        const TYPE_SPECIAL: u32 = 0x3fc;
        const TYPE_GEOMETRY_SMALL: u32 = 0x0ca;
        const TYPE_GEOMETRY_MID: u32 = 0x11d;
        const TYPE_FACTORY_EXTRA: u32 = 0x38b;
        const TYPE_GEOMETRY_HIGH: u32 = 0x40e;
        const TYPE_RETURNS_NULL: u32 = 0x516;
        const CHILD_GEOMETRY_TYPE: u32 = 0x387;
        const FACTORY_GEOMETRY: u32 = 0x38b;
        const FACTORY_SPECIAL: u32 = 0x3fc;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        fn create_child(this: u32, child_type: u32, ped: u32) -> u32 {
            lf_checker_rt::callee_thiscall!(3, u32, this, child_type, ped)
        }

        if read_u32(this.wrapping_add(CURRENT_SUBTASK_OFFSET)) == 0 {
            return 0;
        }

        let child = read_u32(this.wrapping_add(CURRENT_CHILD_OFFSET));
        let child_vtable = read_u32(child);
        type TypeCall = extern "thiscall" fn(u32) -> u32;
        let type_call: TypeCall = core::mem::transmute(
            read_u32(child_vtable.wrapping_add(CHILD_TYPE_SLOT)) as usize,
        );
        let task_type = type_call(child);
        let signed_type = task_type as i32;

        if signed_type > TYPE_SPECIAL as i32 {
            if task_type == TYPE_GEOMETRY_HIGH {
                let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, this, ped);
                return create_child(this, FACTORY_GEOMETRY, ped);
            }
            if task_type == TYPE_RETURNS_NULL {
                return 0;
            }
            return 0;
        }

        if task_type == TYPE_SPECIAL {
            let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, this, ped);
            return create_child(this, FACTORY_GEOMETRY, ped);
        }
        if task_type == TYPE_GEOMETRY_SMALL {
            let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, this, ped);
            return create_child(this, FACTORY_GEOMETRY, ped);
        }
        if task_type == TYPE_GEOMETRY_MID {
            let next_child = read_u32(child.wrapping_add(CHILD_NEXT_TASK_OFFSET));
            if next_child == 0 {
                return 0;
            }
            let next_vtable = read_u32(next_child);
            let next_type_call: TypeCall = core::mem::transmute(
                read_u32(next_vtable.wrapping_add(CHILD_TYPE_SLOT)) as usize,
            );
            if next_type_call(next_child) != CHILD_GEOMETRY_TYPE {
                return 0;
            }
            return create_child(this, FACTORY_SPECIAL, ped);
        }
        if task_type == TYPE_FACTORY_EXTRA {
            return create_child(this, FACTORY_SPECIAL, ped);
        }
        0
    }
}
