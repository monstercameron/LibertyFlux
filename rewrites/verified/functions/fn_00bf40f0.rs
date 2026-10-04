// original: 0x00bf40f0 tag_init
/// Store the tag byte, set the flag bit at +3, and reset the range words.
export!(thiscall, rw_bf40f0(this: *mut u8, kind: u32) -> u32 {
    unsafe {
        let b = kind as u8;
        *this.add(3) |= 1;
        *this = b;
        *((this.add(0x10)) as *mut u32) = 0;
        *((this.add(0x14)) as *mut u32) = 0xFF;
        b as u32
    }
});
