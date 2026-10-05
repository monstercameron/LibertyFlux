// original: 0x00696510 rage::crAnimChannelRawQuaternion::sample_indexed

/// Samples a raw quaternion channel at a fractional index into an output.
///
/// `this` is the channel object with the sample array at `[this+8]`; the
/// stack arguments are the integer `index`, the blend `t` as `f32` bits and
/// the output pointer. Each component is `lo * (1 - t) + hi * t` in the
/// original's order, where `lo` is element `index` and `hi` element
/// `index + 1` (16 bytes per element). The result is then normalised: the
/// squared length folds left (`((x*x + y*y) + z*z) + w*w`), and unless it
/// compares equal to zero the square-root callee (callee 1, cdecl: length
/// bits, answer in ST0) is called and every component multiplied by
/// `1 / root` (component first, as the original). The zero test is the
/// original's `ucomiss`/`lahf`/`test`/`jnp` idiom, under which a NaN length
/// takes the square-root path (unordered sets both ZF and PF, so parity is
/// even and the skip is not taken); only an ordered zero skips. The original
/// uses its dead `t`/output argument slots as scratch for the components, so
/// the stack check is off; the values are observed through the output writes
/// and the callee input. No return value.
///
/// Original: 0x00696510 (thiscall, three stack words, callee pops 12).
lf_checker_rt::export!(thiscall, rw_00696510(this: u32, index: u32, tbits: u32, out: u32) -> u32 {
    unsafe {
        const SAMPLES_OFF: u32 = 8;
        const PAIR_STRIDE: u32 = 0x10;
        const ONE_VA: u32 = 0xFE88E8;
        const SQRT: u32 = 1;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        let base = rd32(this + SAMPLES_OFF);
        let t = f32::from_bits(tbits);
        let one = f32::from_bits(rd32(lf_checker_rt::relocated(ONE_VA)));
        let s = fsub(one, t);
        let row = base.wrapping_add(index.wrapping_mul(2).wrapping_mul(8));
        let mut c = 0u32;
        while c < 4 {
            let off = c * 4;
            let lo = f32::from_bits(rd32(row.wrapping_add(off)));
            let hi = f32::from_bits(rd32(row.wrapping_add(off).wrapping_add(PAIR_STRIDE)));
            let r = fadd(fmul(lo, s), fmul(hi, t));
            wr32(out + off, r.to_bits());
            c += 1;
        }
        let x = f32::from_bits(rd32(out));
        let y = f32::from_bits(rd32(out + 4));
        let z = f32::from_bits(rd32(out + 8));
        let w = f32::from_bits(rd32(out + 12));
        let n2 = fadd(fadd(fadd(fmul(x, x), fmul(y, y)), fmul(z, z)), fmul(w, w));
        if n2 == 0.0 {
            return 0;
        }
        let root: f32 = lf_checker_rt::callee_cdecl!(SQRT, f32, n2.to_bits());
        let inv = fdiv(one, root);
        wr32(out, fmul(x, inv).to_bits());
        wr32(out + 4, fmul(y, inv).to_bits());
        wr32(out + 8, fmul(z, inv).to_bits());
        wr32(out + 12, fmul(w, inv).to_bits());
        0
    }
});
