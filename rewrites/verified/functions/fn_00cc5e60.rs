// original: 0x00CC5E60 euphoria_feedback_init_34 (proposed)

/// Initialise a feedback record: 34 slot rows plus the header and footer.
///
/// Clears 34 consecutive 24-byte rows starting at `this + 8`. Each row gets
/// -1 in its first word and zero in the other five. Then the header words at
/// `+0`/`+4` are set to -1, the eight footer words at `+0x350`-`+0x36c` and
/// the words at `+0x370`/`+0x374` are zeroed, and the flag word at `+0x378`
/// keeps only its top two bits. Returns `this`.
///
/// The row loop counts down from 34 and keeps going while the counter is
/// still non-negative after the decrement, so it runs exactly 34 times.
///
/// Original: 0x00CC5E60 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00cc5e60(this: u32) -> u32 {
    unsafe {
        const ROWS: u32 = 0x22;
        const ROW_STRIDE: u32 = 0x18;
        const FLAG_WORD: u32 = 0x378;
        const FLAG_KEEP: u32 = 0xFFFFC000;
        const FOOTER_FIRST: u32 = 0x350;
        const FOOTER_WORDS: u32 = 8;
        let mut cursor = this.wrapping_add(0x14);
        let mut left = ROWS;
        loop {
            left = left.wrapping_sub(1);
            (cursor.wrapping_sub(0x0c) as *mut u32).write_unaligned(0xFFFF_FFFF);
            (cursor.wrapping_sub(0x08) as *mut u32).write_unaligned(0);
            (cursor as *mut u32).write_unaligned(0);
            (cursor.wrapping_sub(0x04) as *mut u32).write_unaligned(0);
            (cursor.wrapping_add(0x04) as *mut u32).write_unaligned(0);
            (cursor.wrapping_add(0x08) as *mut u32).write_unaligned(0);
            cursor = cursor.wrapping_add(ROW_STRIDE);
            if (left as i32) < 0 {
                break;
            }
        }
        let flags = (this.wrapping_add(FLAG_WORD) as *const u32).read_unaligned();
        (this.wrapping_add(FLAG_WORD) as *mut u32).write_unaligned(flags & FLAG_KEEP);
        (this.wrapping_add(0x370) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x374) as *mut u32).write_unaligned(0);
        (this as *mut u32).write_unaligned(0xFFFF_FFFF);
        (this.wrapping_add(4) as *mut u32).write_unaligned(0xFFFF_FFFF);
        for i in 0..FOOTER_WORDS {
            (this.wrapping_add(FOOTER_FIRST).wrapping_add(i * 4) as *mut u32)
                .write_unaligned(0);
        }
        this
    }
});
