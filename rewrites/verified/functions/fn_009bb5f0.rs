// original: 0x009bb5f0 init_input_vec_zero (proposed)

/// Initialise a 12-byte input vector slot: two zero words and -6.0f.
///
/// Writes 0 to `this+0x00` and `this+0x04` and the float bits 0xC0C00000
/// (-6.0) to `this+0x08`. Returns `this`.
///
/// Edge cases: none; unconditional stores.
///
/// Original: thiscall, `this` in ECX, no stack arguments.
lf_checker_rt::export!(thiscall, rw_009bb5f0(this: u32) -> u32 {
    unsafe {
        const THIRD: u32 = 0xc0c00000;
        (this as *mut u32).write_unaligned(0);
        ((this + 4) as *mut u32).write_unaligned(0);
        ((this + 8) as *mut u32).write_unaligned(THIRD);
        this
    }
});
