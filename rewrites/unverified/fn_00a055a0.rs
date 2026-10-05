// original: 0x00a055a0 NativeImpl_IS_NON_FRAG_OBJECT_SMASHED (native)
/// Test whether a non-fragile object counts as smashed.
///
/// Resolves the probe record for `key` (presetting its first word to -1),
/// scales the squared `rate` by the read-only constant, and runs the probe
/// sweep over two stack records: the 10-word [x, y, z, rec1, rate, 0, 0,
/// 0, scaled, 0] (three middle words are stack garbage in the original;
/// the checker pins them to zero and so does this rewrite) and the 7-word
/// [0, x, y, z, rec1, 0, 1], plus the record pointer itself, which aliases
/// the first record at word 2. When the record word is nonzero the answer
/// is its flag byte at +0x1ac bit 0, else 0 (low byte only). Cdecl.
lf_checker_rt::export!(cdecl, rw_00a055a0(x: u32, y: u32, z: u32, rate: u32, key: u32) -> u32 {
    unsafe {
        const PROBE_CONST: u32 = 0x00fe8ab8;
        const SWEEP_KIND: u32 = 0x0094b7d0;
        const RESOLVE: u32 = 0;
        const SWEEP: u32 = 1;
        const FLAG_OFF: u32 = 0x1ac;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let rf = f32::from_bits(rate);
        let t = mul(rf, rf);
        let c = f32::from_bits(
            (lf_checker_rt::relocated(PROBE_CONST) as *const u32).read_unaligned());
        let scaled = mul(t, c).to_bits();
        let mut rec = [0xffff_ffffu32, 0u32];
        let _: u32 = lf_checker_rt::callee_cdecl!(RESOLVE, u32, key, rec.as_mut_ptr() as u32);
        let w0 = rec[0];
        let w1 = rec[1];
        let mut v1 = [x, y, z, w1, rate, 0u32, 0u32, 0u32, scaled, 0u32];
        let v2 = [0u32, x, y, z, w1, 0u32, 1u32];
        let _: u32 = lf_checker_rt::callee_cdecl!(
            SWEEP, u32, v1.as_ptr() as u32, lf_checker_rt::relocated(SWEEP_KIND),
            v2.as_ptr() as u32, 0x16u32, 0xdu32, key, v1.as_ptr().add(2) as u32);
        if w0 == 0 {
            return 0;
        }
        (((w0 + FLAG_OFF) as *const u8).read() & 1) as u32
    }
});
