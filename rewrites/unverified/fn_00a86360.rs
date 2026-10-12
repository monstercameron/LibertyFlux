// original: 0x00A86360 CMedicTaskInfo::write_task_data
/// Serializes the base task, then two optional entity identifiers, and the
/// low flag bit. Each identifier is zero when its entity pointer or nested
/// identifier pointer is null; otherwise it is the helper's low 16-bit result.
/// The object is in ECX and the writer is the one-word stack argument.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00a86360(task_info: u32, writer: u32) -> u32 {
    const FIRST_ENTITY_OFFSET: usize = 0x1C;
    const SECOND_ENTITY_OFFSET: usize = 0x20;
    const ENTITY_ID_OFFSET: usize = 0x6C;
    const FLAGS_OFFSET: usize = 0x24;

    lf_checker_rt::callee_thiscall!(1, u32, task_info, writer);
    let first_entity = unsafe { ((task_info as *const u8).add(FIRST_ENTITY_OFFSET) as *const u32).read_unaligned() };
    let first_identifier = if first_entity == 0 {
        0
    } else {
        let identifier_pointer = unsafe { ((first_entity as *const u8).add(ENTITY_ID_OFFSET) as *const u32).read_unaligned() };
        if identifier_pointer == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(2, u32, identifier_pointer) & 0xFFFF
        }
    };

    let second_entity = unsafe { ((task_info as *const u8).add(SECOND_ENTITY_OFFSET) as *const u32).read_unaligned() };
    let second_identifier = if second_entity == 0 {
        0
    } else {
        let identifier_pointer = unsafe { ((second_entity as *const u8).add(ENTITY_ID_OFFSET) as *const u32).read_unaligned() };
        if identifier_pointer == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(2, u32, identifier_pointer) & 0xFFFF
        }
    };

    lf_checker_rt::callee_thiscall!(3, u32, writer, first_identifier);
    lf_checker_rt::callee_thiscall!(3, u32, writer, second_identifier);
    let flags = unsafe { ((task_info as *const u8).add(FLAGS_OFFSET)).read() as u32 };
    lf_checker_rt::callee_thiscall!(4, u32, writer, flags & 1)
}
