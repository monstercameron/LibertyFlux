// original: 0x00AEFBF0 clamp_scaled_global (proposed)

/// Clamp a global factor, scale a second global by it, clamp again.
///
/// Reads factor `G_A` and multiplicand `G_B` (both globals) and limit `LIM`
/// (a read-only constant). The factor is clamped into `[0, LIM]`, the
/// product `G_B * factor` into `[0, LIM]` with hard 0.0/1.0 edges (the
/// second clamp returns literal 0.0 and 1.0, not the limit). Every
/// comparison is an ordered SSE comparison: a NaN input passes both clamps
/// of its stage through, since `comiss` reports unordered as above-or-equal.
/// All arithmetic is single precision in the original's operand order.
///
/// Original: 0x00AEFBF0 (cdecl, no arguments, float result on the x87 stack).
lf_checker_rt::export!(cdecl, rw_00aefbf0() -> f32 {
    unsafe {
        const G_FACTOR: u32 = 0x012DDE98;
        const G_VALUE: u32 = 0x0103F998;
        const G_LIMIT: u32 = 0x00FE88E8;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        unsafe fn grd(a: u32) -> f32 {
            unsafe { f32::from_bits((lf_checker_rt::global::<u32>(a)).read_unaligned()) }
        }
        let lim = grd(G_LIMIT);
        let mut x = grd(G_FACTOR);
        if x < 0.0 {
            x = 0.0;
        }
        if x > lim {
            x = lim;
        }
        let y = mul(grd(G_VALUE), x);
        if y < 0.0 {
            return 0.0;
        }
        if y > lim {
            return 1.0;
        }
        y
    }
});
