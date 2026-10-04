// original: 0x00898eb0 audio_cycle_plus2_mod3
/// Steps a selector forward by two modulo 3.
///
/// Reads the word at offset 0x60, adds two, stores the remainder modulo 3 at
/// offset 0x62, and returns the quotient.
export!(thiscall, rw_00898eb0(this: u32) -> u32 {
    unsafe {
        let step =
            core::ptr::read_unaligned(this.wrapping_add(0x60) as *const u16) as u32 + 2;
        core::ptr::write_unaligned(
            this.wrapping_add(0x62) as *mut u16,
            (step % 3) as u16,
        );
        step / 3
    }
});
