// original: 0x00AE9150 initialize_frame_timer_baselines

/// Resets two frame-timer work values, samples the two shared clocks, and
/// stores the coarse sample in the frame start and end baselines. It then
/// clears the elapsed-timer and frame-timer scratch globals. Each sample is
/// obtained from a separate cdecl callback.
///
/// Calling convention: cdecl with no arguments and no return value.
lf_checker_rt::export!(cdecl, rw_00ae9150() -> () {
    unsafe {
        const TIMER_FLAG_VA: u32 = 0x015b_2b84;
        const FRAME_START_VA: u32 = 0x0159_3bc4;
        const FRAME_END_VA: u32 = 0x0159_3bcc;
        const BASELINE_FIRST_VA: u32 = 0x015b_2b78;
        const BASELINE_SECOND_VA: u32 = 0x015b_2b7c;
        const ELAPSED_WORK_VA: u32 = 0x015b_0e80;
        const FRAME_WORK_VA: u32 = 0x015b_0e54;
        const COARSE_REFRESH_CALLEE: u32 = 1;
        const FRAME_SAMPLE_CALLEE: u32 = 2;
        const ELAPSED_SAMPLE_CALLEE: u32 = 3;

        lf_checker_rt::callee_cdecl!(COARSE_REFRESH_CALLEE, u32,);
        lf_checker_rt::global::<u32>(TIMER_FLAG_VA).write_unaligned(0);

        let frame_sample = lf_checker_rt::callee_cdecl!(FRAME_SAMPLE_CALLEE, u32,);
        lf_checker_rt::global::<u32>(FRAME_START_VA).write_unaligned(frame_sample);
        lf_checker_rt::global::<u32>(BASELINE_FIRST_VA).write_unaligned(frame_sample);
        lf_checker_rt::global::<u32>(BASELINE_SECOND_VA).write_unaligned(frame_sample);

        let elapsed_sample = lf_checker_rt::callee_cdecl!(ELAPSED_SAMPLE_CALLEE, u32,);
        lf_checker_rt::global::<u32>(FRAME_END_VA).write_unaligned(elapsed_sample);
        lf_checker_rt::global::<u32>(ELAPSED_WORK_VA).write_unaligned(0);
        lf_checker_rt::global::<u32>(FRAME_WORK_VA).write_unaligned(0);
    }
});
