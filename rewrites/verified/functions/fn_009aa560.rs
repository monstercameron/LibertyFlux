// original: 0x009aa560 speech_pain_table_fetch
/// Reads one byte from a speech-pain table row.
///
/// The receiver owns a byte table beginning at `0x554`. The first argument
/// selects a four-byte row and the second adds a byte position within that
/// row; the selected byte is returned zero-extended. The checked contract
/// supplies row values through 257 and byte positions through 31 inside the
/// receiver's modeled table.
const TABLE_BASE: u32 = 0x554;
const ROW_BYTES: u32 = 4;

export!(thiscall, rw_009aa560(this_: u32, row: u32, byte_in_row: u32) -> u32 {
    let offset = TABLE_BASE
        .wrapping_add(row.wrapping_mul(ROW_BYTES))
        .wrapping_add(byte_in_row);
    let value = unsafe { (this_.wrapping_add(offset) as *const u8).read() };
    u32::from(value)
});
