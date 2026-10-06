// original: 0x009bba00 clear_input_state_words (proposed)

/// Clear the two input-state words to zero.
///
/// Writes 0 to each of the two global words at 0x0128E944 and 0x0128E948.
/// Returns nothing (the original leaves EAX untouched).
///
/// Edge cases: none; unconditional stores.
///
/// Original: no arguments, no return channel.
lf_checker_rt::export!(cdecl, rw_009bba00() -> u32 {
    unsafe {
        const WORD0: u32 = 0x0128e944;
        const WORD1: u32 = 0x0128e948;
        lf_checker_rt::global::<u32>(WORD0).write_unaligned(0);
        lf_checker_rt::global::<u32>(WORD1).write_unaligned(0);
        0
    }
});
