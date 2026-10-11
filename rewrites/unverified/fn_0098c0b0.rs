// original: 0x0098C0B0 audio_vehicle_mix_level_get

/// Selects the returned mix level from the cached voice state, optional
/// vehicle configuration, and three global float values. Voice state zero
/// uses the configured special value only for configuration mode one with
/// the global gate enabled; state one selects the second global value, state
/// zero otherwise selects the fallback, and other nonzero states return 0.
/// With no cached voice, configuration mode four selects the fallback.
lf_checker_rt::export!(thiscall, rw_audio_vehicle_mix_level_get(this: u32) -> f32 {
    const CACHED_VOICE: u32 = 0x9E0;
    const VEHICLE_CONFIG: u32 = 0x820;
    const VOICE_STATE: u32 = 0x13E;
    const CONFIG_MODE: u32 = 0x1304;
    const SPECIAL_MODE: u32 = 1;
    const CONFIG_FALLBACK_MODE: u32 = 4;
    const GLOBAL_GATE: u32 = 0x11D6_FD0;
    const LEVEL_WHEN_GATED: u32 = 0x1038_CC0;
    const LEVEL_FOR_STATE_ONE: u32 = 0x1284_240;
    const FALLBACK_LEVEL: u32 = 0x1038_CBC;

    unsafe {
        let voice = (this.wrapping_add(CACHED_VOICE) as *const u32).read_unaligned();
        let config = (this.wrapping_add(VEHICLE_CONFIG) as *const u32).read_unaligned();

        if voice != 0 {
            let state = (voice.wrapping_add(VOICE_STATE) as *const u8).read();
            if state == 0 {
                if config != 0 {
                    let mode = (config.wrapping_add(CONFIG_MODE) as *const u32).read_unaligned();
                    let gate = lf_checker_rt::global::<u32>(GLOBAL_GATE).read_unaligned();
                    if mode == SPECIAL_MODE && gate == 1 {
                        let bits = lf_checker_rt::global::<u32>(LEVEL_WHEN_GATED).read_unaligned();
                        return f32::from_bits(bits);
                    }
                }
                let bits = lf_checker_rt::global::<u32>(FALLBACK_LEVEL).read_unaligned();
                return f32::from_bits(bits);
            }
            if state == 1 {
                let bits = lf_checker_rt::global::<u32>(LEVEL_FOR_STATE_ONE).read_unaligned();
                return f32::from_bits(bits);
            }
            return 0.0;
        }

        if config != 0 {
            let mode = (config.wrapping_add(CONFIG_MODE) as *const u32).read_unaligned();
            if mode == CONFIG_FALLBACK_MODE {
                let bits = lf_checker_rt::global::<u32>(FALLBACK_LEVEL).read_unaligned();
                return f32::from_bits(bits);
            }
        }
        0.0
    }
});
