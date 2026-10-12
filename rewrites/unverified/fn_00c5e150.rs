// original: 0x00C5E150 CTaskComplexDestroyCarArmed::control_sub_task

/// Control an armed destroy-car task for its character. A missing manager
/// returns null. In control mode, the task's slot +0x4c handles the character
/// directly. Otherwise the manager's slot +0xfc supplies a float speed; the
/// original continues for positive speed or an unordered comparison and
/// returns null for zero or negative speed. Only child model 0x11d is
/// considered. Its optional +0x14 data object selects kind 0xc8 by default,
/// or supplies a kind through its slot +0x0c. Kinds 0x387 and 0x38b run the
/// geometry helper and ordered float gates using task fields +0x50, +0x60,
/// and +0x64. Kind 0x387 accepts an ordered +0x64 > +0x60 or +0x50 > +0x60;
/// kind 0x38b accepts only the latter. A qualifying gate sets move mode to
/// one and creates the task with type 0x3fc and the character. Other paths
/// return the child.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00c5e150(this: u32, ped: u32) -> u32 {
    unsafe {
        const CHILD_OFFSET: u32 = 0x08;
        const MODE_OFFSET: u32 = 0x14;
        const MANAGER_OFFSET: u32 = 0x18;
        const CHILD_DATA_OFFSET: u32 = 0x14;
        const TASK_CONTROL_SLOT: u32 = 0x4c;
        const MANAGER_SPEED_SLOT: u32 = 0xfc;
        const MODEL_SLOT: u32 = 0x0c;
        const KIND_SLOT: u32 = 0x0c;
        const TASK_FIRST_VALUE: u32 = 0x50;
        const TASK_MIDDLE_VALUE: u32 = 0x60;
        const TASK_LAST_VALUE: u32 = 0x64;
        const ARMED_VEHICLE_MODEL: u32 = 0x11d;
        const KIND_DEFAULT: u32 = 0xc8;
        const KIND_0387: u32 = 0x387;
        const KIND_038B: u32 = 0x38b;
        const FACTORY_TASK_TYPE: u32 = 0x3fc;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            unsafe { (address as *const f32).read_unaligned() }
        }

        let manager = read_u32(this.wrapping_add(MANAGER_OFFSET));
        if manager == 0 {
            return 0;
        }
        let child = read_u32(this.wrapping_add(CHILD_OFFSET));

        if (this.wrapping_add(MODE_OFFSET) as *const u8).read() != 0 {
            type ControlCall = extern "thiscall" fn(u32, u32) -> u32;
            let task_vtable = read_u32(this);
            let control_call: ControlCall = core::mem::transmute(
                read_u32(task_vtable.wrapping_add(TASK_CONTROL_SLOT)) as usize,
            );
            return control_call(this, ped);
        }

        type SpeedCall = extern "thiscall" fn(u32) -> f32;
        let manager_vtable = read_u32(manager);
        let speed_call: SpeedCall = core::mem::transmute(
            read_u32(manager_vtable.wrapping_add(MANAGER_SPEED_SLOT)) as usize,
        );
        let speed = speed_call(manager);
        if 0.0 >= speed {
            return 0;
        }

        type ModelCall = extern "thiscall" fn(u32) -> u32;
        let child_vtable = read_u32(child);
        let model_call: ModelCall = core::mem::transmute(
            read_u32(child_vtable.wrapping_add(MODEL_SLOT)) as usize,
        );
        if model_call(child) != ARMED_VEHICLE_MODEL {
            return child;
        }

        let data = read_u32(child.wrapping_add(CHILD_DATA_OFFSET));
        let kind = if data == 0 {
            KIND_DEFAULT
        } else {
            type KindCall = extern "thiscall" fn(u32) -> u32;
            let data_vtable = read_u32(data);
            let kind_call: KindCall = core::mem::transmute(
                read_u32(data_vtable.wrapping_add(KIND_SLOT)) as usize,
            );
            kind_call(data)
        };

        let should_create = match kind {
            KIND_0387 => {
                let _ = lf_checker_rt::callee_thiscall!(5, u32, this, ped);
                let limit = read_f32(this.wrapping_add(TASK_MIDDLE_VALUE));
                let upper = read_f32(this.wrapping_add(TASK_LAST_VALUE));
                let upper_exceeds_limit = upper > limit;
                if upper_exceeds_limit {
                    true
                } else {
                    read_f32(this.wrapping_add(TASK_FIRST_VALUE)) > limit
                }
            }
            KIND_038B => {
                let _ = lf_checker_rt::callee_thiscall!(5, u32, this, ped);
                let current = read_f32(this.wrapping_add(TASK_FIRST_VALUE));
                let limit = read_f32(this.wrapping_add(TASK_MIDDLE_VALUE));
                current > limit
            }
            _ => false,
        };

        if !should_create {
            return child;
        }
        let _ = lf_checker_rt::callee_thiscall!(6, u32, ped, 1);
        lf_checker_rt::callee_thiscall!(7, u32, this, FACTORY_TASK_TYPE, ped)
    }
}
