// original: 0x00A86340 CJackTaskInfo::write_task_data
/// Writes the one-byte task field at offset 0x14 through the supplied
/// serializer, zero-extending the byte to the helper's 32-bit argument.
///
/// The task object is a 32-bit game structure. The serializer is called once
/// with the writer in ECX; its 32-bit return value is left in EAX.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00a86340(task_info: u32, writer: u32) -> u32 {
    const TASK_FIELD_OFFSET: usize = 0x14;
    let field_byte = unsafe { ((task_info as *const u8).add(TASK_FIELD_OFFSET)).read() };
    let serialized_value = u32::from(field_byte);
    lf_checker_rt::callee_thiscall!(1, u32, writer, serialized_value)
}
