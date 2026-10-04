// original: 0x00e63d70 zero_tagged_table_b
/// Zero the 0x400-entry table at 0x120F2B8 with 0xFFFF tags.
///
/// Each 0x14-byte entry gets one zeroed dword and one 0xFFFF word; the rest
/// of each entry is untouched. Returns the end cursor.
export!(cdecl, rw_00e63d70() -> u32 {
    unsafe {
        const TABLE: u32 = 0x120F2B8;
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
