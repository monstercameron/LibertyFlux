// original: 0x008b7650 control_entry_lookup
/// Two-level control entry lookup.
///
/// Selects a control group (the live group from shared state when `which` is
/// -1, otherwise `which` itself), maps the `code` argument through the
/// group's 0x16-byte entries via the tag byte at offset 0x15, then indexes the
/// tag's pointer table by `slot` and returns the word at offset 0x10 of the
/// selected record. Returns 0 whenever any index is out of range.
export!(cdecl, rw_008b7650(code: u32, slot: u32, which: u32) -> u32 {
    unsafe {
        const LIVE_GROUP: u32 = 0x01160C40;
        const COUNT_TABLE: u32 = 0x019D33A4;
        const BASE_TABLE: u32 = 0x019D33A0;
        const TAG_COUNT: u32 = 0x019D30C8;
        const TAG_BASE: u32 = 0x019D30C4;
        const ENTRY_LEN: u32 = 0x16;
        const TAG_OFF: u32 = 0x15;
        const TAGS: u32 = 0x3C;
        const VALUE_OFF: u32 = 0x10;
        if (code as i32) < 0 {
            return 0;
        }
        let group = if which == 0xFFFF_FFFF {
            (relocated(LIVE_GROUP) as *const u32).read()
        } else {
            which
        };
        let row = group.wrapping_mul(3);
        let count_addr = relocated(COUNT_TABLE).wrapping_add(row.wrapping_mul(8));
        let count = (count_addr as *const u16).read() as u32;
        if count == 0 || (count as i32) <= (code as i32) {
            return 0;
        }
        let base_addr = relocated(BASE_TABLE).wrapping_add(row.wrapping_mul(8));
        let base = (base_addr as *const u32).read();
        let tag = (base
            .wrapping_add(code.wrapping_mul(ENTRY_LEN))
            .wrapping_add(TAG_OFF) as *const u8)
            .read() as u32;
        if tag >= TAGS {
            return 0;
        }
        let tag_row = tag.wrapping_mul(3);
        let tag_count_addr = relocated(TAG_COUNT).wrapping_add(tag_row.wrapping_mul(4));
        let tag_count = (tag_count_addr as *const u16).read() as u32;
        if (slot as i32) >= (tag_count as i32) {
            return 0;
        }
        let tag_base_addr = relocated(TAG_BASE).wrapping_add(tag_row.wrapping_mul(4));
        let tag_base = (tag_base_addr as *const u32).read();
        let slot_row = slot.wrapping_mul(3);
        let value_addr = tag_base
            .wrapping_add(slot_row.wrapping_mul(8))
            .wrapping_add(VALUE_OFF);
        (value_addr as *const u32).read()
    }
});
