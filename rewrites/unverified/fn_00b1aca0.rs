// original: 0x00b1aca0 row_word_not_excluded (proposed)

/// Returns 1 unless the row's kind word is one of three excluded values.
///
/// Cdecl of one stack word: an index into a row table with stride 0xE2.
/// Reads the 16-bit word at the row start and returns 0 when it equals
/// 0x7D, 0x7E or 0x88, else 1. Only AL is set; the upper bits stay zero
/// from the word load.
lf_checker_rt::export!(cdecl, rw_00b1aca0(index: u32) -> u32 {
    unsafe {
        const ROW_TABLE: u32 = 0x010401f2;
        const ROW_STRIDE: u32 = 0xe2;
        let addr = ROW_TABLE.wrapping_add(index.wrapping_mul(ROW_STRIDE));
        let word = (lf_checker_rt::relocated(addr) as *const u16).read_unaligned();
        u32::from(!matches!(word, 0x7d | 0x7e | 0x88))
    }
});
