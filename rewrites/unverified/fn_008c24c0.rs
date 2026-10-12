// original: 0x008C24C0 audio_config_reset

/// Reset the audio configuration words to their defaults.
///
/// Writes 1 to the two enable bytes, 0 to the two flag bytes and 15.0
/// (`DEFAULT_LEVEL`, 0x41700000) to the level word. Leaf, cdecl, no
/// arguments; no meaningful return value.
lf_checker_rt::export!(cdecl, rw_008C24C0() -> u32 {
    unsafe {
        const ENABLE_B: u32 = 0x0103_0C69;
        const ENABLE_A: u32 = 0x0103_0C68;
        const FLAG_A: u32 = 0x0116_2530;
        const LEVEL: u32 = 0x0103_0C6C;
        const FLAG_B: u32 = 0x0128_32B4;
        const FLAG_C: u32 = 0x0116_252F;
        const DEFAULT_LEVEL: u32 = 0x4170_0000;
        (lf_checker_rt::relocated(ENABLE_B) as *mut u8).write_unaligned(1);
        (lf_checker_rt::relocated(ENABLE_A) as *mut u8).write_unaligned(1);
        (lf_checker_rt::relocated(FLAG_A) as *mut u8).write_unaligned(0);
        (lf_checker_rt::relocated(LEVEL) as *mut u32).write_unaligned(DEFAULT_LEVEL);
        (lf_checker_rt::relocated(FLAG_B) as *mut u8).write_unaligned(0);
        (lf_checker_rt::relocated(FLAG_C) as *mut u8).write_unaligned(0);
        0
    }
});
