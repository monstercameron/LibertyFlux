// original: 0x008C6CB0 wstr_len
/// Count the 16-bit units before the terminator of a wide string.
///
/// `text` points at the string. Returns the number of nonzero words ahead
/// of the first zero word, exactly as the C library wide-string length.
/// Reads only, makes no calls. Original: cdecl, one stack word.
lf_checker_rt::export!(cdecl, rw_008c6cb0(text: u32) -> u32 {
    unsafe {
        let mut p = text as *const u16;
        let mut n: u32 = 0;
        while p.read_unaligned() != 0 {
            p = p.add(1);
            n = n.wrapping_add(1);
        }
        n
    }
});
