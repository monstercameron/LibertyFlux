// original: 0x00884550 stream_bounded_store (proposed)
/// Store two bounded values into a manager's tables at a looked-up row.
///
/// The row selector comes from the manager (`mgr+0x20`) through the row
/// lookup (intercepted callee 1, thiscall, one argument: the object
/// pointer); the row address is `4 * selector`. Each argument is clamped
/// above to `LIMIT` (`0x5208`): the clamped second argument goes to the
/// table at `mgr+0x64`, the clamped first argument to the table at
/// `mgr+0x60`.
///
/// Original: thiscall, two stack arguments, callee cleans 8, no return value.
lf_checker_rt::export!(thiscall, rw_00884550(this: u32, first: u32, second: u32) -> u32 {
    unsafe {
        const MANAGER: u32 = 0x20;
        const TABLE_A: u32 = 0x60;
        const TABLE_B: u32 = 0x64;
        const LIMIT: u32 = 0x5208;
        const LOOKUP_CALLEE: u32 = 1;
        let manager = ((this + MANAGER) as *const u32).read_unaligned();
        let selector: u32 = lf_checker_rt::callee_thiscall!(LOOKUP_CALLEE, u32, manager, this);
        let row = selector.wrapping_mul(4);
        let clamped_second = if second < LIMIT { second } else { LIMIT };
        let table_b = ((manager + TABLE_B) as *const u32).read_unaligned();
        (table_b.wrapping_add(row) as *mut u32).write_unaligned(clamped_second);
        let clamped_first = if first < LIMIT { first } else { LIMIT };
        let table_a = ((manager + TABLE_A) as *const u32).read_unaligned();
        (table_a.wrapping_add(row) as *mut u32).write_unaligned(clamped_first);
        0
    }
});
