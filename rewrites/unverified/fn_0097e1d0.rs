// original: 0x0097E1D0 ped_say_taxi_hail
/// Emits a taxi-hail audio event and asks the ped speech subsystem to speak it.
///
/// The cdecl inputs are forwarded as the audio-event parameter and ped entity
/// base. The routine calls the entity's audio-event helper, then makes a
/// ten-word thiscall to the ped script-audio subobject with the taxi-hail text,
/// fixed event options, and a one-valued float represented as its 32-bit bits.
const PED_AUDIO_OFFSET: u32 = 0x3C0;
const PED_SCRIPT_AUDIO_OFFSET: u32 = 0x570;
const TAXI_HAIL_TEXT_VA: u32 = 0x00E8_C448;
const TAXI_HAIL_GAIN_BITS: u32 = 0x3F80_0000;
const TAXI_HAIL_EVENT_ID: u32 = 0x2BC;
const EMIT_FULL_CALLEE: u32 = 1;
const PED_SPEECH_CALLEE: u32 = 2;

export!(cdecl, rw_0097e1d0(event_parameter: u32, ped_entity: u32) -> () {
    let ped_audio = ped_entity.wrapping_add(PED_AUDIO_OFFSET);
    let _event_result = lf_checker_rt::callee_thiscall!(
        EMIT_FULL_CALLEE,
        u32,
        ped_audio,
        event_parameter,
    );

    let ped_script_audio = ped_entity.wrapping_add(PED_SCRIPT_AUDIO_OFFSET);
    let _speech_result = lf_checker_rt::callee_thiscall!(
        PED_SPEECH_CALLEE,
        u32,
        ped_script_audio,
        lf_checker_rt::relocated(TAXI_HAIL_TEXT_VA),
        0u32,
        0u32,
        0u32,
        TAXI_HAIL_EVENT_ID,
        0u32,
        0u32,
        TAXI_HAIL_GAIN_BITS,
        0u32,
        0u32,
    );
});
