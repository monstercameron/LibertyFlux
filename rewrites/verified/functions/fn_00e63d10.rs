// original: 0x00e63d10 fill_marker_pairs
/// Fill 0x1000 double-words pairs at 0x12008B0 with 0xFFFFFFFF.
///
/// Writes two marker dwords per 8-byte slot. Returns the cursor one past
/// the last slot, matching the original's EAX.
export!(cdecl, rw_00e63d10() -> u32 {
    unsafe {
        const TABLE: u32 = 0x12008B0;
        const COUNT: usize = 0x1000;
        const STRIDE: usize = 8;
        let mut p = relocated(TABLE) as *mut u32;
        for _ in 0..COUNT {
            *p = 0xFFFFFFFF;
            *p.add(1) = 0xFFFFFFFF;
            p = (p as *mut u8).add(STRIDE) as *mut u32;
        }
        p as u32
    }
});
