// original: 0x008acbc0 audio_filter_state_init
/// Audio filter state constructor: vtable pointer plus default parameters.
///
/// Installs the relocated vtable pointer, zeroes the counters and state
/// words, and sets the two unity gains and the default rate word.
export!(thiscall, rw_008acbc0(this_: *mut u8) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00E7C88C;
        const ONE_BITS: u32 = 0x3F800000; // 1.0f
        const RATE_BITS: u32 = 0x46BAB800;
        *(this_ as *mut u32) = relocated(VTABLE);
        *(this_.add(0x04) as *mut u32) = 0;
        *(this_.add(0x08) as *mut u32) = RATE_BITS;
        *(this_.add(0x0c) as *mut u32) = 0;
        *(this_.add(0x10) as *mut u32) = 0;
        *(this_.add(0x14) as *mut u32) = 0;
        *(this_.add(0x18) as *mut u32) = 0;
        *(this_.add(0x1c) as *mut u32) = ONE_BITS;
        *(this_.add(0x20) as *mut u32) = 0;
        *(this_.add(0x24) as *mut u32) = ONE_BITS;
        *this_.add(0x28) = 0;
        *(this_.add(0x2c) as *mut u32) = 0;
        *(this_.add(0x30) as *mut u32) = 0;
        this_ as u32
    }
});
