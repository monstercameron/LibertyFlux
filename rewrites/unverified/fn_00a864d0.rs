// original: 0x00A864D0 CScriptAnimTaskInfo::write_task_data
/// Serializes the base task, converts the stored animation value through a
/// helper, then writes the task's integer fields and byte flag. The stored
/// float is passed by value to the helper without arithmetic; the writer is
/// the one-word stack argument and the task object is in ECX.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00a864d0(task_info: u32, writer: u32) -> u32 {
    const FIELD_14: usize = 0x14;
    const FIELD_18: usize = 0x18;
    const ANIMATION_VALUE: usize = 0x1C;
    const FIELD_20: usize = 0x20;
    const FLAG_BYTE: usize = 0x24;

    lf_checker_rt::callee_thiscall!(1, u32, task_info, writer);
    let animation_bits = unsafe { ((task_info as *const u8).add(ANIMATION_VALUE) as *const u32).read_unaligned() };
    let animation_value = f32::from_bits(animation_bits);
    let converted = lf_checker_rt::callee_cdecl!(2, u32, animation_value);

    let value_14 = unsafe { ((task_info as *const u8).add(FIELD_14) as *const u32).read_unaligned() };
    lf_checker_rt::callee_thiscall!(3, u32, writer, value_14, 0x0C);
    let value_18 = unsafe { ((task_info as *const u8).add(FIELD_18) as *const u32).read_unaligned() };
    lf_checker_rt::callee_thiscall!(4, u32, writer, value_18, 0x20);
    lf_checker_rt::callee_thiscall!(4, u32, writer, converted, 8);
    let flag = unsafe { ((task_info as *const u8).add(FLAG_BYTE)).read() as u32 };
    lf_checker_rt::callee_thiscall!(4, u32, writer, flag, 3);
    let value_20 = unsafe { ((task_info as *const u8).add(FIELD_20) as *const u32).read_unaligned() };
    lf_checker_rt::callee_thiscall!(4, u32, writer, value_20, 0x20)
}
