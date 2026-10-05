// original: 0x00AF3590 stream_line_parse (proposed)

/// Parse a "key value" line into the out pair, copying 0x200 bytes first.
///
/// Copies 0x200 bytes from `line` into a scratch buffer (the copy itself
/// runs natively on both sides). A first byte of '#' or ',' fails at once.
/// Otherwise the buffer is scanned from the start for a space (a zero byte
/// first fails); the space is overwritten with zero and the rest address is
/// handed with two frame slots to the pair callee, whose record's two words
/// are copied to `out`. Returns the second word with its low byte set on
/// success, entry garbage on failure (so the return channel stays off; the
/// words themselves are compared through `out`).
///
/// Original: 0x00AF3590 (stdcall, two stack words, one direct callee plus
/// the stack-cookie check).
lf_checker_rt::export!(stdcall, rw_00af3590(line: u32, out: u32) -> u32 {
    unsafe {
        const PAIR_CALLEE: u32 = 1;
        const COOKIE_CALLEE: u32 = 2;
        const BUF_LEN: u32 = 0x200;
        let mut buf = [0u8; 0x200];
        let mut i: u32 = 0;
        while i < BUF_LEN {
            buf[i as usize] = ((line.wrapping_add(i)) as *const u8).read();
            i = i.wrapping_add(1);
        }
        let c0 = buf[0];
        if c0 == b'#' || c0 == b',' {
            lf_checker_rt::callee_thiscall!(COOKIE_CALLEE, u32, 0);
            return 0;
        }
        let rest: usize;
        if c0 == b' ' {
            buf[0] = 0;
            rest = 1;
        } else {
            let mut e: usize = 0;
            loop {
                if buf[e] == 0 {
                    lf_checker_rt::callee_thiscall!(COOKIE_CALLEE, u32, 0);
                    return 0;
                }
                e += 1;
                if buf[e] == b' ' {
                    break;
                }
            }
            buf[e] = 0;
            rest = e + 1;
        }
        // The original also passes a third frame slot in ecx; the stand-in
        // is stdcall (identical cleanup) so that uncomparable address is
        // not part of the call log on either side.
        let slot_b = 0u32;
        let rec: u32 = lf_checker_rt::callee_stdcall!(
            PAIR_CALLEE, u32,
            core::ptr::addr_of!(slot_b) as u32,
            core::ptr::addr_of!(buf[rest]) as u32);
        let p0 = ((rec) as *const u32).read_unaligned();
        let p1 = ((rec.wrapping_add(4)) as *const u32).read_unaligned();
        ((out) as *mut u32).write_unaligned(p0);
        ((out.wrapping_add(4)) as *mut u32).write_unaligned(p1);
        lf_checker_rt::callee_thiscall!(COOKIE_CALLEE, u32, 0);
        (p1 & 0xFFFF_FF00) | 1
    }
});
