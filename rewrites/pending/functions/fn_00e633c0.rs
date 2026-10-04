// original: 0x00e633c0 stamp_neg1_strided_11912b0
/// Stamps 0xFFFFFFFF on the first word of 149 records of 0x48 bytes.
///
/// Writes one dword per record starting at 0x11912B0. Returns the address
/// one past the last record, matching the value the original leaves in EAX.
export!(cdecl, rw_00e633c0() -> u32 {
    unsafe {
        const BASE: u32 = 0x11912B0;
        const COUNT: u32 = 149;
        const STRIDE: u32 = 0x48;
        let mut ptr = relocated(BASE);
        let mut remaining = COUNT;
        loop {
            core::ptr::write(ptr as *mut u32, 0xFFFF_FFFF);
            ptr = ptr.wrapping_add(STRIDE);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
        ptr
    }
});
