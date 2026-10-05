// original: 0x00b1acd0 row_word_in_set (proposed)

/// Returns 1 when the row's kind word is one of eight recognised values.
///
/// Cdecl of one stack word: an index into a row table with stride 0xE2.
/// Reads the 16-bit word two bytes into the row and returns 1 when it is
/// 0x34, 0x35, 0x18, 0x19, 8, 9, 0x70 or 0x71, else 0.
lf_checker_rt::export!(cdecl, rw_00b1acd0(index: u32) -> u32 {
    unsafe {
        const ROW_TABLE: u32 = 0x010401f4;
        const ROW_STRIDE: u32 = 0xe2;
        let addr = ROW_TABLE.wrapping_add(index.wrapping_mul(ROW_STRIDE));
        let word = (lf_checker_rt::relocated(addr) as *const u16).read_unaligned();
        u32::from(matches!(word, 0x34 | 0x35 | 0x18 | 0x19 | 8 | 9 | 0x70 | 0x71))
    }
});
