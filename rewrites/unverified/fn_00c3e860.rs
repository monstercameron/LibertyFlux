// original: 0x00c3e860 train_assemble_matrix_from_pair (proposed)
/// Assemble a 3x4 transform in this object from two four-word rows.
///
/// `this` (ECX) receives: words 0..3 of `a` at `+0x10..+0x1c`, words 0..3
/// of `b` at `+0x20..+0x2c` (all as bit copies), then three derived floats:
/// `[+0x00] = [+0x14]*[+0x28] - [+0x18]*[+0x24]`,
/// `[+0x04] = [+0x18]*[+0x20] - [+0x28]*[+0x10]`,
/// `[+0x08] = [+0x24]*[+0x10] - [+0x14]*[+0x20]`,
/// each product formed before the subtraction, in the original's operand
/// order (pinned with `black_box` so the compiler cannot commute them).
/// Bit-exact for all float inputs including NaN. Returns the fourth word
/// of `b` (left in EAX by the copy). No calls.
///
/// Original: 0x00c3e860 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00c3e860(this: u32, a: u32, b: u32) -> u32 {
    unsafe {
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
        wr32(this + 0x18, rd32(a + 8));
        wr32(this + 0x14, rd32(a + 4));
        wr32(this + 0x10, rd32(a));
        wr32(this + 0x1c, rd32(a + 12));
        wr32(this + 0x28, rd32(b + 8));
        wr32(this + 0x20, rd32(b));
        wr32(this + 0x24, rd32(b + 4));
        let b3 = rd32(b + 12);
        wr32(this + 0x2c, b3);
        let t0 = sub(mul(rdf(this + 0x14), rdf(this + 0x28)), mul(rdf(this + 0x18), rdf(this + 0x24)));
        wrf(this, t0);
        let t1 = sub(mul(rdf(this + 0x18), rdf(this + 0x20)), mul(rdf(this + 0x28), rdf(this + 0x10)));
        wrf(this + 4, t1);
        let t2 = sub(mul(rdf(this + 0x24), rdf(this + 0x10)), mul(rdf(this + 0x14), rdf(this + 0x20)));
        wrf(this + 8, t2);
        b3
    }
});
