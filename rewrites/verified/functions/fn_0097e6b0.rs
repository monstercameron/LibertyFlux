// original: 0x0097E6B0 GRUNT_CLIMB_EASY_UP

/// Runs the guarded GRUNT_CLIMB_EASY_UP response for a ped.
///
/// The first incoming cdecl word is unused. The second is treated as
/// the ped object. When its byte fields at `+0x218` and `+0x219` pass
/// the player-state guard, a no-stack-argument helper is called with
/// the ped in ECX. A zero low byte continues to the speech helper,
/// which receives the embedded entity at `+0x570`, a relocated label,
/// a runtime word and fixed stack values. Both incoming stack words
/// remain caller-cleaned; the function has no semantic return value.
lf_checker_rt::export!(cdecl, rw_0097E6B0(_unused: u32, ped_owner: u32) -> u32 {
    const PED_PLAYER_INDEX: u32 = 0x218;
    const PED_IS_PLAYER: u32 = 0x219;
    const PED_SPEECH_ENTITY: u32 = 0x570;
    const GATE_HELPER_ID: u32 = 1;
    const AUDIO_HELPER_ID: u32 = 2;
    const LABEL_VA: u32 = 0x00E8C404;
    const RUNTIME_VALUE_VA: u32 = 0x01284394;
    const ONE_BITS: u32 = 0x3F800000;
    #[inline(always)]
    fn read_u8(address: u32) -> u8 { unsafe { (address as *const u8).read() } }
    let ped = ped_owner;
    if read_u8(ped + PED_PLAYER_INDEX) != 0 || read_u8(ped + PED_IS_PLAYER) == 0 { return 0; }
    let gate_result = lf_checker_rt::callee_thiscall!(GATE_HELPER_ID, u32, ped);
    if gate_result as u8 != 0 { return 0; }
    let speech_entity = ped.wrapping_add(PED_SPEECH_ENTITY);
    let label = lf_checker_rt::relocated(LABEL_VA);
    let runtime_value = unsafe { lf_checker_rt::global::<u32>(RUNTIME_VALUE_VA).read() };
    let _ = lf_checker_rt::callee_thiscall!(
        AUDIO_HELPER_ID, u32, speech_entity, label, 1, 1, runtime_value,
        u32::MAX, 0, 0, ONE_BITS, 0, 0
    );
    0
});
