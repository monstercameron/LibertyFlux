// original: 0x00e6c0b0 veh_ratio_store_13
/// Store the single-precision ratio of two global floats into a global slot.
///
/// Loads `SRC1` (0x01051690) and `SRC2` (0x01051694), divides with one SSE
/// `divss`, and stores the bit-exact quotient into `DST` (0x0171C934).
/// Takes no arguments and leaves EAX untouched (no return channel).
/// Uses SSE intrinsics so the division is the same single-rounded
/// instruction on every build, never x87 double rounding.
///
/// Original: 0x00E6C0B0, cdecl, no arguments.
export!(cdecl, rw_00e6c0b0() -> u32 {
    unsafe {
        use core::arch::x86::{_mm_cvtss_f32, _mm_div_ss, _mm_set_ss};
        const SRC1: u32 = 0x1051690;
        const SRC2: u32 = 0x1051694;
        const DST: u32 = 0x171C934;
        let x = f32::from_bits(*global::<u32>(SRC1));
        let y = f32::from_bits(*global::<u32>(SRC2));
        let q = _mm_cvtss_f32(_mm_div_ss(_mm_set_ss(x), _mm_set_ss(y)));
        *global::<u32>(DST) = q.to_bits();
        0
    }
});
