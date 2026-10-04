// original: 0x00d8e5c0 audio_flags_reset
/// Resets an audio flags block: overlapping zero stores clear bytes 0..5,
/// byte 6 is cleared, byte 7 keeps all but its two low bits, and the dword
/// at +8 is cleared.
export!(thiscall, rw_00d8e5c0(this: *mut u8) -> u32 {
    unsafe {
        (this.add(2) as *mut u32).write_unaligned(0);
        (this as *mut u16).write_unaligned(0);
        *this.add(6) = 0;
        let w = (this.add(6) as *const u16).read_unaligned();
        (this.add(6) as *mut u16).write_unaligned(w & 0xFCFF);
        *(this.add(8) as *mut u32) = 0;
        this as u32
    }
});
