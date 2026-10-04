// original: 0x00b78b20 channel_allocate (proposed)

/// Allocate the first free channel and store its value.
///
/// Scans the 64 channel-used bytes for the first zero. When every channel
/// is taken returns `0xFFFF_FFFF`. Otherwise marks the channel used,
/// stores `value` in its float slot and returns the channel index.
///
/// Original: cdecl with one stack word (a float), plain `ret`.
lf_checker_rt::export!(cdecl, rw_00b78b20(value: f32) -> u32 {
    unsafe {
        const USED: u32 = 0x0167CCE0;
        const SLOTS: u32 = 0x0167CDA0;
        const CHANNELS: u32 = 64;
        const NONE: u32 = 0xFFFF_FFFF;
        let used = lf_checker_rt::relocated(USED);
        let slots = lf_checker_rt::relocated(SLOTS);
        let mut i = 0u32;
        while i < CHANNELS {
            if ((used + i) as *const u8).read() == 0 {
                ((used + i) as *mut u8).write(1);
                ((slots + i * 4) as *mut u32).write_unaligned(value.to_bits());
                return i;
            }
            i += 1;
        }
        NONE
    }
});
