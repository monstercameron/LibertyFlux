// original: 0x00b92670 script_null_token_predicate (proposed)

/// Returns 1 when a name is null or a recognised null token.
///
/// Takes a string pointer: a null pointer, an empty string, a lone newline,
/// `NULL` or `null` (compared byte by byte against four constant strings)
/// all yield 1; anything else yields 0 with the upper bytes set. Only the
/// low return byte is meaningful: on the null-pointer path the upper bytes
/// are entry-register residue, so the contract compares `al` only.
///
/// Original: 0x00B92670 (cdecl, one stack word, flag in al).
lf_checker_rt::export!(cdecl, rw_00b92670(s: u32) -> u32 {
    unsafe {
        const S_EMPTY: u32 = 0x00EB5259;
        const S_NL: u32 = 0x00EB56F8;
        const S_UPPER: u32 = 0x00EB56FC;
        const S_LOWER: u32 = 0x00EB5704;

        #[inline(always)]
        unsafe fn eq(a: u32, b: u32) -> bool {
            unsafe {
                let mut i = 0usize;
                loop {
                    let x = (a as *const u8).add(i).read();
                    let y = (b as *const u8).add(i).read();
                    if x != y {
                        return false;
                    }
                    if x == 0 {
                        return true;
                    }
                    i += 1;
                }
            }
        }

        if s == 0 {
            return 1;
        }
        for c in [S_EMPTY, S_NL, S_UPPER, S_LOWER] {
            if eq(s, lf_checker_rt::relocated(c)) {
                return 1;
            }
        }
        0xFFFF_FF00
    }
});
