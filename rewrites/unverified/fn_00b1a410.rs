// original: 0x00b1a410 lowercase_wstr_in_place (proposed)

/// Lowercases a wide string in place, skipping parenthesised spans.
///
/// Cdecl of one stack word: a pointer to NUL-terminated 16-bit
/// characters; the first character is never touched. Each later
/// character is lowercased (+0x20) when it is an ASCII uppercase letter,
/// or, depending on the two mode bytes: when the first mode byte is
/// 0x72 also 0x8E..0xAD, otherwise (first mode byte not 0x6A and second
/// mode byte zero) also 0xC0..0xDF. A '(' starts a span that ends at the
/// next ')': span contents are never lowercased, and without a
/// terminator the '(' is processed normally. After a run of spaces the
/// next character is skipped, except '(' which opens a span. Returns the
/// address of the terminating NUL.
lf_checker_rt::export!(cdecl, rw_00b1a410(text: u32) -> u32 {
    unsafe {
        const MODE0: u32 = 0x0116c250;
        const MODE1: u32 = 0x0116c253;
        const LOWER_DELTA: u16 = 0x20;
        let mut p = text.wrapping_add(2);
        if ((p) as *const u16).read_unaligned() == 0 {
            return p;
        }
        loop {
            let w = (p as *const u16).read_unaligned();
            let mode0 = (lf_checker_rt::global::<u8>(MODE0) as *const u8).read();
            let lower = if mode0 == 0x72 {
                (0x41u16..=0x5a).contains(&w) || (0x8eu16..=0xad).contains(&w)
            } else if (0x41u16..=0x5a).contains(&w) {
                true
            } else if mode0 == 0x6a {
                false
            } else if (lf_checker_rt::global::<u8>(MODE1) as *const u8).read() != 0 {
                false
            } else {
                (0xc0u16..0xe0).contains(&w)
            };
            if lower {
                (p as *mut u16).write_unaligned(w.wrapping_add(LOWER_DELTA));
            }
            if (p as *const u16).read_unaligned() == 0x28 {
                let save = p;
                let mut q = p;
                loop {
                    let c = ((q.wrapping_add(2)) as *const u16).read_unaligned();
                    q = q.wrapping_add(2);
                    if c == 0x29 || c == 0 {
                        if c == 0 {
                            q = save;
                        }
                        break;
                    }
                }
                p = q;
            }
            if (p as *const u16).read_unaligned() == 0x20 {
                loop {
                    let c = ((p.wrapping_add(2)) as *const u16).read_unaligned();
                    p = p.wrapping_add(2);
                    if c == 0x28 {
                        p = p.wrapping_sub(2);
                        break;
                    }
                    if c != 0x20 {
                        break;
                    }
                }
            }
            p = p.wrapping_add(2);
            if (p as *const u16).read_unaligned() == 0 {
                break;
            }
        }
        p
    }
});
