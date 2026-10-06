// original: 0x008BF5F0 state_block_copy_128c_from_e80 (proposed)

/// Copy the 14-word snapshot block back into the live block.
///
/// The reverse of the neighbouring snapshot copies: reads 14 dwords starting
/// at the snapshot block and writes them in order to the live block. Takes no
/// arguments. Returns the last word copied (left in eax by the final load;
/// cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_008BF5F0() -> u32 {
    unsafe {
        /// First word of the live block (destination).
        const DST: u32 = 0x0116128C;
        /// First word of the snapshot block (source).
        const SRC: u32 = 0x01160E80;
        /// Words copied.
        const WORDS: u32 = 14;
        let mut last = 0u32;
        let mut i = 0u32;
        while i < WORDS {
            let v =
                (lf_checker_rt::relocated(SRC + i * 4) as *const u32).read_unaligned();
            (lf_checker_rt::relocated(DST + i * 4) as *mut u32).write_unaligned(v);
            last = v;
            i += 1;
        }
        last
    }
});
