// original: 0x00a82d00 task_info_get_progress_entry_and_value

/// Find a task-list entry, invoke its second virtual slot, and read its value.
///
/// `this` is the task list; `sequence_id` and `requested_stage` are forwarded
/// to the shared lookup. A null result returns zero. Otherwise the entry's
/// virtual slot at byte offset `+0x04` is called with the entry in ECX, then
/// the 32-bit value at entry offset `+0x24` is returned. The virtual call's
/// return is intentionally ignored.
lf_checker_rt::export!(thiscall, rw_00a82d00(this: u32, sequence_id: u32, requested_stage: u32) -> u32 {
    const LOOKUP_CALLEE: u32 = 1;
    const ENTRY_VALUE_SLOT: u32 = 2;
    const ENTRY_VALUE: u32 = 0x24;

    let entry = lf_checker_rt::callee_thiscall!(LOOKUP_CALLEE, u32, this, sequence_id, requested_stage);
    if entry == 0 {
        return 0;
    }
    let _virtual_result = lf_checker_rt::callee_thiscall!(ENTRY_VALUE_SLOT, u32, entry);
    unsafe { ((entry + ENTRY_VALUE) as *const u32).read_unaligned() }
});
