// original: 0x00b3a660 task_word_scan (proposed)

/// Scan the 16-bit word list of group `index` for `want`. The entry count
/// comes from the group-count table; a non-positive count returns at once.
/// Otherwise the row at the word-table base plus `index` rows is scanned
/// entry by entry and the scan stops at the first 16-bit word equal to
/// `want` (compared full-width, so a `want` above 0xffff never matches).
/// The result keeps the original's exact register contents: the scan address
/// with its low byte cleared, or-ed with 1 when the word was found (the
/// not-found and empty cases leave the low byte zero). Original: 0x00b3a660
/// (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00b3a660(index: u32, want: u32) -> u32 {
    unsafe {
        const COUNT_TABLE: u32 = 0x0169e248;
        const WORD_TABLE: u32 = 0x0169c488;
        const ROW_BYTES: u32 = 80;
        let count = ((lf_checker_rt::relocated(COUNT_TABLE) + index.wrapping_mul(4))
            as *const u32)
            .read() as i32;
        if count <= 0 {
            return index & 0xffff_ff00;
        }
        let row = lf_checker_rt::relocated(WORD_TABLE) + index.wrapping_mul(ROW_BYTES);
        let mut pos = 0u32;
        while pos < count as u32 {
            let word = ((row + pos * 2) as *const u16).read_unaligned() as u32;
            if word == want {
                return ((row + pos * 2) & 0xffff_ff00) | 1;
            }
            pos += 1;
        }
        (row + pos * 2) & 0xffff_ff00
    }
});
