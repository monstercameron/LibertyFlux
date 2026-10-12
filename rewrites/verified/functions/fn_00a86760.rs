// original: 0x00A86760 CSimpleNMShotTaskInfo::write_task_data
/// Calls the die-task serializer, writes two tagged words, a 16-bit field, and a byte field.
///
/// The task object is in ECX and the serializer is the one-word stack argument.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00a86760(task_info: u32, writer: u32) -> u32 {
    const FIELD_0_OFFSET: usize = 0x18;
    let field_0 = unsafe { ((task_info as *const u8).add(FIELD_0_OFFSET) as *const u32).read_unaligned() as u32 };
    const FIRST_WORD_TAG: u32 = 0x6;
    const FIELD_2_OFFSET: usize = 0x1C;
    let field_2 = unsafe { ((task_info as *const u8).add(FIELD_2_OFFSET) as *const u32).read_unaligned() as u32 };
    const SECOND_WORD_TAG: u32 = 0x7;
    const FIELD_4_OFFSET: usize = 0x26;
    let field_4 = unsafe { ((task_info as *const u8).add(FIELD_4_OFFSET) as *const u16).read_unaligned() as u32 };
    const FIELD_5_OFFSET: usize = 0x24;
    let field_5 = unsafe { ((task_info as *const u8).add(FIELD_5_OFFSET) as *const u8).read_unaligned() as u32 };

    let call_result_0 = lf_checker_rt::callee_thiscall!(1, u32, task_info, writer);
    let call_result_1 = lf_checker_rt::callee_thiscall!(2, u32, writer, field_0, FIRST_WORD_TAG);
    let call_result_2 = lf_checker_rt::callee_thiscall!(2, u32, writer, field_2, SECOND_WORD_TAG);
    let call_result_3 = lf_checker_rt::callee_thiscall!(3, u32, writer, field_4);
    let call_result_4 = lf_checker_rt::callee_thiscall!(4, u32, writer, field_5);
    call_result_4
}
