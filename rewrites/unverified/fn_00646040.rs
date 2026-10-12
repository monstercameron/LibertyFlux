// original: 0x00646040 rage::ptxEmitRuleStd::vf20

/// Return the emitter rule's rate value on the x87 stack: zero when the
/// suppress flag byte at `+0x1ec` is set, otherwise the float at `+0x8`
/// widened to 80-bit. Thiscall, no stack arguments, st0 return.
lf_checker_rt::export!(thiscall, rw_00646040(this: u32) -> f64 {
    unsafe {
        const SUPPRESS_FLAG: u32 = 0x1ec;
        const RATE_VALUE: u32 = 0x08;
        let flag = ((this.wrapping_add(SUPPRESS_FLAG)) as *const u8).read();
        if flag != 0 {
            0.0
        } else {
            let bits =
                ((this.wrapping_add(RATE_VALUE)) as *const u32).read_unaligned();
            f32::from_bits(bits) as f64
        }
    }
});
