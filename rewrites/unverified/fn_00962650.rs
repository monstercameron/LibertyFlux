// original: 0x00962650 table_init_120f2c0
/// Initialise the 1024-entry record table starting at 0x120F2C0.
///
/// Writes 0x400 records of 0x14 bytes: a leading dword and word before the
/// base (the first record's header sits 8 bytes below the base address),
/// then per record two zeroed dwords and a zeroed word. The header word
/// before the base is stamped 0xFFFF, not zeroed.
export!(cdecl, rw_00962650() -> u32 {
    unsafe {
        const BASE: u32 = 0x120F2C0;
        const COUNT: u32 = 0x400;
        const STRIDE: u32 = 0x14;
        let mut ptr = relocated(BASE);
        let mut remaining = COUNT;
        loop {
            *(ptr.wrapping_sub(8) as *mut u32) = 0;
            *(ptr.wrapping_sub(4) as *mut u16) = 0xFFFF;
            *(ptr as *mut u32) = 0;
            *(ptr.wrapping_add(4) as *mut u32) = 0;
            *(ptr.wrapping_add(8) as *mut u16) = 0;
            ptr = ptr.wrapping_add(STRIDE);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
        ptr
    }
});
