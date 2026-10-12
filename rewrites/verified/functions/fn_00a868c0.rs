// original: 0x00A868C0 CStandUpTaskInfo::write_task_data
/// Calls the die-task serializer, then writes the 32-bit fields at offsets 0x18 and 0x1C with tag 0x10.
///
/// The task object is in ECX and the serializer is the one-word stack argument.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00a868c0(task_info: u32, writer: u32) -> u32 {
    const FIELD_0_OFFSET: usize = 0x18;
    let field_0 = unsafe { ((task_info as *const u8).add(FIELD_0_OFFSET) as *const u32).read_unaligned() as u32 };
    const TASK_WORD_TAG: u32 = 0x10;
    const FIELD_2_OFFSET: usize = 0x1C;
    let field_2 = unsafe { ((task_info as *const u8).add(FIELD_2_OFFSET) as *const u32).read_unaligned() as u32 };

    let call_result_0 = lf_checker_rt::callee_thiscall!(1, u32, task_info, writer);
    let call_result_1 = lf_checker_rt::callee_thiscall!(2, u32, writer, field_0, TASK_WORD_TAG);
    let call_result_2 = lf_checker_rt::callee_thiscall!(2, u32, writer, field_2, TASK_WORD_TAG);
    call_result_2
}
