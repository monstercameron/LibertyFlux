// original: 0x0061DC10 net_fill_invalid

/// Mark a record invalid: zero the header, fill the table with -1.
///
/// Zeroes the three header words, then fills 64 dwords at `this+0x10`
/// with -1: two seeded directly plus 62 moved. (The original does the
/// move with an overlapping forward block copy whose source words are
/// all overwritten before being read, so the same -1 fill results.)
/// Returns `this`.
/// Original: 0x0061DC10 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_0061DC10(this: u32) -> u32 {
    unsafe {
        const TABLE_WORDS: u32 = 64;
        ((this) as *mut u32).write_unaligned(0);
        ((this + 4) as *mut u32).write_unaligned(0);
        ((this + 8) as *mut u32).write_unaligned(0);
        for i in 0..TABLE_WORDS {
            ((this + 0x10 + i * 4) as *mut u32).write_unaligned(0xFFFF_FFFF);
        }
        this
    }
});
