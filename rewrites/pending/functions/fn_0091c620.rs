// original: 0x0091C620 TextBackendLineHeight
/// Scale one per-index table float by the global text factor and a second
/// per-index factor.
///
/// Queries the index helper, loads the float at index 72 of the table at
/// `0x0119BF20`, multiplies it by the global float at `0x00E85F6C` and then
/// by the float at index 72 of the table at `0x0119BF54`, returning the
/// product on `ST0`. Both multiplies use SSE intrinsics so they are
/// bit-exact with the original.
export!(cdecl, rw_0091c620() -> f32 {
    unsafe {
        use core::arch::x86::{_mm_cvtss_f32, _mm_mul_ss, _mm_set_ss};
        let idx: u32 = callee_cdecl!(1, u32,);
        let off = idx.wrapping_mul(9).wrapping_mul(8);
        let t = f32::from_bits(*((relocated(0x119BF20) + off) as *const u32));
        let g = f32::from_bits(*global::<u32>(0xE85F6C));
        let u = f32::from_bits(*((relocated(0x119BF54) + off) as *const u32));
        let p = _mm_mul_ss(_mm_set_ss(t), _mm_set_ss(g));
        _mm_cvtss_f32(_mm_mul_ss(p, _mm_set_ss(u)))
    }
});
