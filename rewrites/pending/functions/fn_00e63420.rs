// original: 0x00e63420 init_records_680x24
/// Initializes 680 records of 0x24 bytes starting at 0x1195F88.
///
/// Each record gets the same nine dwords: two bound constants and a -1 tag
/// spread over offsets -0x18 through +8 relative to the record cursor.
/// Returns the address one past the last record, matching EAX.
export!(cdecl, rw_00e63420() -> u32 {
    unsafe {
        const BASE: u32 = 0x1195F88;
        const COUNT: u32 = 680;
        const STRIDE: u32 = 0x24;
        const HI: u32 = 0x49742400;
        const LO: u32 = 0xC9742400;
        let mut ptr = relocated(BASE);
        let mut remaining = COUNT;
        loop {
            let p = ptr as *mut u32;
            core::ptr::write(p.offset(-6), HI);
            core::ptr::write(p.offset(-5), LO);
            core::ptr::write(p.offset(-4), LO);
            core::ptr::write(p.offset(-3), HI);
            core::ptr::write(p.offset(-2), HI);
            core::ptr::write(p.offset(-1), LO);
            core::ptr::write(p, LO);
            core::ptr::write(p.offset(1), HI);
            core::ptr::write(p.offset(2), 0xFFFF_FFFF);
            ptr = ptr.wrapping_add(STRIDE);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
        ptr
    }
});
