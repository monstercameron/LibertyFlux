// original: 0x00b4baa0 event_type_pair_in_table

/// Search the four 32-bit values in one 16-byte precedence-table row.
/// The first stack argument selects a row; the second supplies the value.
/// Return one on any equality and zero when no field matches.
lf_checker_rt::export!(cdecl, rw_00B4BAA0(row_index: u32, needle: u32) -> u32 {
    const TABLE_VA: u32 = 0x0166_60F8;
    const ROW_STRIDE: u32 = 0x10;
    const WORD_COUNT: u32 = 4;
    let table = lf_checker_rt::global::<u32>(TABLE_VA);
    let row_offset_words = row_index.wrapping_mul(ROW_STRIDE / 4) as usize;
    let row = table.wrapping_add(row_offset_words);
    for slot in 0..WORD_COUNT {
        // SAFETY: the contract fills the selected table row for each trial.
        if unsafe { row.add(slot as usize).read_unaligned() } == needle {
            return 1;
        }
    }
    0
});
