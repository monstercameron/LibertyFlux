// original: 0x005D30A0 search_state_reset

/// Clear the nine discontiguous words that hold vehicle search state.
///
/// The routine has no arguments, returns the relocated address of the first
/// word, and writes zero to the nine 32-bit globals at the declared offsets.
/// The gaps between these words belong to other state and are left untouched.
lf_checker_rt::export!(cdecl, rw_005d30a0() -> u32 {
    unsafe {
        const SEARCH_STATE_WORDS: [u32; 9] = [
            0x019E8610, 0x019E8614, 0x019E8620, 0x019E8624, 0x019E8628,
            0x019E8630, 0x019E8634, 0x019E8638, 0x019E8640,
        ];

        for file_va in SEARCH_STATE_WORDS {
            lf_checker_rt::global::<u32>(file_va).write_unaligned(0);
        }
        lf_checker_rt::relocated(SEARCH_STATE_WORDS[0])
    }
});
