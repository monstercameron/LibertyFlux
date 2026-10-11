// original: 0x00be08a0 CTaskComplexUseSequence::vf1
/// Create a sequence task from the word at `this + 0x14` when the manager exists, then copy two words from offsets 0x18 and 0x1c and one byte from offset 0x24 into the returned task. The fields are written in that order. The original faults on a null destination during the stores; the rewrite uses the same offsets and faults. Calling convention: thiscall with no stack arguments; returns the destination pointer.
lf_checker_rt::export!(thiscall, rw_00be08a0(this: u32) -> u32 {
    const TASK_MANAGER_POINTER_VA: u32 = 0x0167_E2A0;
    const TASK_MANAGER_LOOKUP: u32 = 1;
    const CREATE_SEQUENCE_TASK: u32 = 2;
    let read_word = |address: u32| unsafe { (address as *const u32).read_unaligned() };
    let read_byte = |address: u32| unsafe { (address as *const u8).read() };
    let write_word = |address: u32, value: u32| unsafe { (address as *mut u32).write_unaligned(value) };
    let write_byte = |address: u32, value: u8| unsafe { (address as *mut u8).write(value) };
    let task_manager_context = unsafe { lf_checker_rt::global::<u32>(TASK_MANAGER_POINTER_VA).read_unaligned() };
    let manager_context = lf_checker_rt::callee_thiscall!(TASK_MANAGER_LOOKUP, u32, task_manager_context);
    let sequence = if manager_context == 0 { 0 } else {
        lf_checker_rt::callee_thiscall!(CREATE_SEQUENCE_TASK, u32, manager_context,
            read_word(this.wrapping_add(0x14)))
    };
    write_word(sequence.wrapping_add(0x18), read_word(this.wrapping_add(0x18)));
    write_word(sequence.wrapping_add(0x1C), read_word(this.wrapping_add(0x1C)));
    write_byte(sequence.wrapping_add(0x24), read_byte(this.wrapping_add(0x24)));
    sequence
});
