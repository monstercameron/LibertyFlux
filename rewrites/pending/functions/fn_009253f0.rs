// original: 0x009253F0 input_flag_classify
/// Classify two flag words into 0, 1 or 2.
///
/// Returns 0 when the value word is zero or the mode word has bit 10 set.
/// Otherwise, when the mode word has bit 8 set, returns 2 if the value has
/// bit 10 set, else bit 8 of the value; when bit 8 is clear, returns 2 if
/// the value has any of bits 8 or 10 set, else 1.
export!(cdecl, rw_009253F0(a0: u32, a1: u32) -> u32 {
    if a1 == 0 {
        return 0;
    }
    if a0 & 0x400 != 0 {
        return 0;
    }
    if a0 & 0x100 != 0 {
        if a1 & 0x400 != 0 {
            2
        } else {
            (a1 >> 8) & 1
        }
    } else if a1 & 0x500 != 0 {
        2
    } else {
        1
    }
});
