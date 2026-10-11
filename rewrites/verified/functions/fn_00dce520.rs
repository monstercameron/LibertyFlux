// original: 0x00DCE520 aim_task_advance_deadline
/// Copies the current aim deadline into its previous-deadline field,
/// marks the deadline as initialized, then stores the requested value.
/// The three fields occupy offsets 0x7c, 0x80, and 0x84.
lf_checker_rt::export!(thiscall, rw_00dce520(task: u32, deadline: u32) -> () {
    const INITIALIZED_OFFSET: usize = 0x7c;
    const DEADLINE_OFFSET: usize = 0x80;
    const PREVIOUS_DEADLINE_OFFSET: usize = 0x84;
    // SAFETY: the checker contract supplies a writable task object.
    unsafe {
        let bytes = task as *mut u8;
        let current = bytes.add(DEADLINE_OFFSET) as *mut u32;
        (bytes.add(PREVIOUS_DEADLINE_OFFSET) as *mut u32).write(current.read());
        bytes.add(INITIALIZED_OFFSET).write(1);
        current.write(deadline);
    }
});
