// original: 0x00A86720 CSimpleNMFlinchTaskInfo::write_task_data
/// Calls the die-task serializer, writes the 19-byte record at offset 0x20, then writes the word at 0x36 and byte at 0x34.
///
/// The task object is in ECX and the serializer is the one-word stack argument.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00a86720(task_info: u32, writer: u32) -> u32 {
    const RECORD_BYTE_COUNT: u32 = 0x13;
    const FIELD_1_OFFSET: usize = 0x36;
    let field_1 = unsafe { ((task_info as *const u8).add(FIELD_1_OFFSET) as *const u16).read_unaligned() as u32 };
    const FIELD_2_OFFSET: usize = 0x34;
    let field_2 = unsafe { ((task_info as *const u8).add(FIELD_2_OFFSET) as *const u8).read_unaligned() as u32 };
    const TASK_RECORD_OFFSET_0: u32 = 0x20;
    let task_record_0 = task_info.wrapping_add(TASK_RECORD_OFFSET_0);

    let call_result_0 = lf_checker_rt::callee_thiscall!(1, u32, task_info, writer);
    let call_result_1 = lf_checker_rt::callee_thiscall!(2, u32, writer, task_record_0, RECORD_BYTE_COUNT);
    let call_result_2 = lf_checker_rt::callee_thiscall!(3, u32, writer, field_1);
    let call_result_3 = lf_checker_rt::callee_thiscall!(4, u32, writer, field_2);
    call_result_3
}
