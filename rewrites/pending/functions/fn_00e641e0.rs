// original: 0x00e641e0 audio_ratio_store_4
/// Store the single-precision ratio of two global floats into a global slot.
///
/// Loads `0x010388EC` and `0x010388F0`, divides with SSE (`divss`), and stores
/// the bit-exact quotient into `0x012202DC`. Takes no arguments and returns nothing;
/// entry registers are ignored. Uses SSE intrinsics so the division is the
/// same single-rounded instruction on every build, never x87 double rounding.
export!(cdecl, rw_00e641e0() -> u32 {
    unsafe {
        use core::arch::x86::{_mm_cvtss_f32, _mm_div_ss, _mm_set_ss};
        let a = f32::from_bits(*global::<u32>(0x10388EC));
        let b = f32::from_bits(*global::<u32>(0x10388F0));
        let q = _mm_cvtss_f32(_mm_div_ss(_mm_set_ss(a), _mm_set_ss(b)));
        *global::<u32>(0x12202DC) = q.to_bits();
        0
    }
});
