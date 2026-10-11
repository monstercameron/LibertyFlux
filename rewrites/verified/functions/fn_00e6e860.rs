// original: 0x00E6E860 mainloop_timing_state_reset

/// Reset the mainloop timing state block: set its first word to one, clear
/// the next byte flag, zero the following two words, set the next two words
/// to all bits one, and clear the final byte flag. The function takes no
/// arguments and has no semantic return value.
lf_checker_rt::export!(cdecl, rw_00e6e860() -> () {
    const STATE_WORD_0_VA: u32 = 0x018e_509c;
    const STATE_BYTE_1_VA: u32 = 0x018e_50a0;
    const STATE_WORD_2_VA: u32 = 0x018e_50a4;
    const STATE_WORD_3_VA: u32 = 0x018e_50a8;
    const STATE_WORD_4_VA: u32 = 0x018e_50ac;
    const STATE_WORD_5_VA: u32 = 0x018e_50b0;
    const STATE_BYTE_6_VA: u32 = 0x018e_50b4;

    unsafe {
        lf_checker_rt::global::<u32>(STATE_WORD_0_VA).write(1);
        lf_checker_rt::global::<u8>(STATE_BYTE_1_VA).write(0);
        lf_checker_rt::global::<u32>(STATE_WORD_2_VA).write(0);
        lf_checker_rt::global::<u32>(STATE_WORD_3_VA).write(0);
        lf_checker_rt::global::<u32>(STATE_WORD_4_VA).write(u32::MAX);
        lf_checker_rt::global::<u32>(STATE_WORD_5_VA).write(u32::MAX);
        lf_checker_rt::global::<u8>(STATE_BYTE_6_VA).write(0);
    }
});
