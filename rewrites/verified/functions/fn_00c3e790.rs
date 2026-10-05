// original: 0x00c3e790 train_assemble_matrix_from_ptr (proposed)
/// Assemble a 3x4 transform in this object from a row plus helper output.
///
/// `this` (ECX) receives the matrix, `src` points at four input words.
/// Calls helper id 1 (cdecl: scratch pointer, src) which fills four words
/// at the scratch pointer (the contract scripts them). Then, with
/// `h0..h3` the helper words: copies `src[0..3]` words to `+0x10..+0x1c`,
/// `h0..h3` to `+0x20..+0x2c`, and derives three floats in the original's
/// operand order (pinned with `black_box`):
/// `[+0x00] = [+0x14]*h2 - [+0x18]*h1`,
/// `[+0x04] = [+0x18]*[+0x20] - [+0x28]*[+0x10]`,
/// `[+0x08] = [+0x24]*[+0x10] - [+0x14]*[+0x20]`.
/// The first two products read the helper values from registers holding
/// the same bits just stored. Returns `src[3]`. Bit-exact for all float
/// inputs including NaN.
///
/// Original: 0x00c3e790 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c3e790(this: u32, src: u32) -> u32 {
    unsafe {
        const HELPER: u32 = 1;
        #[inline(always)]
        unsafe fn rd32(p: u32) -> u32 {
            unsafe { (p as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(p: u32, v: u32) {
            unsafe { (p as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn mul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
        }
        #[inline(always)]
        fn sub(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) - core::hint::black_box(y)
        }
        #[inline(always)]
        unsafe fn rdf(p: u32) -> f32 {
            unsafe { f32::from_bits(rd32(p)) }
        }
        #[inline(always)]
        unsafe fn wrf(p: u32, v: f32) {
            unsafe { wr32(p, v.to_bits()) }
        }
        let mut hw = [0u32; 4];
        let hp = hw.as_mut_ptr() as u32;
        let _: u32 = lf_checker_rt::callee_cdecl!(HELPER, u32, hp, src);
        let h0 = hw[0];
        let h1 = hw[1];
        let h2 = hw[2];
        let h3 = hw[3];
        wr32(this + 0x18, rd32(src + 8));
        wr32(this + 0x14, rd32(src + 4));
        wr32(this + 0x10, rd32(src));
        let s3 = rd32(src + 12);
        wr32(this + 0x1c, s3);
        wr32(this + 0x20, h0);
        wr32(this + 0x2c, h3);
        wr32(this + 0x24, h1);
        wr32(this + 0x28, h2);
        let t0 = sub(mul(rdf(this + 0x14), f32::from_bits(h2)), mul(rdf(this + 0x18), f32::from_bits(h1)));
        wrf(this, t0);
        let t1 = sub(mul(rdf(this + 0x18), rdf(this + 0x20)), mul(rdf(this + 0x28), rdf(this + 0x10)));
        wrf(this + 4, t1);
        let t2 = sub(mul(rdf(this + 0x24), rdf(this + 0x10)), mul(rdf(this + 0x14), rdf(this + 0x20)));
        wrf(this + 8, t2);
        s3
    }
});
