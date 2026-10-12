// original: 0x00A867B0 CSimpleSidewaysDiveTaskInfo::write_task_data
/// Calls the die-task serializer, then writes whether the 32-bit field at offset 0x18 is nonzero.
///
/// The task object is in ECX and the serializer is the one-word stack argument.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00a867b0(task_info: u32, writer: u32) -> u32 {
    const FIELD_0_OFFSET: usize = 0x18;
    let field_0 = unsafe { ((task_info as *const u8).add(FIELD_0_OFFSET) as *const u32).read_unaligned() as u32 };

    let call_result_0 = lf_checker_rt::callee_thiscall!(1, u32, task_info, writer);
    let call_result_1 = lf_checker_rt::callee_thiscall!(2, u32, writer, u32::from(field_0 != 0));
    call_result_1
}
