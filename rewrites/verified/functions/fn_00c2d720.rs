// original: 0x00c2d720 reset_record
/// Resets a record to its blank state and returns it.
///
/// Stamps -1 / 0xFFFF headers, zeroes the payload words (leaving bytes
/// +6..+7 untouched) and clears the trailing marker byte.
export!(thiscall, rw_00c2d720(this: *mut u8) -> u32 {
    unsafe {
        *(this as *mut u32) = 0xFFFF_FFFF;
        *(this.add(4) as *mut u16) = 0xFFFF;
        for off in [8usize, 0xC, 0x10, 0x14, 0x18, 0x1C, 0x20, 0x24, 0x2C, 0x28] {
            *(this.add(off) as *mut u32) = 0;
        }
        *this.add(0x30) = 0;
        this as u32
    }
});
