// original: 0x00e610d0 clear_bytes_32x100_19fe6b0
/// Clear the first byte of each of 32 records of 0x100 bytes at 0x019FE6B0.
///
/// Returns the address one past the last record.
export!(cdecl, rw_00e610d0() -> u32 {
    unsafe {
        const BASE: u32 = 0x019FE6B0;
        const COUNT: usize = 32;
        const STRIDE: u32 = 0x100;
        let mut ptr = relocated(BASE);
        for _ in 0..COUNT {
            *(ptr as *mut u8) = 0;
            ptr = ptr.wrapping_add(STRIDE);
        }
        ptr
    }
});
