// original: 0x00b057b0 reset_input_state_partial

/// Partial reset of an input-state block: same guarded-word clearing as the
/// full reset, but sets the flag byte to 1 and leaves +0x50 untouched.
export!(thiscall, rw_00b057b0(obj: *mut u8) -> u32 {
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
        *obj.add(0x0c) = 1;
        0
    }
});
