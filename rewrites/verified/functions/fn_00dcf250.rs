// original: 0x00dcf250 csv_read_token (proposed)

/// Read one delimiter-terminated token into `buf` (capacity `maxlen`) and
/// NUL-terminate it. Returns 1, or 0 when the token was truncated.
///
/// `this` points to the reader. Characters come from the next-char callee
/// and the end test from the end callee (both thiscall, object in ECX).
/// Carriage returns are skipped without being stored. A NUL, the delimiter
/// byte at `+0x41a`, or the end flag ends the token with success. When
/// `maxlen` characters have been stored, the last one is replaced by NUL
/// and 0 is returned. (With `maxlen` 0 the count can never match, so the
/// loop only ends at a terminator or the end flag.)
///
/// Original: 0x00DCF250 (thiscall, two stack words, byte result).
lf_checker_rt::export!(thiscall, rw_00dcf250(this: u32, buf: u32, maxlen: u32) -> u32 {
    unsafe {
        /// End-of-input predicate callee id.
        const AT_END: u32 = 1;
        /// Next-character callee id.
        const NEXT: u32 = 2;
        /// Field delimiter byte.
        const DELIM: u32 = 0x41a;
        const CR: u8 = 0x0d;
        (buf as *mut u8).write(0);
        let e: u32 = lf_checker_rt::callee_thiscall!(AT_END, u32, this);
        if (e as u8) != 0 {
            (buf as *mut u8).write(0);
            return 1;
        }
        let delim = ((this + DELIM) as *const u8).read();
        let mut out = buf;
        let mut count: u32 = 0;
        loop {
            let c: u32 = lf_checker_rt::callee_thiscall!(NEXT, u32, this);
            let c = c as u8;
            if c == CR {
                // Carriage returns are skipped, not stored.
            } else if c == delim || c == 0 {
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
