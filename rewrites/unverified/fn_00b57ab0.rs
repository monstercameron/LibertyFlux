// original: 0x00B57AB0 integrate_clamped_accumulator

/// Integrate the per-object rate over one elapsed-time value and store the
/// accumulated result at byte offset `0x48`. The rate is at `0x4c`; the
/// function computes `rate * elapsed + previous` in that operand order,
/// clamps only ordered negative results to positive zero, preserves negative
/// zero and NaNs, and uses thiscall with one 32-bit float argument.
lf_checker_rt::export!(thiscall, rw_b57ab0(this: *mut u8, elapsed: f32) -> () {
    unsafe {
        const ACCUMULATOR_OFFSET: usize = 0x48;
        const RATE_OFFSET: usize = 0x4c;

        let rate = (this.add(RATE_OFFSET) as *const f32).read_unaligned();
        let previous = (this.add(ACCUMULATOR_OFFSET) as *const f32).read_unaligned();
        let scaled_elapsed = core::hint::black_box(rate)
            * core::hint::black_box(elapsed);
        let accumulated = core::hint::black_box(scaled_elapsed)
            + core::hint::black_box(previous);
        let stored = if accumulated < 0.0 {
            f32::from_bits(0)
        } else {
            accumulated
        };
        (this.add(ACCUMULATOR_OFFSET) as *mut f32).write_unaligned(stored);
    }
});
