// original: 0x00AF4540 stream_name_join_hash (proposed)

/// Join two names with a dot in the static buffer, tail-hash the result.
///
/// Copies `head` into the static join buffer, appends the two separator
/// bytes (a dot and a zero), appends `tail` after the new end, then tail-
/// calls the hash callee with (buffer, 0) and returns its result. The
/// buffer keeps stale bytes past the new end; the hash reads up to the
/// first zero.
///
/// Original: 0x00AF4540 (cdecl, two stack words, one tail-call callee).
lf_checker_rt::export!(cdecl, rw_00af4540(head: u32, tail: u32) -> u32 {
    unsafe {
        const HASH_CALLEE: u32 = 1;
        const JOIN_BUF: u32 = 0x015F8940;
        const SEPARATOR: u32 = 0x00EA7DD0;
        let buf = lf_checker_rt::relocated(JOIN_BUF);
        let mut i: u32 = 0;
        loop {
            let c = ((head.wrapping_add(i)) as *const u8).read();
            ((buf.wrapping_add(i)) as *mut u8).write(c);
            if c == 0 {
                break;
            }
            i = i.wrapping_add(1);
        }
        let sep = (lf_checker_rt::global::<u16>(SEPARATOR)).read_unaligned();
        ((buf.wrapping_add(i)) as *mut u16).write_unaligned(sep);
        let mut j: u32 = 0;
        while ((buf.wrapping_add(j)) as *const u8).read() != 0 {
            j = j.wrapping_add(1);
        }
        let mut k: u32 = 0;
        loop {
            let c = ((tail.wrapping_add(k)) as *const u8).read();
            ((buf.wrapping_add(j).wrapping_add(k)) as *mut u8).write(c);
            if c == 0 {
                break;
            }
            k = k.wrapping_add(1);
        }
        lf_checker_rt::callee_cdecl!(HASH_CALLEE, u32, buf, 0)
    }
});
