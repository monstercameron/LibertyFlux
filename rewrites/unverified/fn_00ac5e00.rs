// original: 0x00AC5E00 range_map_coefficients (proposed)

/// Map an input range onto coefficients, or write the identity pair.
///
/// When the low byte of `flag` is zero the original writes 1.0 to `out_b` and
/// 0 to `out_a` (cdecl, five words: flag, `lo`, `hi`, `out_a`, `out_b`).
/// Otherwise, with `q = 1 / (hi - lo)`, it writes `-q` to `out_a` and
/// `1 + q * lo` to `out_b`, using the game's 1.0 constant and sign flip.
/// The float operation order is the original's; division by zero yields
/// infinity exactly as the original's `divss` does. No value is returned.
lf_checker_rt::export!(cdecl, rw_00AC5E00(flag: u32, lo: u32, hi: u32, out_a: u32, out_b: u32) -> u32 {
    unsafe {
        const ONE_ADDR: u32 = 0x00FE88E8;
        const SIGN: u32 = 0x8000_0000;
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn neg(a: f32) -> f32 {
            core::hint::black_box(f32::from_bits(core::hint::black_box(a).to_bits() ^ SIGN)) // opaque: LLVM reassociates a visible fneg(fmul)
        }
        unsafe {
            if flag & 0xff == 0 {
                (out_b as *mut u32).write_unaligned(0x3f80_0000);
                (out_a as *mut u32).write_unaligned(0);
            } else {
                let one: f32 = (lf_checker_rt::relocated(ONE_ADDR) as *const f32).read_unaligned();
                let lof = f32::from_bits(lo);
                let hif = f32::from_bits(hi);
                let diff = sub(hif, lof);
                let q = div(one, diff);
                (out_a as *mut f32).write_unaligned(q);
                let qb = neg(mul(q, lof));
                (out_b as *mut f32).write_unaligned(qb);
                let na = neg((out_a as *const f32).read_unaligned());
                (out_a as *mut f32).write_unaligned(na);
                let r = sub(one, (out_b as *const f32).read_unaligned());
                (out_b as *mut f32).write_unaligned(r);
            }
            0
        }
    }
});
