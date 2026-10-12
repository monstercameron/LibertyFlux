// original: 0x00A84C70 CComplexPickUpAndCarryObjectTaskInfo::deserialize

/// Deserializes the status-and-target base, reads a typed 32-bit code through
/// a pointer to its saved-this stack slot, adds 0x48 with 32-bit wrapping, and
/// stores the resulting pointer-like value at object offset 0x24. The method
/// returns that same value in EAX. It is thiscall and consumes one stream
/// pointer.
lf_checker_rt::export!(thiscall, rw_00A84C70(task_info: u32, stream: u32) -> u32 {
    const BASE_DESERIALIZE: u32 = 1;
    const READ_TASK_CODE: u32 = 2;
    const TASK_CODE_TYPE: u32 = 2;
    const TASK_CODE_BIAS: u32 = 0x48;
    const RESULT_FIELD_OFFSET: u32 = 0x24;

    let _ = lf_checker_rt::callee_thiscall!(BASE_DESERIALIZE, u32, task_info, stream);

    // The original points the reader at its saved ECX slot. Its initial word
    // is the task-info pointer, so the stand-in sees the same pre-call value.
    let mut task_code = task_info;
    let _ = lf_checker_rt::callee_thiscall!(
        READ_TASK_CODE,
        u32,
        stream,
        (&mut task_code as *mut u32 as usize) as u32,
        TASK_CODE_TYPE,
    );
    let result = task_code.wrapping_add(TASK_CODE_BIAS);
    unsafe {
        core::ptr::write_unaligned(
            task_info.wrapping_add(RESULT_FIELD_OFFSET) as *mut u32,
            result,
        );
    }
    result
});
