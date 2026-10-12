// original: 0x00A86530 CSequenceTaskInfo::write_task_data
/// Writes a tagged word, two sign-extended byte fields with tag 6, then an unsigned byte field.
///
/// The task object is in ECX and the serializer is the one-word stack argument.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00a86530(task_info: u32, writer: u32) -> u32 {
    const FIELD_0_OFFSET: usize = 0x14;
    let field_0 = unsafe { ((task_info as *const u8).add(FIELD_0_OFFSET) as *const u32).read_unaligned() as u32 };
    const WORD_TAG: u32 = 0x8;
    const FIELD_2_OFFSET: usize = 0x18;
    let field_2 = unsafe { ((task_info as *const u8).add(FIELD_2_OFFSET) as *const u8).read_unaligned() as u32 };
    const BYTE_TAG: u32 = 0x6;
    const FIELD_4_OFFSET: usize = 0x19;
    let field_4 = unsafe { ((task_info as *const u8).add(FIELD_4_OFFSET) as *const u8).read_unaligned() as u32 };
    const FIELD_5_OFFSET: usize = 0x1A;
    let field_5 = unsafe { ((task_info as *const u8).add(FIELD_5_OFFSET) as *const u8).read_unaligned() as u32 };

    let call_result_0 = lf_checker_rt::callee_thiscall!(1, u32, writer, field_0, WORD_TAG);
    let call_result_1 = lf_checker_rt::callee_thiscall!(2, u32, writer, i8::from_ne_bytes([field_2 as u8]) as i32 as u32, BYTE_TAG);
    let call_result_2 = lf_checker_rt::callee_thiscall!(2, u32, writer, i8::from_ne_bytes([field_4 as u8]) as i32 as u32, BYTE_TAG);
    let call_result_3 = lf_checker_rt::callee_thiscall!(3, u32, writer, field_5);
    call_result_3
}
