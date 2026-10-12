// original: 0x00880BA0 audio_format_keys_equal

/// Compare the first format-key pointer with the sixteen bytes stored at the
/// fourth stack argument.
///
/// The reference key is copied before the first pointer is checked. A null
/// first pointer returns true without reading through it. Otherwise all four
/// 32-bit words are compared in order, returning false at the first mismatch
/// and true when the complete keys match. The stdcall routine consumes four
/// stack arguments; the second and third arguments do not affect the result.
lf_checker_rt::export!(stdcall, rw_00880ba0(first_key: u32, _arg2: u32, _arg3: u32, reference_key: u32) -> u32 {
    unsafe {
        const KEY_BYTES: u32 = 16;
        const KEY_WORD_BYTES: u32 = 4;
        const KEY_WORDS: u32 = KEY_BYTES / KEY_WORD_BYTES;

        let mut reference_words = [0u32; KEY_WORDS as usize];
        for word_index in 0..KEY_WORDS {
            reference_words[word_index as usize] = (reference_key
                .wrapping_add(word_index.wrapping_mul(KEY_WORD_BYTES))
                as *const u32)
                .read_unaligned();
        }

        if first_key == 0 {
            return 1;
        }

        for word_index in 0..KEY_WORDS {
            let first_word = (first_key
                .wrapping_add(word_index.wrapping_mul(KEY_WORD_BYTES))
                as *const u32)
                .read_unaligned();
            if first_word != reference_words[word_index as usize] {
                return 0;
            }
        }

        1
    }
});
