// original: 0x00A86940 CTargetTaskInfo::write_task_data
/// Writes the 16-bit field at offset 0x18 through the supplied serializer.
///
/// The task object is in ECX and the serializer is the one-word stack argument.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00a86940(task_info: u32, writer: u32) -> u32 {
    const FIELD_0_OFFSET: usize = 0x18;
    let field_0 = unsafe { ((task_info as *const u8).add(FIELD_0_OFFSET) as *const u16).read_unaligned() as u32 };

    let call_result_0 = lf_checker_rt::callee_thiscall!(1, u32, writer, field_0);
    call_result_0
}
