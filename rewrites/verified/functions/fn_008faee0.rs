// original: 0x008FAEE0 Text_GetByHash
/// Length of a NUL-terminated 16-bit string (wide strlen).
///
/// Returns 0 for a null pointer, else the number of words before the
/// first zero word. Quirk: only the low 16 bits of eax are written,
/// so the upper 16 bits of the result are the pointer's high bits.
/// Cdecl, one stack argument, full eax compared.
export!(cdecl, rw_008faee0(s: u32) -> u32 {
    unsafe {
        if s == 0 {
            return 0;
        }
        let mut n: u32 = 0;
        let mut p = s;
        if ((p as *const u16).read_unaligned() != 0) {
            loop {
                p = p.wrapping_add(2);
                n = n.wrapping_add(1);
                if ((p as *const u16).read_unaligned() == 0) {
                    break;
                }
            }
        }
        (s & 0xFFFF0000) | (n & 0xFFFF)
    }
});
