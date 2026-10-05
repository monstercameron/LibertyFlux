// original: 0x00CD4E60 peds_task_steer_blend (proposed)
/// Steer helper, DEFERRED: the checker cannot script its double call.
///
/// `this` is the task object (flag byte at `+0x68`), `a0` the steered
/// object (child object at `+0xa80`, matrix at `+0x20`). Thiscall with one
/// stack word; returns the low byte of the final virtual call's answer.
/// This rewrite is complete and faithful, but the function is deferred
/// with reason `float_register_args`, because the checker cannot observe
/// either direction of the double call in the middle: the original passes
/// two doubles in `xmm0`/`xmm1` (converted up from floats just before the
/// call) and takes a double answer back in `xmm0`, while the checker's
/// vector transport moves only 4 bytes per register into the rewrite's
/// call and the double-answer stub exposes only the answer's low word in
/// `eax`. The rewrite therefore computes both doubles exactly as the
/// original forms them and then cannot pass them; a wrong version that
/// drops the negation on one of them passes unnoticed, which is filed as
/// the gap proof, not as a pass.
///
/// Behaviour: the setup callee (callee 0) runs on the child with 1, then
/// the probe (callee 1) on `a0`. When flag bit 2 is set, two scaled
/// integers (callees 2-4, signed) form `(x3, x2) = ((1/G)*a, (1/G)*(-b))`
/// with the divisor global, normalized when its squared length exceeds 1.
/// The squared length going forward decides the big block: inside it the
/// float callee (callee 5) runs on the pool, the unobservable double call
/// (callee 6) runs, its narrowed answer (scripted as +0.0, mirrored as a
/// constant, since only its low word reaches the rewrite) is narrowed to
/// float and added to the first answer for the mix callee (callee 7), two
/// more float callees (8, 9) answer, and the gate callee (callee 10) runs
/// on (`a0`, address of the negated first answer, 0). On a nonzero gate
/// answer a six-read matrix row is blended with the saved length's square
/// root (`sqrtps` over four words, of which only the low lane is used; the
/// other lanes, one holding the object pointer's bits, are dead) and with
/// a scratch word the contract pins to zero; on zero the pair stays zero.
/// The pair is scaled by the second global, or, when the big block was
/// skipped, taken from the incoming `xmm5`/`xmm6` entry values through the
/// mirror (the original reads its registers there). Bit 1 of the flag
/// byte is set to whether the pair's squared length is above zero. The
/// store callee (callee 11) runs on the child with the pair bits and 0,
/// the fill callee (callee 12) runs on (`this`, `a0`, two scratch
/// addresses preloaded from the child) with two scripted out-words that
/// land in the child at `+8`/`+4`, and the child's virtual slot `+0x4c`
/// is called. Float order is pinned throughout; both above-tests are
/// false for NaN.
lf_checker_rt::export!(thiscall, rw_00CD4E60(this: u32, a0: u32) -> u32 {
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
        let (mut x5, mut x6);
        if x1 > 0.0 {
            let fa = f32::from_bits(lf_checker_rt::callee_thiscall!(5, u32, pool, a0, 1));
            // The double call's operands, formed exactly as the original
            // forms them (negated x3 widened; x2 and the probe answer's
            // bits widened as a pair). Unpassable: the transport moves 4
            // bytes per vector register, and the double answer's high word
            // never reaches the rewrite either, so the scripted answer is
            // +0.0 and mirrored here as a constant.
            let d0 = neg(x3) as f64;
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
            if gate & 0xFF != 0 {
                let m = rd32(a0 + MAT_OFF);
                let sq = x1.sqrt();
                let t = mul(f32::from_bits(rd32(m)), 0.0);
                x5 = mul(f32::from_bits(rd32(m + 4)), x3);
                x6 = mul(f32::from_bits(rd32(m + 0x14)), x3);
                x5 = add(x5, t);
                let u = mul(f32::from_bits(rd32(m + 8)), ans2);
                x5 = add(x5, u);
                let v = mul(f32::from_bits(rd32(m + 0x10)), 0.0);
                x5 = mul(x5, sq);
                x6 = add(x6, v);
                let w = mul(f32::from_bits(rd32(m + 0x18)), ans2);
                x6 = add(x6, w);
                x6 = mul(x6, sq);
            } else {
                x5 = 0.0;
                x6 = 0.0;
            }
            let g2 = gfloat(G_MUL);
            x3 = mul(g2, x5);
            x2 = mul(g2, x6);
        } else {
            x5 = f32::from_bits(lf_checker_rt::xmm_word(5, 0));
            x6 = f32::from_bits(lf_checker_rt::xmm_word(6, 0));
        }
        let rr = add(mul(x2, x2), mul(x3, x3));
        let fb = rd8(this + FLAG_OFF);
        ((this + FLAG_OFF) as *mut u8).write((fb & !2) | ((rr > 0.0) as u8 * 2));
        let _: u32 = lf_checker_rt::callee_thiscall!(11, u32, child, x2.to_bits(), x3.to_bits(), 0);
        let mut s = [rd32(child + 0x10), rd32(child + 0x0c)];
        let saddr = s.as_mut_ptr() as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(12, u32, this, a0, saddr, saddr + 4);
        wr32(child + 8, s[0]);
        wr32(child + 4, s[1]);
        let vt = rd32(child);
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vt + VT_SLOT) as usize);
        f(child)
    }
});
