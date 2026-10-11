// original: 0x00be06a0 CTaskComplexMoveSequence::vf1
/// Create a move-sequence task when the global task-manager lookup succeeds, pass its interior at `+0x20` and the source interior at `this + 0x20` to the field-copy helper, then copy the source words at offsets 0x68 and 0x64 into the returned task. The copy helper is a scripted callee; the two explicit field stores remain this function's writes. A null returned task faults on the explicit stores just as the original does. Calling convention: thiscall with no stack arguments; returns the new task pointer.
lf_checker_rt::export!(thiscall, rw_00be06a0(this: u32) -> u32 {
    const TASK_MANAGER_POINTER_VA: u32 = 0x0167_E2A0;
    const TASK_MANAGER_LOOKUP: u32 = 1;
    const CREATE_MOVE_SEQUENCE: u32 = 2;
    const COPY_MOVE_FIELDS: u32 = 3;
    let read_word = |address: u32| unsafe { (address as *const u32).read_unaligned() };
    let write_word = |address: u32, value: u32| unsafe { (address as *mut u32).write_unaligned(value) };
    let task_manager_context = unsafe { lf_checker_rt::global::<u32>(TASK_MANAGER_POINTER_VA).read_unaligned() };
    let manager_context = lf_checker_rt::callee_thiscall!(TASK_MANAGER_LOOKUP, u32, task_manager_context);
    let destination = if manager_context == 0 { 0 } else {
        lf_checker_rt::callee_thiscall!(CREATE_MOVE_SEQUENCE, u32, manager_context)
    };
    let _ = lf_checker_rt::callee_thiscall!(COPY_MOVE_FIELDS, u32,
        destination.wrapping_add(0x20), this.wrapping_add(0x20));
    write_word(destination.wrapping_add(0x68), read_word(this.wrapping_add(0x68)));
    write_word(destination.wrapping_add(0x64), read_word(this.wrapping_add(0x64)));
    destination
});
