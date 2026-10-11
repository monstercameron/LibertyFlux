// original: 0x00b7bb30 CTaskComplexCarDriveWander::vf20

/// Mark the pointed-to subtask as a wander task, then forward this object
/// and the original argument to the shared drive-to-point implementation.
/// The argument object points to its nested task at `+0x224`; the byte flag
/// at nested offset `+0x2dc` is set to one. The callee's result is returned.
///
/// Original ABI: thiscall with one pointer argument and an unconditional
/// tail jump after the byte store.
lf_checker_rt::export!(thiscall, rw_00b7bb30(this: u32, task: u32) -> u32 {
    unsafe {
        const NESTED_TASK: u32 = 0x224;
        const WANDER_FLAG: u32 = 0x2dc;
        let nested = ((task + NESTED_TASK) as *const u32).read_unaligned();
        ((nested + WANDER_FLAG) as *mut u8).write(1);
        lf_checker_rt::callee_thiscall!(1, u32, this, task)
    }
});
