// original: 0x008FC120 text_match_7878
/// Run the word-pattern matcher with expected words (0x78, 0x7a).
///
/// Forwards the string with the two constants and returns the
/// matcher's boolean result. Cdecl, one stack argument.
export!(cdecl, rw_008fc120(s: u32) -> u32 {
    unsafe {
        callee_cdecl!(1, u32, s, 0x78, 0x7a)
    }
});
