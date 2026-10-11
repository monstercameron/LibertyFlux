// original: 0x008972A0 aud_environment_sound_set_pair

/// Store a full word at the audio object's parameter slot and the low half of
/// the second argument at its adjacent configuration slot. The original is a
/// thiscall routine with two stack arguments and callee cleanup. Its EAX result
/// combines the upper half of the first argument with the low half of the
/// second argument.
export!(thiscall, rw_008972a0(audio: u32, parameter_word: u32, configuration_word: u32) -> u32 {
    unsafe {
        const PARAMETER_WORD: u32 = 0xD4;
        const CONFIGURATION_WORD: u32 = 0xE6;

        ((audio + PARAMETER_WORD) as *mut u32).write_unaligned(parameter_word);
        ((audio + CONFIGURATION_WORD) as *mut u16)
            .write_unaligned(configuration_word as u16);

        (parameter_word & 0xFFFF_0000) | (configuration_word & 0xFFFF)
    }
});
