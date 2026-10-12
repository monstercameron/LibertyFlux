// original: 0x00A84B60 CComplexJumpTaskInfo::deserialize

/// Deserializes the common task-info base, then reads three byte fields into
/// consecutive offsets 0x18, 0x19 and 0x1A. The stream calls remain in their
/// original order. This thiscall method consumes one stream pointer and
/// returns the last byte-reader's EAX value.
lf_checker_rt::export!(thiscall, rw_00A84B60(task_info: u32, stream: u32) -> u32 {
    const BASE_DESERIALIZE: u32 = 1;
    const READ_BYTE: u32 = 2;
    const FIRST_FLAG_OFFSET: u32 = 0x18;

    let _ = lf_checker_rt::callee_thiscall!(BASE_DESERIALIZE, u32, task_info, stream);
    let _ = lf_checker_rt::callee_thiscall!(
        READ_BYTE,
        u32,
        stream,
        task_info.wrapping_add(FIRST_FLAG_OFFSET),
    );
    let _ = lf_checker_rt::callee_thiscall!(
        READ_BYTE,
        u32,
        stream,
        task_info.wrapping_add(FIRST_FLAG_OFFSET + 1),
    );
    lf_checker_rt::callee_thiscall!(
        READ_BYTE,
        u32,
        stream,
        task_info.wrapping_add(FIRST_FLAG_OFFSET + 2),
    )
});
