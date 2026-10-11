// original: 0x00B69D50 reset_emitter_slots

/// Clear the global emitter-list head, then clear the two pointer words in
/// each of twenty fixed-size emitter slots and set the readiness byte. Return
/// the loop's final table cursor, matching the original's EAX value. All
/// absolute data references are relocated against the mapped image base.
lf_checker_rt::export!(cdecl, rw_00B69D50() -> u32 {
    unsafe {
        const HEAD_VA: u32 = 0x0167_0544;
        const FIRST_SLOT_WORD_VA: u32 = 0x0167_0554;
        const SLOT_STRIDE: u32 = 0x60;
        const SLOT_COUNT: u32 = 20;
        const READY_VA: u32 = 0x0104_6ad0;
        const END_CURSOR_VA: u32 = 0x0167_0cd4;

        (lf_checker_rt::relocated(HEAD_VA) as *mut u32).write_volatile(0);
        let first = lf_checker_rt::relocated(FIRST_SLOT_WORD_VA);
        for slot in 0..SLOT_COUNT {
            let second_word = first.wrapping_add(slot.wrapping_mul(SLOT_STRIDE));
            (second_word.wrapping_sub(4) as *mut u32).write_volatile(0);
            (second_word as *mut u32).write_volatile(0);
        }
        (lf_checker_rt::relocated(READY_VA) as *mut u8).write_volatile(1);
        lf_checker_rt::relocated(END_CURSOR_VA)
    }
});
