// original: 0x00be4ab0 task_state_zero_span (proposed)

/// Zero the 124-byte state span at the start of a task object.
///
/// Writes zero to the 31 consecutive 32-bit words at `this + 0x00` through
/// `this + 0x78`, in ascending order, then returns zero. No branches, no
/// calls; the only observable effects are the 31 stores and the zero return.
///
/// Original: 0x00be4ab0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00be4ab0(this: u32) -> u32 {
    unsafe {
        const WORDS: u32 = 31;
        let mut i = 0u32;
        while i < WORDS {
            (this.wrapping_add(i.wrapping_mul(4)) as *mut u32).write_unaligned(0);
            i += 1;
        }
        0
    }
});
