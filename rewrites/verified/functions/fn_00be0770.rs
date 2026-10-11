// original: 0x00be0770 CTaskComplexSitDownThenIdleThenStandUp::vf1
/// Create the sit-idle-stand sequence using twelve stack words after the manager in ECX. The arguments include task fields at 0x34, 0x38, 0x30, 0x6c, 0x60, 0x68 and 0x64; `this + 0x20` when byte 0x80 is nonzero (otherwise null); `this + 0x74`; and zero-extended bytes at 0x84, 0x81 and 0x83. A missing manager returns zero. The helper return is propagated. Calling convention: thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00be0770(this: u32) -> u32 {
    const TASK_MANAGER_POINTER_VA: u32 = 0x0167_E2A0;
    const TASK_MANAGER_LOOKUP: u32 = 1;
    const CREATE_IDLE_SEQUENCE: u32 = 2;
    let read_word = |address: u32| unsafe { (address as *const u32).read_unaligned() };
    let read_byte = |address: u32| unsafe { (address as *const u8).read() };
    let task_manager_context = unsafe { lf_checker_rt::global::<u32>(TASK_MANAGER_POINTER_VA).read_unaligned() };
    let manager_context = lf_checker_rt::callee_thiscall!(TASK_MANAGER_LOOKUP, u32, task_manager_context);
    if manager_context == 0 {
        0
    } else {
        let animation_data = if read_byte(this.wrapping_add(0x80)) != 0 { this.wrapping_add(0x20) } else { 0 };
        lf_checker_rt::callee_thiscall!(CREATE_IDLE_SEQUENCE, u32, manager_context,
            read_word(this.wrapping_add(0x34)), read_word(this.wrapping_add(0x38)), animation_data,
            this.wrapping_add(0x74), read_word(this.wrapping_add(0x30)),
            u32::from(read_byte(this.wrapping_add(0x84))), read_word(this.wrapping_add(0x6C)),
            read_word(this.wrapping_add(0x60)), u32::from(read_byte(this.wrapping_add(0x81))),
            read_word(this.wrapping_add(0x68)), read_word(this.wrapping_add(0x64)),
            u32::from(read_byte(this.wrapping_add(0x83))))
    }
});
