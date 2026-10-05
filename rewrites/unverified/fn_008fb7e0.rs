// original: 0x008FB7E0 text_is_special_or_ascii
/// Test a string against two expected-word patterns.
///
/// Runs the word-pattern matcher with (0x7a, 0x78) and, if that
/// fails, with (0x807a, 0x78); returns 1 when either matches, else 0.
/// Cdecl, one stack argument (forwarded opaquely).
export!(cdecl, rw_008fb7e0(s: u32) -> u32 {
    unsafe {
        let r1: u32 = callee_cdecl!(1, u32, s, 0x7a, 0x78);
        if (r1 as u8) != 0 {
            return 1;
        }
        let r2: u32 = callee_cdecl!(2, u32, s, 0x807a, 0x78);
        if (r2 as u8) != 0 { 1 } else { 0 }
    }
});
