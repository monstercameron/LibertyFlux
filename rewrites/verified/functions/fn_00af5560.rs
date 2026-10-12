// original: 0x00AF5560 pad_has_active_input

/// Return whether the pad state has any active input. The object begins with
/// a pointer to two state masks: a bit is active when it is set in the first
/// mask and differs between the masks. Four additional object words are
/// checked for nonzero action values. The result is consumed from AL.
lf_checker_rt::export!(thiscall, rw_00af5560(controller: u32) -> u32 {
    const STATE_POINTER: u32 = 0x00;
    const ACTION_WORDS: [u32; 4] = [0x04, 0x0C, 0x08, 0x10];
    const STATE_CURRENT: u32 = 0x04;
    const STATE_PREVIOUS: u32 = 0x08;

    unsafe {
        let state = (controller.wrapping_add(STATE_POINTER) as *const u32).read_unaligned();
        let current = (state.wrapping_add(STATE_CURRENT) as *const u32).read_unaligned();
        let previous = (state.wrapping_add(STATE_PREVIOUS) as *const u32).read_unaligned();
        let changed_active_bit = current & (current ^ previous) != 0;
        let action_word_nonzero = ACTION_WORDS.iter().any(|offset| {
            (controller.wrapping_add(*offset) as *const u32).read_unaligned() != 0
        });
        u32::from(changed_active_bit || action_word_nonzero)
    }
});
