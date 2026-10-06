// original: 0x008f7a90 input_axis0_over_threshold (proposed)

/// Report whether the axis-0 sampler value is above the threshold.
///
/// The sampler callee (thiscall: ECX threaded from entry, one stack word 0)
/// returns a float on the x87 stack; the rewrite reads the same scripted
/// bits from EAX, which the stub mirrors there. The value is compared
/// against the global threshold with an ordered-above test: NaN on either
/// side reports 0, and +0.0 versus -0.0 reports 0, exactly like Rust's `>`
/// on `f32`. Returns 1 or 0.
///
/// Thiscall: thread-through ECX, no stack words.
lf_checker_rt::export!(thiscall, rw_008f7a90(this: u32) -> u32 {
    unsafe {
        const C_SAMPLER: u32 = 1;
        const G_THRESH: u32 = 0xfe8960;
        let bits: u32 = lf_checker_rt::callee_thiscall!(C_SAMPLER, u32, this, 0);
        let v = f32::from_bits(bits);
        let m = f32::from_bits((lf_checker_rt::global::<u32>(G_THRESH)).read_unaligned());
        if v > m { 1 } else { 0 }
    }
});
