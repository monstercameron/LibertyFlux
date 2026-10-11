// original: 0x009685B0 clear_timing_sample_fields

/// Clears the six paired 32-bit timing sample slots beginning at object
/// offsets `0x31e0` and `0x31f8`. The original has no arguments beyond the
/// ECX object pointer and does not define a semantic EAX result.
lf_checker_rt::export!(thiscall, rw_009685b0(this: u32) -> u32 {
    unsafe {
        const SAMPLE_FIELDS: [u32; 12] = [
            0x31e0, 0x31f8, 0x31e4, 0x31fc, 0x31e8, 0x3200,
            0x31ec, 0x3204, 0x31f0, 0x3208, 0x31f4, 0x320c,
        ];
        for offset in SAMPLE_FIELDS {
            (this.wrapping_add(offset) as *mut u32).write_unaligned(0);
        }
    }
    0
});
