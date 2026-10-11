// original: 0x00a25940 camera_mode_angle_value

/// Return the binary32 angle selected by the mode byte at offset 0x214: the
/// zero mode selects the first fixed value, mode one the second, and every
/// other byte returns positive zero.
lf_checker_rt::export!(thiscall, rw_00a25940(this: u32) -> f32 {
    const MODE: u32 = 0x214;
    const MODE_ZERO: u32 = 0x3fc9_0fdb;
    const MODE_ONE: u32 = 0x3f49_0fdb;
    let mode = unsafe { (this.wrapping_add(MODE) as *const u8).read() };
    f32::from_bits(match mode {
        0 => MODE_ZERO,
        1 => MODE_ONE,
        _ => 0,
    })
});
