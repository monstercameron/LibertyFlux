// original: 0x0087B2C0 rage::crmtNodeAddSubtract::append_debug_info

/// Append a node's hierarchy, numeric fields, and pointer fields to a logger.
///
/// `this` is the add/subtract node; its two single-precision values are at
/// `+0x20` and `+0x24`, and the two pointer-sized values are at `+0x28` and
/// `+0x2c`. The stack argument is the logger context. The function calls the
/// hierarchy dumper, then formats the first two values as promoted doubles
/// using the weight/rate format string, then formats the two pointer fields.
/// The final formatter's EAX result is returned. All calls are intercepted.
///
/// Original: thiscall with one stack pointer; the variadic formatter receives
/// each double as low and high 32-bit stack words.
lf_checker_rt::export!(thiscall, rw_0087b2c0(this: u32, logger_context: u32) -> u32 {
    const WEIGHT_OFFSET: u32 = 0x20;
    const RATE_OFFSET: u32 = 0x24;
    const FILTER_OFFSET: u32 = 0x28;
    const MODIFIER_OFFSET: u32 = 0x2C;
    const WEIGHT_RATE_FORMAT: u32 = 0x00FC8914;
    const FILTER_MODIFIER_FORMAT: u32 = 0x00FC88EC;
    const DUMP_HIERARCHY: u32 = 1;
    const FORMAT_WEIGHT_RATE: u32 = 2;
    const FORMAT_FILTER_MODIFIER: u32 = 3;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    fn promoted_f32_words(single_bits: u32) -> (u32, u32) {
        let promoted = f64::from(f32::from_bits(single_bits)).to_bits();
        (promoted as u32, (promoted >> 32) as u32)
    }

    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(DUMP_HIERARCHY, u32, this, logger_context);

        let weight_bits = read_u32(this.wrapping_add(WEIGHT_OFFSET));
        let rate_bits = read_u32(this.wrapping_add(RATE_OFFSET));
        let (weight_low, weight_high) = promoted_f32_words(weight_bits);
        let (rate_low, rate_high) = promoted_f32_words(rate_bits);
        let _ = lf_checker_rt::callee_cdecl!(
            FORMAT_WEIGHT_RATE,
            u32,
            logger_context,
            lf_checker_rt::relocated(WEIGHT_RATE_FORMAT),
            weight_low,
            weight_high,
            rate_low,
            rate_high
        );

        let filter = read_u32(this.wrapping_add(FILTER_OFFSET));
        let modifier = read_u32(this.wrapping_add(MODIFIER_OFFSET));
        lf_checker_rt::callee_cdecl!(
            FORMAT_FILTER_MODIFIER,
            u32,
            logger_context,
            lf_checker_rt::relocated(FILTER_MODIFIER_FORMAT),
            filter,
            modifier
        )
    }
});
