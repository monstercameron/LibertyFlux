// original: 0x00601CA0 ensure_pool_table (proposed)

/// Make sure the manager's pool table exists, creating it on first use.
///
/// `this` points to the manager. The 16-bit flag at `READY` (+0x272)
/// tells whether the table at `TABLE` (+0x26c) is already set up. When
/// the flag is zero the function sets it, calls the table allocator
/// (callee 1, thiscall with `this` and the constant 1), records the
/// allocator's answer as the table, sets the 16-bit generation at `GEN`
/// (+0x270) to 1, and returns the allocator's answer. When the flag is
/// already set it only refreshes `GEN` to 1 and returns 1. Thiscall:
/// `this` in ECX, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00601CA0(this: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x26c;
        const GEN: u32 = 0x270;
        const READY: u32 = 0x272;
        const TABLE_CALLEE: u32 = 1;

        if ((this + READY) as *const u16).read() == 0 {
            ((this + READY) as *mut u16).write(1);
            let table: u32 = lf_checker_rt::callee_thiscall!(TABLE_CALLEE, u32, this, 1);
            ((this + GEN) as *mut u16).write(1);
            ((this + TABLE) as *mut u32).write(table);
            table
        } else {
            ((this + GEN) as *mut u16).write(1);
            1
        }
    }
});
