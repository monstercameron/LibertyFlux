// original: 0x008CA660 stream_stats_clear (proposed)

/// Clears the streaming statistics block: zeroes the dword at `BASE`, the
/// bytes at `+0x04`, `+0x05` and `+0x07` (the byte at `+0x06` is deliberately
/// left untouched), and the dwords at `+0x08`, `+0x0C` and `+0x10`.
///
/// No arguments (cdecl); no return value.
lf_checker_rt::export!(cdecl, rw_008CA660() -> u32 {
    unsafe {
        /// Base of the statistics block.
        const BASE: u32 = 0x116D278;
        let base = lf_checker_rt::relocated(BASE);
        (base as *mut u32).write(0);
        ((base + 0x04) as *mut u8).write(0);
        ((base + 0x05) as *mut u8).write(0);
        ((base + 0x07) as *mut u8).write(0);
        ((base + 0x08) as *mut u32).write(0);
        ((base + 0x0C) as *mut u32).write(0);
        ((base + 0x10) as *mut u32).write(0);
        0
    }
});
