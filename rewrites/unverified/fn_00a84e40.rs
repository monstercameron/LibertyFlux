// original: 0x00A84E40 CMeleeTaskInfo::deserialize

/// Deserializes the target-task base, reads a typed 32-bit value through a
/// pointer to a zero-initialized frame word, stores that value at object
/// offset 0x1C, and returns it. The method is thiscall and consumes one
/// stream pointer.
lf_checker_rt::export!(thiscall, rw_00A84E40(task_info: u32, stream: u32) -> u32 {
    const BASE_DESERIALIZE: u32 = 1;
    const READ_VALUE: u32 = 2;
    const VALUE_TYPE: u32 = 0x20;
    const MELEE_VALUE_OFFSET: u32 = 0x1C;

    let _ = lf_checker_rt::callee_thiscall!(BASE_DESERIALIZE, u32, task_info, stream);
    let mut decoded_value = 0u32;
    let _ = lf_checker_rt::callee_thiscall!(
        READ_VALUE,
        u32,
        stream,
        (&mut decoded_value as *mut u32 as usize) as u32,
        VALUE_TYPE,
    );
    unsafe {
        core::ptr::write_unaligned(
            task_info.wrapping_add(MELEE_VALUE_OFFSET) as *mut u32,
            decoded_value,
        );
    }
    decoded_value
});