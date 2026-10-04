// original: 0x00ad0070 NativeImpl_RESET_CAR_WHEELS
/// Reset wheel state and release the attached sub-object.
///
/// Sets flag bit 1 and clears bits 0, 4 and 5 of the flag word, spreads a
/// seed value (zero, or the field at 0x20 when the argument byte is clear)
/// over three fields, zeroes the neighbouring scalar fields, releases the
/// sub-object at 0xd0 through a helper when one is attached, then writes the
/// trailing defaults including two 1.0 factors. Returns the helper's answer
/// when it ran, otherwise the flag word.
export!(thiscall, rw_00ad0070(this: *mut u8, reset: u8) -> u32 {
    unsafe {
        let flags = this.add(0x164) as *mut u32;
        let f = (*flags & !0x31) | 2;
        *flags = f;
        let seed: f32 = if reset != 0 {
            0.0
        } else {
            *(this.add(0x20) as *const f32)
        };
        *(this.add(0x78) as *mut f32) = seed;
        *(this.add(0x74) as *mut f32) = seed;
        *(this.add(0x70) as *mut f32) = seed;
        *(this.add(0x7c) as *mut u32) = 0;
        *(this.add(0x80) as *mut u32) = 0;
        *(this.add(0x84) as *mut u32) = 0;
        *(this.add(0xb8) as *mut u32) = 0;
        *(this.add(0xb4) as *mut u32) = 0;
        *(this.add(0xb0) as *mut u32) = 0;
        let slot = this.add(0xd0) as *mut u32;
        // The return register keeps the helper's answer when it runs.
        let answer = if *slot != 0 {
            callee_thiscall!(1, u32, *slot, slot as u32)
        } else {
            f
        };
        *slot = 0;
        *(this.add(0x168) as *mut u32) = 0;
        *(this.add(0xe0) as *mut f32) = 1.0;
        *(this.add(0xe4) as *mut f32) = 1.0;
        answer
    }
});
