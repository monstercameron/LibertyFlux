// original: 0x00c08c40 stream_state_set_pair (proposed)

/// Store two caller words into the streaming state object.
///
/// `this` points to the object; `first` goes to `FIRST` and `second` to
/// `SECOND`. Returns `second` (what the original leaves in `eax`).
///
/// Original: 0x00c08c40 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00c08c40(this: u32, first: u32, second: u32) -> u32 {
    unsafe {
        const FIRST: u32 = 0x110;
        const SECOND: u32 = 0x114;
        (this.wrapping_add(FIRST) as *mut u32).write_unaligned(first);
        (this.wrapping_add(SECOND) as *mut u32).write_unaligned(second);
        second
    }
});
