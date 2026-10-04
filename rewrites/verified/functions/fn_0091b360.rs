// original: 0x0091B360 char_map_copy
/// Copy a NUL-terminated byte string through a per-byte mapping call.
///
/// Returns immediately when either pointer is null (leaving `EAX`
/// untouched, so the return channel is uncompared). Otherwise measures the
/// source length and copies only when `len + 1 < maxlen`; each source byte
/// is zero-extended and passed to the mapping helper, whose low byte is
/// stored, and the destination is always NUL-terminated on the copy path
/// (including the empty-string case, which stores just the terminator).
export!(cdecl, rw_0091b360(src: u32, dst: u32, maxlen: u32) -> u32 {
    unsafe {
        if src == 0 || dst == 0 {
            return 0;
        }
        let s = src as *const u8;
        let d = dst as *mut u8;
        let mut len: u32 = 0;
        while *s.add(len as usize) != 0 {
            len = len.wrapping_add(1);
        }
        if len.wrapping_add(1) >= maxlen {
            return 0;
        }
        if len == 0 {
            *d = 0;
            return 0;
        }
        let mut i: u32 = 0;
        while i < len {
            let b = *s.add(i as usize);
            let mapped: u32 = callee_cdecl!(1, u32, b as u32);
            *d.add(i as usize) = mapped as u8;
            i = i.wrapping_add(1);
        }
        *d.add(len as usize) = 0;
        0
    }
});
