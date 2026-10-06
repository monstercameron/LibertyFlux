// original: 0x008BE280 state_block_copy_e80_from_128c (proposed)

/// Copy the 14-word live input-state block into the snapshot block.
///
/// Reads 14 dwords starting at the live block and writes them in order to the
/// snapshot block. Takes no arguments. Returns the last word copied: the
/// original keeps each loaded word in eax as it stores it, so eax holds the
/// final word at the plain `ret` (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_008BE280() -> u32 {
    unsafe {
        /// First word of the snapshot block (destination).
        const DST: u32 = 0x01160E80;
        /// First word of the live block (source).
        const SRC: u32 = 0x0116128C;
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
