// original: 0x00d29770 targeting_float_reset (proposed)

/// Reset the float at `this + 0x224` to 4.0.
///
/// A one-store setter used when a targeting entry is recycled. No stack
/// arguments; the original leaves `eax` untouched, so the return channel is
/// not compared.
///
/// Original: 0x00D29770 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00d29770(this: u32) -> u32 {
    unsafe {
        const FOUR_BITS: u32 = 0x4080_0000; // 4.0f
        (this.wrapping_add(0x224) as *mut u32).write_unaligned(FOUR_BITS);
        0
    }
});
