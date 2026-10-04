// original: 0x0091EF60 NativeImpl_SET_MENU_COLUMN
/// Copy a wide string with a character limit, always NUL-terminated.
///
/// Copies `src` to `dst` up to `max` characters when `max` is non-negative
/// (a zero limit stores just the terminator), or to the end of the string
/// when `max` is negative or the string is shorter; the destination always
/// gets a NUL terminator, including for an empty source. Returns `dst`.
/// (The merged symbol name looks unrelated to this bounded copy; it is kept
/// as the label with that caveat.)
export!(cdecl, rw_0091ef60(dst: u32, src: u32, max: i32) -> u32 {
    unsafe {
        let d = dst as *mut u16;
        let s = src as *const u16;
        if *s == 0 {
            *d = 0;
            return dst;
        }
        let mut n: u32 = 0;
        loop {
            if max >= 0 && n >= max as u32 {
                break;
            }
            *d.add(n as usize) = *s.add(n as usize);
            n += 1;
            if *s.add(n as usize) == 0 {
                break;
            }
        }
        *d.add(n as usize) = 0;
        dst
    }
});
