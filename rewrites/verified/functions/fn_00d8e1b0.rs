// original: 0x00d8e1b0 audio_state_reset_small
/// Resets a small audio state block: two dwords and a flag byte to zero.
export!(thiscall, rw_00d8e1b0(this: *mut u8) -> u32 {
    unsafe {
        *(this.add(4) as *mut u32) = 0;
        *(this.add(8) as *mut u32) = 0;
        *this.add(0xC) = 0;
        this as u32
    }
});
