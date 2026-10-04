// original: 0x008a09d0 audSound_ctor_init_playback_fields
/// Constructor tail that initialises playback fields of a sound object.
///
/// Runs the base constructor, then writes the default sentinel words, unit
/// gains, silence levels, zeroed accumulators and the channel mask. Returns
/// `this`.
export!(thiscall, rw_008a09d0(this: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        let w = |off: usize| this.add(off) as *mut u32;
        *w(0x28) = 0xffff_ffff;
        *w(0x2c) = 0xffff_ffff;
        // Unit gains (1.0f bit patterns).
        *w(0x30) = 0x3f80_0000;
        *w(0x34) = 0x3f80_0000;
        *w(0x38) = 0;
        *(this.add(0x3c) as *mut u16) = 0;
        *w(0x70) = 0x0001_ffff;
        *this.add(0x74) = 0;
        // Silence floor (-100.0f bit patterns).
        *w(0x40) = 0xc2c8_0000;
        *w(0x44) = 0xc2c8_0000;
        // Inverted unit (-1.0f bit patterns).
        *w(0x48) = 0xbf80_0000;
        *w(0x4c) = 0xbf80_0000;
        *w(0x50) = 0;
        *w(0x54) = 0;
        *w(0x58) = 0;
        *w(0x5c) = 0;
        *w(0x60) = 0;
        this as u32
    }
});
