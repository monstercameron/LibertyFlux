// original: 0x0091F010 find_wide_substring
use lf_checker_rt::{callee_cdecl, export};

/// Search a wide string for a wide needle of helper-given length.
///
/// Returns null for a null needle. Otherwise asks the length helper for the
/// needle length and scans the haystack for a run of that many equal wide
/// characters. A full run returns the haystack pointer just past the match;
/// no match returns null. A zero length matches at the start (or at the
/// terminator of an empty haystack).
export!(cdecl, rw_0091f010(hay: u32, ndl: u32) -> u32 {
    unsafe {
        if ndl == 0 {
            return 0;
        }
        let len: i32 = callee_cdecl!(1, u32, ndl) as i32;  // a-S06: signed (original jumps signed)
        let mut run: i32 = 0;
        let mut esi = hay;
        if *(esi as *const u16) != 0 {
            loop {
                if run >= len {
                    break;
                }
                let ax = *(esi as *const u16);
                let bx = *((ndl + (run as u32) * 2) as *const u16);
                if ax == bx {
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