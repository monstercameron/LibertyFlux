// original: 0x009B0A80 audPoliceScanner::pick_report_voices

/// Resolves two voice choices from a report record. It calls the same voice
/// selector with the record's first voice descriptor and a second descriptor
/// chosen by the count byte, writes each returned word through its output
/// pointer, and returns the second selector result.
lf_checker_rt::export!(thiscall, rw_009b0a80(this: u32, report: u32, first_out: u32, second_out: u32) -> u32 {
    const VOICE_COUNT: u32 = 0x0a;
    const FIRST_VOICE: u32 = 0x0b;
    const VOICE_STRIDE: u32 = 8;
    const SECOND_VOICE: u32 = 0x0b;
    const SECOND_DATA: u32 = 0x0c;

    unsafe {
        let first_count = ((report + VOICE_COUNT) as *const u8).read() as u32;
        let first_descriptor = report.wrapping_add(FIRST_VOICE);
        let first_result = lf_checker_rt::callee_thiscall!(1, u32, this, first_descriptor, first_count);
        (first_out as *mut u32).write_unaligned(first_result);

        let second_row = report.wrapping_add(first_count.wrapping_mul(VOICE_STRIDE));
        let second_count = ((second_row + SECOND_VOICE) as *const u8).read() as u32;
        let second_descriptor = second_row.wrapping_add(SECOND_DATA);
        let second_result = lf_checker_rt::callee_thiscall!(1, u32, this, second_descriptor, second_count);
        (second_out as *mut u32).write_unaligned(second_result);
        second_result
    }
});
