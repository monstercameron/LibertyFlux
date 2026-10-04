// original: 0x00b32e40 task_time_pair_update (proposed)

/// Keep the longer of two durations in a task's time pair.
///
/// `this` points to a task record holding two durations at `+0x30` and
/// `+0x34`. `first` and `second` are a candidate pair. When the candidate
/// total exceeds the stored total (both sums wrapping), the stored pair is
/// replaced by (`second`, `first`); otherwise it is left alone. Returns the
/// candidate total.
///
/// Original: 0x00b32e40 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00b32e40(this: u32, first: u32, second: u32) -> u32 {
    unsafe {
        const STORED_A: u32 = 0x30;
        const STORED_B: u32 = 0x34;
        let stored = (this as *const u32).byte_add(STORED_A as usize).read_unaligned()
            .wrapping_add((this as *const u32).byte_add(STORED_B as usize).read_unaligned());
        let candidate = first.wrapping_add(second);
        if candidate > stored {
            (this as *mut u32).byte_add(STORED_A as usize).write_unaligned(second);
            (this as *mut u32).byte_add(STORED_B as usize).write_unaligned(first);
        }
        candidate
    }
});
