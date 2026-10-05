// original: 0x00dc37f0 ped_task_score_update (proposed)

/// Score a ped task target: pick one of four paths from global switches,
/// compute a distance or call-score into the output slot, then run a shared
/// projection/clamp tail and return a status code.
///
/// Arguments: `a1` is the key object (its value selects paths A/B, word at
/// `+4` is a masked table index, word at `+0` bit 12 selects paths C/D);
/// `a2` is a scratch object (float result at `+0`, table base at `+0x64`);
/// `a3` is only forwarded to the gate callee; `a4` is a three-float vector;
/// `a5` is the output object (score at `+0`); `a6` is a second output
/// (word at `+0` set on path C). `a0` is never read. The global object at
/// `GOBJ` supplies flag bits and reference vectors; a fixed manager address
/// is the `this` pointer of most calls.
///
/// Behaviour: if the gate callee answers zero, return 2. Otherwise a dword
/// and a byte switch choose the A/B region (key match plus a check call, or
/// the solve-eight path) or the C/D region (direct norm plus marker word,
/// or the solve-seven path). Every path stores a non-negative-root score
/// (`1/sqrt(x)` when `x > 0`, else `x`) into `a5`, then the tail subtracts
/// it from the reference at `+0x84`, clamps at zero, projects the `a4`
/// vector onto the reference segment with a clamped factor, and adds a
/// capped surplus when the projected distance exceeds the direct one.
/// Returns 0, except path B which returns whether the solve-eight answer's
/// low byte was zero. On path B that low byte lives in a scratch slot and is
/// only tested for the return code; the reference pointer stays the
/// global object on every path. The high bytes of the
/// flag word passed to the solve-eight callee are read before ever being
/// written on the mismatch route; the contract fills uninitialised stack
/// with zero, so they read as zero here.
///
/// Calling convention: cdecl, seven stack words. The float operation order
/// is the original's.
lf_checker_rt::export!(cdecl, rw_00dc37f0(_a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    unsafe {
        const THIS_C: u32 = 0x016C_6830;
        const GOBJ: u32 = 0x016C_7448;
        const G_DW: u32 = 0x0104_827C;
        const G_BY: u32 = 0x016B_6B23;
        const G_K0: u32 = 0x017A_2340;
        const G_K1: u32 = 0x017A_2344;
        const G_CD: u32 = 0x0104_8272;
        const G_V0: u32 = 0x017A_22E0;
        const G_V1: u32 = 0x017A_22E4;
        const G_V2: u32 = 0x017A_22E8;
        const FZERO: u32 = 0x00FE_8628;
        const FONE: u32 = 0x00FE_88E8;
        const FK25: u32 = 0x00FE_87E4;
        const FK50: u32 = 0x00FE_8B68;
        const IMM_A: u32 = 0x017A_1DD0;
        const IMM_B: u32 = 0x017A_22E0;

        const C_GATE: u32 = 1;
        const C_CHK: u32 = 2;
        const C_PAIR: u32 = 3;
        const C_SOL7: u32 = 4;
        const C_SQRT: u32 = 5;
        const C_SOL8: u32 = 6;
        const C_APPLY: u32 = 7;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn gf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(a))) }
        }
        #[inline(always)]
        unsafe fn gu32(a: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(a)) }
        }
        #[inline(always)]
        unsafe fn gu8(a: u32) -> u8 {
            unsafe { rd8(lf_checker_rt::relocated(a)) }
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
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        let this_c = lf_checker_rt::relocated(THIS_C);
        let g = gu32(GOBJ);
        let imm_a = lf_checker_rt::relocated(IMM_A);
        let imm_b = lf_checker_rt::relocated(IMM_B);
        let zero = gf(FZERO);
        let one = gf(FONE);

        let gate: u32 = lf_checker_rt::callee_thiscall!(C_GATE, u32, this_c, g, a3);
        if gate as u8 == 0 {
            return 2;
        }

        // Frame slots: l1 starts as the global object, l2 starts zero.
        let l1: u32 = g;
        let mut l2: u32 = 0;
        let edx: u32;
        if gu32(G_DW) == 0 || gu8(G_BY) != 0 {
            if gu8(G_CD) != 0 && ((rd32(a1) >> 12) & 1) == 0 {
                // Path C: direct norm plus marker.
                lf_checker_rt::callee_thiscall!(C_APPLY, u32, a2, a1, a4);
                let d0 = sub(rdf(g.wrapping_add(0x50)), rdf(a4));
                let d1 = sub(rdf(g.wrapping_add(0x54)), rdf(a4.wrapping_add(4)));
                let d2 = sub(rdf(g.wrapping_add(0x58)), rdf(a4.wrapping_add(8)));
                let n = add(add(mul(d0, d0), mul(d1, d1)), mul(d2, d2));
                wrf(a5, n);
                wr32(a6, 0x7f);
            } else {
                // Path D: solve-seven score.
                lf_checker_rt::callee_cdecl!(C_PAIR, u32, a2, a1);
                let idx = rd32(a1.wrapping_add(4)) & 0x1ffff;
                let der = rd32(a2.wrapping_add(0x64)).wrapping_add(idx.wrapping_mul(8));
                let r: f32 = lf_checker_rt::callee_thiscall!(C_SOL7, f32, this_c, a1, imm_a, der,
                    g.wrapping_add(0x50), a4, a6, 0u32);
                wrf(a5, r);
            }
            let x = rdf(a5);
            if x > zero {
                let s: f32 = lf_checker_rt::callee_cdecl!(C_SQRT, f32, x.to_bits());
                wrf(a5, div(one, s));
            }
            edx = 0;
        } else if a1 == gu32(G_K0) && a1 == gu32(G_K1) {
            l2 = g.wrapping_add(0x40);
            let bit = (rd32(g.wrapping_add(0x14)) >> 9) & 1;
            let t: u32 = lf_checker_rt::callee_thiscall!(C_CHK, u32, this_c,
                g.wrapping_add(0x40), g.wrapping_add(0x50), bit);
            if (t as u8) != 0 {
                // Path B through the failed check.
                let al = (!rd8(a1)) & 1;
                lf_checker_rt::callee_cdecl!(C_PAIR, u32, a2, a1);
                l2 = (l2 & 0xffff_ff00) | al as u32;
                let idx = rd32(a1.wrapping_add(4)) & 0x1ffff;
                let der = rd32(a2.wrapping_add(0x64)).wrapping_add(idx.wrapping_mul(8));
                let o: u32 = lf_checker_rt::callee_thiscall!(C_SOL8, u32, this_c, a1, imm_a, der,
                    imm_b, g.wrapping_add(0x50), a4, a6, l2);
                let d0 = sub(rdf(a4), rdf(g.wrapping_add(0x50)));
                let d1 = sub(rdf(a4.wrapping_add(4)), rdf(g.wrapping_add(0x54)));
                let d2 = sub(rdf(a4.wrapping_add(8)), rdf(g.wrapping_add(0x58)));
                let n = add(add(mul(d0, d0), mul(d1, d1)), mul(d2, d2));
                wrf(a5, n);
                let x = rdf(a5);
                if x > zero {
                    let s: f32 = lf_checker_rt::callee_cdecl!(C_SQRT, f32, x.to_bits());
                    wrf(a5, div(one, s));
                }
                edx = if (o & 0xff) == 0 { 1 } else { 0 };
            } else {
                // Path A: solve-seven score with the saved slot.
                lf_checker_rt::callee_cdecl!(C_PAIR, u32, a2, a1);
                let idx = rd32(a1.wrapping_add(4)) & 0x1ffff;
                let der = rd32(a2.wrapping_add(0x64)).wrapping_add(idx.wrapping_mul(8));
                let r: f32 = lf_checker_rt::callee_thiscall!(C_SOL7, f32, this_c, a1, imm_a, der,
                    l2, a4, a6, 0u32);
                wrf(a5, r);
                let x = rdf(a5);
                if x > zero {
                    let s: f32 = lf_checker_rt::callee_cdecl!(C_SQRT, f32, x.to_bits());
                    wrf(a5, div(one, s));
                }
                edx = 0;
            }
        } else {
            // Path B through the key mismatch.
            let al = (!rd8(a1)) & 1;
            lf_checker_rt::callee_cdecl!(C_PAIR, u32, a2, a1);
            l2 = (l2 & 0xffff_ff00) | al as u32;
            let idx = rd32(a1.wrapping_add(4)) & 0x1ffff;
            let der = rd32(a2.wrapping_add(0x64)).wrapping_add(idx.wrapping_mul(8));
            let o: u32 = lf_checker_rt::callee_thiscall!(C_SOL8, u32, this_c, a1, imm_a, der,
                imm_b, g.wrapping_add(0x50), a4, a6, l2);
            let d0 = sub(rdf(a4), rdf(g.wrapping_add(0x50)));
            let d1 = sub(rdf(a4.wrapping_add(4)), rdf(g.wrapping_add(0x54)));
            let d2 = sub(rdf(a4.wrapping_add(8)), rdf(g.wrapping_add(0x58)));
            let n = add(add(mul(d0, d0), mul(d1, d1)), mul(d2, d2));
            wrf(a5, n);
            let x = rdf(a5);
            if x > zero {
                let s: f32 = lf_checker_rt::callee_cdecl!(C_SQRT, f32, x.to_bits());
                wrf(a5, div(one, s));
            }
            edx = if (o & 0xff) == 0 { 1 } else { 0 };
        }

        // Shared tail: clamp, project, capped surplus.
        let g0 = gf(G_V0);
        let g1 = gf(G_V1);
        let g2 = gf(G_V2);
        let mut x = sub(rdf(l1.wrapping_add(0x84)), rdf(a5));
        if !(x > 0.0) {
            x = 0.0;
        }
        wrf(a5, x);
        let t54 = rdf(l1.wrapping_add(0x54));
        let a0v = rdf(a4);
        let a1v = rdf(a4.wrapping_add(4));
        let t50 = rdf(l1.wrapping_add(0x50));
        let e0 = sub(t50, g0);
        let mut f0 = sub(a0v, g0);
        let mut f1 = sub(a1v, g1);
        let l2slot = t50;
        let mut f2 = sub(rdf(a4.wrapping_add(8)), g2);
        let l1slot = t54;
        let mut e1 = sub(t54, g1);
        let mut p0 = mul(e0, f0);
        let t58 = rdf(l1.wrapping_add(0x58));
        e1 = mul(e1, f1);
        let mut n1 = mul(f1, f1);
        e1 = add(e1, p0);
        p0 = sub(t58, g2);
        let l0slot = t58;
        p0 = mul(p0, f2);
        e1 = add(e1, p0);
        p0 = mul(f0, f0);
        n1 = add(n1, p0);
        p0 = mul(f2, f2);
        n1 = add(n1, p0);
        let mut t = div(e1, n1);
        if 0.0 > t {
            t = 0.0;
        } else if t > one {
            t = one;
        }
        f1 = mul(f1, t);
        f0 = mul(f0, t);
        f1 = add(f1, g1);
        let mut q0 = sub(g1, l1slot);
        f0 = add(f0, g0);
        let mut q1 = sub(g0, l2slot);
        f2 = mul(f2, t);
        f1 = sub(f1, l1slot);
        f0 = sub(f0, rdf(l1.wrapping_add(0x50)));
        f2 = add(f2, g2);
        let mut q2 = sub(g2, l0slot);
        q0 = mul(q0, q0);
        q1 = mul(q1, q1);
        f2 = sub(f2, rdf(l1.wrapping_add(0x58)));
        f1 = mul(f1, f1);
        f0 = mul(f0, f0);
        q0 = add(q0, q1);
        q2 = mul(q2, q2);
        f1 = add(f1, f0);
        f2 = mul(f2, f2);
        q0 = add(q0, q2);
        f1 = add(f1, f2);
        let s0 = core::hint::black_box(q0).sqrt();
        let s1 = core::hint::black_box(f1).sqrt();
        if s0 > s1 {
            let d = sub(s0, s1);
            let cap = mul(rdf(l1.wrapping_add(0x84)), gf(FK25));
            let mut dd = mul(d, gf(FK50));
            if !(cap > dd) {
                dd = cap;
            }
            x = add(x, dd);
            wrf(a5, x);
        }
        edx
    }
});
