// original: 0x0089a640 rage::audEnvelopeSound::audEnvelopeSound
/// Constructor of the envelope-sound object (thiscall, no arguments).
///
/// Runs the base-class constructor through the intercepted callee, then
/// installs this class's virtual table and default field values: three
/// unit gains, zeroed counters and a released-slot marker byte. The flag
/// byte keeps its other bits and drops the construction-in-progress bits.
/// Returns the object pointer.
export!(thiscall, rw_0089a640(this: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        *this.add(0xF8) &= 0xF1;
        *(this as *mut u32) = relocated(0x00E79B84);
        core::ptr::write_unaligned(this.add(0xF5) as *mut u16, 0);
        *(this.add(0xB0) as *mut u32) = 0x3F80_0000;
        *(this.add(0xB4) as *mut u32) = 0x3F80_0000;
        *(this.add(0xB8) as *mut u32) = 0x3F80_0000;
        *(this.add(0xBC) as *mut u32) = 0;
        *(this.add(0xC0) as *mut u32) = 0;
        *(this.add(0xC4) as *mut u32) = 0;
        *(this.add(0xC8) as *mut u32) = 0xFFFF_FFFF;
        *(this.add(0xCC) as *mut u32) = 0xFFFF_FFFF;
        *(this.add(0xD0) as *mut u32) = 0xFFFF_FFFF;
        *(this.add(0xD4) as *mut u32) = 0;
        *(this.add(0xD8) as *mut u32) = 0;
        *(this.add(0xDC) as *mut u32) = 0;
        *(this.add(0xE0) as *mut u32) = 0;
        *(this.add(0xE4) as *mut u32) = 0;
        *this.add(0xF7) = 0xFF;
        this as u32
    }
});

