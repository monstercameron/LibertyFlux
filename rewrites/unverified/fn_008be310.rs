// original: 0x008BE310 state_block_copy_e80_from_12c4 (proposed)

/// Copy the 14-word stored input-state block into the snapshot block.
///
/// Same shape as the neighbouring block copies: reads 14 dwords starting at
/// the stored block and writes them in order to the snapshot block. Takes no
/// arguments. Returns the last word copied (left in eax by the final load;
/// cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_008BE310() -> u32 {
    unsafe {
        /// First word of the snapshot block (destination).
        const DST: u32 = 0x01160E80;
        /// First word of the stored block (source).
        const SRC: u32 = 0x011612C4;
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
