// original: 0x00bf73f0 scale_signed_bytes
/// Scale three signed bytes into floats (leaf, no calls).
///
/// Reads the signed bytes at this `+0x1e/0x1f/0x20`, converts each exactly
/// to float and multiplies by the shared scale factor (a global float), in
/// the original's order (convert, then value times scale). Stores the three
/// results to `out[0..3]`. Returns the third byte sign-extended to 32 bits.
/// Bit-exact for all 256 byte values times any scale, NaN scale included.
///
/// Original: 0x00BF73F0 (thiscall, one stack word: out).
lf_checker_rt::export!(thiscall, rw_00bf73f0(this: u32, out: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        const FIRST_BYTE: u32 = 0x1e;
        const SCALE_GLOB: u32 = 0x00fe8700;
        let scale: f32 = lf_checker_rt::global::<f32>(SCALE_GLOB).read();
        let mut last: i8 = 0;
        let mut i = 0u32;
        while i < 3 {
            let b: i8 = ((this + FIRST_BYTE + i) as *const i8).read();
            let v = fmul(b as f32, scale);
            ((out + i * 4) as *mut u32).write_unaligned(v.to_bits());
            last = b;
            i += 1;
        }
        last as i32 as u32
    }
});
