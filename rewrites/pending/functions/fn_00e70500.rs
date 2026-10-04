// original: 0x00e70500 clear_slots_512x2c
/// Clear the 512-entry slot array ending at 0x01A0B090.
///
/// Walks 512 records of 0x2C bytes from the top down, zeroing each entry's
/// counter, state words and payload bytes and setting its two generation
/// words to all-ones. Returns the final record pointer, matching the value
/// the original leaves in EAX.
export!(cdecl, rw_00e70500() -> u32 {
    unsafe {
        const TOP: u32 = 0x01A0B090;
        const COUNT: u32 = 512;
        const STRIDE: u32 = 0x2C;
        let mut ptr = relocated(TOP);
        let mut remaining = COUNT;
        loop {
            ptr = ptr.wrapping_sub(STRIDE);
            core::ptr::write_unaligned((ptr.wrapping_sub(8)) as *mut u32, 0);
            core::ptr::write_unaligned((ptr.wrapping_sub(4)) as *mut u32, 0xFFFF_FFFF);
            core::ptr::write_unaligned(ptr as *mut u16, 0);
            core::ptr::write_unaligned((ptr + 4) as *mut u32, 0xFFFF_FFFF);
            core::ptr::write_unaligned((ptr + 8) as *mut u16, 0);
            core::ptr::write_bytes((ptr + 0xC) as *mut u8, 0, 16);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
        ptr
    }
});
