// original: 0x00C4A320 camera_spline_update_frame_samples

/// Walk the source spline's frame samples and forward each to the destination.
///
/// The receiver in ECX supplies a signed sample count at byte offset 0x140.
/// The two stack arguments are the source spline and destination sample area.
/// For each index from zero to count minus one, the function asks the source
/// helper for that sample, then calls the orientation-copy helper with the
/// destination slot at `index * 0x30`. Counts at or below zero skip the loop.
/// A final source helper runs in every case and its 32-bit result is returned.
/// All three outgoing calls are routed through checker stand-ins and retain
/// their original order and arguments.
lf_checker_rt::export!(thiscall, rw_c4a320(this: u32, source: u32, destination: u32) -> u32 {
    const SAMPLE_COUNT_OFFSET: u32 = 0x140;
    const SAMPLE_STRIDE: u32 = 0x30;
    const GET_SAMPLE: u32 = 1;
    const COPY_ORIENTATION: u32 = 2;
    const FINALIZE_SOURCE: u32 = 3;

    let sample_count = unsafe {
        (this.wrapping_add(SAMPLE_COUNT_OFFSET) as *const i32).read_unaligned()
    };
    let mut index = 0i32;
    while index < sample_count {
        let sample = lf_checker_rt::callee_thiscall!(GET_SAMPLE, u32, source, index as u32);
        let destination_slot = destination.wrapping_add((index as u32).wrapping_mul(SAMPLE_STRIDE));
        let _ = lf_checker_rt::callee_thiscall!(
            COPY_ORIENTATION,
            u32,
            destination_slot,
            sample,
        );
        index = index.wrapping_add(1);
    }

    lf_checker_rt::callee_thiscall!(FINALIZE_SOURCE, u32, source)
});
