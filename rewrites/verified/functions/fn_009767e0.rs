// original: 0x009767e0 audio_level_from_bytes (proposed)

/// Derive a clamped level from a gain and three packed fields, on ST0.
///
/// Reads a 16-bit field (+0x1E) and two bytes (+0x1B, +0x1C) from `obj`,
/// converts each to float exactly, and with the read-only constants A and B
/// computes q = max(gain - b1 * (A / (w * B)), 0) / (b2 * (A / (w * B))),
/// returning q when A > q and 1.0 otherwise. Both comparisons follow the
/// original's conditional jumps exactly (unordered counts as not-above).
/// Original: 0x009767E0 (stdcall, two stack words).
lf_checker_rt::export!(stdcall, rw_009767e0(gain: f32, obj: u32) -> f32 {
    unsafe {
        const W_OFF: u32 = 0x1E;
        const B1_OFF: u32 = 0x1B;
        const B2_OFF: u32 = 0x1C;
        const A: f32 = f32::from_bits(0x3f800000);
        const B: f32 = f32::from_bits(0x3c23d70a);
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        let w = ((obj.wrapping_add(W_OFF)) as *const u16).read_unaligned() as f32;
        let b1 = ((obj.wrapping_add(B1_OFF)) as *const u8).read() as f32;
        let b2 = ((obj.wrapping_add(B2_OFF)) as *const u8).read() as f32;
        let x0 = mul(w, B);
        let r = div(A, x0);
        let x0 = mul(b1, r);
        let x2 = mul(b2, r);
        let t = sub(gain, x0);
        let t = if t > 0.0 { t } else { 0.0 };
        let q = div(t, x2);
        if A > q { q } else { 1.0 }
    }
});
