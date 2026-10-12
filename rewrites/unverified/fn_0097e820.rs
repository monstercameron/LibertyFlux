// original: 0x0097E820 JEERING

/// Plays the jeering voice response for a ped.
///
/// The second stack argument is the ped pointer; the first is unused. A
/// quarter-second voice probe decides whether to request the first response.
/// If that request reports success, the function returns. Otherwise it asks
/// the audio helper for a fallback value and sends a second response request.
/// The audio entity is the ped subobject at `+0x570`; response labels are
/// relocated static strings, and the helper answers arrive through checker
/// stand-ins so this rewrite covers the caller's decisions and call ordering.
lf_checker_rt::export!(thiscall, rw_0097E820(this: u32, _unused: u32, ped: u32) -> () {
    const PED_AUDIO_ENTITY: u32 = 0x570;
    const QUARTER_SECOND_BITS: u32 = 0x3E800000;
    const UNIT_VALUE_BITS: u32 = 0x3F800000;
    const NEGATIVE_ONE: u32 = 0xFFFFFFFF;
    const VOICE_PROBE_CALL: u32 = 1;
    const FIRST_RESPONSE_CALL: u32 = 2;
    const FALLBACK_VALUE_CALL: u32 = 3;
    const RETRY_RESPONSE_CALL: u32 = 4;
    const FIRST_LABEL_VA: u32 = 0x00E8C3E0;
    const FALLBACK_LABEL_VA: u32 = 0x00E8C3E8;
    const RETRY_LABEL_VA: u32 = 0x00E8C3F4;
    let voice_available = lf_checker_rt::callee_cdecl!(VOICE_PROBE_CALL, u8, QUARTER_SECOND_BITS);
    if voice_available != 0 {
        let audio_entity = ped.wrapping_add(PED_AUDIO_ENTITY);
        let requested = lf_checker_rt::callee_thiscall!(
            FIRST_RESPONSE_CALL, u8, audio_entity,
            lf_checker_rt::relocated(FIRST_LABEL_VA), 0, 1, 0, NEGATIVE_ONE,
            0, 0, UNIT_VALUE_BITS, this, 0,
        );
        if requested != 0 { return; }
    }
    let fallback_value = lf_checker_rt::callee_cdecl!(
        FALLBACK_VALUE_CALL, u32, lf_checker_rt::relocated(FALLBACK_LABEL_VA), 0,
    );
    let audio_entity = ped.wrapping_add(PED_AUDIO_ENTITY);
    let _ = lf_checker_rt::callee_thiscall!(
        RETRY_RESPONSE_CALL, u8, audio_entity,
        lf_checker_rt::relocated(RETRY_LABEL_VA), 0, 1, fallback_value,
        NEGATIVE_ONE, 0, 0, UNIT_VALUE_BITS, this, 0,
    );
});
