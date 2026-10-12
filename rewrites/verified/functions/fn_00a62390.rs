// original: 0x00A62390 intel_table_set_slot0

/// Forwards two arguments and a fixed table slot to the task-table helper.
/// The helper object is the caller's object plus 0x44; the slot is 0.
/// The original callee removes all three forwarded stack words, while this
/// wrapper removes its two incoming words. Its EAX value is not a semantic
/// result and is not compared.
lf_checker_rt::export!(thiscall, rw_00A62390(this: u32, first_value: u32, second_value: u32) -> u32 {
    const TASK_TABLE: u32 = 0x44;
    const SLOT: u32 = 0;
    let helper_object = this.wrapping_add(TASK_TABLE);
    lf_checker_rt::callee_thiscall!(1, u32, helper_object, first_value, SLOT, second_value);
    0
});
