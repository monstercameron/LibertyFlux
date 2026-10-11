// original: 0x00612A50 physics_sum_track_weights

/// Sum the weight at +4 of each track pointer in the selected byte range,
/// starting at the byte index `this + 0x10e` and counting `this + 0x10f`
/// entries. An empty range returns the relocated threshold constant; a
/// nonempty range returns the greater of the sum and that threshold, with
/// unordered comparisons selecting the sum. The pointers are a 32-bit game
/// array reached from the stack argument's +0xd4 field.
lf_checker_rt::export!(thiscall, rw_00612A50(this: u32, track_container: u32) -> core::arch::x86::__m128 {
    #[inline(always)]
    fn add_ordered(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }
    unsafe {
        const START_INDEX: u32 = 0x10e;
        const TRACK_COUNT: u32 = 0x10f;
        const TRACKS_FIELD: u32 = 0xd4;
        const TRACK_WEIGHT: u32 = 4;
        const THRESHOLD_VA: u32 = 0x00fe_86b4;
        let start = ((this + START_INDEX) as *const u8).read() as u32;
        let count = ((this + TRACK_COUNT) as *const u8).read() as u32;
        let mut sum = 0.0f32;
        if count != 0 {
            let tracks = (((track_container + TRACKS_FIELD) as *const u32).read_unaligned());
            for step in 0..count {
                let track = ((tracks + (start + step) * 4) as *const u32).read_unaligned();
                let weight = ((track + TRACK_WEIGHT) as *const f32).read_unaligned();
                sum = add_ordered(sum, weight);
            }
        }
        let threshold = lf_checker_rt::global::<f32>(THRESHOLD_VA).read_unaligned();
        let result = if threshold > sum { threshold } else { sum };
        core::arch::x86::_mm_set_ss(result)
    }
});
