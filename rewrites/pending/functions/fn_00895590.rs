// original: 0x00895590 audio_voice_param_block_copy
//! Copy a 0x6F-byte voice parameter block from `src` into `this`.
//!
//! Layout (all offsets from the block start): four header dwords, six
//! config words, eight float parameters, twelve body dwords, three flag
//! bytes. Returns `this`.

lf_rb69_rt::export!(thiscall, rw_00895590(this: *mut u8, src: *const u8) -> *mut u8 {
    unsafe {
        let d = this;
        let s = src;
        // Four header dwords.
        for i in 0..4u32 {
            let o = (i * 4) as usize;
            *(d.add(o) as *mut u32) = *(s.add(o) as *const u32);
        }
        // Six config words.
        for i in 0..6u32 {
            let o = 0x10 + (i * 2) as usize;
            *(d.add(o) as *mut u16) = *(s.add(o) as *const u16);
        }
        // Eight float parameters (bitwise copies).
        for i in 0..8u32 {
            let o = 0x1c + (i * 4) as usize;
            *(d.add(o) as *mut u32) = *(s.add(o) as *const u32);
        }
        // Twelve body dwords.
        for i in 0..12u32 {
            let o = 0x3c + (i * 4) as usize;
            *(d.add(o) as *mut u32) = *(s.add(o) as *const u32);
        }
        // Three flag bytes.
        for i in 0..3u32 {
            let o = 0x6c + i as usize;
            *d.add(o) = *s.add(o);
        }
        this
    }
});
