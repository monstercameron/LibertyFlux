// original: 0x00A84CA0 CComplexSlideIntoCoverInfo::deserialize

/// Deserializes the common task-info base, checks the stream's slide flag,
/// and on the true branch reads an index from the stream, advances by the
/// sum of two stream fields, and maps that index to a subtask id. The false
/// branch maps index zero. The mapped id is stored at object offset 0x18
/// and returned. The method is thiscall and consumes one stream pointer.
lf_checker_rt::export!(thiscall, rw_00A84CA0(task_info: u32, stream: u32) -> u32 {
    const BASE_DESERIALIZE: u32 = 1;
    const HAS_SLIDE_DATA: u32 = 2;
    const READ_INDEX: u32 = 3;
    const ADVANCE_STREAM: u32 = 4;
    const INDEX_TO_SUBTASK: u32 = 5;
    const FLAG_TYPE: u32 = 5;
    const INDEX_TYPE: u32 = 5;
    const SUBTASK_FIELD_OFFSET: u32 = 0x18;

    let _ = lf_checker_rt::callee_thiscall!(BASE_DESERIALIZE, u32, task_info, stream);
    let advanced_stream = stream.wrapping_add(4);
    let has_slide_data = lf_checker_rt::callee_thiscall!(
        HAS_SLIDE_DATA,
        u32,
        advanced_stream,
        FLAG_TYPE,
    ) & 0xff != 0;

    let index = if has_slide_data {
        let mut parsed_index = stream;
        let stream_value = unsafe { core::ptr::read_unaligned(advanced_stream as *const u32) };
        let stream_count = unsafe { core::ptr::read_unaligned(stream.wrapping_add(8) as *const u32) };
        let stream_end = unsafe { core::ptr::read_unaligned(stream.wrapping_add(16) as *const u32) };
        let _ = lf_checker_rt::callee_cdecl!(
            READ_INDEX,
            u32,
            stream_value,
            (&mut parsed_index as *mut u32 as usize) as u32,
            INDEX_TYPE,
            stream_count.wrapping_add(stream_end),
        );
        let _ = lf_checker_rt::callee_thiscall!(
            ADVANCE_STREAM,
            u32,
            advanced_stream,
            FLAG_TYPE,
        );
        parsed_index
    } else {
        0
    };

    let mapped_id = lf_checker_rt::callee_thiscall!(INDEX_TO_SUBTASK, u32, task_info, index);
    unsafe {
        core::ptr::write_unaligned(
            task_info.wrapping_add(SUBTASK_FIELD_OFFSET) as *mut u32,
            mapped_id,
        );
    }
    mapped_id
});