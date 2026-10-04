// original: 0x00e63d40 zero_tagged_table_a
/// Zero the 0x818-entry table at 0x1208970 with 0xFFFF tags.
///
/// Each 8-byte entry gets one zeroed dword and one 0xFFFF word; the last two
/// bytes per entry are untouched. Returns the end cursor.
export!(cdecl, rw_00e63d40() -> u32 {
    unsafe {
        const TABLE: u32 = 0x1208970;
        const COUNT: usize = 0x818;
        const STRIDE: usize = 8;
        let mut p = relocated(TABLE) as *mut u8;
        for _ in 0..COUNT {
            *(p as *mut u32) = 0;
            *(p.add(4) as *mut u16) = 0xFFFF;
            p = p.add(STRIDE);
        }
        p as u32
    }
});
