// original: 0x00e64050 audio_ratio_store_2
/// Store the single-precision ratio of two global floats into a global slot.
///
/// Loads `0x01037994` and `0x01037998`, divides with SSE (`divss`), and stores
/// the bit-exact quotient into `0x01218484`. Takes no arguments and returns nothing;
/// entry registers are ignored. Uses SSE intrinsics so the division is the
/// same single-rounded instruction on every build, never x87 double rounding.
export!(cdecl, rw_00e64050() -> u32 {
    unsafe {
        use core::arch::x86::{_mm_cvtss_f32, _mm_div_ss, _mm_set_ss};
        let a = f32::from_bits(*global::<u32>(0x1037994));
        let b = f32::from_bits(*global::<u32>(0x1037998));
        let q = _mm_cvtss_f32(_mm_div_ss(_mm_set_ss(a), _mm_set_ss(b)));
        *global::<u32>(0x1218484) = q.to_bits();
        0
    }
});
