// original: 0x00ab5ca0 stream_req_reset (proposed)

/// Reset an 11-lane streaming request block in place and return it.
///
/// Eleven iterations clear, per lane `i`: the flag bytes at `+0x5c+i` and
/// `+0x67+i`, the word at `+0x04+4*i` and the word at `+0x30+4*i`. Then the
/// header word at `+0` is cleared and the mode byte at `+0x72` is forced to
/// `(old & 0xE8) | 0x28`, keeping bits 7, 6, 5 and 3 while setting bits 5
/// and 3 and clearing the rest. Returns the block pointer.
///
/// Original: 0x00ab5ca0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00ab5ca0(block: u32) -> u32 {
    unsafe {
        const LANES: u32 = 11;
        const FLAG_A: u32 = 0x5C;
        const FLAG_B: u32 = 0x67;
        const WORD_A: u32 = 0x04;
        const WORD_B: u32 = 0x30;
        const MODE_OFF: u32 = 0x72;
        const MODE_KEEP: u8 = 0xE8;
        const MODE_SET: u8 = 0x28;
        let mut i = 0u32;
        while i < LANES {
            ((block + FLAG_A + i) as *mut u8).write(0);
            ((block + FLAG_B + i) as *mut u8).write(0);
            ((block + WORD_A + 4 * i) as *mut u32).write_unaligned(0);
            ((block + WORD_B + 4 * i) as *mut u32).write_unaligned(0);
            i += 1;
        }
        (block as *mut u32).write_unaligned(0);
        let mode = ((block + MODE_OFF) as *const u8).read();
        ((block + MODE_OFF) as *mut u8).write((mode & MODE_KEEP) | MODE_SET);
        block
    }
});
