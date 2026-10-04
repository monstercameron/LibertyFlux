// original: 0x0089d580 audio_table_query_flag
/// Audio table query: resolve the entry selected by the object's row/column
/// bytes; optionally report one flag bit of it through the out pointer, and
/// return the entry's first word. cdecl/2.
export!(cdecl, rw_0089d580(obj: u32, out: u32) -> u32 {
    unsafe {
        const ROW_STRIDE: u32 = 0x6F40;
        const TABLE_OFF: u32 = 0x6F10;
        const ROW_IDX: u32 = 0x40;
        const COL_IDX: u32 = 0x48;
        const VALUE_OFF: u32 = 0xE0;
        const FLAG_OFF: u32 = 0xEE;
        const STRIDE: u32 = 0x115D964;
        const TABLE: u32 = 0x115D988;
        const ABSENT: u32 = 0xFF;
        let col = *((obj + COL_IDX) as *const u8) as u32;
        let entry_ptr = if col == ABSENT {
            0
        } else {
            let row = *((obj + ROW_IDX) as *const u8) as u32;
            let stride = *global::<u32>(STRIDE);
            let table = *global::<u32>(TABLE);
            let slot = table
                .wrapping_add(row.wrapping_mul(ROW_STRIDE))
                .wrapping_add(TABLE_OFF);
            let entry = (slot as *const u32).read_unaligned();
            col.wrapping_mul(stride).wrapping_add(entry)
        };
        if out != 0 {
            let flags = *((entry_ptr + FLAG_OFF) as *const u8);
            *(out as *mut u8) = (flags >> 2) & 1;
        }
        ((entry_ptr + VALUE_OFF) as *const u32).read_unaligned()
    }
});
