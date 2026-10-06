// original: 0x0094EAA0 Text_AppendHashKeySubstring (symbols)

/// Copy a string into the first free word slot of a text table.
///
/// `this` (ECX) is the table, `src` the string, `flag` selects the region
/// (only its low byte is tested). With a non-zero flag the 32 head slots
/// at `this + HEAD_BASE + i * STRIDE` are scanned for the first zero word:
/// the string is copied there through callee 1 (`dst`, `src`, `COPY_LEN`
/// on the stack), the mark byte at `this + i + MARK_BASE` is set to 1 and
/// the flag byte at `this + i + FLAG_BASE` to 0, and the index is returned.
/// With a zero flag the 192 main slots at `this + i * STRIDE` are scanned
/// the same way but without any mark bytes. When no slot is free the
/// return is -1. Same shape as Text_AppendLiteralString; only the copy
/// callee differs.
///
/// Original: 0x0094EAA0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_0094EAA0(this: u32, src: u32, flag: u32) -> u32 {
    unsafe {
        const HEAD_BASE: u32 = 0x9600;
        const STRIDE: u32 = 0xC8;
        const HEAD_COUNT: u32 = 0x20;
        const MAIN_COUNT: u32 = 0xC0;
        const MARK_BASE: u32 = 0xAF00;
        const FLAG_BASE: u32 = 0xAF20;
        const COPY_LEN: u32 = 0x64;
        const COPY: u32 = 1;
        const FULL: u32 = 0xFFFF_FFFF;
        if (flag & 0xFF) != 0 {
            let mut i = 0u32;
            let mut p = this.wrapping_add(HEAD_BASE);
            while i < HEAD_COUNT {
                if (p as *const u16).read_unaligned() == 0 {
                    lf_checker_rt::callee_cdecl!(COPY, u32, p, src, COPY_LEN);
                    (this.wrapping_add(i).wrapping_add(MARK_BASE) as *mut u8).write(1);
                    (this.wrapping_add(i).wrapping_add(FLAG_BASE) as *mut u8).write(0);
                    return i;
                }
                i += 1;
                p = p.wrapping_add(STRIDE);
            }
            FULL
        } else {
            let mut i = 0u32;
            let mut p = this;
            while i < MAIN_COUNT {
                if (p as *const u16).read_unaligned() == 0 {
                    lf_checker_rt::callee_cdecl!(COPY, u32, p, src, COPY_LEN);
                    return i;
                }
                i += 1;
                p = p.wrapping_add(STRIDE);
            }
            FULL
        }
    }
});
