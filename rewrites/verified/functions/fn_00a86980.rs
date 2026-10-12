// original: 0x00A86980 CVehicleTaskInfo::write_task_data
/// Serializes the base vehicle task and a selected model field. The value at
/// offset 0x24 selects the field at 0x24 or 0x28 and emits a matching flag.
/// The object's virtual slot at 0x50 returns a signed count: nonpositive
/// values are returned directly; positive values cause a second query and a
/// final write using the field at 0x2C. The object is in ECX.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00a86980(task_info: u32, writer: u32) -> u32 {
    const MODEL_KIND: usize = 0x24;
    const ALTERNATE_MODEL: usize = 0x28;
    const LAST_FIELD: usize = 0x2C;
    const VTABLE_SLOT: usize = 0x50;

    lf_checker_rt::callee_thiscall!(1, u32, task_info, writer);
    let model_kind = unsafe { ((task_info as *const u8).add(MODEL_KIND) as *const u32).read_unaligned() };
    let (has_model_kind, model_value) = if model_kind == u32::MAX {
        (0, unsafe { ((task_info as *const u8).add(ALTERNATE_MODEL) as *const u32).read_unaligned() })
    } else {
        (1, model_kind)
    };
    lf_checker_rt::callee_thiscall!(2, u32, writer, has_model_kind);
    lf_checker_rt::callee_thiscall!(3, u32, writer, model_value, 5);

    let table = unsafe { (task_info as *const u32).read_unaligned() };
    let method = unsafe { ((table as *const u8).add(VTABLE_SLOT) as *const u32).read_unaligned() };
    let query: extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(method as usize) };
    let count = query(task_info);
    if (count as i32) <= 0 {
        return count;
    }

    let actual_count = query(task_info);
    let last_value = unsafe { ((task_info as *const u8).add(LAST_FIELD) as *const u32).read_unaligned() };
    lf_checker_rt::callee_thiscall!(5, u32, writer, last_value, actual_count)
}
