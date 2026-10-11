// original: 0x006036B0 rm_ptfx_manager_set_words_4b0_4b4
/// Store the two incoming 32-bit state words at manager offsets `+0x4b0` and
/// `+0x4b4`. The second word is also returned unchanged; the thiscall entry
/// uses ECX for the manager and the callee removes both stack arguments.
lf_checker_rt::export!(thiscall, rm_ptfx_manager_set_words_4b0_4b4(
    manager: u32,
    first_word: u32,
    second_word: u32
) -> u32 {
    const FIRST_STATE_WORD: u32 = 0x4b0;
    const SECOND_STATE_WORD: u32 = 0x4b4;
    unsafe {
        core::ptr::write_volatile(
            manager.wrapping_add(FIRST_STATE_WORD) as *mut u32,
            first_word,
        );
        core::ptr::write_volatile(
            manager.wrapping_add(SECOND_STATE_WORD) as *mut u32,
            second_word,
        );
    }
    second_word
});
