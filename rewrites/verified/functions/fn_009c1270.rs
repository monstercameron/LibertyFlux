// original: 0x009C1270 ped_task_record_replace_object
/// Replaces the object pointer stored in a selected task record's field at
/// byte offset 0x1c. An unsigned index outside 0 through 31 is ignored. When
/// the old and new pointers differ, it releases a nonnull old object, writes
/// the new pointer, and retains a nonnull replacement. The two helper calls
/// receive the address of the global slot as their stack argument.
lf_checker_rt::export!(cdecl, rw_009c1270(record_index: u32, replacement: u32) -> () {
    const TABLE_FIRST_SLOT_VA: u32 = 0x0129_246C;
    const RECORD_STRIDE: u32 = 0xD0;
    const RECORD_COUNT: u32 = 32;

    if record_index >= RECORD_COUNT {
        return;
    }

    let slot_va = TABLE_FIRST_SLOT_VA.wrapping_add(record_index.wrapping_mul(RECORD_STRIDE));
    let slot = lf_checker_rt::global::<u32>(slot_va);
    let current = unsafe { slot.read() };
    if current == replacement {
        return;
    }

    let slot_pointer = lf_checker_rt::relocated(slot_va);
    if current != 0 {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, current, slot_pointer);
    }
    unsafe { slot.write(replacement) };
    if replacement != 0 {
        let _ = lf_checker_rt::callee_thiscall!(2, u32, replacement, slot_pointer);
    }
});
