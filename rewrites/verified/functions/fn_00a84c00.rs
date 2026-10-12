// original: 0x00A84C00 CComplexNewUseCoverInfo::deserialize

/// Deserializes the common task-info base, reads a seven-byte block into
/// offset 0x18 using the stream advanced by four bytes, then reads six byte
/// fields into offsets 0x1C through 0x21 from that same advanced stream. The
/// thiscall method consumes one stream pointer and returns the last byte
/// reader's EAX value.
lf_checker_rt::export!(thiscall, rw_00A84C00(task_info: u32, stream: u32) -> u32 {
    const BASE_DESERIALIZE: u32 = 1;
    const READ_BLOCK: u32 = 2;
    const READ_BYTE: u32 = 3;
    const BLOCK_OFFSET: u32 = 0x18;
    const BLOCK_LENGTH: u32 = 7;
    const FIRST_FLAG_OFFSET: u32 = 0x1C;

    let _ = lf_checker_rt::callee_thiscall!(BASE_DESERIALIZE, u32, task_info, stream);

    let advanced_stream = stream.wrapping_add(4);
    let _ = lf_checker_rt::callee_thiscall!(
        READ_BLOCK,
        u32,
        advanced_stream,
        task_info.wrapping_add(BLOCK_OFFSET),
        BLOCK_LENGTH,
    );
    let mut result = 0;
    let mut byte_index = 0;
    while byte_index < 6 {
        result = lf_checker_rt::callee_thiscall!(
            READ_BYTE,
            u32,
            advanced_stream,
            task_info.wrapping_add(FIRST_FLAG_OFFSET + byte_index),
        );
        byte_index += 1;
    }
    result
});
