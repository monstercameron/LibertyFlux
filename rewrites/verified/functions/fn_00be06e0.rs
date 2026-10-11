// original: 0x00be06e0 CTaskComplexReact::vf1
/// Create a reaction task through the global manager. The manager receives six stack words in order: fields at offsets 0x34, 0x46 zero-extended from a byte, 0x30, and 0x3c; the pointer `this + 0x20`; and the byte at 0x48. The final byte is pushed as a full word after a temporary stack slot is only partly overwritten, so the contract compares that argument's low byte only. A null manager returns zero. Calling convention: thiscall with no stack arguments; returns the manager helper's EAX value.
lf_checker_rt::export!(thiscall, rw_00be06e0(this: u32) -> u32 {
    const TASK_MANAGER_POINTER_VA: u32 = 0x0167_E2A0;
    const TASK_MANAGER_LOOKUP: u32 = 1;
    const CREATE_REACTION: u32 = 2;
    let read_word = |address: u32| unsafe { (address as *const u32).read_unaligned() };
    let read_byte = |address: u32| unsafe { (address as *const u8).read() };
    let task_manager_context = unsafe { lf_checker_rt::global::<u32>(TASK_MANAGER_POINTER_VA).read_unaligned() };
    let manager_context = lf_checker_rt::callee_thiscall!(TASK_MANAGER_LOOKUP, u32, task_manager_context);
    if manager_context == 0 {
        0
    } else {
        lf_checker_rt::callee_thiscall!(CREATE_REACTION, u32, manager_context,
            read_word(this.wrapping_add(0x34)),
            u32::from(read_byte(this.wrapping_add(0x46))),
            read_word(this.wrapping_add(0x30)), read_word(this.wrapping_add(0x3C)),
            this.wrapping_add(0x20), u32::from(read_byte(this.wrapping_add(0x48))))
    }
});
