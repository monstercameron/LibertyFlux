// original: 0x00683f30 quat_dof_apply_twist (proposed)

/// Twist a quaternion frame by a signed angle using a parameter block.
///
/// `this` is a frame whose quaternion lives at `+0x10`/`+0x14`/`+0x18`/
/// `+0x1c` (x, y, z, w) with a flag byte at `+0x04`; `p` supplies four
/// parameter words at `+0x10`..`+0x1f`; `f` arrives in `xmm2`. A set flag
/// bit `0x10`, or a `f` that is zero or NaN, ends the call with no writes
/// and no calls. A negative `f` takes the solver path: the analysis callee
/// derives three direction words and one angle word from the parameter
/// block, and when the direction is long enough and the angle non-zero the
/// sine/cosine callees scale the direction into a five-word bundle that is
/// folded into the frame quaternion; short or zero angles reuse the
/// parameter block as the bundle instead. A positive `f` takes the direct
/// path: the fetch callee fills a four-word block from the parameters and
/// `f`, and a second fold writes the frame quaternion. The function
/// returns void.
///
/// Original: 0x00683f30 (thiscall, one stack word, `xmm2` float entry, no
/// result). The sine/cosine inputs are computed but only their scripted
/// answers flow downstream (see contract).
lf_checker_rt::export!(thiscall, rw_00683F30(this: u32, p: u32) -> u32 {
    unsafe {
        const ANALYZE: u32 = 1;
        const SIN: u32 = 2;
        const COS: u32 = 3;
        const FETCH: u32 = 4;
        const FLAGS_OFF: u32 = 0x04;
        const SKIP_FLAG: u8 = 0x10;
        const QX: u32 = 0x10;
        const QY: u32 = 0x14;
        const QZ: u32 = 0x18;
        const QW: u32 = 0x1c;
        const SIGN: u32 = 0x8000_0000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
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
        #[inline(always)]
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN)
        }

        /// Solver-path fold of bundle (a, b, c, d, e) into the frame.
        #[inline(always)]
        unsafe fn join(this: u32, a: f32, b: f32, c: f32, d: f32, e: f32) {
            unsafe {
                let x = rdf(this + QX);
                let y = rdf(this + QY);
                let z = rdf(this + QZ);
                let w = rdf(this + QW);
                let w_new = add(add(add(mul(x, a), mul(w, b)), mul(y, d)), mul(z, c));
                wrf(this + QW, w_new);
                let x_new = add(sub(sub(mul(x, b), mul(w, a)), mul(y, c)), mul(z, d));
                wrf(this + QX, x_new);
                let y_new = add(sub(sub(mul(y, b), mul(w, d)), mul(z, e)), mul(x, c));
                let z_new = add(sub(sub(mul(z, b), mul(w, c)), mul(x, d)), mul(y, e));
                wrf(this + QZ, z_new);
                wrf(this + QY, y_new);
            }
        }

        let f = f32::from_bits(lf_checker_rt::xmm_word(2, 0));
        let flags: u8 = unsafe { ((this + FLAGS_OFF) as *const u8).read() };
        if flags & SKIP_FLAG != 0 {
            return 0;
        }
        // Parameter block copy.
        let mut blk = [0u32; 4];
        for i in 0..4u32 {
            blk[i as usize] = unsafe { rd32(p + 0x10 + i * 4) };
        }
        let b0 = f32::from_bits(blk[0]);
        let b1 = f32::from_bits(blk[1]);
        let b2 = f32::from_bits(blk[2]);
        let b3 = f32::from_bits(blk[3]);
        if f < 0.0 {
            // Solver path.
            let mut p1 = [0u32; 1];
            let mut p2 = [0u32; 3];
            let _a: u32 = lf_checker_rt::callee_thiscall!(
                ANALYZE,
                u32,
                blk.as_mut_ptr() as u32,
                p2.as_mut_ptr() as u32,
                p1.as_mut_ptr() as u32
            );
            let w40 = f32::from_bits(p2[0]);
            let w44 = f32::from_bits(p2[1]);
            let w48 = f32::from_bits(p2[2]);
            let w20 = f32::from_bits(p1[0]);
            let sum3 = add(add(mul(w40, w40), mul(w44, w44)), mul(w48, w48));
            let t1: f32 = unsafe { rdf(lf_checker_rt::relocated(0x00FE88DC)) };
            let use_direct: bool;
            if sum3 > t1 {
                let sq = mul(w20, w20);
                let t2: f32 = unsafe { rdf(lf_checker_rt::relocated(0x00FE8628)) };
                use_direct = !(sq > t2);
            } else {
                use_direct = true;
            }
            if !use_direct {
                let t3: f32 = unsafe { rdf(lf_checker_rt::relocated(0x00FE8830)) };
                let u = mul(neg(mul(w20, f)), t3);
                let s: f32 = f32::from_bits(lf_checker_rt::callee_cdecl!(SIN, u32,));
                let c: f32 = {
                    let _ = u;
                    f32::from_bits(lf_checker_rt::callee_cdecl!(COS, u32,))
                };
                let e20 = mul(w40, s);
                let e18 = mul(w44, s);
                let e10 = mul(w48, s);
                unsafe { join(this, e20, c, e10, e18, e20) };
            } else {
                unsafe { join(this, b0, b3, b2, b1, b0) };
            }
            return 0;
        }
        if f > 0.0 {
            // Direct path.
            let _a: u32 = lf_checker_rt::callee_thiscall!(
                FETCH,
                u32,
                blk.as_mut_ptr() as u32,
                f.to_bits()
            );
            let w0 = f32::from_bits(blk[0]);
            let w1 = f32::from_bits(blk[1]);
            let w2 = f32::from_bits(blk[2]);
            let w3 = f32::from_bits(blk[3]);
            let x = unsafe { rdf(this + QX) };
            let y = unsafe { rdf(this + QY) };
            let z = unsafe { rdf(this + QZ) };
            let w = unsafe { rdf(this + QW) };
            let x_new = sub(add(add(mul(x, w3), mul(w, w0)), mul(z, w1)), mul(y, w2));
            unsafe { wrf(this + QX, x_new) };
            let z_new = sub(add(add(mul(z, w3), mul(w, w2)), mul(y, w0)), mul(x, w1));
            unsafe { wrf(this + QZ, z_new) };
            let w_new = sub(sub(sub(mul(w, w3), mul(x, w0)), mul(y, w1)), mul(z, w2));
            unsafe { wrf(this + QW, w_new) };
            let y_new = sub(add(add(mul(y, w3), mul(w, w1)), mul(x, w2)), mul(z, w0));
            unsafe { wrf(this + QY, y_new) };
        }
        // else: zero or NaN angle, no-op.
        0
    }
});
