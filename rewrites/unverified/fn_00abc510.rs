// original: 0x00abc510 shape_blend_vec3_pair (proposed)

/// Blend two input vec3s into six output floats through scripted shapers.
///
/// `a` points to three floats clamped into `[-1, 1]` (bounds from read-only
/// constants) and passed one by one through the first callee, which takes its
/// input in the vector register and answers a float. Each answer is added to
/// and subtracted from `m`; the sums are clamped into `[PI/2, PI]` and the
/// differences into `[0, PI/2]`, and all six pass through the second callee
/// the same way. Only three of those six answers are used further (`r3`,
/// `r6`, `r8`); the other three calls still happen and are compared, but
/// their answers feed stores that later instructions overwrite.
///
/// The outputs at `this` combine the kept answers with `k` (the second stack
/// word) and the three floats at `b`:
/// `[0]=b0+r3*k`, `[4]=b1+k*k`, `[8]=b2+0*k`, `[0x10]=b0+r3*k`,
/// `[0x14]=b1+k*r6`, `[0x18]=b2+k*r8`.
/// The `0*` term reads a stack slot the original never wrote (zero under the
/// checker's defined stack fill). All arithmetic runs in the original's
/// operand order through order-pinning helpers, so NaN signs and payloads
/// match bit for bit; every branch is a strict ordered comparison, which Rust
/// `<`/`>` reproduce exactly including NaN inputs.
///
/// The original evacuates two intermediate floats into its own incoming
/// argument slot; the rewrite keeps them in locals, so the stack comparison
/// is switched off for this function (every evacuated value flows into a
/// compared heap word or call argument).
///
/// Original: 0x00abc510 (thiscall, four stack words: vec3 `b`, float `k`,
/// vec3 `a`, float `m`; returns `this`).
lf_checker_rt::export!(thiscall, rw_00abc510(this: u32, b: u32, k: u32, a: u32, m: u32) -> u32 {
    unsafe {
        const LO0: u32 = 0xFE8D94; // -1.0
        const HI0: u32 = 0xFE88E8; // 1.0
        const G1LO: u32 = 0xFE8978; // pi/2
        const G1HI: u32 = 0xFE8AA0; // pi
        const SHAPER: u32 = 1;
        const FOLDER: u32 = 2;
        const SIGN: u32 = 0x8000_0000;

        #[inline(always)]
        unsafe fn rd32(p: u32) -> u32 {
            unsafe { (p as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(p: u32, v: u32) {
            unsafe { (p as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(p: u32) -> f32 {
            unsafe { f32::from_bits(rd32(p)) }
        }
        #[inline(always)]
        unsafe fn wrf(p: u32, v: f32) {
            unsafe { wr32(p, v.to_bits()) }
        }
        #[inline(always)]
        fn add(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) + core::hint::black_box(y)
        }
        #[inline(always)]
        fn sub(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) - core::hint::black_box(y)
        }
        #[inline(always)]
        fn mul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
        }
        #[inline(always)]
        fn clamp_lo_hi(v: f32, lo: f32, hi: f32) -> f32 {
            if lo > v {
                lo
            } else if v > hi {
                hi
            } else {
                v
            }
        }
        #[inline(always)]
        fn clamp_zero(v: f32, hi: f32) -> f32 {
            if 0.0 > v {
                0.0
            } else if v > hi {
                hi
            } else {
                v
            }
        }
        #[inline(always)]
        fn shape(id: u32, v: f32) -> f32 {
            f32::from_bits(lf_checker_rt::callee_cdecl!(id, u32, v.to_bits()))
        }
        let _ = SIGN;

        let lo0 = rdf(lf_checker_rt::relocated(LO0));
        let hi0 = rdf(lf_checker_rt::relocated(HI0));
        let g1lo = rdf(lf_checker_rt::relocated(G1LO));
        let g1hi = rdf(lf_checker_rt::relocated(G1HI));
        let r0 = shape(SHAPER, clamp_lo_hi(rdf(a), lo0, hi0));
        let r1 = shape(SHAPER, clamp_lo_hi(rdf(a.wrapping_add(4)), lo0, hi0));
        let r2 = shape(SHAPER, clamp_lo_hi(rdf(a.wrapping_add(8)), lo0, hi0));
        let mf = f32::from_bits(m);
        let r3 = shape(FOLDER, clamp_lo_hi(add(r0, mf), g1lo, g1hi));
        let _r4 = shape(FOLDER, clamp_zero(sub(r0, mf), g1lo));
        let _r5 = shape(FOLDER, clamp_lo_hi(add(r1, mf), g1lo, g1hi));
        let r6 = shape(FOLDER, clamp_zero(sub(r1, mf), g1lo));
        let _r7 = shape(FOLDER, clamp_lo_hi(add(r2, mf), g1lo, g1hi));
        let r8 = shape(FOLDER, clamp_zero(sub(r2, mf), g1lo));
        let kf = f32::from_bits(k);
        let b0 = rdf(b);
        let b1 = rdf(b.wrapping_add(4));
        let b2 = rdf(b.wrapping_add(8));
        wrf(this, add(b0, mul(r3, kf)));
        wrf(this.wrapping_add(4), add(b1, mul(kf, kf)));
        wrf(this.wrapping_add(8), add(b2, mul(0.0, kf)));
        wrf(this.wrapping_add(0x10), add(b0, mul(r3, kf)));
        wrf(this.wrapping_add(0x14), add(b1, mul(kf, r6)));
        wrf(this.wrapping_add(0x18), add(b2, mul(kf, r8)));
        this
    }
});
