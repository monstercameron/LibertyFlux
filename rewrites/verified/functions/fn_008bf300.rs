// original: 0x008BF300 input_state_changed_check (proposed)

/// Report whether the snapshot block differs from the live block at the
/// watched indexes.
///
/// Walks the 14 words of the snapshot and live blocks together but compares
/// only the seven watched positions (indexes 0, 1, 9, 10, 11, 12, 13); the
/// middle words are skipped outright. Returns 1 in al on the first mismatch,
/// 0 if every watched word matches. Only al is set (the upper bytes of eax
/// keep their incoming value), so the contract compares al (cdecl, no stack
/// words; ecx/esi are scratch).
lf_checker_rt::export!(cdecl, rw_008BF300() -> u32 {
    unsafe {
        /// First word of the snapshot block.
        const SNAPSHOT: u32 = 0x01160E80;
        /// First word of the live block.
        const LIVE: u32 = 0x0116128C;
        /// Indexes compared; all others are skipped.
        const WATCHED: [u32; 7] = [0, 1, 9, 10, 11, 12, 13];
        for idx in WATCHED {
            let a = (lf_checker_rt::relocated(SNAPSHOT + idx * 4) as *const u32)
                .read_unaligned();
            let b = (lf_checker_rt::relocated(LIVE + idx * 4) as *const u32)
                .read_unaligned();
            if a != b {
                return 1;
            }
        }
        0
    }
});
