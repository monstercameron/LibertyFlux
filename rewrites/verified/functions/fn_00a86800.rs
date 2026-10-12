// original: 0x00A86800 CSitDownIdleThenStandTaskInfo::write_task_data
/// Serializes the base task, writes two 32-bit fields (incrementing the
/// second with wrapping arithmetic), passes the stored angle and one-turn
/// constant to the writer, writes two byte flags, and supplies a 19-byte
/// task payload. The stored angle's 32-bit float representation is transported
/// as a stack word without arithmetic.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00a86800(task_info: u32, writer: u32) -> u32 {
    const FIRST_FIELD: usize = 0x18;
    const SECOND_FIELD: usize = 0x1C;
    const ANGLE_BITS: usize = 0x20;
    const FIRST_FLAG: usize = 0x24;
    const SECOND_FLAG: usize = 0x25;
    const PAYLOAD: usize = 0x30;
    const ONE_TURN_BITS: u32 = 0x40C90FDB;

    lf_checker_rt::callee_thiscall!(1, u32, task_info, writer);
    let first_value = unsafe { ((task_info as *const u8).add(FIRST_FIELD) as *const u32).read_unaligned() };
    lf_checker_rt::callee_thiscall!(2, u32, writer, first_value, 3);
    let second_value = unsafe { ((task_info as *const u8).add(SECOND_FIELD) as *const u32).read_unaligned() };
    lf_checker_rt::callee_thiscall!(2, u32, writer, second_value.wrapping_add(1), 3);

    let angle_bits = unsafe { ((task_info as *const u8).add(ANGLE_BITS) as *const u32).read_unaligned() };
    lf_checker_rt::callee_thiscall!(3, u32, writer, angle_bits, ONE_TURN_BITS, 8);

    let first_flag = unsafe { ((task_info as *const u8).add(FIRST_FLAG)).read() as u32 };
    lf_checker_rt::callee_thiscall!(4, u32, writer, first_flag);
    let second_flag = unsafe { ((task_info as *const u8).add(SECOND_FLAG)).read() as u32 };
    lf_checker_rt::callee_thiscall!(4, u32, writer, second_flag);

    let payload_pointer = task_info.wrapping_add(PAYLOAD as u32);
    lf_checker_rt::callee_thiscall!(5, u32, writer, payload_pointer, 0x13)
}
