// original: 0x008CA600 stream_context_clear (proposed)

/// Clears a streaming context: zeroes the header dwords at `+0x00`, `+0x04`,
/// `+0x0C`, `+0x90` and `+0x94`, sets the flag byte at `+0x08` to 1, and
/// zeroes two 32-dword blocks at `+0x10` and `+0x98`. Bytes `+0x09`-`+0x0B`,
/// `+0x8D`-`+0x8F` and `+0x95`-`+0x97` are left untouched.
///
/// Thiscall on the context pointer in ECX; no return value. (The inventory
/// size truncates the clearing loop; the rewrite covers the whole loop.)
lf_checker_rt::export!(thiscall, rw_008CA600(this: u32) -> u32 {
    unsafe {
        /// Number of dwords in each cleared block.
        const BLOCK_WORDS: u32 = 32;
        /// Start of the first cleared block.
        const BLOCK0: u32 = 0x10;
        /// Start of the second cleared block.
        const BLOCK1: u32 = 0x98;
        /// Flag byte set to 1.
        const FLAG: u32 = 8;
        let w = |off: u32, v: u32| ((this + off) as *mut u32).write_unaligned(v);
        w(0x00, 0);
        w(0x04, 0);
        w(0x90, 0);
        ((this + FLAG) as *mut u8).write(1);
        w(0x0C, 0);
        w(0x94, 0);
        let mut i = 0u32;
        while i < BLOCK_WORDS {
            w(BLOCK0 + i * 4, 0);
            w(BLOCK1 + i * 4, 0);
            i += 1;
        }
        0
    }
});
