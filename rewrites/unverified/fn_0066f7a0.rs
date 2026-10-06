// original: 0x0066F7A0 rage::fiTokenizer::read_until_delimiter

/// Read characters into `buf` until the delimiter byte, skipping leading
/// whitespace, trimming trailing whitespace, and terminating with NUL.
///
/// `this` points to the tokenizer, `buf` holds up to `size` bytes, `delim`
/// is the delimiter (only its low byte is read, sign-extended) and `flags`
/// controls whitespace skipping (only its low byte matters: zero stores every
/// character). At most `size - 1` characters are stored, so sizes below 2
/// store nothing; the return is the stored length, or -1 when the read failed
/// before anything was stored (a NUL is still written when the length is 0).
///
/// Each character comes from the pushback stack first (count at `+0x18`,
/// cells at `+0x1c`, popped last-in-first-out and SIGN-extended), then from
/// the stream buffer (`+0x08` buffer, `+0x10` position, `+0x14` end, compared
/// SIGNED, zero-extended), then from the refill callee (callee 1, thiscall on
/// the stream with a one-byte cell and length 1, zero-extended; any answer
/// but 1 fails the read). A newline increments the line number at `+0x08`; a
/// semicolon runs the comment skipper (callee 2, thiscall on the tokenizer);
/// -1 (only reachable from the pushback) fails the read. A character equal to
/// the delimiter ends the read. When skipping is on and no character has been
/// stored yet, space, tab, newline, carriage return and NUL are dropped. The
/// loop bound (`stored < size - 1`) is a signed comparison of small values;
/// the callee-1 answer is compared for equality with 1.
///
/// The original passes its incoming `size` argument slot as the refill cell,
/// so the refill overwrites that slot; the rewrite uses its own cell seeded
/// with `size` (the contract snapshots it and scripts the refill bytes), and
/// this proof runs with the stack comparison off. Started from the stream
/// field names of `rage::fiTokenizer::vf27` (lane r-s435).
///
/// Original: 0x0066F7A0 (thiscall, four stack words, 2 calls).
lf_checker_rt::export!(thiscall, rw_0066F7A0(this: u32, buf: u32, size: u32, delim: u32, flags: u32) -> u32 {
    unsafe {
        const LINE: u32 = 0x08;
        const STREAM: u32 = 0x0c;
        const PUSHBACK_COUNT: u32 = 0x18;
        const PUSHBACK_BUF: u32 = 0x1c;
        const STREAM_BUF: u32 = 0x08;
        const STREAM_POS: u32 = 0x10;
        const STREAM_END: u32 = 0x14;
        const NEWLINE: i32 = 0x0a;
        const COMMENT: i32 = 0x3b;
        const END_OF_INPUT: i32 = -1;
        const REFILL: u32 = 1;
        const SKIP_COMMENT: u32 = 2;

        let limit = (size as i32).wrapping_sub(1);
        let mut stored = 0i32;
        let mut skipping = true;
        let mut failed = false;
        if limit > 0 {
            // Mirrors the original's refill cell (its incoming `size` slot):
            // seeded with `size`, the refill callee overwrites it on both
            // sides, and the contract snapshots it at each call.
            let mut cell: u32 = size;
            let delimiter = ((delim & 0xff) as u8) as i8 as i32;
            let skip_ws = (flags & 0xff) != 0;
            loop {
                // Fetch one character.
                let ch: i32;
                let count = ((this + PUSHBACK_COUNT) as *const u32).read_unaligned();
                if count != 0 {
                    let left = count.wrapping_sub(1);
                    ((this + PUSHBACK_COUNT) as *mut u32).write_unaligned(left);
                    ch = ((this + PUSHBACK_BUF + left) as *const i8).read() as i32;
                } else {
                    let stream = ((this + STREAM) as *const u32).read_unaligned();
                    let pos = ((stream + STREAM_POS) as *const u32).read_unaligned();
                    let end = ((stream + STREAM_END) as *const u32).read_unaligned();
                    if (pos as i32) < (end as i32) {
                        let sbuf = ((stream + STREAM_BUF) as *const u32).read_unaligned();
                        ch = ((sbuf.wrapping_add(pos)) as *const u8).read() as i32;
                        ((stream + STREAM_POS) as *mut u32)
                            .write_unaligned(pos.wrapping_add(1));
                    } else {
                        let got: u32 = lf_checker_rt::callee_thiscall!(
                            REFILL,
                            u32,
                            stream,
                            &mut cell as *mut u32 as u32,
                            1
                        );
                        if got != 1 {
                            failed = true;
                            break;
                        }
                        ch = (cell & 0xff) as i32;
                    }
                }
                if ch == NEWLINE {
                    let line = ((this + LINE) as *const u32).read_unaligned();
                    ((this + LINE) as *mut u32).write_unaligned(line.wrapping_add(1));
                } else if ch == COMMENT {
                    lf_checker_rt::callee_thiscall!(SKIP_COMMENT, u32, this);
                } else if ch == END_OF_INPUT {
                    failed = true;
                    break;
                }
                if ch == delimiter {
                    break;
                }
                let drop = skip_ws
                    && skipping
                    && (ch == 0x20 || ch == 9 || ch == NEWLINE || ch == 0x0d || ch == 0);
                if !drop {
                    ((buf.wrapping_add(stored as u32)) as *mut u8).write(ch as u8);
                    skipping = false;
                    stored += 1;
                }
                if !(stored < limit) {
                    break;
                }
            }
        }
        if failed && stored == 0 {
            stored = -1;
        }
        // Trim trailing whitespace.
        if stored > 0 {
            let mut c = stored - 1;
            loop {
                let b = ((buf.wrapping_add(c as u32)) as *const i8).read() as i32;
                if b == 0x20 || b == 9 || b == NEWLINE || b == 0x0d || b == 0 {
                    stored -= 1;
                    c -= 1;
                    if c >= 0 {
                        continue;
                    }
                }
                break;
            }
        }
        if stored >= 0 {
            ((buf.wrapping_add(stored as u32)) as *mut u8).write(0);
        }
        stored as u32
    }
});
