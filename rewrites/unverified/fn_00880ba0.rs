// original: 0x00880BA0 audio_format_keys_equal

/// Compare the four 32-bit words of the left and right audio-format keys.
///
/// The first and fourth stack arguments are the key addresses. The two middle
/// words are part of the original ABI but are not read. The comparison is
/// bitwise and returns one for equal keys, zero otherwise.
lf_checker_rt::export!(stdcall, rw_00880ba0(left: u32, _unused_one: u32, _unused_two: u32, right: u32) -> u32 {
    unsafe {
        const KEY_WORDS: usize = 4;
        let mut word = 0;
        while word < KEY_WORDS {
            let lhs = (left as *const u32).add(word).read_unaligned();
            let rhs = (right as *const u32).add(word).read_unaligned();
            if lhs != rhs { return 0; }
            word += 1;
        }
        1
    }
});
