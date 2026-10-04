// original: 0x00e63480 stamp_neg1_6
/// Stamps 0xFFFFFFFF on six scattered global dwords.
///
/// The six targets sit 0x14-0x1C apart starting at 0x119BF18. No return
/// value: the original leaves EAX untouched.
export!(cdecl, rw_00e63480() -> () {
    unsafe {
        const BASE: u32 = 0x119BF18;
        const OFFSETS: [u32; 6] = [0x00, 0x1C, 0x30, 0x48, 0x64, 0x78];
        let base = relocated(BASE);
        let mut i = 0;
        while i < OFFSETS.len() {
            core::ptr::write((base + OFFSETS[i]) as *mut u32, 0xFFFF_FFFF);
            i += 1;
        }
    }
});
