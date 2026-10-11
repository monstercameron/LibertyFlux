// original: 0x00DE46B0 UIRowLayout::vf130

/// Copy the stack float's 32-bit representation to `field_1E4_f` at `+0x1E4`.
///
/// The original uses `movss` without arithmetic, so every finite value and NaN payload passes through unchanged. `thiscall`: ECX is the object; no other memory or return value is meaningful.
lf_checker_rt::export!(thiscall, rw_00de46b0(this_ptr: u32, value_bits: u32) -> u32 {
    const FLOAT_BITS_OFFSET: u32 = 0x1E4;
    // movss copies this value as bits; avoid float canonicalisation.
    unsafe { ((this_ptr.wrapping_add(FLOAT_BITS_OFFSET)) as *mut u32)
        .write_unaligned(value_bits) };
    0
});
