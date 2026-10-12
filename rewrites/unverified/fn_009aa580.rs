// original: 0x009aa580 audSpeechManager::voice_entry_flag_get
/// Returns a voice-entry flag when the requested signed index is valid.
///
/// The receiver contains an optional table pointer at offset `0x9D0`. Its
/// 16-bit entry count is at table offset ten, and entries are `0x19` bytes
/// apart beginning at offset `0x18`. Negative indices, a missing table, and
/// indices at or beyond the count return zero.
const TABLE_LINK: u32 = 0x9D0;
const TABLE_COUNT: u32 = 0x0A;
const ENTRY_BASE: u32 = 0x18;
const ENTRY_STRIDE: u32 = 0x19;

export!(thiscall, rw_009aa580(this_: u32, entry_index: i32) -> u32 {
    let table = unsafe {
        (this_.wrapping_add(TABLE_LINK) as *const u32).read()
    };
    if table == 0 || entry_index < 0 {
        return 0;
    }
    let entry_count = unsafe {
        (table.wrapping_add(TABLE_COUNT) as *const u16).read_unaligned() as u32
    };
    let index = entry_index as u32;
    if index >= entry_count {
        return 0;
    }
    let flag = unsafe {
        (table
            .wrapping_add(ENTRY_BASE)
            .wrapping_add(index.wrapping_mul(ENTRY_STRIDE)) as *const u8)
            .read()
    };
    u32::from(flag)
});
