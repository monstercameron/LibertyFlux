// original: 0x00cc5e60 state_block_reset
/// Reset a 0x380-byte animation state block to its quiescent pattern.
///
/// Thirty-five 0x18-byte slots starting at offset 8 are stamped with a -1 tag
/// followed by five zero words (the counter runs 33 down through -1); the
/// high flag bits at 0x378 are cleared, the two link words at 0x370/0x374 and
/// the eight words at 0x350..0x36c are zeroed, and the head words are set to
/// -1. Returns the block pointer.
export!(thiscall, rw_00cc5e60(this: *mut u8) -> u32 {
    unsafe {
        for i in 0..35usize {
            let slot = this.add(8 + i * 0x18) as *mut u32;
            *slot = 0xFFFF_FFFF;
            *slot.add(1) = 0;
            *slot.add(2) = 0;
            *slot.add(3) = 0;
            *slot.add(4) = 0;
            *slot.add(5) = 0;
        }
        let flags = this.add(0x378) as *mut u32;
        *flags &= 0xFFFF_C000;
        *(this.add(0x370) as *mut u32) = 0;
        *(this.add(0x374) as *mut u32) = 0;
        *(this as *mut u32) = 0xFFFF_FFFF;
        *(this.add(4) as *mut u32) = 0xFFFF_FFFF;
        for w in 0..8usize {
            *(this.add(0x350 + w * 4) as *mut u32) = 0;
        }
        this as u32
    }
});
