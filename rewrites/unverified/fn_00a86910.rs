// original: 0x00A86910 CComplexOpenVehicleDoorTaskInfo::write_task_data
/// Serializes the base task and asks the object's virtual slot at byte offset
/// 0x44 for a quantity. If the first call returns nonzero, the same method is
/// called again and its result is written with the stored 32-bit field.
/// The object is in ECX and the writer is the one-word stack argument.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00a86910(task_info: u32, writer: u32) -> u32 {
    const FIELD_18: usize = 0x18;
    const VTABLE_SLOT: usize = 0x44;

    lf_checker_rt::callee_thiscall!(1, u32, task_info, writer);
    let table = unsafe { (task_info as *const u32).read_unaligned() };
    let method = unsafe { ((table as *const u8).add(VTABLE_SLOT) as *const u32).read_unaligned() };
    let invoke: extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(method as usize) };
    let first_result = invoke(task_info);
    if first_result == 0 {
        return 0;
    }

    let second_result = invoke(task_info);
    let field = unsafe { ((task_info as *const u8).add(FIELD_18) as *const u32).read_unaligned() };
    lf_checker_rt::callee_thiscall!(2, u32, writer, field, second_result)
}
