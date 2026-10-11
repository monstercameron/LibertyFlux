// original: 0x009cb460 store_timing_window_words

/// Copy four incoming words into distinct timing state slots, copy the current shared sample into a fifth slot, and return that sample.
lf_checker_rt::export!(cdecl, rw_009cb460(first: u32, second: u32, third: u32, fourth: u32) -> eax {
    const SLOT_FIRST_VA: u32 = 0x01295834;
    const SLOT_SECOND_VA: u32 = 0x01295848;
    const SLOT_THIRD_VA: u32 = 0x0129584C;
    const SLOT_FOURTH_VA: u32 = 0x01295850;
    const SAMPLE_VA: u32 = 0x01173608;
    const SLOT_SAMPLE_VA: u32 = 0x0129583C;
    unsafe {
        lf_checker_rt::global::<u32>(SLOT_FIRST_VA).write_unaligned(first);
        lf_checker_rt::global::<u32>(SLOT_SECOND_VA).write_unaligned(second);
        lf_checker_rt::global::<u32>(SLOT_THIRD_VA).write_unaligned(third);
        lf_checker_rt::global::<u32>(SLOT_FOURTH_VA).write_unaligned(fourth);
        let sample = lf_checker_rt::global::<u32>(SAMPLE_VA).read_unaligned();
        lf_checker_rt::global::<u32>(SLOT_SAMPLE_VA).write_unaligned(sample);
        sample
    }
});
