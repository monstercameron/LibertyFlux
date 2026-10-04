// original: 0x00a1d910 cam_mode_constant (proposed)

/// Selects a tuning constant by mode, with a special case for mode 2.
///
/// `this` points to a record with a mode word at `+MODE_OFF`. Mode 1
/// selects `K_MODE1` (-0.165), mode 3 selects `K_MODE3` (-0.07), mode 2
/// selects 0.0 when the argument equals 1 and `K_MODE2` (-0.1) otherwise,
/// and any other mode selects 0.0. The result is returned on the x87
/// register stack (`st0`).
///
/// Original: 0x00a1d910 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a1d910(this: u32, arg: u32) -> f32 {
    unsafe {
        const MODE_OFF: u32 = 0x360;
        const K_MODE1: f32 = f32::from_bits(0xbe28_f5c3); // -0.165
        const K_MODE2: f32 = f32::from_bits(0xbdcc_cccd); // -0.1
        const K_MODE3: f32 = f32::from_bits(0xbd8f_5c29); // -0.07
        match ((this + MODE_OFF) as *const u32).read_unaligned() {
            1 => K_MODE1,
            2 => {
                if arg == 1 {
                    0.0
                } else {
                    K_MODE2
                }
            }
            3 => K_MODE3,
            _ => 0.0,
        }
    }
});
