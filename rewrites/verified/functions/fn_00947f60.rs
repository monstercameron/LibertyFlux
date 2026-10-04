// original: 0x00947f60 slot_table_store_first_free
/// Store a value in the first free (0xFFFFFFFF) slot of a 16-entry table.
/// Returns the stored value, or 16 when the table is full.
export!(thiscall, rw_00947f60(table: *mut u32, value: u32) -> u32 {
    unsafe {
        for i in 0..16usize {
            if *table.add(i) == 0xFFFF_FFFF {
                *table.add(i) = value;
                return value;
            }
        }
        16
    }
});
