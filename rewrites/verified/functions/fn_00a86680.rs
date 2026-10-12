// original: 0x00A86680 CSimpleMeleeActionResultTaskInfo::write_task_data
/// Serializes the base task, then an optional entity identifier, a 32-bit
/// result field, and one byte of flags. A missing entity or nested identifier
/// produces zero; the helper's identifier result is truncated to 16 bits.
/// The task object is in ECX and the writer is the one-word stack argument.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00a86680(task_info: u32, writer: u32) -> u32 {
    const ENTITY_POINTER_OFFSET: usize = 0x18;
    const RESULT_VALUE_OFFSET: usize = 0x1C;
    const FLAGS_OFFSET: usize = 0x20;
    const NESTED_IDENTIFIER_OFFSET: usize = 0x6C;

    lf_checker_rt::callee_thiscall!(1, u32, task_info, writer);
    let entity = unsafe { ((task_info as *const u8).add(ENTITY_POINTER_OFFSET) as *const u32).read_unaligned() };
    let identifier = if entity == 0 {
        0
    } else {
        let nested = unsafe { ((entity as *const u8).add(NESTED_IDENTIFIER_OFFSET) as *const u32).read_unaligned() };
        if nested == 0 { 0 } else { lf_checker_rt::callee_thiscall!(2, u32, nested) }
    };
    lf_checker_rt::callee_thiscall!(3, u32, writer, identifier & 0xFFFF);

    let result_value = unsafe { ((task_info as *const u8).add(RESULT_VALUE_OFFSET) as *const u32).read_unaligned() };
    lf_checker_rt::callee_thiscall!(4, u32, writer, result_value, 0x20);

    let flags = unsafe { ((task_info as *const u8).add(FLAGS_OFFSET)).read() as u32 };
    lf_checker_rt::callee_thiscall!(5, u32, writer, flags)
}
