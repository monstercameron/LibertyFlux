// original: 0x00C5E230 CTaskComplexGangDriveby::control_sub_task

/// Control a gang-driveby task for its character. The child task is at +0x08
/// and carries its flags at +0x0c. Unless the character's current task state
/// selects the bypass, call the child's slot +0x14 with action values one and
/// then two until handled; a handled result sets child flag bit 0x2 and
/// returns null. Otherwise ask the child type at slot +0x0c. For type 0x2c5,
/// character bytes +0x218 and +0x219 gate the ped-task helper and a final
/// compatibility call. If both allow control, return this task's slot +0x4c
/// result. Other paths return the child. Direct helper calls retain their
/// original conventions and argument order.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00c5e230(this: u32, ped: u32) -> u32 {
    unsafe {
        const CHILD_OFFSET: u32 = 0x08;
        const CHILD_FLAGS_OFFSET: u32 = 0x0c;
        const CHILD_GATE_SLOT: u32 = 0x14;
        const CHILD_TYPE_SLOT: u32 = 0x0c;
        const CHILD_CONTROL_SLOT: u32 = 0x4c;
        const PED_CURRENT_TASK_OFFSET: u32 = 0x0b30;
        const PED_TASK_FLAGS_OFFSET: u32 = 0x26c;
        const PED_FLAG_A_OFFSET: u32 = 0x218;
        const PED_FLAG_B_OFFSET: u32 = 0x219;
        const PED_TASK_FLAG_MASK: u8 = 0x04;
        const CHILD_EARLY_FLAG_MASK: u32 = 0x01;
        const CHILD_HANDLED_FLAG_MASK: u32 = 0x02;
        const TASK_TYPE_NEEDS_CONTROL: u32 = 0x2c5;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let child = read_u32(this.wrapping_add(CHILD_OFFSET));
        let ped_current_task = read_u32(ped.wrapping_add(PED_CURRENT_TASK_OFFSET));
        let bypass_child_gates = ped_current_task != 0
            && ((ped.wrapping_add(PED_TASK_FLAGS_OFFSET) as *const u8).read()
                & PED_TASK_FLAG_MASK) != 0;

        if !bypass_child_gates {
            let child_flags = read_u32(child.wrapping_add(CHILD_FLAGS_OFFSET));
            if child_flags & CHILD_EARLY_FLAG_MASK != 0 {
                return 0;
            }

            type GateCall = extern "thiscall" fn(u32, u32, u32, u32) -> u8;
            let child_vtable = read_u32(child);
            let gate_call: GateCall = core::mem::transmute(
                read_u32(child_vtable.wrapping_add(CHILD_GATE_SLOT)) as usize,
            );
            if gate_call(child, ped, 1, 0) != 0 {
                let flags_address = child.wrapping_add(CHILD_FLAGS_OFFSET);
                let current_flags = read_u32(flags_address);
                (flags_address as *mut u32).write_unaligned(
                    current_flags | CHILD_HANDLED_FLAG_MASK,
                );
                return 0;
            }
            if gate_call(child, ped, 2, 0) != 0 {
                let flags_address = child.wrapping_add(CHILD_FLAGS_OFFSET);
                let current_flags = read_u32(flags_address);
                (flags_address as *mut u32).write_unaligned(
                    current_flags | CHILD_HANDLED_FLAG_MASK,
                );
                return 0;
            }
        }

        type TypeCall = extern "thiscall" fn(u32) -> u32;
        let child_vtable = read_u32(child);
        let type_call: TypeCall = core::mem::transmute(
            read_u32(child_vtable.wrapping_add(CHILD_TYPE_SLOT)) as usize,
        );
        if type_call(child) != TASK_TYPE_NEEDS_CONTROL {
            return child;
        }
        if (ped.wrapping_add(PED_FLAG_A_OFFSET) as *const u8).read() != 0
            || (ped.wrapping_add(PED_FLAG_B_OFFSET) as *const u8).read() == 0
        {
            return child;
        }

        let helper_accepts: u8 = lf_checker_rt::callee_cdecl!(4, u8, ped);
        if helper_accepts == 0 {
            return child;
        }
        let can_control: u8 = lf_checker_rt::callee_thiscall!(
            5,
            u8,
            child,
            ped,
            1,
            0
        );
        if can_control == 0 {
            return child;
        }

        type ControlCall = extern "thiscall" fn(u32, u32) -> u32;
        let task_vtable = read_u32(this);
        let control_call: ControlCall = core::mem::transmute(
            read_u32(task_vtable.wrapping_add(CHILD_CONTROL_SLOT)) as usize,
        );
        control_call(this, ped)
    }
}
