// original: 0x00A865A0 CSimpleCarDriveTaskInfo::write_task_data
/// Calls the complex-door serializer, then writes bits zero through five of the flags word at offset 0x1C as separate booleans.
///
/// The task object is in ECX and the serializer is the one-word stack argument.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00a865a0(task_info: u32, writer: u32) -> u32 {
    const FIELD_0_OFFSET: usize = 0x1C;
    let field_0 = unsafe { ((task_info as *const u8).add(FIELD_0_OFFSET) as *const u32).read_unaligned() as u32 };

    let call_result_0 = lf_checker_rt::callee_thiscall!(1, u32, task_info, writer);
    let call_result_1 = lf_checker_rt::callee_thiscall!(2, u32, writer, ((field_0 >> 0) & 1));
    let call_result_2 = lf_checker_rt::callee_thiscall!(2, u32, writer, ((field_0 >> 1) & 1));
    let call_result_3 = lf_checker_rt::callee_thiscall!(2, u32, writer, ((field_0 >> 2) & 1));
    let call_result_4 = lf_checker_rt::callee_thiscall!(2, u32, writer, ((field_0 >> 3) & 1));
    let call_result_5 = lf_checker_rt::callee_thiscall!(2, u32, writer, ((field_0 >> 4) & 1));
    let call_result_6 = lf_checker_rt::callee_thiscall!(2, u32, writer, ((field_0 >> 5) & 1));
    call_result_6
}
