// original: 0x00e63da0 zero_tagged_table_c
/// Zero the 0x400-entry table at 0x11FA028 with 0xFFFF tags.
///
/// Same shape as [`rw_00e63d70`] over a different table. Returns the end
/// cursor.
export!(cdecl, rw_00e63da0() -> u32 {
    unsafe {
        const TABLE: u32 = 0x11FA028;
        const COUNT: usize = 0x400;
        const STRIDE: usize = 0x14;
        let mut p = relocated(TABLE) as *mut u8;
        for _ in 0..COUNT {
            *(p as *mut u32) = 0;
            *(p.add(4) as *mut u16) = 0xFFFF;
            p = p.add(STRIDE);
        }
        p as u32
    }
});
