// original: 0x00b05650 reset_input_state_full

/// Full reset of an input-state block: clears the eight guarded words (and
/// each word's 6-byte-earlier dword when the word was already zero), zeroes
/// the companion words, clears the flag byte, and stamps -1.0 at +0x50.
export!(thiscall, rw_00b05650(obj: *mut u8) -> u32 {
    unsafe {
        // Offsets of the guarded words cleared by the two state resets.
        for k in [0x16u32, 0x1e, 0x26, 0x2e, 0x36, 0x3e, 0x46, 0x4e] {
            let w = obj.add(k as usize) as *mut u16;
            if *w == 0 {
                *w = 0;
                *(obj.add((k - 6) as usize) as *mut u32) = 0;
            }
            *(obj.add((k - 2) as usize) as *mut u16) = 0;
        }
        *obj.add(0x0c) = 0;
        *(obj.add(0x50) as *mut u32) = 0xBF80_0000;
        0
    }
});
