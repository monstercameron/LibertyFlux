// original: 0x00974D60 SOURCE_ENVIRONMENT_TO_REVERB_SIZE
/// Reset the source-environment to reverb-size mapping to its defaults.
///
/// Zeroes the environment slots, installs the default curve constants,
/// re-registers the eight reverb presets through callee 1, then installs
/// the default bus gains. Takes no inputs; the observable behaviour is the
/// global writes and the eight calls.
export!(cdecl, rw_00974D60() -> u32 {
    unsafe {
        // Environment slots start cleared.
        for addr in [0x0121_F9A9u32, 0x0121_F694, 0x0121_F695, 0x0121_F696] {
            *(global::<u8>(addr)) = 0;
        }
        // Default curve constants (mix of integer ids and float cutoffs).
        for (addr, val) in [
            (0x0121_F680u32, 0x0000_0000u32),
            (0x0121_F684, 0x0000_0000),
            (0x0121_F688, 0x0000_0000),
            (0x0121_F68C, 0x0000_0000),
            (0x0103_8840, 0x0000_000F),
            (0x0121_F690, 0x0000_0000),
            (0x0121_F698, 0x0000_0000),
            (0x0121_F69C, 0x0000_0000),
            (0x0103_8844, 0x3E80_0000), // 0.25f32
            (0x0103_8848, 0x3DCC_CCCD), // 0.1f32
            (0x0103_884C, 0x3E4C_CCCD), // 0.2f32
            (0x0103_8850, 0x3EB8_51EC),
            (0x0121_F6A0, 0x0000_0000),
            (0x0103_8854, 0x0000_0019),
        ] {
            *(global::<u32>(addr)) = val;
        }
        // Re-register the eight reverb presets: (slot object, preset data).
        let mut answer = 0;
        for (slot, preset) in [
            (0x0121_FA10u32, 0x00E8_B944u32),
            (0x0121_FA60, 0x00E8_B968),
            (0x0121_F9D4, 0x00E8_B98C),
            (0x0121_FA38, 0x00E8_B9B4),
            (0x0121_FAD8, 0x00E8_B9D8),
            (0x0121_F9AC, 0x00E8_BA00),
            (0x0121_FAB0, 0x00E8_BA2C),
            (0x0121_FA88, 0x00E8_BA5C),
        ] {
            answer = callee_thiscall!(1, u32, relocated(slot), relocated(preset));
        }
        // Default bus gains.
        for (addr, val) in [
            (0x0121_F978u32, 0x3F80_0000u32), // 1.0f32
            (0x0121_F97C, 0x3F34_FDF4),
            (0x0121_F980, 0x3F7D_70A4),
            (0x0121_F984, 0x3F66_6666),
            (0x0121_F988, 0x3F80_0000),
            (0x0121_F98C, 0x3F80_0000),
            (0x0121_F990, 0x3F80_0000),
            (0x0121_F994, 0x4220_0000), // 40.0f32
            (0x0121_F998, 0x40A0_0000), // 5.0f32
            (0x0121_F99C, 0x0000_0000),
            (0x0121_F9A0, 0xC1A0_0000), // -20.0f32
            (0x0121_F9A4, 0xBF80_0000), // -1.0f32
        ] {
            *(global::<u32>(addr)) = val;
        }
        answer
    }
});
