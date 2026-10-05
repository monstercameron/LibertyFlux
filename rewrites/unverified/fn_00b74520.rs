// original: 0x00b74520 fall_blend_compute (proposed)

/// Compute a randomised fall-blend record into the output buffer.
///
/// `this` holds the source vectors (direction pair at `+DX`/`+DY`, basis
/// pair at `+BX`/`+BY`, position triple at `+PX`/`+PY`/`+PZ`, flag byte at
/// `+FLAG`); `out` receives a 0x40-byte record; `index` selects the
/// parameter entry (`TABLE[index]`).
///
/// Behaviour: the record is initialised from the source vectors with
/// zero padding, a 1.0 at `+0x28`, and the (zero) stack slot value at the
/// four `+0xC`-spaced tail words. When no direction flag bit is set, two
/// random values jitter the position's first two words
/// (`rng*K_A*K_B - K_C + current`, in that operand order). Otherwise a
/// non-negative magnitude is derived from the direction length and the
/// entry's range words, a normalised direction scaled by it is added to
/// (flag bit 3) or subtracted from the position triple. When the entry's
/// second range exceeds 8.0, a random angle feeds two trig kernels whose
/// results rotate the record's leading nine words; the `+0x20` triple
/// and then the leading triple are normalised (a zero squared length
/// yields a zero scale, never a division). Every float operation keeps
/// the original's operand order.
///
/// Returns nothing meaningful (callee residue plus flag bits); the
/// contract compares the record, not the return register.
///
/// Original: 0x00b74520 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00b74520(this: u32, out: u32, index: u32) -> u32 {
    unsafe {
        const DX: u32 = 0x14;
        const DY: u32 = 0x18;
        const BX: u32 = 0x0c;
        const BY: u32 = 0x10;
        const PX: u32 = 0x00;
        const PY: u32 = 0x04;
        const PZ: u32 = 0x08;
        const FLAG: u32 = 0x2a;
        const TABLE: u32 = 0x01295cd8;
        const RNG0: u32 = 0x00fe8684;
        const RNG1A: u32 = 0x00fe8898;
        const RNG1B: u32 = 0x00fe881c;
        const RNG2B: u32 = 0x00fe87e8;
        const RNG3B: u32 = 0x00eb2294;
        const RNG3C: u32 = 0x00e9b9c4;
        const K_HALF: u32 = 0x00fe8830;
        const K_ONE: u32 = 0x00fe88e8;
        const K_EIGHT: u32 = 0x00fe8afc;

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
        #[inline(always)]
        fn sqrt(a: f32) -> f32 {
            core::hint::black_box(a).sqrt()
        }
        // The original's normalise-or-zero idiom (ucomiss against zero,
        // lahf/test/jp): a zero squared length scales by zero, anything
        // else (including NaN) by one over its root.
        #[inline(always)]
        fn inv_len(n: f32) -> f32 {
            if n == 0.0 {
                0.0
            } else {
                div(1.0, sqrt(n))
            }
        }

        let entry = rd32(
            lf_checker_rt::relocated(TABLE).wrapping_add(index.wrapping_mul(4)),
        );
        let dx = rdf(this + DX);
        let dy = rdf(this + DY);
        wrf(out, dx);
        wrf(out + 0x0c, 0.0);
        wrf(out + 4, dy);
        wr32(out + 8, 0);
        wrf(out + 0x10, rdf(this + BX));
        wrf(out + 0x1c, 0.0);
        wrf(out + 0x14, rdf(this + BY));
        wr32(out + 0x18, 0);
        wrf(out + 0x2c, 0.0);
        wr32(out + 0x20, 0);
        wr32(out + 0x24, 0);
        wr32(out + 0x28, 0x3f80_0000);
        wrf(out + 0x30, rdf(this + PX));
        wrf(out + 0x34, rdf(this + PY));
        wrf(out + 0x38, rdf(this + PZ));
        wrf(out + 0x3c, 0.0);
        let flag = rd8(this + FLAG);
        if flag & 0x0c == 0 {
            for off in [0x30u32, 0x34] {
                let r = lf_checker_rt::callee_cdecl!(0, u32,) as i32 as f32;
                let j = sub(
                    mul(mul(r, rdf(lf_checker_rt::relocated(RNG0))), rdf(lf_checker_rt::relocated(RNG1A))),
                    rdf(lf_checker_rt::relocated(RNG1B)),
                );
                wrf(out + off, add(j, rdf(out + off)));
            }
        } else {
            let n1 = add(mul(dx, dx), mul(dy, dy));
            let d = sub(rdf(entry + 0x30), rdf(entry + 0x20));
            let mut x3 = mul(sub(sqrt(n1), d), rdf(lf_checker_rt::relocated(K_HALF)));
            let r = lf_checker_rt::callee_cdecl!(0, u32,) as i32 as f32;
            x3 = sub(
                x3,
                mul(mul(r, rdf(lf_checker_rt::relocated(RNG0))), rdf(lf_checker_rt::relocated(RNG2B))),
            );
            if x3 < 0.0 {
                x3 = 0.0;
            }
            let n2 = add(mul(dy, dy), mul(dx, dx));
            let inv = inv_len(n2);
            let x4 = mul(mul(dx, inv), x3);
            let x5 = mul(mul(dy, inv), x3);
            let x1 = mul(mul(inv, 0.0), x3);
            if flag & 8 != 0 {
                wrf(out + 0x30, add(x4, rdf(out + 0x30)));
                wrf(out + 0x34, add(x5, rdf(out + 0x34)));
                wrf(out + 0x38, add(x1, rdf(out + 0x38)));
            } else {
                wrf(out + 0x30, sub(rdf(out + 0x30), x4));
                wrf(out + 0x34, sub(rdf(out + 0x34), x5));
                wrf(out + 0x38, sub(rdf(out + 0x38), x1));
            }
        }
        let t = sub(rdf(entry + 0x34), rdf(entry + 0x24));
        if rdf(lf_checker_rt::relocated(K_EIGHT)) > t {
            let r = lf_checker_rt::callee_cdecl!(0, u32,) as i32 as f32;
            let ang = sub(
                mul(mul(r, rdf(lf_checker_rt::relocated(RNG0))), rdf(lf_checker_rt::relocated(RNG3B))),
                rdf(lf_checker_rt::relocated(RNG3C)),
            );
            let sin = f32::from_bits(lf_checker_rt::callee_cdecl!(1, u32, ang.to_bits()));
            let cos = f32::from_bits(lf_checker_rt::callee_cdecl!(2, u32, ang.to_bits()));
            let e2 = rdf(out + 4);
            let e4 = rdf(out + 0x14);
            let e1 = rdf(out);
            let e3 = rdf(out + 8);
            let e0 = rdf(out + 0x10);
            let e18 = rdf(out + 0x18);
            wrf(out + 0x10, mul(e0, sin));
            wrf(out + 0x14, mul(e4, sin));
            let x5 = add(mul(e0, cos), mul(e1, sin));
            wrf(out + 0x18, mul(e18, sin));
            let x4 = add(mul(e4, cos), mul(e2, sin));
            let x2 = add(mul(e18, cos), mul(e3, sin));
            wrf(out + 0x10, sub(mul(e0, sin), mul(e1, cos)));
            wrf(out + 0x14, sub(mul(e4, sin), mul(e2, cos)));
            wrf(out + 0x18, sub(mul(e18, sin), mul(e3, cos)));
            wrf(out, x5);
            wrf(out + 4, x4);
            wrf(out + 8, x2);
        }
        let mut x3 = rdf(out + 0x24);
        let mut x5 = rdf(out + 0x20);
        let mut x4 = rdf(out + 0x28);
        let n = add(add(mul(x5, x5), mul(x3, x3)), mul(x4, x4));
        let inv2 = inv_len(n);
        x4 = mul(x4, inv2);
        x3 = mul(x3, inv2);
        wrf(out + 0x28, x4);
        x5 = mul(x5, inv2);
        wrf(out + 0x24, x3);
        wrf(out + 0x20, x5);
        let v0 = sub(mul(rdf(out + 0x14), x4), mul(rdf(out + 0x18), x3));
        wrf(out, v0);
        let v4 = sub(mul(rdf(out + 0x20), rdf(out + 0x18)), mul(rdf(out + 0x10), rdf(out + 0x28)));
        wrf(out + 4, v4);
        // Note the order: the +0x10 word times +0x24, minus the +0x20
        // word times +0x14.
        let v8 = sub(mul(rdf(out + 0x10), rdf(out + 0x24)), mul(rdf(out + 0x20), rdf(out + 0x14)));
        wrf(out + 8, v8);
        let n3 = add(add(mul(v0, v0), mul(v4, v4)), mul(v8, v8));
        let inv3 = inv_len(n3);
        let w8 = mul(v8, inv3);
        let w4 = mul(v4, inv3);
        wrf(out + 8, w8);
        let w0 = mul(v0, inv3);
        wrf(out + 4, w4);
        wrf(out, w0);
        wrf(out + 0x10, sub(mul(w8, rdf(out + 0x24)), mul(w4, rdf(out + 0x28))));
        wrf(out + 0x14, sub(mul(rdf(out + 0x28), w0), mul(rdf(out + 0x20), w8)));
        wrf(out + 0x18, sub(mul(rdf(out + 0x20), w4), mul(rdf(out + 0x24), w0)));
        0
    }
});
