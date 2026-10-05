// original: 0x00870180 rage::crmtComposerOptimizedData::vf2

/// Reset the composer's bulk tables: run the shared teardown (callee 1) on the
/// header at `this` + 4, clear the 64-entry value array starting at `this` +
/// 0xE48 (two words per entry, stride 0x34), rebuild the 63-entry pointer
/// table at `this` + 0x20 (each entry points 0x2C past itself, stride 0x30)
/// and clear the count word at `this` + 0xBF0.
///
/// Both loop counters count down and are compared as signed/unsigned-exact
/// (64 and 63 iterations respectively, verified by the heap diff). The
/// returned value is the last pointer-table entry written (`this` + 0x790),
/// left in the return register by the original's final address computation.
///
/// Original: 0x00870180 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00870180(this: u32) -> u32 {
    const HEADER_OFF: u32 = 4;
    const VALUES_BASE: u32 = 0xe4c;
    const VALUES_STRIDE: u32 = 0x34;
    const VALUE_COUNT: u32 = 64;
    const TABLE_BASE: u32 = 0x20;
    const TABLE_STRIDE: u32 = 0x30;
    const TABLE_COUNT: u32 = 63;
    const SELF_OFF: u32 = 0x2c;
    const COUNT_OFF: u32 = 0xbf0;
    unsafe {
        lf_checker_rt::callee_thiscall!(1, u32, this + HEADER_OFF);
        let mut slot = this + VALUES_BASE;
        let mut remaining = VALUE_COUNT;
        while remaining != 0 {
            ((slot - 4) as *mut u32).write_unaligned(0);
            (slot as *mut u32).write_unaligned(0);
            slot += VALUES_STRIDE;
            remaining -= 1;
        }
        let mut entry = this + TABLE_BASE;
        let mut last: u32 = 0;
        let mut left = TABLE_COUNT;
        while left != 0 {
            let target = entry + SELF_OFF;
            (entry as *mut u32).write_unaligned(target);
            entry += TABLE_STRIDE;
            last = target;
            left -= 1;
        }
        ((this + COUNT_OFF) as *mut u32).write_unaligned(0);
        last
    }
});
