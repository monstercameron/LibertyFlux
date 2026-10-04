// original: 0x00a65210 classify_index_two_level
/// Classifies the small index by the object's two-level state.
///
/// When the state dword at +0x1300 reads 1, or else the linked record's
/// flag word at +0xEC has bit 9 set, indices 6 and 8 map to 0 and anything
/// else to -1. Otherwise 6 maps to 1, 7 to 0, 8 to 2 and anything else to
/// -1. Returns the class as a full dword.
export!(cdecl, rw_00a65210(p: u32, v: u32) -> u32 {
    unsafe {
        let alt = if *((p + 0x1300) as *const u32) == 1 {
            true
        } else {
            let inner = *((p + 0xDC8) as *const u32);
            let bits = *((inner + 0xEC) as *const u32);
            (bits >> 9) & 1 == 1
        };
        if alt {
            if v == 6 || v == 8 {
                0
            } else {
                u32::MAX
            }
        } else if v == 6 {
            1
        } else if v == 7 {
            0
        } else if v == 8 {
            2
        } else {
            u32::MAX
        }
    }
});
