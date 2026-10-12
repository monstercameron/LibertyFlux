// original: 0x00A86960 CDieInfo::write_task_data
/// Writes the 32-bit field at offset 0x14 with tag 0x20 through the supplied serializer.
///
/// The task object is in ECX and the serializer is the one-word stack argument.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00a86960(task_info: u32, writer: u32) -> u32 {
    const FIELD_0_OFFSET: usize = 0x14;
    let field_0 = unsafe { ((task_info as *const u8).add(FIELD_0_OFFSET) as *const u32).read_unaligned() as u32 };
    const TASK_WORD_TAG: u32 = 0x20;

    let call_result_0 = lf_checker_rt::callee_thiscall!(1, u32, writer, field_0, TASK_WORD_TAG);
    call_result_0
}
