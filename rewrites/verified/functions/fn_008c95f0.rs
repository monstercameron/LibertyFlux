// original: 0x008C95F0 stream_parse_indexed_name (proposed)

/// Parses a two-digit index off a streaming name: fetches the name prefix
/// from the prefix callee (callee 0, no arguments) and compares `name`
/// against it with the compare callee (callee 1, called with the name, the
/// prefix and the prefix length, like memcmp). When the name is exactly two
/// bytes longer than the prefix, the prefix matches (callee 1 reports 0)
/// and the two trailing bytes are decimal digits, stores the parsed number
/// in `*out`; otherwise leaves `*out` alone.
///
/// Two stack arguments (cdecl): the name pointer and the output pointer; no
/// return value.
lf_checker_rt::export!(cdecl, rw_008C95F0(name: u32, out: u32) -> u32 {
    unsafe {
        /// Prefix callee id.
        const PREFIX: u32 = 0;
        /// Compare callee id.
        const COMPARE: u32 = 1;
        /// Extra bytes a valid name carries past the prefix.
        const SUFFIX_LEN: u32 = 2;
        unsafe fn strlen(s: u32) -> u32 {
            unsafe {
                let mut n = 0u32;
                while ((s + n) as *const u8).read() != 0 {
                    n += 1;
                }
                n
            }
        }
        let prefix: u32 = lf_checker_rt::callee_cdecl!(PREFIX, u32,);
        let plen = strlen(prefix);
        let nlen = strlen(name);
        if nlen != plen.wrapping_add(SUFFIX_LEN) {
            return 0;
        }
        let cmp: u32 = lf_checker_rt::callee_cdecl!(COMPARE, u32, name, prefix, plen);
        if cmp != 0 {
            return 0;
        }
        let d0 = ((name + plen) as *const u8).read();
        if d0.wrapping_sub(b'0') > 9 {
            return 0;
        }
        let d1 = ((name + plen + 1) as *const u8).read();
        if d1.wrapping_sub(b'0') > 9 {
            return 0;
        }
        let v = (d0 - b'0') as u32 * 10 + (d1 - b'0') as u32;
        (out as *mut u32).write_unaligned(v);
        0
    }
});
