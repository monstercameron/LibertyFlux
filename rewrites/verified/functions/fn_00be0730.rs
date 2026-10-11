// original: 0x00be0730 CTaskComplexSequence::vf1
/// Create a sequence task through the manager with no stack arguments. The sequence-copy helper receives interiors at destination `+0x14` and source `this + 0x14`. The function then copies the source words at 0x5c and 0x58 to the new task in that order and returns the task pointer. The helper's other effects are supplied by its scripted stand-in. Calling convention: thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00be0730(this: u32) -> u32 {
    const TASK_MANAGER_POINTER_VA: u32 = 0x0167_E2A0;
    const TASK_MANAGER_LOOKUP: u32 = 1;
    const CREATE_SEQUENCE: u32 = 2;
    const COPY_SEQUENCE_FIELDS: u32 = 3;
    let read_word = |address: u32| unsafe { (address as *const u32).read_unaligned() };
    let write_word = |address: u32, value: u32| unsafe { (address as *mut u32).write_unaligned(value) };
    let write_byte = |address: u32, value: u8| unsafe { (address as *mut u8).write(value) };
    let task_manager_context = unsafe { lf_checker_rt::global::<u32>(TASK_MANAGER_POINTER_VA).read_unaligned() };
    let manager_context = lf_checker_rt::callee_thiscall!(TASK_MANAGER_LOOKUP, u32, task_manager_context);
    let destination = if manager_context == 0 { 0 } else {
        lf_checker_rt::callee_thiscall!(CREATE_SEQUENCE, u32, manager_context)
    };
    let _ = lf_checker_rt::callee_thiscall!(COPY_SEQUENCE_FIELDS, u32,
        destination.wrapping_add(0x14), this.wrapping_add(0x14));
    write_word(destination.wrapping_add(0x5C), read_word(this.wrapping_add(0x5C)));
    write_word(destination.wrapping_add(0x58), read_word(this.wrapping_add(0x58)));
    destination
});
