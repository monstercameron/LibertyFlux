// original: 0x00a21ef0 task_record_set_id_and_flags

/// Store the task identifier and the low 16 bits of its state on the task
/// object, then clear its dirty byte. Four stack words are accepted; the last
/// two are ignored. The return comparison covers AX because only AX is set.
lf_checker_rt::export!(thiscall, rw_00a21ef0(this: u32, task_id: u32, task_flags: u32,
                                             _unused_2: u32, _unused_3: u32) -> u32 {
    const TASK_ID: u32 = 0x504;
    const TASK_FLAGS: u32 = 0x508;
    const DIRTY: u32 = 0x58a;
    unsafe {
        (this.wrapping_add(TASK_ID) as *mut u32).write_unaligned(task_id);
        (this.wrapping_add(TASK_FLAGS) as *mut u16).write_unaligned(task_flags as u16);
        (this.wrapping_add(DIRTY) as *mut u8).write(0);
    }
    task_flags & 0xffff
});
