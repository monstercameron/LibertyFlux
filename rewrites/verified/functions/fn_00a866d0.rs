// original: 0x00A866D0 CSimpleNMBraceTaskInfo::write_task_data
/// Calls the die-task serializer, then writes the 16-bit field at offset 0x1E and the byte at offset 0x1C.
///
/// The task object is in ECX and the serializer is the one-word stack argument.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00a866d0(task_info: u32, writer: u32) -> u32 {
    const FIELD_0_OFFSET: usize = 0x1E;
    let field_0 = unsafe { ((task_info as *const u8).add(FIELD_0_OFFSET) as *const u16).read_unaligned() as u32 };
    const FIELD_1_OFFSET: usize = 0x1C;
    let field_1 = unsafe { ((task_info as *const u8).add(FIELD_1_OFFSET) as *const u8).read_unaligned() as u32 };

    let call_result_0 = lf_checker_rt::callee_thiscall!(1, u32, task_info, writer);
    let call_result_1 = lf_checker_rt::callee_thiscall!(2, u32, writer, field_0);
    let call_result_2 = lf_checker_rt::callee_thiscall!(3, u32, writer, field_1);
    call_result_2
}
