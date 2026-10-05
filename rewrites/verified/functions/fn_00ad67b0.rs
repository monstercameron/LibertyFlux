// original: 0x00AD67B0 audio_reset_state_words (proposed)

/// Reset the audio state words and the 72-entry word table to zero.
///
/// Zeroes 0x48 words at the table base, then five scattered global words
/// (cdecl/0, no arguments). Returns 0.
lf_checker_rt::export!(cdecl, rw_00ad67b0() -> u32 {
    unsafe {
        const TABLE: u32 = 0x0154E358;
        const WORDS: usize = 0x48;
        for i in 0..WORDS {
            lf_checker_rt::global::<u32>(TABLE).add(i).write(0);
        }
        lf_checker_rt::global::<u32>(0x0154E300).write(0);
        lf_checker_rt::global::<u32>(0x01550EAC).write(0);
        lf_checker_rt::global::<u32>(0x01552C60).write(0);
        lf_checker_rt::global::<u32>(0x0154E304).write(0);
        lf_checker_rt::global::<u32>(0x0154EC48).write(0);
        0
    }
});
