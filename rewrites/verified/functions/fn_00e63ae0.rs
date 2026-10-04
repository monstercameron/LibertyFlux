// original: 0x00e63ae0 fill_markers
/// Fill the 256-entry table at 0x11EE5C4 with 0xFFFFFFFF markers.
///
/// Each 0x7C-byte slot gets three marker dwords (at offsets -0x1C, 0 and +4
/// from the slot cursor); all other bytes are untouched. Returns the cursor
/// one past the last slot, matching the original's EAX.
export!(cdecl, rw_00e63ae0() -> u32 {
    unsafe {
        const CURSOR0: u32 = 0x11EE5E0;
        const COUNT: usize = 0x100;
        const STRIDE: isize = 0x7C;
        let mut p = relocated(CURSOR0) as *mut u32;
        for _ in 0..COUNT {
            *p.offset(-7) = 0xFFFFFFFF;
            *p = 0xFFFFFFFF;
            *p.add(1) = 0xFFFFFFFF;
            p = (p as *mut u8).add(STRIDE as usize) as *mut u32;
        }
        p as u32
    }
});
