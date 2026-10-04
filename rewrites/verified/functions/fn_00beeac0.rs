// original: 0x00beeac0 ped_mover_blend_a (proposed)

/// Blend a ped mover's base offset toward a source offset by factor `t`,
/// after aligning and mixing its orientation through three helpers.
///
/// `this` is the mover: word `+0x10` seeds the orientation callee, which
/// fills a four-float quaternion buffer. That quaternion is dotted with the
/// four direction floats at `dir` (accumulated as
/// `(dir1*q1+dir0*q0)+dir2*q2+dir3*q3`); a strictly negative dot flips the
/// sign of all four words so the quaternion faces the direction. The mixer
/// callee then combines the seed, the aligned quaternion and the direction
/// into a four-float accumulator, which the consumer callee reads. `out`
/// receives the base words from `this+0x14/+0x18/+0x1c` at `+0x30/+0x34/+0x38`,
/// each blended with the matching source float as `base*(1-t)+src*t` (first
/// and third terms keep the original's `(1-t)*base` operand order), and the
/// accumulator's last word at `+0x3c`.
///
/// A NaN dot product is unordered and skips the sign flip, matching the
/// original's `comiss`+`jbe`.
///
/// Original: thiscall, four stack words (`out`, `dir`, `src`, `t`), no
/// meaningful return value. Twin of 0x00beec60 with every `this` offset four
/// lower.
lf_checker_rt::export!(thiscall, rw_00beeac0(this: u32, out: u32, dir: u32, src: u32, t: u32) -> u32 {
    unsafe {
        const QUAT_SEED: u32 = 0x10;
        const BASE_X: u32 = 0x14;
        const BASE_Y: u32 = 0x18;
        const BASE_Z: u32 = 0x1c;
        const OUT_X: u32 = 0x30;
        const OUT_Y: u32 = 0x34;
        const OUT_Z: u32 = 0x38;
        const OUT_W: u32 = 0x3c;
        const SIGN_BIT: u32 = 0x8000_0000;
        const ONE: f32 = 1.0;
        const CALLEE_QUAT: u32 = 1;
        const CALLEE_MIX: u32 = 2;
        const CALLEE_USE: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let mut q = [0u32; 4];
        lf_checker_rt::callee_cdecl!(CALLEE_QUAT, u32, q.as_mut_ptr() as u32, rd32(this + QUAT_SEED));
        let tt = f32::from_bits(t);
        let qf = [
            f32::from_bits(q[0]),
            f32::from_bits(q[1]),
            f32::from_bits(q[2]),
            f32::from_bits(q[3]),
        ];
        let mut dot = add(mul(rdf(dir + 4), qf[1]), mul(rdf(dir), qf[0]));
        dot = add(dot, mul(rdf(dir + 8), qf[2]));
        dot = add(dot, mul(rdf(dir + 12), qf[3]));
        if 0.0 > dot {
            for w in q.iter_mut() {
                *w ^= SIGN_BIT;
            }
        }
        let mut acc = [0u32; 4];
        lf_checker_rt::callee_thiscall!(CALLEE_MIX, u32, acc.as_mut_ptr() as u32, t, q.as_mut_ptr() as u32, dir);
        lf_checker_rt::callee_thiscall!(CALLEE_USE, u32, out, acc.as_mut_ptr() as u32);

        wr32(out + OUT_X, rd32(this + BASE_X));
        wr32(out + OUT_Y, rd32(this + BASE_Y));
        wr32(out + OUT_Z, rd32(this + BASE_Z));
        let k = sub(ONE, tt);
        let ax = mul(rdf(src), tt);
        let az = mul(rdf(src + 8), tt);
        let ay = mul(rdf(src + 4), tt);
        let bx = mul(rdf(out + OUT_X), k);
        let by = mul(k, rdf(out + OUT_Y));
        let bz = mul(k, rdf(out + OUT_Z));
        wrf(out + OUT_X, add(bx, ax));
        wrf(out + OUT_Y, add(by, ay));
        wrf(out + OUT_Z, add(bz, az));
        wr32(out + OUT_W, acc[3]);
        0
    }
});
