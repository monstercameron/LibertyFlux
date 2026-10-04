// original: 0x008AB4E0 audio_slot_init
/// Initialise an audio slot object: clear the state words, set the mode
/// flag bit 1 while preserving the other flag bits, and stamp the header.
export!(thiscall, rw_008AB4E0(obj: *mut u8) -> u32 {
    unsafe {
        let w = |o: usize| (obj.add(o) as *mut u32);
        *w(0x24) = 0;
        *w(0x28) = 0;
        *w(0x30) = 0;
        *w(0x2C) = 0;
        *w(0x34) = 0;
        *w(0x38) = 0;
        let flags = obj.add(0x3C);
        *flags = (*flags & 0xFA) | 2;
        *(obj as *mut u16) = 0x100;
        *w(4) = 0;
        *w(8) = 0;
        *w(0xC) = 0;
        *w(0x10) = 0;
        *w(0x14) = 0;
        *w(0x18) = 0;
        *(obj.add(0x20) as *mut u16) = 0;
        obj as u32
    }
});
