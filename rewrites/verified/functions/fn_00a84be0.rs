// original: 0x00A84BE0 CComplexNewGetInVehicleTaskInfo::deserialize

/// Deserializes the vehicle-task base, then reads one byte into the field at
/// byte offset 0x30. The stream reader is called once after the base method.
/// This thiscall method consumes one stream pointer and returns the reader's
/// EAX value.
lf_checker_rt::export!(thiscall, rw_00A84BE0(task_info: u32, stream: u32) -> u32 {
    const BASE_DESERIALIZE: u32 = 1;
    const READ_BYTE: u32 = 2;
    const TASK_VALUE_OFFSET: u32 = 0x30;

    let _ = lf_checker_rt::callee_thiscall!(BASE_DESERIALIZE, u32, task_info, stream);
    lf_checker_rt::callee_thiscall!(
        READ_BYTE,
        u32,
        stream,
        task_info.wrapping_add(TASK_VALUE_OFFSET),
    )
});
