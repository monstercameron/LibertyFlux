// original: 0x008C23A0 audio_set_suspended

/// Raise the three audio-suspended flag bytes.
///
/// Writes 1 to `FLAG_A`, `FLAG_B` and `FLAG_C`. Leaf, cdecl, no arguments;
/// no meaningful return value.
lf_checker_rt::export!(cdecl, rw_008C23A0() -> u32 {
    unsafe {
        const FLAG_A: u32 = 0x0116_18F2;
        const FLAG_B: u32 = 0x0115_DBE4;
        const FLAG_C: u32 = 0x0115_DC14;
        (lf_checker_rt::relocated(FLAG_A) as *mut u8).write_unaligned(1);
        (lf_checker_rt::relocated(FLAG_B) as *mut u8).write_unaligned(1);
        (lf_checker_rt::relocated(FLAG_C) as *mut u8).write_unaligned(1);
        0
    }
});
