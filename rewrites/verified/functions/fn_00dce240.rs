// original: 0x00DCE240 CTaskSimpleAimGun::vf5
/// Sets the task's pending-state flag, forwards the first stack argument
/// to its update helper, clears the low enabled bit and three transient
/// handles, then returns one. The other two stack words are not read.
lf_checker_rt::export!(thiscall, rw_00dce240(
    task: u32, action_id: u32, _context: u32, _flags: u32
) -> u32 {
    const UPDATE_ID: u32 = 1;
    const PENDING_OFFSET: usize = 0x9c;
    const FLAGS_OFFSET: usize = 0xa4;
    const CLEARED_HANDLES: [usize; 3] = [0x80, 0x84, 0xa8];
    let _ = lf_checker_rt::callee_thiscall!(UPDATE_ID, u32, task, action_id);
    // SAFETY: the contract supplies a writable task object.
    unsafe {
        let bytes = task as *mut u8;
        (bytes.add(PENDING_OFFSET) as *mut u32).write(1);
        let flags = (bytes.add(FLAGS_OFFSET) as *mut u32).read();
        (bytes.add(FLAGS_OFFSET) as *mut u32).write(flags & !1);
        for offset in CLEARED_HANDLES {
            (bytes.add(offset) as *mut u32).write(0);
        }
    }
    1
});
