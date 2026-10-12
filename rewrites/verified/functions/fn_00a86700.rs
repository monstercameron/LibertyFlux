// original: 0x00A86700 CSimpleNMExplosionTaskInfo::write_task_data
/// Calls the die-task serializer, then writes the 19-byte record beginning at offset 0x20.
///
/// The task object is in ECX and the serializer is the one-word stack argument.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00a86700(task_info: u32, writer: u32) -> u32 {
    const RECORD_BYTE_COUNT: u32 = 0x13;
    const TASK_RECORD_OFFSET_0: u32 = 0x20;
    let task_record_0 = task_info.wrapping_add(TASK_RECORD_OFFSET_0);

    let call_result_0 = lf_checker_rt::callee_thiscall!(1, u32, task_info, writer);
    let call_result_1 = lf_checker_rt::callee_thiscall!(2, u32, writer, task_record_0, RECORD_BYTE_COUNT);
    call_result_1
}
