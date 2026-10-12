// original: 0x009aa780 speech_pain_voice_lookup
/// Resolves a speech-pain voice entry from the receiver's index tables.
///
/// A byte at receiver offset `0x540 + row` supplies a sub-index. The method
/// adds twice `row`, triples that sum, and returns the 32-bit value stored at
/// offset `0x4FC` plus four times the resulting index.
const ROW_MAP: u32 = 0x540;
const VOICE_TABLE: u32 = 0x4FC;
const TWO_ROWS: u32 = 2;
const THREE_TIMES: u32 = 3;
const WORD_BYTES: u32 = 4;

export!(thiscall, rw_009aa780(this_: u32, row: u32) -> u32 {
    let sub_index = unsafe {
        (this_.wrapping_add(ROW_MAP).wrapping_add(row) as *const u8).read()
    } as u32;
    let voice_index = sub_index
        .wrapping_add(row.wrapping_mul(TWO_ROWS))
        .wrapping_mul(THREE_TIMES);
    let entry_offset = VOICE_TABLE.wrapping_add(voice_index.wrapping_mul(WORD_BYTES));
    unsafe { (this_.wrapping_add(entry_offset) as *const u32).read() }
});
