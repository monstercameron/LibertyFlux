// original: 0x0097EAC0 ped_task_replace_audio_subtask

/// Replaces the task's nested audio subtask and initializes its status fields.
///
/// The task owns a pointer slot at `+0xb4` and status words at `+0xac`,
/// `+0xb0` and `+0xb8`. A non-null previous value is passed by pointer to a
/// destructor. A zero first argument selects the global default object;
/// otherwise the second pointer can select a special global value when its
/// masked type flags and state match. The constructor receives the selected
/// object in ECX and a pointer to the task's `+0xb4` slot. The final stack word
/// is copied to `+0xb0`. The three comparisons and the return stack cleanup
/// use the original 32-bit calling convention.
lf_checker_rt::export!(thiscall, rw_0097EAC0(this: u32, initial: u32, candidate: u32, final_status: u32) -> () {
    const OLD_TASK_SLOT: u32 = 0xB4;
    const TASK_VALUE: u32 = 0xB8;
    const TASK_STATE: u32 = 0xAC;
    const TASK_STATUS: u32 = 0xB0;
    const CANDIDATE_FLAGS: u32 = 0x28;
    const CANDIDATE_STATE: u32 = 0x1304;
    const DESCRIBE_CALL: u32 = 1;
    const CONSTRUCT_CALL: u32 = 2;
    const GLOBAL_DEFAULT_VA: u32 = 0x01231290;
    const GLOBAL_SPECIAL_VALUE_VA: u32 = 0x01231308;
    const SPECIAL_FLAG_MASK: u32 = 0x3C0;
    const SPECIAL_FLAG: u32 = 0x80;
    const READY_STATE: u32 = 1;
    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    unsafe fn write_word(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value); }
    }
    let slot = this.wrapping_add(OLD_TASK_SLOT);
    let previous = unsafe { read_word(slot) };
    if previous != 0 {
        lf_checker_rt::callee_thiscall!(DESCRIBE_CALL, u32, previous, slot);
        unsafe { write_word(slot, 0); }
    }
    if initial == 0 {
        let global_default = unsafe { lf_checker_rt::global::<u32>(GLOBAL_DEFAULT_VA).read() };
        let default_object = unsafe { read_word(global_default) };
        unsafe {
            write_word(this.wrapping_add(TASK_VALUE), default_object);
            write_word(this.wrapping_add(TASK_STATE), 0);
            write_word(this.wrapping_add(TASK_STATUS), 0);
        }
        return;
    }
    unsafe { write_word(this.wrapping_add(TASK_VALUE), initial); }
    if candidate != 0 {
        let flags = unsafe { read_word(candidate.wrapping_add(CANDIDATE_FLAGS)) };
        if flags & SPECIAL_FLAG_MASK == SPECIAL_FLAG
            && unsafe { read_word(candidate.wrapping_add(CANDIDATE_STATE)) } == READY_STATE
        {
            let special_value = unsafe { lf_checker_rt::global::<u32>(GLOBAL_SPECIAL_VALUE_VA).read() };
            unsafe { write_word(this.wrapping_add(TASK_VALUE), special_value); }
        }
    }
    unsafe { write_word(this.wrapping_add(TASK_STATE), 0); }
    unsafe { write_word(slot, candidate); }
    lf_checker_rt::callee_thiscall!(CONSTRUCT_CALL, u32, candidate, slot);
    unsafe { write_word(this.wrapping_add(TASK_STATUS), final_status); }
});
