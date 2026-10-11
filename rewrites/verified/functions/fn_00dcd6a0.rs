// original: 0x00DCD6A0 reload_task_clear_state
/// Clears the task's transient state fields and, when the low byte of the
/// flag argument is nonzero, selects stage 3. The method is thiscall and
/// pops one argument; the flag comparison is unsigned byte-versus-zero.
lf_checker_rt::export!(thiscall, rw_00dcd6a0(task: u32, flag_word: u32) -> () {
    const LINK_OFFSET: usize = 0x4c;
    const STAGE_OFFSET: usize = 0x54;
    const TIMER_OFFSET: usize = 0x64;
    // SAFETY: the checker contract supplies a writable task object.
    unsafe {
        let bytes = task as *mut u8;
        (bytes.add(LINK_OFFSET) as *mut u32).write(0);
        (bytes.add(TIMER_OFFSET) as *mut u32).write(0);
        if flag_word as u8 != 0 {
            (bytes.add(STAGE_OFFSET) as *mut u32).write(3);
        }
    }
});
