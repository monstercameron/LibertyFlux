// original: 0x0060A860 physics_sum_unmasked_track_channels

/// Sum one channel from each selected track in the parameter object's list.
/// Tracks whose index bit is set in the limits container's bitmap are skipped. Otherwise the +4
/// value is added when the corresponding limit is positive or unordered, and
/// the track's +8 value is added when its limit is nonpositive and its +0x94
/// flag is nonzero. The scalar sum is returned in XMM0; each addition keeps
/// the original left-to-right order. This is a 32-bit thiscall with two
/// pointer arguments on the stack.
lf_checker_rt::export!(thiscall, rw_0060A860(this: u32, parameter: u32, limits_container: u32) -> core::arch::x86::__m128 {
    #[inline(always)]
    fn add_ordered(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }
    unsafe {
        const BITMAP_HOLDER: u32 = 0x0c;
        const PARAMETER_TRACKS: u32 = 0xd4;
        const TRACK_COUNT: u32 = 0x1f3;
        const TRACK_INDEX: u32 = 0x0c;
        const CHANNEL_FLAG: u32 = 0x94;
        const POSITIVE_CHANNEL: u32 = 4;
        const ALTERNATE_CHANNEL: u32 = 8;

        let bitmap_holder = ((limits_container + BITMAP_HOLDER) as *const u32).read_unaligned();
        let bitmap_words = (bitmap_holder as *const u32).read_unaligned();
        let count = ((parameter + TRACK_COUNT) as *const u8).read() as u32;
        let track_array = ((parameter + PARAMETER_TRACKS) as *const u32).read_unaligned();
        let limit_array = ((limits_container + 4) as *const u32).read_unaligned();
        let mut total = 0.0f32;
        for position in 0..count {
            let track = ((track_array + position * 4) as *const u32).read_unaligned();
            let index = ((track + TRACK_INDEX) as *const u8).read() as u32;
            let bitmap_word = ((bitmap_words + (index >> 5) * 4) as *const u32).read_unaligned();
            if bitmap_word & (1u32 << (index & 31)) != 0 {
                continue;
            }
            let limit = ((limit_array + index * 4) as *const f32).read_unaligned();
            let flag = ((track + CHANNEL_FLAG) as *const u32).read_unaligned();
            let use_alternate = !(limit.is_nan() || 0.0 < limit) && flag != 0;
            let channel_offset = if use_alternate { ALTERNATE_CHANNEL } else { POSITIVE_CHANNEL };
            let value = ((track + channel_offset) as *const f32).read_unaligned();
            total = add_ordered(total, value);
        }
        core::arch::x86::_mm_set_ss(total)
    }
});
