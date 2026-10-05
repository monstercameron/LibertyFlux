// original: 0x00ab1a10 stream_release_entry (proposed)

/// Release one streaming entry, fast or elaborate path by loader flags.
///
/// When both flag bytes at `+0x151461`/`+0x151462` of the loader are clear,
/// forwards `(a0, a1)` to the bulk-release callee and returns. Otherwise
/// charges the entry's weight at `a1 + 0x3c` against one of the three
/// budget words at `a0 + 0xc0`/`+0xc4`/`+0xc8` (picked by the tag bytes at
/// `a1 + 0x50`/`+0x51`, skipped entirely when the done byte at `a1 + 0x80`
/// is set), tears the entry down through the teardown callee, unlinks it
/// through the unlink callee on `a0 + 0xb4`, and detaches it through the
/// detach callee. No return value. Note the argument roles: the entry is
/// the second word, the budget holder the first.
///
/// Callees: 1 = bulk release (thiscall, two words), 2 = teardown
/// (thiscall, no words), 3 = unlink (thiscall, one word), 4 = detach
/// (thiscall, one word).
///
/// Original: 0x00ab1a10 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00ab1a10(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const BULK: u32 = 1;
        const TEARDOWN: u32 = 2;
        const UNLINK: u32 = 3;
        const DETACH: u32 = 4;
        const FLAG_A: u32 = 0x151461;
        const FLAG_B: u32 = 0x151462;
        const DONE_OFF: u32 = 0x80;
        const TAG_A: u32 = 0x50;
        const TAG_B: u32 = 0x51;
        const WEIGHT_OFF: u32 = 0x3C;
        const BUDGET_C: u32 = 0xC0;
        const BUDGET_B: u32 = 0xC4;
        const BUDGET_A: u32 = 0xC8;
        const UNLINK_OFF: u32 = 0xB4;
        if ((this + FLAG_A) as *const u8).read() == 0
            && ((this + FLAG_B) as *const u8).read() == 0
        {
            lf_checker_rt::callee_thiscall!(BULK, u32, this, a0, a1);
            return 0;
        }
        if ((a1 + DONE_OFF) as *const u8).read() == 0 {
            let w = ((a1 + WEIGHT_OFF) as *const u32).read_unaligned();
            if ((a1 + TAG_A) as *const u8).read() != 0 {
                let c = ((a0 + BUDGET_A) as *const u32).read_unaligned();
                ((a0 + BUDGET_A) as *mut u32).write_unaligned(c.wrapping_sub(w));
            } else if ((a1 + TAG_B) as *const u8).read() != 0 {
                let c = ((a0 + BUDGET_B) as *const u32).read_unaligned();
                ((a0 + BUDGET_B) as *mut u32).write_unaligned(c.wrapping_sub(w));
            } else {
                let c = ((a0 + BUDGET_C) as *const u32).read_unaligned();
                ((a0 + BUDGET_C) as *mut u32).write_unaligned(c.wrapping_sub(w));
            }
        }
        lf_checker_rt::callee_thiscall!(TEARDOWN, u32, a1);
        lf_checker_rt::callee_thiscall!(UNLINK, u32, a0.wrapping_add(UNLINK_OFF), a1);
        lf_checker_rt::callee_thiscall!(DETACH, u32, this, a1);
        0
    }
});
