// original: 0x00962250 fill_ff_12008b0
/// Fill 0x8000 bytes at 0x12008B0 with 0xFF.
///
/// Writes two 0xFFFFFFFF dwords per step for 0x1000 steps, then returns the
/// end pointer.
export!(cdecl, rw_00962250() -> u32 {
    unsafe {
        const BASE: u32 = 0x12008B0;
        const COUNT: u32 = 0x1000;
        let mut ptr = relocated(BASE);
        let mut remaining = COUNT;
        loop {
            *(ptr as *mut u32) = 0xFFFFFFFF;
            *(ptr.wrapping_add(4) as *mut u32) = 0xFFFFFFFF;
            ptr = ptr.wrapping_add(8);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
        ptr
    }
});
