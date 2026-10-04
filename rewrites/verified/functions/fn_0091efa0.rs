// original: 0x0091EFA0 wstr_find_cstr
/// Search a wide string for a byte-string needle, low byte against low byte.
///
/// Returns null for a null needle. Otherwise measures the NUL-terminated
/// needle and scans the wide haystack for a run whose low bytes equal the
/// needle bytes (the needle byte is sign-extended, so bytes at or above
/// `0x80` never match). A full run returns the haystack pointer just past
/// the match; no match returns null. An empty needle matches at the start
/// (or at the terminator of an empty haystack).
export!(cdecl, rw_0091efa0(hay: u32, ndl: u32) -> u32 {
    unsafe {
        if ndl == 0 {
            return 0;
        }
        let mut len: u32 = 0;
        while *((ndl + len) as *const u8) != 0 {
            len += 1;
        }
        let mut run: u32 = 0;
        let mut esi = hay;
        if *(esi as *const u16) != 0 {
            loop {
                if run >= len {
                    break;
                }
                let cx = (*((ndl + run) as *const u8) as i8) as u16;
                let ax = *(esi as *const u16) & 0xFF;
                if ax == cx {
                    run += 1;
                } else {
                    run = 0;
                }
                esi = esi.wrapping_add(2);
                if *(esi as *const u16) == 0 {
                    break;
                }
            }
        }
        if run == len {
            esi
        } else {
            0
        }
    }
});
