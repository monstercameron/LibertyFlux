// original: 0x00892480 audsound_reset_playback_fields
/// Copies the word at 0x42 to 0x44 and clears the playback state fields.
///
/// Reads the word at offset 0x42 into the word at 0x44, zeroes the dwords at
/// 0x1c, 0x18, 0x14 and 0x78, the word at 0x46 and the byte at 0x4d, and sets
/// the dword at 0x40 to all-ones. The return register is untouched (it keeps
/// the caller's entry EAX), so no return channel is compared.
/// Original: 0x00892480 (thiscall, no stack arguments).
export!(thiscall, rw_00892480(this: *mut u8) -> () {
    unsafe {
        const COPY_SRC: usize = 0x42;
        const COPY_DST: usize = 0x44;
        const ALL_ONES: usize = 0x40;
        let w = *(this.add(COPY_SRC) as *const u16);
        *(this.add(COPY_DST) as *mut u16) = w;
        *(this.add(0x1c) as *mut u32) = 0;
        *(this.add(ALL_ONES) as *mut u32) = 0xffff_ffff;
        *(this.add(0x18) as *mut u32) = 0;
        *(this.add(0x14) as *mut u32) = 0;
        *(this.add(0x78) as *mut u32) = 0;
        *(this.add(0x46) as *mut u16) = 0;
        *this.add(0x4d) = 0;
    }
});
