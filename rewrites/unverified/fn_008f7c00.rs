// original: 0x008f7c00 input_axis1_over_threshold (proposed)

/// Report whether the axis-1 value is above the threshold.
///
/// `obj` is the input device object. The sampler callee (thiscall:
/// ECX = `obj`, one stack word 1) always runs first and returns a float on
/// the x87 stack, which the rewrite reads as scripted bits from EAX. When
/// the override byte at `+0x454` is nonzero the sampled value is discarded
/// and the global override float is tested instead. The test is
/// ordered-above against the global threshold: NaN on either side reports
/// 0, like Rust's `>` on `f32`. Returns 1 or 0.
///
/// Thiscall: object in ECX, no stack words.
lf_checker_rt::export!(thiscall, rw_008f7c00(obj: u32) -> u32 {
    unsafe {
        const C_SAMPLER: u32 = 1;
        const OVERRIDE_OFF: u32 = 0x454;
        const G_OVERRIDE: u32 = 0xfe89b8;
        const G_THRESH: u32 = 0xfe8960;
        let bits: u32 = lf_checker_rt::callee_thiscall!(C_SAMPLER, u32, obj, 1);
        let v = if ((obj + OVERRIDE_OFF) as *const u8).read() != 0 {
            f32::from_bits((lf_checker_rt::global::<u32>(G_OVERRIDE)).read_unaligned())
        } else {
            f32::from_bits(bits)
        };
        let m = f32::from_bits((lf_checker_rt::global::<u32>(G_THRESH)).read_unaligned());
        if v > m { 1 } else { 0 }
    }
});
