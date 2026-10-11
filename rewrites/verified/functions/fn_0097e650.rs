// original: 0x0097E650 DRINK

/// Sends a fixed speech-task label for DRINK to the shared ped audio helper.
///
/// The first incoming cdecl word is unused. `ped_owner`, the second
/// incoming word, points to a `CPed`-like object with its speech
/// entity at byte offset 0x570. The helper receives that entity in
/// ECX and ten fixed stack words, including the relocated label
/// pointer and the bit pattern for 1.0. Its EAX result is returned.
/// The wrapper writes no memory and uses caller-cleaned cdecl input.
lf_checker_rt::export!(cdecl, rw_0097E650(_unused: u32, ped_owner: u32) -> u32 {
    const SPEECH_ENTITY_OFFSET: u32 = 0x570;
    const AUDIO_HELPER_ID: u32 = 1;
    const LABEL_VA: u32 = 0x00E8C3FC;
    const SPEECH_WEIGHT: u32 = 0x3F800000;
    let speech_entity = ped_owner.wrapping_add(SPEECH_ENTITY_OFFSET);
    let label = lf_checker_rt::relocated(LABEL_VA);
    lf_checker_rt::callee_thiscall!(AUDIO_HELPER_ID, u32, speech_entity, label, 0, 0, 0, u32::MAX, 0, 0, SPEECH_WEIGHT, 0, 0)
});
