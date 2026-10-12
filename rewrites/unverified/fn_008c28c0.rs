// original: 0x008C28C0 audio_clear_suspended

/// Lower the three audio-suspended flag bytes.
///
/// Writes 0 to `FLAG_A`, `FLAG_B` and `FLAG_C`. Leaf, cdecl, no arguments;
/// no meaningful return value.
lf_checker_rt::export!(cdecl, rw_008C28C0() -> u32 {
    unsafe {
        const FLAG_A: u32 = 0x0116_18F2;
        const FLAG_B: u32 = 0x0115_DBE4;
        const FLAG_C: u32 = 0x0115_DC14;
        (lf_checker_rt::relocated(FLAG_A) as *mut u8).write_unaligned(0);
        (lf_checker_rt::relocated(FLAG_B) as *mut u8).write_unaligned(0);
        (lf_checker_rt::relocated(FLAG_C) as *mut u8).write_unaligned(0);
        0
    }
});
