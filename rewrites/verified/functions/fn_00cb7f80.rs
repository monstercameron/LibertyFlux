// original: 0x00cb7f80 exclusive_flag_pair_check
/// Whether the task holds the first of two exclusive flags (leaf).
///
/// Reads the flag word at `+0xB4` of the task object (`this`, thiscall).
/// Returns the word with its low byte forced to 1 when bit `0x10` is set
/// and bit `0x20` is clear (the original's `(an instruction of the original)` keeps the upper
/// bytes); otherwise returns the word with its low byte cleared (the
/// original's `(an instruction of the original)` clears only the low byte). No calls, no writes.
lf_checker_rt::export!(thiscall, rw_00cb7f80(this: u32) -> u32 {
    unsafe {
        /// Flag word holding the two exclusive bits.
        const FLAGS_OFF: u32 = 0xB4;
        /// First bit: must be set. Second bit: must be clear.
        const NEED_SET: u32 = 0x10;
        const NEED_CLEAR: u32 = 0x20;
        let flags = ((this + FLAGS_OFF) as *const u32).read_unaligned();
        if flags & NEED_SET != 0 && flags & NEED_CLEAR == 0 {
            (flags & 0xFFFFFF00) | 1
        } else {
            flags & 0xFFFFFF00
        }
    }
});
