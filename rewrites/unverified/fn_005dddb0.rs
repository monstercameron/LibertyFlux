// original: 0x005DDDB0 select_medic_action_by_ped_type

/// Select a treatment action from the task type and the pedestrian's state.
///
/// The object at offset eight supplies a virtual type value. Values from
/// 0x70 through 0x72 are accepted; other values return zero. For accepted
/// types, pedestrian flag bit two and its task pointer choose either the
/// normal treatment constructor, the alternate constructor, or the manager's
/// fallback constructor. Calls are intercepted using the object's 32-bit
/// vtable slot three and the game's thiscall stack convention.
lf_checker_rt::export!(thiscall, rw_005dddb0(this: u32, pedestrian: u32) -> u32 {
    select_medic_action_by_ped_type(this, pedestrian, false)
});

fn select_medic_action_by_ped_type(this: u32, pedestrian: u32, widen_type_range: bool) -> u32 {
    const ACTION_OBJECT_OFFSET: u32 = 8;
    const ENTITY_OFFSET: u32 = 0x14;
    const TASK_KIND_OFFSET: u32 = 0x18;
    const VECTOR_OFFSET: u32 = 0x30;
    const VARIANT_OFFSET: u32 = 0x42;
    const PED_FLAGS_OFFSET: u32 = 0x26C;
    const PED_TASK_OFFSET: u32 = 0xB30;
    const FIRST_ACTION_TYPE: u32 = 0x70;
    const TASK_MANAGER: u32 = 0x0167E2A0;
    let last_action_delta = if widen_type_range { 3 } else { 2 };

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    let action_object = unsafe { read_u32(this.wrapping_add(ACTION_OBJECT_OFFSET)) };
    let action_type = lf_checker_rt::callee_thiscall!(1, u32, action_object);
    let action_delta = action_type.wrapping_sub(FIRST_ACTION_TYPE);
    if action_delta > last_action_delta {
        return 0;
    }

    let flags = unsafe { read_u32(pedestrian.wrapping_add(PED_FLAGS_OFFSET)) };
    let ped_task = unsafe { read_u32(pedestrian.wrapping_add(PED_TASK_OFFSET)) };
    if flags & 4 != 0 && ped_task != 0 {
        let owning_pedestrian = lf_checker_rt::callee_thiscall!(3, u32, ped_task, 0u32);
        let manager = unsafe { lf_checker_rt::global::<u32>(TASK_MANAGER).read_unaligned() };
        let factory = lf_checker_rt::callee_thiscall!(2, u32, manager);
        if factory == 0 {
            return 0;
        }
        let entity = unsafe { read_u32(this.wrapping_add(ENTITY_OFFSET)) };
        let task_kind = unsafe { read_u32(this.wrapping_add(TASK_KIND_OFFSET)) };
        let vector = this.wrapping_add(VECTOR_OFFSET);
        let variant = unsafe { (this.wrapping_add(VARIANT_OFFSET) as *const u8).read() } as u32;
        if owning_pedestrian == pedestrian {
            lf_checker_rt::callee_thiscall!(4, u32, factory, entity, task_kind, vector, variant)
        } else {
            lf_checker_rt::callee_thiscall!(5, u32, factory, entity, task_kind, vector, variant)
        }
    } else {
        let manager = unsafe { lf_checker_rt::global::<u32>(TASK_MANAGER).read_unaligned() };
        let factory = lf_checker_rt::callee_thiscall!(2, u32, manager);
        if factory == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(6, u32, factory)
    }
}
