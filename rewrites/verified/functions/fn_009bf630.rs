// original: 0x009BF630 ped_task_select_event_entry
/// Looks up the receiver's child object through its virtual query method.
/// If that method returns an object with a nonzero table field, calls the
/// scripted resolver and returns the 16-bit field at offset 0x0e of the
/// selected table entry. The receiver's entry index is an unsigned dword.
/// The method call uses the object's vtable slot and the resolver is a
/// thiscall with the returned object in ECX.
lf_checker_rt::export!(cdecl, rw_009bf630(receiver: u32) -> u16 {
    const CHILD_OFFSET: u32 = 0x88;
    const ENTRY_INDEX_OFFSET: u32 = 0xC0;
    const TABLE_POINTER_OFFSET: u32 = 0xD4;
    const ENTRY_VALUE_OFFSET: u32 = 0x0E;

    let child = unsafe { ((receiver + CHILD_OFFSET) as *const u32).read() };
    if child == 0 {
        return 0;
    }

    let event = lf_checker_rt::callee_thiscall!(1, u32, child);
    if event == 0 {
        return 0;
    }
    let has_table = unsafe { ((event + 0x64) as *const u32).read() };
    if has_table == 0 {
        return 0;
    }

    let resolved = lf_checker_rt::callee_thiscall!(2, u32, event);
    let table = unsafe { ((resolved + TABLE_POINTER_OFFSET) as *const u32).read() };
    let entry_index = unsafe { ((receiver + ENTRY_INDEX_OFFSET) as *const u32).read() };
    let selected_entry = unsafe {
        ((table.wrapping_add(entry_index.wrapping_mul(4))) as *const u32).read()
    };
    unsafe { ((selected_entry + ENTRY_VALUE_OFFSET) as *const u16).read() }
});
