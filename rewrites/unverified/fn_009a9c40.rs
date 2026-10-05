// original: 0x009a9c40 indexed_slot_lookup
/// Load the slot pointer selected by a per-row tag byte.
///
/// `row` is a small row number. The tag byte at `this + row + 0x368`
/// is added to three times the row, tripled again and scaled by 32,
/// giving a byte offset into the slot array at `this+0x10`, whose word
/// there is returned. Thiscall, one stack word, dword result.
export!(thiscall, rw_009A9C40(this: u32, row: u32) -> u32 {
    unsafe {
        const TAG_TABLE: u32 = 0x368;
        const SLOT_ARRAY: u32 = 0x10;
        let tag = ((this + row + TAG_TABLE) as *const u8).read() as u32;
        let off = (tag + row * 3) * 3 * 32;
        ((this + off + SLOT_ARRAY) as *const u32).read_unaligned()
    }
});
