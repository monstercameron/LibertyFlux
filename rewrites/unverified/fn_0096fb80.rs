// original: 0x0096fb80 audio_spatial_update (proposed)

/// Update eight spatial-audio channels from emitter data and thread-local
/// listener state, writing per-channel gains and positions.
///
/// `this` points to the channel bank (`~0x2000` bytes). Thread-local slot 0
/// holds a block whose dword at `+TLS_ROW_OFF`, shifted by `TLS_ROW_SHIFT`,
/// selects a 64-byte row of the shared float table at `TLS_TABLE`; the
/// row's first three floats (`T0..T2`) seed every channel. Each of the 8
/// outer iterations processes one channel (`D` at `this+CH_OFF+4k`,
/// `E` at `this+8+0x30k`, `S` at `this+S_OFF+0x10k`).
///
/// Each channel runs three inner passes (`ebx` = -1, 0, 1) that call the
/// filter helper (thiscall on `this+FILT_OFF`, one float argument, float
/// result) twice: `fB` from table `A_OFF` at `(edi*3)*4` and `fA` from
/// table `B_OFF` at `edi*4`, where `edi` is `(ebx+outer) mod 8` (truncated,
/// so -1 occurs). Passes with `ebx != 0` scale both answers by `K_PASS`
/// and accumulate; the `ebx == 0` pass additionally builds two masked
/// direction vectors: each compares a length-squared against `K_TINY`
/// (strictly-greater wins; unordered takes the zero side) and `K_HUGE`
/// twice, normalises by the root, blends with the constant mask
/// `(0, 1, 0, 0)` lane-wise, and either spreads `1/sqrt(3)` (when
/// `fA+fB` is not above `K_S`) or combines the blends with `1/(fA+fB)`
/// shares. Lanes the original never stored (upper lanes of the mask and
/// work vectors, three scalar slots) read uninitialised stack, proven as
/// zero via the contract's stack fill. Accumulators add `(fA'+fB')` and
/// `(A17*fB'+acc) + A13*fA'` per pass in that order.
///
/// The channel is then finished: `p5 = acc1*K_OUT`, `p3 = acc2*K_OUT`,
/// `[D-0x20] = p5`, `[D] = p3/p5` unless `-p5` is below zero or NaN (then
/// `K_FALLBACK`), and `[S-0x84..-0x7c] = E[0..2]*[D] + T[0..2]`,
/// `[S-0x78] = 0`. Returns 8. Float operation order is the original's
/// (SSE scalar/packed, lane-exact); comparisons reproduce `comiss` NaN
/// behaviour (unordered takes `jbe`/`jb`, not `ja`).
///
/// Original: 0x0096fb80 (thiscall, no stack arguments, plain return).
lf_checker_rt::export!(thiscall, rw_0096fb80(this: u32) -> u32 {
    unsafe {
        const TLS_TABLE: u32 = 0x0115e420;
        const TLS_ROW_OFF: u32 = 0x70;
        const TLS_ROW_SHIFT: u32 = 6;
        const CH_OFF: u32 = 0x1c80;
        const S_OFF: u32 = 0x1d54;
        const FILT_OFF: u32 = 0x1ca0;
        const A_OFF: u32 = 0x17e0;
        const B_OFF: u32 = 0x1358;
        const G_XMM3: u32 = 0x017ad148;
        const G_TINY: u32 = 0x0110dad8;
        const G_HUGE1: u32 = 0x0110dad4;
        const G_HUGE2: u32 = 0x0110dad0;
        const G_MASK: u32 = 0x0110db50;
        const G_ONE: u32 = 0x00fe88e8;
        const G_PASS: u32 = 0x00fe8800;
        const G_S: u32 = 0x00fe870c;
        const G_SQRT3: u32 = 0x00fe8a94;
        const G_OUT: u32 = 0x00e8b7f8;
        const G_NEG: u32 = 0x00fe8d94;
        const G_FALLBACK: u32 = 0x00fe8b40;
        const CALLEE_FILT: u32 = 1;

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
        unsafe fn g32(va: u32) -> f32 {
            unsafe { f32::from_bits((lf_checker_rt::global::<u32>(va) as *const u32).read()) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn filt(obj: u32, x: f32) -> f32 {
            unsafe { lf_checker_rt::callee_thiscall!(CALLEE_FILT, f32, obj, x.to_bits()) }
        }
        /// Lane-wise `(v & m) | ((!m) & c)` over raw bit lanes.
        #[inline(always)]
        fn blend4(v: [u32; 4], m: [u32; 4], c: [u32; 4]) -> [u32; 4] {
            [
                (v[0] & m[0]) | ((!m[0]) & c[0]),
                (v[1] & m[1]) | ((!m[1]) & c[1]),
                (v[2] & m[2]) | ((!m[2]) & c[2]),
                (v[3] & m[3]) | ((!m[3]) & c[3]),
            ]
        }

        let tiny = g32(G_TINY);
        let huge1 = g32(G_HUGE1);
        let huge2 = g32(G_HUGE2);
        let one = g32(G_ONE);
        let k_pass = g32(G_PASS);
        let k_s = g32(G_S);
        let k_out = g32(G_OUT);
        let k_neg = g32(G_NEG);
        let k_fallback = g32(G_FALLBACK);
        let c3 = g32(G_XMM3);
        let mask0 = rd32(lf_checker_rt::relocated(G_MASK));
        let mask1 = rd32(lf_checker_rt::relocated(G_MASK).wrapping_add(4));
        let mask2 = rd32(lf_checker_rt::relocated(G_MASK).wrapping_add(8));
        let mask3 = rd32(lf_checker_rt::relocated(G_MASK).wrapping_add(12));
        let filt_obj = this.wrapping_add(FILT_OFF);

        // Thread-local listener row.
        let tls = lf_checker_rt::tls_slot(0);
        let row = rd32(tls.wrapping_add(TLS_ROW_OFF));
        let tbase = lf_checker_rt::relocated(TLS_TABLE).wrapping_add(row.wrapping_shl(TLS_ROW_SHIFT));
        let t0 = rdf(tbase);
        let t1 = rdf(tbase.wrapping_add(4));
        let t2 = rdf(tbase.wrapping_add(8));

        let mut d = this.wrapping_add(CH_OFF);
        let mut e = this.wrapping_add(8);
        let mut s = this.wrapping_add(S_OFF);
        let mut outer: i32 = 0;
        while outer < 8 {
            let mut acc1: f32 = 0.0;
            let mut acc2: f32 = 0.0;
            let mut ebx: i32 = -1;
            while ebx < 2 {
                // Truncated mod 8, as the original's and/jns/dec/or/inc.
                let edi: i32 = (ebx + outer) % 8;
                let a_arg = rdf(
                    (this as i32)
                        .wrapping_add(A_OFF as i32)
                        .wrapping_add(edi.wrapping_mul(12)) as u32,
                );
                let f_b: f32 = filt(filt_obj, a_arg);
                let b_arg = rdf(
                    (this as i32)
                        .wrapping_add(B_OFF as i32)
                        .wrapping_add(edi.wrapping_mul(4)) as u32,
                );
                let f_a: f32 = filt(filt_obj, b_arg);
                // Per-pass shares: scaled on ebx != 0, raw on ebx == 0.
                let (sh_b, sh_a) = if ebx != 0 {
                    (mul(f_b, k_pass), mul(f_a, k_pass))
                } else {
                    // First direction vector from (S-0x9c4, S-0x9c0).
                    let v6 = rdf(s.wrapping_sub(0x9c4));
                    let v5 = rdf(s.wrapping_sub(0x9c0));
                    let q = add(mul(v5, v5), mul(v6, v6));
                    let m0 = if q > tiny { c3 } else { 0.0 };
                    let m1 = if q > huge1 { c3 } else { 0.0 };
                    let m2 = if q > huge2 { c3 } else { 0.0 };
                    let r = core::hint::black_box(q).sqrt();
                    let k = div(one, r);
                    let blend0 = blend4(
                        [mul(v6, k).to_bits(), mul(v5, k).to_bits(), mul(k, 0.0).to_bits(), 0],
                        [m2.to_bits(), m1.to_bits(), m0.to_bits(), 0],
                        [mask0, mask1, mask2, mask3],
                    );
                    // Second direction vector from (S-0x844, S-0x840).
                    let v5b = rdf(s.wrapping_sub(0x844));
                    let v6b = rdf(s.wrapping_sub(0x840));
                    let q2 = add(mul(v5b, v5b), mul(v6b, v6b));
                    let n0 = if q2 > tiny { c3 } else { 0.0 };
                    let n1 = if q2 > huge1 { c3 } else { 0.0 };
                    let n3 = if q2 > huge2 { c3 } else { 0.0 };
                    let r2 = core::hint::black_box(q2).sqrt();
                    let k2 = div(one, r2);
                    let blend1 = blend4(
                        [mul(v5b, k2).to_bits(), mul(v6b, k2).to_bits(), mul(k2, 0.0).to_bits(), 0],
                        [n3.to_bits(), n1.to_bits(), n0.to_bits(), 0],
                        [mask0, mask1, mask2, mask3],
                    );
                    let sum = add(f_a, f_b);
                    if !(sum > k_s) {
                        let t = div(one, core::hint::black_box(g32(G_SQRT3)).sqrt());
                        wrf(s.wrapping_sub(4), t);
                        wrf(s, t);
                        wrf(s.wrapping_add(4), t);
                    } else {
                        let g = div(one, sum);
                        let g_a = mul(g, f_a);
                        let g_b = mul(g, f_b);
                        let b0 = f32::from_bits(blend1[0]);
                        let b1 = f32::from_bits(blend1[1]);
                        let b2 = f32::from_bits(blend1[2]);
                        let w0 = f32::from_bits(blend0[0]);
                        let o3 = add(mul(g_a, b1), mul(g_b, 0.0));
                        let o4 = add(mul(g_a, b0), mul(g_b, w0));
                        let o6 = add(mul(g_a, b2), mul(g_b, 0.0));
                        wrf(s.wrapping_add(8), 0.0);
                        wrf(s, o3);
                        wrf(s.wrapping_sub(4), o4);
                        wrf(s.wrapping_add(4), o6);
                    }
                    (f_b, f_a)
                };
                let a17 = rdf(
                    (this as i32)
                        .wrapping_add(A_OFF as i32)
                        .wrapping_add(edi.wrapping_mul(4)) as u32,
                );
                let a13 = rdf(
                    (this as i32)
                        .wrapping_add(B_OFF as i32)
                        .wrapping_add(edi.wrapping_mul(4)) as u32,
                );
                acc1 = add(add(sh_a, sh_b), acc1);
                acc2 = add(add(mul(a17, sh_b), acc2), mul(a13, sh_a));
                ebx += 1;
            }
            let p5 = mul(acc1, k_out);
            let p3 = mul(acc2, k_out);
            let n5 = mul(p5, k_neg);
            wrf(d.wrapping_sub(0x20), p5);
            let w3 = if n5 >= 0.0 { div(p3, p5) } else { k_fallback };
            wrf(d, w3);
            let e0 = add(mul(rdf(e.wrapping_sub(8)), w3), t0);
            let e1 = add(mul(rdf(e.wrapping_sub(4)), w3), t1);
            let e2 = add(mul(rdf(e), w3), t2);
            wrf(s.wrapping_sub(0x84), e0);
            wrf(s.wrapping_sub(0x80), e1);
            wrf(s.wrapping_sub(0x7c), e2);
            wrf(s.wrapping_sub(0x78), 0.0);
            d = d.wrapping_add(4);
            e = e.wrapping_add(0x30);
            s = s.wrapping_add(0x10);
            outer += 1;
        }
        8
    }
});
