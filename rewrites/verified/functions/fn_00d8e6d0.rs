// original: 0x00d8e6d0 audio_block_zero_wide
/// Zeroes a wide audio block: dwords at +0/4/8 and +0x20..0x3C.
export!(thiscall, rw_00d8e6d0(this: *mut u8) -> u32 {
    unsafe {
        let w = this as *mut u32;
        *w = 0;
        *w.add(1) = 0;
        *w.add(2) = 0;
        *w.add(8) = 0;
        *w.add(9) = 0;
        *w.add(10) = 0;
        *w.add(11) = 0;
        *w.add(15) = 0;
        *w.add(14) = 0;
        *w.add(13) = 0;
        *w.add(12) = 0;
        this as u32
    }
});
