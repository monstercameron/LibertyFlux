// original: 0x00CBFD80 CPedMoveBlendOnFoot::vf19
/// Clamp two blend weights in the on-foot object. When flag bit one at
/// byte `0x50` is set, both weights at `0x0C` and `0x10` are clamped to
/// `[-2, 2]`; otherwise the first is cleared and the second is clamped to
/// `[0, 3]`. Ordered comparisons leave NaN bit patterns unchanged. The
/// thiscall method has no stack arguments.
lf_checker_rt::export!(thiscall, rw_00cbfd80(this: *mut u8) -> u32 {
    const FLAGS_OFFSET: usize = 0x50;
    const FIRST_WEIGHT_OFFSET: usize = 0x0C;
    const SECOND_WEIGHT_OFFSET: usize = 0x10;
    const LOWER_WEIGHT: f32 = -2.0;
    const ACTIVE_UPPER_WEIGHT: f32 = 2.0;
    const IDLE_UPPER_WEIGHT: f32 = 3.0;
    let flags = unsafe { this.add(FLAGS_OFFSET).read() };
    let clamp = |offset: usize, low: f32, high: f32| {
        let slot = unsafe { this.add(offset).cast::<u32>() };
        let value = f32::from_bits(unsafe { slot.read_unaligned() });
        if value < low {
            unsafe { slot.write_unaligned(low.to_bits()) };
        } else if value > high {
            unsafe { slot.write_unaligned(high.to_bits()) };
        }
    };
    if flags & 2 != 0 {
        clamp(FIRST_WEIGHT_OFFSET, LOWER_WEIGHT, ACTIVE_UPPER_WEIGHT);
        clamp(SECOND_WEIGHT_OFFSET, LOWER_WEIGHT, ACTIVE_UPPER_WEIGHT);
    } else {
        unsafe { this.add(FIRST_WEIGHT_OFFSET).cast::<u32>().write_unaligned(0) };
        clamp(SECOND_WEIGHT_OFFSET, 0.0, IDLE_UPPER_WEIGHT);
    }
    0
});
