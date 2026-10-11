// original: 0x00C4EBB0 lookup_task_resource_status

/// Initialize two output words, then resolve a task resource when present.
///
/// The stack arguments are output pointers, a nullable task-record pointer,
/// and a state code. Both outputs start at `u32::MAX`. A non-null task record
/// supplies a resource id at `+0x18`; the lookup helper may return a record
/// whose value at `+0x28` replaces the first output. On success, state 2 maps
/// to `0xde`, state 3 to `0xdf`, and every other state to `0xe0` in the second
/// output. The return value follows the original: the resolved first word for
/// state 2, otherwise the selected status word, or zero when lookup fails.
lf_checker_rt::export!(cdecl, rw_00c4ebb0(value_out: u32, state_out: u32, task_record: u32, state: u32) -> u32 {
    const RESOURCE_LOOKUP: u32 = 0;
    unsafe {
        (value_out as *mut u32).write_unaligned(u32::MAX);
        (state_out as *mut u32).write_unaligned(u32::MAX);
    }
    if task_record == 0 { return 0; }
    let resource_id = unsafe { (task_record.wrapping_add(0x18) as *const u32).read_unaligned() };
    let resolved = unsafe { lf_checker_rt::callee_cdecl!(RESOURCE_LOOKUP, u32, resource_id) };
    if resolved == 0 { return 0; }
    let value = unsafe { (resolved.wrapping_add(0x28) as *const u32).read_unaligned() };
    unsafe { (value_out as *mut u32).write_unaligned(value); }
    if state == 2 {
        unsafe { (state_out as *mut u32).write_unaligned(0xde); }
        value
    } else {
        let mapped = if state == 3 { 0xdf } else { 0xe0 };
        unsafe { (state_out as *mut u32).write_unaligned(mapped); }
        mapped
    }
});
