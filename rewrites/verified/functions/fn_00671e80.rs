// original: 0x00671E80 streaming_wildcard_match_ascii

/// Match a NUL-terminated ASCII pattern against a NUL-terminated string.
/// The pattern supports `?` for one non-NUL byte and `*` for any number of
/// bytes, folds ASCII uppercase letters to lowercase, and returns zero on a
/// match or one on a mismatch. Consecutive stars are treated as one star.
/// ECX is the pattern pointer and EDX is the text pointer.
///
/// Original: 0x00671E80 (fastcall, two register arguments, no stack arguments).
lf_checker_rt::export!(fastcall, rw_00671e80(pattern: u32, text: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd8(address: u32) -> u8 { unsafe { (address as *const u8).read() } }
        #[inline(always)]
        fn fold_ascii(ch: u8) -> u8 {
            if (b'A'..=b'Z').contains(&ch) { ch + 0x20 } else { ch }
        }
        let (mut p, mut t) = (0u32, 0u32);
        let mut star_after = None;
        let mut star_text = 0u32;
        loop {
            let pc = rd8(pattern.wrapping_add(p));
            let tc = rd8(text.wrapping_add(t));
            if pc == b'*' {
                while rd8(pattern.wrapping_add(p)) == b'*' { p = p.wrapping_add(1); }
                if rd8(pattern.wrapping_add(p)) == 0 { return 0; }
                star_after = Some(p);
                star_text = t;
                continue;
            }
            if pc == 0 && tc == 0 { return 0; }
            if tc != 0 && (pc == b'?' || fold_ascii(pc) == fold_ascii(tc)) {
                p = p.wrapping_add(1);
                t = t.wrapping_add(1);
                continue;
            }
            if let Some(after_star) = star_after {
                if rd8(text.wrapping_add(star_text)) == 0 { return 1; }
                star_text = star_text.wrapping_add(1);
                p = after_star;
                t = star_text;
                continue;
            }
            return 1;
        }
    }
});
