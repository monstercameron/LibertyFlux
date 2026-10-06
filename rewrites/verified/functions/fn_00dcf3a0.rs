// original: 0x00dcf3a0 csv_read_quoted_token (proposed)

/// Read one token like the plain reader, except a double quote toggles
/// quoted mode and the delimiter only ends the token outside quotes.
///
/// `this`, `buf` and `maxlen` are as in the plain token reader. A `"`
/// (0x22) flips the in-quote flag and is never stored; carriage returns are
/// skipped; NUL always ends the token; the delimiter at `+0x41a` ends it
/// only while unquoted. Truncation at `maxlen` replaces the last stored
/// char with NUL and returns 0, otherwise the token is NUL-terminated and
/// 1 is returned. The end callee is consulted before the first char and
/// after every char.
///
/// Original: 0x00DCF3A0 (thiscall, two stack words, byte result).
lf_checker_rt::export!(thiscall, rw_00dcf3a0(this: u32, buf: u32, maxlen: u32) -> u32 {
    unsafe {
        /// End-of-input predicate callee id.
        const AT_END: u32 = 1;
        /// Next-character callee id.
        const NEXT: u32 = 2;
        /// Field delimiter byte.
        const DELIM: u32 = 0x41a;
        const CR: u8 = 0x0d;
        const QUOTE: u8 = 0x22;
        (buf as *mut u8).write(0);
        let e: u32 = lf_checker_rt::callee_thiscall!(AT_END, u32, this);
        if (e as u8) != 0 {
            (buf as *mut u8).write(0);
            return 1;
        }
        let delim = ((this + DELIM) as *const u8).read();
        let mut quoted = false;
        let mut out = buf;
        let mut count: u32 = 0;
        loop {
            let c: u32 = lf_checker_rt::callee_thiscall!(NEXT, u32, this);
            let c = c as u8;
            if c == QUOTE {
                quoted = !quoted;
            } else if c == CR {
                // Skipped like the plain reader.
            } else if c == 0 || (!quoted && c == delim) {
                (out as *mut u8).write(0);
                return 1;
            } else {
                (out as *mut u8).write(c);
                count = count.wrapping_add(1);
                out = out.wrapping_add(1);
                if count == maxlen {
                    (buf.wrapping_add(maxlen).wrapping_sub(1) as *mut u8).write(0);
                    return 0;
                }
            }
            let e: u32 = lf_checker_rt::callee_thiscall!(AT_END, u32, this);
            if (e as u8) != 0 {
                (out as *mut u8).write(0);
                return 1;
            }
        }
    }
});
