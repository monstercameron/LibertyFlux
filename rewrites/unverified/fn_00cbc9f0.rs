// original: 0x00CBC9F0 CTaskComplexMoveFollowPointRoute::vf12
/// Copy the task handle at byte offset `0x24` from the task object into the
/// output record at byte offset `0x38`. The method is thiscall, takes the
/// output record as one stack argument, and returns that output pointer in
/// EAX.
lf_checker_rt::export!(thiscall, rw_00cbc9f0(this: *mut u8, output: *mut u8) -> u32 {
    const TASK_HANDLE_OFFSET: usize = 0x24;
    const OUTPUT_HANDLE_OFFSET: usize = 0x38;

    // SAFETY: the caller supplies valid task and output records.
    let handle = unsafe { this.add(TASK_HANDLE_OFFSET).cast::<u32>().read_unaligned() };
    unsafe {
        output
            .add(OUTPUT_HANDLE_OFFSET)
            .cast::<u32>()
            .write_unaligned(handle);
    }
    output as u32
});
