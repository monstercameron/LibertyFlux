// Wrong version of fn_00CD4E60: the double-input negation is dropped.
// It must PASS: the doubles are unpassable, so no check can see them.
lf_checker_rt::export!(thiscall, mut_00CD4E60(this: u32, a0: u32) -> u32 {
    unsafe {
        const CHILD_OFF: u32 = 0xa80;
        const FLAG_OFF: u32 = 0x68;
        const MAT_OFF: u32 = 0x20;
        const G_DIV: u32 = 0x01051954;
        const G_MUL: u32 = 0x0105195C;
        const ONE: u32 = 0x00FE88E8;
        const POOL: u32 = 0x0128E310;
        const SIGN: u32 = 0x80000000;
        const VT_SLOT: u32 = 0x4c;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        fn gfloat(va: u32) -> f32 {
            f32::from_bits(unsafe { (lf_checker_rt::global::<u32>(va) as *const u32).read_unaligned() })
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
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN)
        }

        let pool = lf_checker_rt::relocated(POOL);
        let child = rd32(a0 + CHILD_OFF);
        let _: u32 = lf_checker_rt::callee_thiscall!(0, u32, child, 1);
        let r: u32 = lf_checker_rt::callee_thiscall!(1, u32, a0);
        let (mut x3, mut x2) = (0.0f32, 0.0f32);
        if rd8(this + FLAG_OFF) & 4 != 0 {
            let r2: u32 = lf_checker_rt::callee_thiscall!(2, u32, r);
            let r3: u32 = lf_checker_rt::callee_cdecl!(3, u32, r2);
            let fr3 = (r3 as i32) as f32;
            let r4: u32 = lf_checker_rt::callee_thiscall!(4, u32, r);
            let r5: u32 = lf_checker_rt::callee_cdecl!(3, u32, r4);
            let inv = div(gfloat(ONE), gfloat(G_DIV));
            let mut t2 = mul(inv, neg((r5 as i32) as f32));
            let mut t3 = mul(inv, fr3);
            let norm = add(mul(t2, t2), mul(t3, t3));
            if norm > gfloat(ONE) {
                let inv_n = div(1.0, norm.sqrt());
                t2 = mul(t2, inv_n);
                t3 = mul(t3, inv_n);
            }
            x3 = t3;
            x2 = t2;
        }
        let x1 = add(mul(x2, x2), mul(x3, x3));
        let ran_big = x1 > 0.0;
        if ran_big {
            let fa = f32::from_bits(lf_checker_rt::callee_thiscall!(5, u32, pool, a0, 1));
            // The double call's operands, formed exactly as the original
            // forms them (negated x3 widened; x2 and the probe answer's
            // bits widened as a pair). Unpassable: the transport moves 4
            // bytes per vector register, and the double answer's high word
            // never reaches the rewrite either, so the scripted answer is
            // +0.0 and mirrored here as a constant.
            let d0 = x3 as f64; // negation dropped (mutant: must PASS to prove the gap)
            let d1 = (x2 as f64, f32::from_bits(r) as f64);
            let _ = (d0, d1);
            let _dans: u32 = lf_checker_rt::callee_cdecl!(6, u32,);
            let mixed = add(fa, 0.0);
            let fb = f32::from_bits(lf_checker_rt::callee_thiscall!(7, u32, 0, mixed.to_bits()));
            // Both float callees take fb in xmm0 (logged and transported).
            let a1: u32 = lf_checker_rt::callee_cdecl!(8, u32, fb.to_bits());
            let neg1 = neg(f32::from_bits(a1));
            let a2: u32 = lf_checker_rt::callee_cdecl!(9, u32, fb.to_bits());
            let ans2 = f32::from_bits(a2);
            let gate: u32 = lf_checker_rt::callee_cdecl!(10, u32, a0, (&neg1 as *const f32) as u32, 0);
            let (mut x5, mut x6);
            if gate & 0xFF != 0 {
                // Note the frame shift: the gate call's argument cleanup
                // leaves esp four above the base, so every slot below sits
                // four higher than in the pre-gate code, and the square
                // root takes the stored object pointer's bits (its low
                // lane; the other three sqrtps lanes are dead).
                let m = rd32(a0 + MAT_OFF);
                let sq = f32::from_bits(this).sqrt();
                let t = mul(f32::from_bits(rd32(m)), neg1);
                x5 = mul(f32::from_bits(rd32(m + 4)), ans2);
                x6 = mul(f32::from_bits(rd32(m + 0x14)), ans2);
                x5 = add(x5, t);
                let u = mul(f32::from_bits(rd32(m + 8)), 0.0);
                x5 = add(x5, u);
                let v = mul(f32::from_bits(rd32(m + 0x10)), neg1);
                x5 = mul(x5, sq);
                x6 = add(x6, v);
                let w = mul(f32::from_bits(rd32(m + 0x18)), 0.0);
                x6 = add(x6, w);
                x6 = mul(x6, sq);
            } else {
                x5 = 0.0;
                x6 = 0.0;
            }
            let g2 = gfloat(G_MUL);
            x3 = mul(g2, x5);
            x2 = mul(g2, x6);
        }
        // Skipped (or flag clear): x3/x2 stay as computed above.
        let rr = add(mul(x2, x2), mul(x3, x3));
        let fb = rd8(this + FLAG_OFF);
        ((this + FLAG_OFF) as *mut u8).write((fb & !2) | ((rr > 0.0) as u8 * 2));
        let _: u32 = lf_checker_rt::callee_thiscall!(11, u32, child, x2.to_bits(), x3.to_bits());
        let mut s = [rd32(child + 0x10), rd32(child + 0x0c)];
        let saddr = s.as_mut_ptr() as u32;
        // ecx is whatever sits four below the gate call's cleanup: `this`
        // when the big block ran (its stored slot), else the saved
        // squared length (zeros when the flag was clear).
        let ecx12 = if ran_big { this } else { x1.to_bits() };
        let _: u32 = lf_checker_rt::callee_thiscall!(12, u32, ecx12, a0, saddr, saddr + 4);
        wr32(child + 8, s[0]);
        wr32(child + 4, s[1]);
        let vt = rd32(child);
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vt + VT_SLOT) as usize);
        f(child)
    }
});
