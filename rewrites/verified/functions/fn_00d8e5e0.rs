// original: 0x00d8e5e0 audio_matrix_identity_reset
/// Resets an audio matrix block: all zero except three 1.0 diagonal words
/// at +0x10, +0x24 and +0x38.
export!(thiscall, rw_00d8e5e0(this: *mut u8) -> u32 {
    unsafe {
        let w = this as *mut u32;
        *w.add(2) = 0;
        *w.add(1) = 0;
        *w = 0;
        *w.add(4) = 0x3F80_0000;
        *w.add(5) = 0;
        *w.add(6) = 0;
        *w.add(8) = 0;
        *w.add(9) = 0x3F80_0000;
        *w.add(10) = 0;
        *w.add(12) = 0;
        *w.add(13) = 0;
        *w.add(14) = 0x3F80_0000;
        *w.add(18) = 0;
        *w.add(17) = 0;
        *w.add(16) = 0;
        this as u32
    }
});
