// original: 0x00a055a0 NativeImpl_IS_NON_FRAG_OBJECT_SMASHED (native)
/// Run the object probe sweep over two stack records; always answers 0.
///
/// Resolves the 5-word probe record for `key` (first word preset to -1),
/// scales the squared `rate` by the read-only constant 4.0, and runs the
/// sweep over the 10-word record [x, y, z, rec4, rate, 0, 0, 0, scaled, 0]
/// and the 8-word record [scaled, 0, 0, 0, x, y, z, rec4] with a trailing
/// flag word of 1, plus the record pointer (which aliases the first record
/// one word below). The smashed/not word the original reads afterwards is
/// the sweep record's own zero word, so the answer is always 0 and the
/// flag-byte path is dead. Cdecl, five words.
lf_checker_rt::export!(cdecl, rw_00a055a0(x: u32, y: u32, z: u32, rate: u32, key: u32) -> u32 {
    unsafe {
        const PROBE_CONST: u32 = 0x00fe8ab8;
        const SWEEP_KIND: u32 = 0x0094b7d0;
        const RESOLVE: u32 = 0;
        const SWEEP: u32 = 1;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let rf = f32::from_bits(rate);
        let t = mul(rf, rf);
        let c = f32::from_bits(
            (lf_checker_rt::relocated(PROBE_CONST) as *const u32).read_unaligned());
        let scaled = mul(t, c).to_bits();
        let mut rec = [0xffff_ffffu32, 0u32, 0u32, 0u32, 0u32];
        let _: u32 = lf_checker_rt::callee_cdecl!(RESOLVE, u32, key, rec.as_mut_ptr() as u32);
        let w0 = rec[0];
        let w4 = rec[4];
        let frame = [w0, x, y, z, w4, rate, 0u32, 0u32, 0u32, scaled, 0u32];
        let v2 = [scaled, 0u32, 0u32, 0u32, x, y, z, w4, w0, 1u32];
        let _: u32 = lf_checker_rt::callee_cdecl!(
            SWEEP, u32, frame.as_ptr().add(1) as u32, lf_checker_rt::relocated(SWEEP_KIND),
            v2.as_ptr() as u32, 0x16u32, 0xdu32, key, frame.as_ptr() as u32);
        0
    }
});
