// original: 0x005FD170 streaming_is_value_above_threshold

/// Return whether the receiver's current value exceeds the streaming limit.
///
/// The receiver holds a context pointer at `+0xb4`; the context's single
/// precision value is at `+0x70`. The value is compared as an ordered float
/// against the engine's single precision constant. Only a strictly greater
/// value returns one, so equality and unordered NaN comparisons return zero.
/// The result is an 8-bit value in AL; the function has no stack arguments.
lf_checker_rt::export!(thiscall, rw_005fd170(this: u32) -> u8 {
    unsafe {
        const CONTEXT_PTR: u32 = 0xb4;
        const CONTEXT_VALUE: u32 = 0x70;
        const LIMIT_FILE_VA: u32 = 0x00fe8628;

        let context = (this.wrapping_add(CONTEXT_PTR) as *const u32).read_unaligned();
        let value_bits = (context.wrapping_add(CONTEXT_VALUE) as *const u32).read_unaligned();
        let value = f32::from_bits(value_bits);
        let limit = lf_checker_rt::global::<f32>(LIMIT_FILE_VA).read_unaligned();
        u8::from(value > limit)
    }
});
