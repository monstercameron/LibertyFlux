// original: 0x00C5E060 CTaskComplexDestroyCar::control_sub_task

/// Control the current destruction subtask for a character. The task stores
/// its child at +0x08, a mode byte at +0x14, and a manager at +0x18; the
/// character state word is at +0x2b0 and its byte flag at +0x219. A null
/// manager or a speed for which ordered `0.0 >= speed` is true returns null.
/// In mode, forward the character to the task's control slot. Otherwise
/// inspect the child model: model 0x3ed may ask the weapon helper and uses a
/// signed state-word comparison; model 0x3ec may ask the vehicle helper.
/// Eligible cases pass the selected model kind, then the character, to the
/// task constructor. All helper calls preserve the original call order and
/// argument order; pointer fields are read only on paths that use them.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00c5e060(this: u32, ped: u32) -> u32 {
    unsafe {
        const CHILD_OFFSET: u32 = 0x08;
        const MODE_OFFSET: u32 = 0x14;
        const MANAGER_OFFSET: u32 = 0x18;
        const MODEL_SLOT_OFFSET: u32 = 0x0c;
        const CONTROL_SLOT_OFFSET: u32 = 0x4c;
        const SPEED_SLOT_OFFSET: u32 = 0xfc;
        const PED_STATE_OFFSET: u32 = 0x2b0;
        const PED_FLAG_OFFSET: u32 = 0x219;
        const HELPER_OBJECT_FLAG_OFFSET: u32 = 0x60;
        const MODEL_KIND_A: u32 = 0x3ec;
        const MODEL_KIND_B: u32 = 0x3ed;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let manager = read_u32(this.wrapping_add(MANAGER_OFFSET));
        if manager == 0 {
            return 0;
        }

        type SpeedCall = extern "thiscall" fn(u32) -> f32;
        let manager_vtable = read_u32(manager);
        let speed_call: SpeedCall = core::mem::transmute(
            read_u32(manager_vtable.wrapping_add(SPEED_SLOT_OFFSET)) as usize,
        );
        let speed = speed_call(manager);
        if 0.0 >= speed {
            return 0;
        }

        let helper_object = lf_checker_rt::callee_thiscall!(
            4,
            u32,
            ped.wrapping_add(PED_STATE_OFFSET)
        );

        if (this.wrapping_add(MODE_OFFSET) as *const u8).read() != 0 {
            type ControlCall = extern "thiscall" fn(u32, u32) -> u32;
            let task_vtable = read_u32(this);
            let control_call: ControlCall = core::mem::transmute(
                read_u32(task_vtable.wrapping_add(CONTROL_SLOT_OFFSET)) as usize,
            );
            return control_call(this, ped);
        }

        let entity = read_u32(this.wrapping_add(CHILD_OFFSET));
        type ModelCall = extern "thiscall" fn(u32) -> u32;
        let entity_vtable = read_u32(entity);
        let model_call: ModelCall = core::mem::transmute(
            read_u32(entity_vtable.wrapping_add(MODEL_SLOT_OFFSET)) as usize,
        );
        let model = model_call(entity);
        let next_kind;

        if model == MODEL_KIND_B {
            if helper_object == 0
                || read_u32(helper_object.wrapping_add(HELPER_OBJECT_FLAG_OFFSET)) != 0
                || (ped.wrapping_add(PED_FLAG_OFFSET) as *const u8).read() != 0
            {
                return entity;
            }
            let _ = lf_checker_rt::callee_thiscall!(
                5,
                u32,
                ped.wrapping_add(PED_STATE_OFFSET),
                9,
                0,
                1,
                0
            );
            if (read_u32(ped.wrapping_add(PED_STATE_OFFSET)) as i32) > 1 {
                return entity;
            }
            next_kind = MODEL_KIND_A;
        } else if model == MODEL_KIND_A {
            if helper_object == 0 {
                return entity;
            }
            let vehicle_manager = lf_checker_rt::callee_thiscall!(
                4,
                u32,
                ped.wrapping_add(PED_STATE_OFFSET)
            );
            if core::hint::black_box(lf_checker_rt::callee_thiscall!(
                6,
                u8,
                vehicle_manager
            )) != 0 {
                return entity;
            }
            next_kind = MODEL_KIND_B;
        } else {
            return entity;
        }

        let should_create = lf_checker_rt::callee_thiscall!(
            7,
            u8,
            entity,
            ped,
            1,
            0
        );
        if should_create == 0 {
            return entity;
        }
        lf_checker_rt::callee_thiscall!(
            8,
            u32,
            this,
            next_kind,
            ped
        )
    }
}
