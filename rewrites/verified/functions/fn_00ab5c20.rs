// original: 0x00ab5c20 stream_slot_reset (proposed)

/// Reset a streaming slot in place and return it.
///
/// Keeps only the top three bits of the flag byte at `+0`, clears the tag
/// byte at `+1`, and zeroes every word from `+0x08` through `+0x84`
/// inclusive (bytes `+2..+8` are left alone). Returns the slot pointer.
///
/// Original: 0x00ab5c20 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00ab5c20(slot: u32) -> u32 {
    unsafe {
        const KEEP_MASK: u8 = 0xE0;
        const TAG_OFF: u32 = 0x01;
        const FIRST_WORD: u32 = 0x08;
        const LAST_WORD: u32 = 0x84;
        let flags = (slot as *const u8).read();
        (slot as *mut u8).write(flags & KEEP_MASK);
        ((slot + TAG_OFF) as *mut u8).write(0);
        let mut off = FIRST_WORD;
        while off <= LAST_WORD {
            ((slot + off) as *mut u32).write_unaligned(0);
            off += 4;
        }
        slot
    }
});
