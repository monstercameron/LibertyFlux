// original: 0x00b78dd0 channels_reset (proposed)

/// Reset all 64 channels to the unused state.
///
/// Clears every channel-used byte, sets every channel tag word to 1 (the
/// original loads the tag once, outside the loop, so it is the same for all
/// channels) and every channel float slot to -1.0. Returns 64, the channel
/// count left in the counter by the loop. The range check on entry can never
/// fail and its trap branch is never taken.
///
/// Original: cdecl with no stack words, plain `ret`.
lf_checker_rt::export!(cdecl, rw_00b78dd0() -> u32 {
    unsafe {
        const USED: u32 = 0x0167CCE0;
        const TAGS: u32 = 0x0167CD20;
        const SLOTS: u32 = 0x0167CDA0;
        const CHANNELS: u32 = 64;
        const EMPTY_VALUE: u32 = 0xBF80_0000;
        let used = lf_checker_rt::relocated(USED);
        let tags = lf_checker_rt::relocated(TAGS);
        let slots = lf_checker_rt::relocated(SLOTS);
        let mut i = 0u32;
        while i < CHANNELS {
            ((used + i) as *mut u8).write(0);
            ((tags + i * 2) as *mut u16).write_unaligned(1);
            ((slots + i * 4) as *mut u32).write_unaligned(EMPTY_VALUE);
            i += 1;
        }
        CHANNELS
    }
});
