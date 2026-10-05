// original: 0x00e6bf70 veh_ratio_store_07
/// Store the single-precision ratio of two global floats into a global slot.
///
/// Loads `SRC1` (0x01050D70) and `SRC2` (0x01050D74), divides with one SSE
/// `divss`, and stores the bit-exact quotient into `DST` (0x0171BBC8).
/// Takes no arguments and leaves EAX untouched (no return channel).
/// Uses SSE intrinsics so the division is the same single-rounded
/// instruction on every build, never x87 double rounding.
///
/// Original: 0x00E6BF70, cdecl, no arguments.
export!(cdecl, rw_00e6bf70() -> u32 {
    unsafe {
        use core::arch::x86::{_mm_cvtss_f32, _mm_div_ss, _mm_set_ss};
        const SRC1: u32 = 0x1050D70;
        const SRC2: u32 = 0x1050D74;
        const DST: u32 = 0x171BBC8;
        let x = f32::from_bits(*global::<u32>(SRC1));
        let y = f32::from_bits(*global::<u32>(SRC2));
        let q = _mm_cvtss_f32(_mm_div_ss(_mm_set_ss(x), _mm_set_ss(y)));
        *global::<u32>(DST) = q.to_bits();
        0
    }
});
