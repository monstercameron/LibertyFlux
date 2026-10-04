// original: 0x00e61210 reset_records_16x50_1a00728
/// Reset 16 records of 0x50 bytes starting at 0x01A00728.
///
/// Each record is stamped with the same mixed pattern: two zeroed qwords, a
/// zeroed dword, then alternating all-ones marker dwords and zeroed words
/// and fields (offsets -0x14/-0x0C/-0x04/+0x10/+0x18 all-ones, the rest
/// zero). The bytes at +0x02/+0x03, +0x16/+0x17 and +0x1E/+0x1F of every
/// record are left untouched. The original repeats several of these stores
/// redundantly with identical values; one store per location is kept here.
/// Returns the address one past the last record.
export!(cdecl, rw_00e61210() -> u32 {
    unsafe {
        const BASE: u32 = 0x01A00728;
        const COUNT: usize = 16;
        const STRIDE: u32 = 0x50;
        const FULL: u32 = 0xFFFF_FFFF;
        let mut ptr = relocated(BASE);
        for _ in 0..COUNT {
            *(ptr.wrapping_sub(0x28) as *mut u64) = 0;
            *(ptr.wrapping_sub(0x20) as *mut u64) = 0;
            *(ptr.wrapping_sub(0x18) as *mut u32) = 0;
            *(ptr.wrapping_sub(0x14) as *mut u32) = FULL;
            *(ptr.wrapping_sub(0x10) as *mut u16) = 0;
            *(ptr.wrapping_sub(0x0C) as *mut u32) = FULL;
            *(ptr.wrapping_sub(8) as *mut u16) = 0;
            *(ptr.wrapping_sub(4) as *mut u32) = FULL;
            *(ptr as *mut u16) = 0;
            *(ptr.wrapping_add(4) as *mut u32) = 0;
            *(ptr.wrapping_add(8) as *mut u32) = 0;
            *(ptr.wrapping_add(0x0C) as *mut u32) = 0;
            *(ptr.wrapping_add(0x10) as *mut u32) = FULL;
            *(ptr.wrapping_add(0x14) as *mut u16) = 0;
            *(ptr.wrapping_add(0x18) as *mut u32) = FULL;
            *(ptr.wrapping_add(0x1C) as *mut u16) = 0;
            *(ptr.wrapping_add(0x20) as *mut u32) = 0;
            ptr = ptr.wrapping_add(STRIDE);
        }
        ptr
    }
});
