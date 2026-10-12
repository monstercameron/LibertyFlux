// original: 0x00A86580 CPlayRandomAmbientsInfo::write_task_data
/// Calls the die-task serializer, then writes the byte at offset 0x18 as an unsigned word.
///
/// The task object is in ECX and the serializer is the one-word stack argument.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00a86580(task_info: u32, writer: u32) -> u32 {
    const FIELD_0_OFFSET: usize = 0x18;
    let field_0 = unsafe { ((task_info as *const u8).add(FIELD_0_OFFSET) as *const u8).read_unaligned() as u32 };

    let call_result_0 = lf_checker_rt::callee_thiscall!(1, u32, task_info, writer);
    let call_result_1 = lf_checker_rt::callee_thiscall!(2, u32, writer, field_0);
    call_result_1
}
