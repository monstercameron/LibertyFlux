// original: 0x00be05d0 CTaskComplexControlMovement::vf1
/// Combine two optional movement weights and a task parameter through the task manager. `this + 0x18` and `this + 0x14` hold pointers to objects whose vtable slot at byte offset 4 returns a weight; null objects contribute zero. The manager lookup is skipped when the global context lookup returns null. The final callee receives the manager in ECX and four stack words in order: movement weight, animation weight, the field at `this + 0x1c`, and the float bits at `this + 0x20`. Float bits are forwarded unchanged. Calling convention: thiscall with no stack arguments; returns the final callee's EAX value.
lf_checker_rt::export!(thiscall, rw_00be05d0(this: u32) -> u32 {
    const TASK_MANAGER_POINTER_VA: u32 = 0x0167_E2A0;
    const TASK_MANAGER_LOOKUP: u32 = 1;
    const WEIGHT_SLOT_OFFSET: u32 = 0x04;
    const APPLY_MOVEMENT: u32 = 4;
    const ANIMATION_POINTER_OFFSET: u32 = 0x18;
    const MOVEMENT_POINTER_OFFSET: u32 = 0x14;
    const MOVEMENT_PARAMETER_OFFSET: u32 = 0x1C;
    const ANIMATION_WEIGHT_OFFSET: u32 = 0x20;
    let read_word = |address: u32| unsafe { (address as *const u32).read_unaligned() };
    let task_manager_context = unsafe { lf_checker_rt::global::<u32>(TASK_MANAGER_POINTER_VA).read_unaligned() };
    let manager = lf_checker_rt::callee_thiscall!(TASK_MANAGER_LOOKUP, u32, task_manager_context);
    if manager == 0 {
        0
    } else {
        let animation = read_word(this.wrapping_add(ANIMATION_POINTER_OFFSET));
        let animation_weight = if animation == 0 { 0 } else {
            let vtable = read_word(animation);
            let target = read_word(vtable.wrapping_add(WEIGHT_SLOT_OFFSET));
            let get_weight: extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(target as usize) };
            get_weight(animation)
        };
        let movement = read_word(this.wrapping_add(MOVEMENT_POINTER_OFFSET));
        let movement_weight = if movement == 0 { 0 } else {
            let vtable = read_word(movement);
            let target = read_word(vtable.wrapping_add(WEIGHT_SLOT_OFFSET));
            let get_weight: extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(target as usize) };
            get_weight(movement)
        };
        lf_checker_rt::callee_thiscall!(APPLY_MOVEMENT, u32, manager,
            movement_weight, animation_weight,
            read_word(this.wrapping_add(MOVEMENT_PARAMETER_OFFSET)),
            read_word(this.wrapping_add(ANIMATION_WEIGHT_OFFSET)))
    }
});
