// original: 0x00A84D10 CJackTaskInfo::deserialize

/// Reads one 32-bit task value from the stream into object offset 0x14 and
/// returns the reader's 32-bit result. The method is thiscall and consumes
/// one stream pointer; the reader is represented by a scripted direct-call
/// stand-in in the proof contract.
lf_checker_rt::export!(thiscall, rw_00A84D10(task_info: u32, stream: u32) -> u32 {
    const READ_TASK_VALUE: u32 = 1;
    const TASK_VALUE_OFFSET: u32 = 0x14;
    let destination = task_info.wrapping_add(TASK_VALUE_OFFSET);
    lf_checker_rt::callee_thiscall!(READ_TASK_VALUE, u32, stream, destination)
});