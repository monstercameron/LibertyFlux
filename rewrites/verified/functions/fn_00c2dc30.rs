// original: 0x00c2dc30 clear_flag_group
/// Clears a flag group and its payload unless the busy bit is set.
///
/// Returns the flags word shifted right by 0x14; when its low bit is
/// clear, masks the stored flags and zeroes five payload words.
export!(thiscall, rw_00c2dc30(this: *mut u8) -> u32 {
    unsafe {
        let w = *(this.add(0x10) as *const u32);
        let shr = w >> 0x14;
        if (shr as u8) & 1 == 0 {
            *(this.add(0x10) as *mut u32) = (w & 0xFFF7_01FF) | 0x100;
            *(this.add(0x14) as *mut u32) = 0;
            *(this.add(0x18) as *mut u32) = 0;
            *(this.add(0x1C) as *mut u32) = 0;
            *(this.add(0x20) as *mut u32) = 0;
            *(this.add(0x24) as *mut u32) = 0;
        }
        shr
    }
});
