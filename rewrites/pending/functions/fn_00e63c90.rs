// original: 0x00e63c90 zero_byte_table
/// Zero the 1500-entry table at 0x11F7110.
///
/// Each 8-byte entry contributes one zeroed dword and one zeroed byte; the
/// remaining three bytes per entry are untouched. Returns the cursor one
/// past the last entry, matching the original's EAX.
export!(cdecl, rw_00e63c90() -> u32 {
    unsafe {
        const TABLE: u32 = 0x11F7110;
        const COUNT: usize = 0x5DC;
        const STRIDE: usize = 8;
        let mut p = relocated(TABLE) as *mut u8;
        for _ in 0..COUNT {
            *(p as *mut u32) = 0;
            *p.add(4) = 0;
            p = p.add(STRIDE);
        }
        p as u32
    }
});
