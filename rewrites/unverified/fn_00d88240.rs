// original: 0x00d88240 ui_hit_chain (proposed)

/// Run a chain of sixteen separator tests over rotated offset vectors,
/// returning 1 at the first test that fires and 0 when none does.
///
/// Arguments: `angle` (radians, as bits), eight pointers to two-float
/// vectors (`a1`..`a8`), two offset floats `w1`/`w2` (as bits), a scale
/// `sc` (as bits) and a flag word whose low byte selects the vector
/// assignment. The original also writes its updated offsets back into its
/// two incoming float slots; those two words are the only stack comparison
/// this proof narrows (see the contract), because a Rust rewrite cannot
/// address its own argument slots. The scale multiplier (1.7) is read from
/// the image's writable data, the sign mask from its read-only constants.
///
/// Algorithm: rotate: `w1 -= cos(angle)*sc`, then `w1 *= K` and
/// `w2 = (w2 - sin(angle)*sc) * K` with K the data multiplier; when the
/// flag byte is nonzero negate both. The flag also swaps which argument
/// feeds each of the two probe vectors, the direction vector, the anchor
/// and the two side vectors. Sixteen calls to the separator callee follow,
/// each taking eight floats (two probe-vector halves plus six frame
/// values); the first nine use the anchor-side values, the last seven
/// reuse the second-half accumulators. Each call's low answer byte is
/// tested: nonzero returns 1 at once, otherwise the chain continues and
/// the last answer decides. All float arithmetic keeps the original's
/// operand order; the cosine/sine callees take their argument in XMM0 and
/// return it there (the stub mirrors the bits to EAX, which is what the
/// rewrite reads).
///
/// Original: 0x00D88240 (cdecl, thirteen stack words; low byte result).
lf_checker_rt::export!(cdecl, rw_00d88240(angle: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, w1b: u32, w2b: u32, scb: u32, flag: u32) -> u32 {
    unsafe {
        const CAL_COS: u32 = 1;
        const CAL_SIN: u32 = 2;
        const CAL_SEP: u32 = 3;
        const C_MULT: u32 = 0x01056d58;
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
        fn bb(x: f32) -> f32 {
            core::hint::black_box(x)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            bb(a) * bb(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            bb(a) + bb(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            bb(a) - bb(b)
        }
        #[inline(always)]
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN)
        }
        #[inline(always)]
        unsafe fn sep(b0: u32, b1: u32, b2: u32, b3: u32, b4: u32, b5: u32, b6: u32, b7: u32) -> u32 {
            unsafe { lf_checker_rt::callee_cdecl!(CAL_SEP, u32, b0, b1, b2, b3, b4, b5, b6, b7) }
        }

        let sc = f32::from_bits(scb);
        let cos = f32::from_bits(lf_checker_rt::callee_cdecl!(CAL_COS, u32, angle));
        let mut w1 = sub(f32::from_bits(w1b), mul(cos, sc));
        let sin = f32::from_bits(lf_checker_rt::callee_cdecl!(CAL_SIN, u32, angle));
        let t2 = mul(sin, sc);
        let mut w2 = f32::from_bits(w2b);
        let k = rdf(lf_checker_rt::relocated(C_MULT));
        w1 = mul(w1, k);
        w2 = sub(w2, t2);
        w2 = mul(w2, k);
        // Vector assignment; the flag also negates the rotated offsets.
        let (d, a, cc, s98, s9c, esi, edi);
        if (flag & 0xFF) == 0 {
            d = a5;
            a = a1;
            cc = a2;
            s98 = a7;
            s9c = a8;
            esi = a3;
            edi = a4;
        } else {
            w1 = neg(w1);
            w2 = neg(w2);
            d = a1;
            a = a5;
            cc = a6;
            s98 = a3;
            s9c = a4;
            esi = a7;
            edi = a8;
        }
        let s40 = if (flag & 0xFF) == 0 { a6 } else { a2 };
        let a0 = rd32(a);
        let a1v = rd32(a + 4);
        let c0 = rd32(cc);
        let c1 = rd32(cc + 4);
        let e0 = rd32(esi);
        let e1 = rd32(esi + 4);
        let f0 = rd32(edi);
        let f1v = rd32(edi + 4);
        let d0 = rdf(d);
        let d1 = rdf(d + 4);
        // Block 1.
        let x0a = add(w1, d0);
        let x3a = add(rdf(s40), d0);
        let x4a = add(rdf(s40 + 4), d1);
        let x0b = add(w2, d1);
        let x0c = add(x3a, w1);
        let x1a = sub(x3a, d0);
        let x0d = add(x4a, w2);
        let x0e = sub(x4a, d1);
        if sep(a0, a1v, c0, c1, d0.to_bits(), d1.to_bits(), x0e.to_bits(), x1a.to_bits()) & 0xFF != 0 {
            return 1;
        }
        // Blocks 2-4: differences of block 1 values.
        let d0b2 = sub(x0b, x4a);
        let d1b2 = sub(x0a, x3a);
        if sep(a0, a1v, c0, c1, x3a.to_bits(), x4a.to_bits(), d1b2.to_bits(), d0b2.to_bits()) & 0xFF != 0 {
            return 1;
        }
        let d0b3 = sub(x0d, x0b);
        let d1b3 = sub(x0c, x0a);
        if sep(a0, a1v, c0, c1, x0a.to_bits(), x0b.to_bits(), d1b3.to_bits(), d0b3.to_bits()) & 0xFF != 0 {
            return 1;
        }
        let d0b4 = sub(d1, x0d);
        let d1b4 = sub(d0, x0c);
        if sep(a0, a1v, c0, c1, x0c.to_bits(), x0d.to_bits(), d1b4.to_bits(), d0b4.to_bits()) & 0xFF != 0 {
            return 1;
        }
        // Blocks 5-8: side vectors against blocks 1-4 values.
        if sep(e0, e1, f0, f1v, d0.to_bits(), d1.to_bits(), x1a.to_bits(), x0e.to_bits()) & 0xFF != 0 {
            return 1;
        }
        if sep(e0, e1, f0, f1v, x3a.to_bits(), x4a.to_bits(), d1b2.to_bits(), d0b2.to_bits()) & 0xFF != 0 {
            return 1;
        }
        if sep(e0, e1, f0, f1v, x0a.to_bits(), x0b.to_bits(), d1b3.to_bits(), d0b3.to_bits()) & 0xFF != 0 {
            return 1;
        }
        if sep(e0, e1, f0, f1v, x0c.to_bits(), x0d.to_bits(), d1b4.to_bits(), d0b4.to_bits()) & 0xFF != 0 {
            return 1;
        }
        // Block 9: second-half accumulators from the saved pair.
        let v0 = rdf(s98);
        let v1 = rdf(s98 + 4);
        let w0 = rdf(s9c);
        let w1v = rdf(s9c + 4);
        let w1v1 = add(w1v, v1);
        let n0d = add(w1, v0);
        let n4a = add(v0, w0);
        let n0a = add(w1v1, w2);
        let w1sub = sub(w1v1, v1);
        let n0b = add(n4a, w1);
        let w0sub = sub(n4a, v0);
        if sep(a0, a1v, c0, c1, v0.to_bits(), v1.to_bits(), w0sub.to_bits(), w1sub.to_bits()) & 0xFF != 0 {
            return 1;
        }
        // Blocks 10-12: differences of block 9 values.
        let n0c = add(w2, v1);
        let d0b10 = sub(n0c, w1v1);
        let d1b10 = sub(n0d, n4a);
        if sep(a0, a1v, c0, c1, n4a.to_bits(), w1v1.to_bits(), d1b10.to_bits(), d0b10.to_bits()) & 0xFF != 0 {
            return 1;
        }
        let d0b11 = sub(n0a, n0c);
        let d1b11 = sub(n0b, n0d);
        if sep(a0, a1v, c0, c1, n0d.to_bits(), n0c.to_bits(), d1b11.to_bits(), d0b11.to_bits()) & 0xFF != 0 {
            return 1;
        }
        let d0b12 = sub(v1, n0a);
        let d1b12 = sub(v0, n0b);
        if sep(a0, a1v, c0, c1, n0b.to_bits(), n0a.to_bits(), d1b12.to_bits(), d0b12.to_bits()) & 0xFF != 0 {
            return 1;
        }
        // Blocks 13-16: side vectors against blocks 9-12 values.
        if sep(e0, e1, f0, f1v, v0.to_bits(), v1.to_bits(), w0sub.to_bits(), w1v1.to_bits()) & 0xFF != 0 {
            return 1;
        }
        if sep(e0, e1, f0, f1v, n4a.to_bits(), w1v1.to_bits(), d1b10.to_bits(), d0b10.to_bits()) & 0xFF != 0 {
            return 1;
        }
        if sep(e0, e1, f0, f1v, n0d.to_bits(), n0c.to_bits(), d1b11.to_bits(), d0b11.to_bits()) & 0xFF != 0 {
            return 1;
        }
        let last: u32 = sep(e0, e1, f0, f1v, n0b.to_bits(), n0a.to_bits(), d1b12.to_bits(), d0b12.to_bits());
        ((last & 0xFF) != 0) as u32
    }
});
