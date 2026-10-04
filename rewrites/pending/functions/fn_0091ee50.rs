// original: 0x0091EE50 wstr_upper_copy
/// Copy a wide string while uppercasing it under the global language mode.
///
/// Copies `src` to `dst` including the NUL terminator and returns `dst`.
/// Each character is folded by subtracting `0x20` when the mode selects it:
/// plain `a`-`z` always folds; in mode `0x72` the range `0xAE..=0xCD` folds
/// too and everything else copies; outside mode `0x72`, mode `0x6A` or a
/// nonzero byte at `0x0116C253` copies everything else, otherwise characters
/// at or above `0xE0` fold and the rest copy. An empty source stores just the
/// terminator.
export!(cdecl, rw_0091ee50(dst: u32, src: u32) -> u32 {
    unsafe {
        let d = dst as *mut u16;
        let s = src as *const u16;
        if *s == 0 {
            *d = 0;
            return dst;
        }
        let lang = *global::<u8>(0x116C250);
        let aux = *global::<u8>(0x116C253);
        let mut i: usize = 0;
        loop {
            let c = *s.add(i);
            if c == 0 {
                break;
            }
            let o = if lang == 0x72 {
                if (0x61..=0x7A).contains(&c) {
                    c.wrapping_sub(0x20)
                } else if c < 0xAE || c > 0xCD {
                    c
                } else {
                    c.wrapping_sub(0x20)
                }
            } else if (0x61..=0x7A).contains(&c) {
                c.wrapping_sub(0x20)
            } else if lang == 0x6A || aux != 0 || c < 0xE0 {
                c
            } else {
                c.wrapping_sub(0x20)
            };
            *d.add(i) = o;
            i += 1;
        }
        *d.add(i) = 0;
        dst
    }
});
