// original: 0x00A86630 CSimpleClimbTaskInfo::write_task_data
/// Calls the die-task serializer, writes four byte fields, then writes the 19-byte record at offset 0x20.
///
/// The task object is in ECX and the serializer is the one-word stack argument.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00a86630(task_info: u32, writer: u32) -> u32 {
    const FIELD_0_OFFSET: usize = 0x18;
    let field_0 = unsafe { ((task_info as *const u8).add(FIELD_0_OFFSET) as *const u8).read_unaligned() as u32 };
    const FIELD_1_OFFSET: usize = 0x19;
    let field_1 = unsafe { ((task_info as *const u8).add(FIELD_1_OFFSET) as *const u8).read_unaligned() as u32 };
    const FIELD_2_OFFSET: usize = 0x1A;
    let field_2 = unsafe { ((task_info as *const u8).add(FIELD_2_OFFSET) as *const u8).read_unaligned() as u32 };
    const FIELD_3_OFFSET: usize = 0x1B;
    let field_3 = unsafe { ((task_info as *const u8).add(FIELD_3_OFFSET) as *const u8).read_unaligned() as u32 };
    const RECORD_BYTE_COUNT: u32 = 0x13;
    const TASK_RECORD_OFFSET_0: u32 = 0x20;
    let task_record_0 = task_info.wrapping_add(TASK_RECORD_OFFSET_0);

    let call_result_0 = lf_checker_rt::callee_thiscall!(1, u32, task_info, writer);
    let call_result_1 = lf_checker_rt::callee_thiscall!(2, u32, writer, field_0);
    let call_result_2 = lf_checker_rt::callee_thiscall!(2, u32, writer, field_1);
    let call_result_3 = lf_checker_rt::callee_thiscall!(2, u32, writer, field_2);
    let call_result_4 = lf_checker_rt::callee_thiscall!(2, u32, writer, field_3);
    let call_result_5 = lf_checker_rt::callee_thiscall!(3, u32, writer, task_record_0, RECORD_BYTE_COUNT);
    call_result_5
}
