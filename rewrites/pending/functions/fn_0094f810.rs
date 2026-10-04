// original: 0x0094f810 reset_object
/// Reset an object to its freshly created state.
///
/// Zeroes the header words, stamps the empty tag (`-1`), clears three table
/// entries (dword + half-word each) and the status byte. Returns the object.
export!(thiscall, rw_0094f810(obj: *mut u8) -> u32 {
    unsafe {
        let word = |o: usize| obj.add(o) as *mut u32;
        *word(0) = 0;
        *word(4) = 0;
        *word(0x14) = 0;
        *word(0x18) = 0xFFFF_FFFF;
        *word(8) = 0;
        *word(0xC) = 0;
        for i in 0..3 {
            *word(0xA0 + i * 4) = 0;
            *(obj.add(0xAC + i * 2) as *mut u16) = 0;
        }
        *obj.add(0x20) = 0;
        obj as u32
    }
});
