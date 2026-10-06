// original: 0x008BF680 state_block_copy_12c4_from_128c (proposed)

/// Copy the 14-word live block into the stored block.
///
/// Reads 14 dwords starting at the live block and writes them in order to the
/// stored block. Takes no arguments. Returns the last word copied (left in
/// eax by the final load; cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_008BF680() -> u32 {
    unsafe {
        /// First word of the stored block (destination).
        const DST: u32 = 0x011612C4;
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
