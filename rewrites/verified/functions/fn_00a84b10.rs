// original: 0x00A84B10 CComplexGunTaskInfo::deserialize

/// Deserializes the status-task base, reads a typed word into byte offset
/// 0x40, reads three byte fields at offsets 0x44 through 0x46, then reads a
/// final typed word at offset 0x48. The stream helpers run in that order.
/// This thiscall method consumes one stream pointer and returns the final
/// typed reader's EAX value.
lf_checker_rt::export!(thiscall, rw_00A84B10(task_info: u32, stream: u32) -> u32 {
    const BASE_DESERIALIZE: u32 = 1;
    const READ_WORD: u32 = 2;
    const READ_BYTE: u32 = 3;
    const FIRST_WORD_OFFSET: u32 = 0x40;
    const FIRST_WORD_TYPE: u32 = 3;
    const BYTE_FIELD_OFFSET: u32 = 0x44;
    const LAST_WORD_OFFSET: u32 = 0x48;
    const LAST_WORD_TYPE: u32 = 2;

    let _ = lf_checker_rt::callee_thiscall!(BASE_DESERIALIZE, u32, task_info, stream);
    let _ = lf_checker_rt::callee_thiscall!(
        READ_WORD,
        u32,
        stream,
        task_info.wrapping_add(FIRST_WORD_OFFSET),
        FIRST_WORD_TYPE,
    );
    let _ = lf_checker_rt::callee_thiscall!(
        READ_BYTE,
        u32,
        stream,
        task_info.wrapping_add(BYTE_FIELD_OFFSET),
    );
    let _ = lf_checker_rt::callee_thiscall!(
        READ_BYTE,
        u32,
        stream,
        task_info.wrapping_add(BYTE_FIELD_OFFSET + 1),
    );
    let _ = lf_checker_rt::callee_thiscall!(
        READ_BYTE,
        u32,
        stream,
        task_info.wrapping_add(BYTE_FIELD_OFFSET + 2),
    );
    lf_checker_rt::callee_thiscall!(
        READ_WORD,
        u32,
        stream,
        task_info.wrapping_add(LAST_WORD_OFFSET),
        LAST_WORD_TYPE,
    )
});
