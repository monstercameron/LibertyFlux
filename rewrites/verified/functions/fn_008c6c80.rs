// original: 0x008C6C80 wstr_copy
/// Copy a wide string including its terminator.
///
/// Copies 16-bit units from `src` to `dst` until a zero word, which is
/// copied too. Returns the destination address of the terminator written.
/// The caller guarantees the destination has room. Reads and writes only
/// the two buffers, makes no calls. Original: cdecl, two stack words.
lf_checker_rt::export!(cdecl, rw_008c6c80(dst: u32, src: u32) -> u32 {
    unsafe {
        let mut d = dst as *mut u16;
        let mut s = src as *const u16;
        loop {
            let w = s.read_unaligned();
            d.write_unaligned(w);
            if w == 0 {
                break d as u32;
            }
            d = d.add(1);
            s = s.add(1);
        }
    }
});
