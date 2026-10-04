// original: 0x00b3b330 task_point_in_secondary_bounds (proposed)

/// Test whether the three floats at `point` lie inside the secondary
/// task-bounds box. Returns 0 when the secondary-ready flag byte is clear.
/// Otherwise each component must satisfy `lo <= v <= hi` with the ordered
/// float comparison the original's branch-on-greater implements (an unordered
/// NaN comparison fails the greater test, so NaN passes that bound check).
/// Only the low result byte is behaviour: on the flag-clear path the upper
/// 24 bits of the result are the caller's entry register, which no rewrite
/// can observe, so the contract compares the low byte only. Original:
/// 0x00b3b330 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00b3b330(point: u32) -> u32 {
    unsafe {
        const READY_FLAG: u32 = 0x01662492;
        const FIRST_SLOTS: u32 = 0x016624e0;
        const SECOND_SLOTS: u32 = 0x016624d0;
        if lf_checker_rt::global::<u8>(READY_FLAG).read() == 0 {
            return 0;
        }
        for lane in 0..3u32 {
            let off = lane * 4;
            let lo = f32::from_bits(
                lf_checker_rt::global::<u32>(FIRST_SLOTS + off).read(),
            );
            let hi = f32::from_bits(
                lf_checker_rt::global::<u32>(SECOND_SLOTS + off).read(),
            );
            let v = f32::from_bits(
                ((point + off) as *const u32).read_unaligned(),
            );
            if lo > v || v > hi {
                return 0;
            }
        }
        1
    }
});
