// original: 0x00962720 table_zero_11ff078
/// Zero the 0x200-entry table starting at 0x11FF078.
///
/// Writes 0x200 records of 0xC bytes: two leading dwords below the base
/// address, then one zero byte per record step.
export!(cdecl, rw_00962720() -> u32 {
    unsafe {
        const BASE: u32 = 0x11FF078;
        const COUNT: u32 = 0x200;
        const STRIDE: u32 = 0xC;
        let mut ptr = relocated(BASE);
        let mut remaining = COUNT;
        loop {
            *(ptr.wrapping_sub(4) as *mut u32) = 0;
            *(ptr.wrapping_sub(8) as *mut u32) = 0;
            *(ptr as *mut u8) = 0;
            ptr = ptr.wrapping_add(STRIDE);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
        ptr
    }
});
