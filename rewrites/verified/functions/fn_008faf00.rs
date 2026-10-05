// original: 0x008FAF00 stream_mode_word_select
/// Select one of two streaming mode words by a flag byte.
///
/// Returns the word at the first global when the low byte of the
/// argument is nonzero, else the word at the second global. Cdecl,
/// one stack argument.
export!(cdecl, rw_008faf00(flag: u32) -> u32 {
    unsafe {
        const WORD_IF_SET: u32 = 0x118e7c4;
        const WORD_IF_CLEAR: u32 = 0x118e7cc;
        if (flag as u8) != 0 {
            *global::<u32>(WORD_IF_SET)
        } else {
            *global::<u32>(WORD_IF_CLEAR)
        }
    }
});
