// original: 0x00e610a0 clear_pairs_32x140_19f92e0
/// Clear two bytes in each of 32 records of 0x140 bytes at 0x019F92E0.
///
/// Zeroes the first byte and the byte at offset 0x40 of every record.
/// Returns the address one past the last record.
export!(cdecl, rw_00e610a0() -> u32 {
    unsafe {
        const BASE: u32 = 0x019F92E0;
        const COUNT: usize = 32;
        const STRIDE: u32 = 0x140;
        let mut ptr = relocated(BASE);
        for _ in 0..COUNT {
            *(ptr as *mut u8) = 0;
            *((ptr.wrapping_add(0x40)) as *mut u8) = 0;
            ptr = ptr.wrapping_add(STRIDE);
        }
        ptr
    }
});
