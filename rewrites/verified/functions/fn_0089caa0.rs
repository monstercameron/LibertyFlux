// original: 0x0089caa0 audio_table_deref_or_null
/// Audio table reader: resolve the entry selected by the object's row/column
/// bytes and return the word it points at, or null when the entry is empty.
/// thiscall/0.
export!(thiscall, rw_0089caa0(this: u32) -> u32 {
    unsafe {
        const ROW_STRIDE: u32 = 0x6F40;
        const TABLE_OFF: u32 = 0x6F14;
        const ROW_IDX: u32 = 0x40;
        const COL_IDX: u32 = 0xB4;
        const STRIDE: u32 = 0x115D968;
        const TABLE: u32 = 0x115D988;
        let row = *((this + ROW_IDX) as *const u8) as u32;
        let col = *((this + COL_IDX) as *const u8) as u32;
        let stride = *global::<u32>(STRIDE);
        let table = *global::<u32>(TABLE);
        let slot = table
            .wrapping_add(row.wrapping_mul(ROW_STRIDE))
            .wrapping_add(TABLE_OFF);
        let entry = (slot as *const u32).read_unaligned();
        let ptr = col.wrapping_mul(stride).wrapping_add(entry);
        if ptr == 0 {
            0
        } else {
            (ptr as *const u32).read_unaligned()
        }
    }
});
