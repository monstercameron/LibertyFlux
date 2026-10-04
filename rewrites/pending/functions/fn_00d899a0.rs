// original: 0x00d899a0 ui_candidate_pair_adjust (proposed)

/// Match two candidate objects, adjust two stored angles over up to eight
/// rounds, and report the outcome through flag bytes.
///
/// `a` and `b` are actor objects (vtable at `+0`, matrix pointer at `+0x20`,
/// mode byte at `+0xe6e`, sub-object at `+0xe48`); `p1` and `p0` point to the
/// two angle floats; `f18`/`f1c` are flag bytes; `mode` selects the round
/// count; `pf`/`pg` receive the outcome byte and `ph` gates entry and
/// records a strong match.
///
/// Entry: when `mode` is 0 and `f1c` is 0 nothing happens, as when `*ph` is
/// already set. Then type gates run: for `b` modes 2, 7 and 6 callee 0 is
/// asked about `b+0xe48` and a match with `a` ends the call, and for `a`
/// modes 0xa, 0xb, 0xd and 0xc a match with `b` does the same.
///
/// The core compares the XY distance between the two matrices against a
/// direction from callee 1 (negated when bit 0 of `b+0xe73` is set): a
/// negative dot product ends the call, and in mode 1 with `f18` clear a
/// second direction is checked against 0.7, with a normalise-and-recheck
/// step through callee 2 on the way past. Four probe blocks follow the same
/// shape: two virtual calls (slots `+0x60`/`+0x64`, callee ids 7 and 8) pick
/// a float pair around 0.25, callee 3..6 transforms it with the matrix, and
/// a matrix row scaled by a third virtual-call difference plus 0.5 lands in
/// a result buffer. A fifth comparison sets a flag word from two more
/// virtual results, then four slot-`+0xec` calls (id 9) feed a length.
///
/// Two counted loops (8 rounds, or 2 when `mode` is 1 and `f18` is 0) each
/// run two more slot-`+0xec` calls and callee 10/11 over all the result
/// buffers; a non-1 answer ends that loop. Each accepted round moves `p0`
/// down (loop 1) or `p1` up (loop 2) by about 0.1047 radians with a 2pi
/// wrap and marks progress. With progress made, callee 12 combines two
/// probe words with the matrix to pick `pf` or `pg` for the outcome byte
/// (and `ph` when at least 3 rounds ran), then callee 13 may set flag bit
/// `0x10` on `b` and callee 14 closes the pair.
///
/// Float order is the original's; comparisons follow `comiss` semantics
/// (unordered counts as not-greater). Original: 0x00d899a0 (cdecl, ten
/// stack words, no return value).
lf_checker_rt::export!(cdecl, rw_00d899a0(
    a: u32,
    b: u32,
    p1: u32,
    p0: u32,
    f18: u32,
    f1c: u32,
    mode: u32,
    pf: u32,
    pg: u32,
    ph: u32,
) -> u32 {
    unsafe {
        const MATRIX: u32 = 0x20;
        const MODE: u32 = 0xe6e;
        const SUB: u32 = 0xe48;
        const NEGBIT: u32 = 0xe73;
        const FLAG: u32 = 0xf1f;
        const FLAG_BIT: u8 = 0x10;
        const VT_A: u32 = 0x60;
        const VT_B: u32 = 0x64;
        const VT_C: u32 = 0xec;
        const C_DOT2: u32 = 0x00fe8874; // 0.7
        const C_ZERO: u32 = 0x00fe8628; // 0.0
        const C_Q: u32 = 0x00fe87e4; // 0.25
        const C_HALF: u32 = 0x00fe8830; // 0.5
        const C_TWO_PI: u32 = 0x00fe8aec;
        const C_STEP: u32 = 0x00eec904; // ~0.10472
        const SIGN: u32 = 0x8000_0000;

        #[inline(always)]
        unsafe fn rd32(x: u32) -> u32 {
            unsafe { (x as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(x: u32) -> u8 {
            unsafe { (x as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(x: u32) -> f32 {
            unsafe { f32::from_bits(rd32(x)) }
        }
        #[inline(always)]
        unsafe fn wr8(x: u32, v: u8) {
            unsafe { (x as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wrf(x: u32, v: f32) {
            unsafe { (x as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn glob(va: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(va))) }
        }
        #[inline(always)]
        fn mul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
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
        fn neg(x: f32) -> f32 {
            f32::from_bits(x.to_bits() ^ SIGN)
        }
        #[inline(always)]
        unsafe fn v0(obj: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + VT_A) as usize);
                f(obj)
            }
        }
        #[inline(always)]
        unsafe fn v1(obj: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + VT_B) as usize);
                f(obj)
            }
        }
        #[inline(always)]
        unsafe fn v2(obj: u32, arg: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + VT_C) as usize);
                f(obj, arg)
            }
        }
        #[inline(always)]
        unsafe fn gate_match(obj: u32) -> u32 {
            unsafe { lf_checker_rt::callee_thiscall!(0, u32, obj.wrapping_add(SUB)) }
        }

        if mode == 0 && (f1c as u8) == 0 {
            return 0;
        }
        if rd8(ph) != 0 {
            return 0;
        }
        let bm = rd8(b + MODE);
        if bm == 2 && gate_match(b) == a {
            return 0;
        }
        if bm == 7 && gate_match(b) == a {
            return 0;
        }
        if bm == 6 && gate_match(b) == a {
            return 0;
        }
        let am = rd8(a + MODE);
        if (am == 0x0a || am == 0x0b || am == 0x0d || am == 0x0c) && gate_match(a) == b {
            return 0;
        }

        let amat = rd32(a + MATRIX);
        let bmat = rd32(b + MATRIX);
        let dx = sub(rdf(amat + 0x30), rdf(bmat + 0x30));
        let dy = sub(rdf(amat + 0x34), rdf(bmat + 0x34));
        let mut buf40 = [0u32; 3];
        let r = lf_checker_rt::callee_thiscall!(1, u32, b, buf40.as_mut_ptr() as u32);
        let (mut vx, mut vy, mut vz) = (rdf(r), rdf(r.wrapping_add(4)), rdf(r.wrapping_add(8)));
        if rd8(b + NEGBIT) & 1 != 0 {
            vx = neg(vx);
            vy = neg(vy);
            vz = neg(vz);
        }
        let dot = add(mul(vy, dy), mul(vx, dx));
        if 0.0 > dot {
            return 0;
        }
        if mode == 1 && (f18 as u8) == 0 {
            lf_checker_rt::callee_thiscall!(1, u32, a, buf40.as_mut_ptr() as u32);
            let (o0, o1, o2) = (
                f32::from_bits(buf40[0]),
                f32::from_bits(buf40[1]),
                f32::from_bits(buf40[2]),
            );
            let d2 = add(add(mul(o1, vy), mul(o0, vx)), mul(o2, vz));
            if d2 > glob(C_DOT2) {
                let mut nb = [dx.to_bits(), dy.to_bits(), 0u32];
                lf_checker_rt::callee_thiscall!(2, u32, nb.as_mut_ptr() as u32);
                let (n0, n1, n2) = (
                    f32::from_bits(nb[0]),
                    f32::from_bits(nb[1]),
                    f32::from_bits(nb[2]),
                );
                let d3 = add(add(mul(n1, vy), mul(n0, vx)), mul(n2, vz));
                if d3 > glob(C_DOT2) {
                    return 0;
                }
            }
        }

        let w = add(mul(rdf(bmat + 4), dy), mul(rdf(bmat), dx));
        let (t0, t1);
        if dot > glob(C_ZERO) {
            let r0 = v0(b);
            t0 = sub(rdf(r0), glob(C_Q));
            let r1 = v1(b);
            t1 = add(rdf(r1.wrapping_add(4)), glob(C_Q));
        } else {
            let r0 = v0(b);
            t0 = sub(rdf(r0), glob(C_Q));
            let r1 = v0(b);
            t1 = sub(rdf(r1.wrapping_add(4)), glob(C_Q));
        }
        // Probe blocks: each runs one transform callee (3..6) whose 4-float
        // answer is copied to a result buffer, then forms the next buffer
        // from a matrix row scaled by a virtual-call difference plus 0.5.
        let z = 0.0f32;
        let mut dummy = [0u32; 4];
        let mut b50 = [0u32; 3];
        let mut b40 = [0u32; 3];
        let mut b90 = [0u32; 3];
        let mut b70 = [0u32; 3];
        let mut b60 = [0u32; 3];
        let mut bA0 = [0u32; 4];
        let mut bB0 = [0u32; 4];
        let mut bC0 = [0u32; 4];
        let inw = [t0.to_bits(), t1.to_bits(), 0u32];
        let s = lf_checker_rt::callee_cdecl!(
            3, u32, dummy.as_mut_ptr() as u32, bmat, inw.as_ptr() as u32
        );
        b50[0] = rd32(s);
        b50[1] = rd32(s.wrapping_add(4));
        b50[2] = rd32(s.wrapping_add(8));
        let c3 = add(sub(rdf(v1(b)), rdf(v0(b))), glob(C_HALF));
        b40[0] = add(add(mul(c3, rdf(bmat)), mul(rdf(bmat + 0x10), z)), mul(rdf(bmat + 0x20), z))
            .to_bits();
        b40[1] = add(add(mul(rdf(bmat + 4), c3), mul(rdf(bmat + 0x14), z)), mul(rdf(bmat + 0x24), z))
            .to_bits();
        b40[2] = add(add(mul(rdf(bmat + 8), c3), mul(rdf(bmat + 0x18), z)), mul(rdf(bmat + 0x28), z))
            .to_bits();
        let t10;
        if w > 0.0 {
            t10 = add(rdf(v1(b)), glob(C_Q));
        } else {
            t10 = sub(rdf(v0(b)), glob(C_Q));
        }
        let t12 = sub(rdf(v0(b).wrapping_add(4)), glob(C_Q));
        let inw = [t10.to_bits(), t12.to_bits(), 0u32];
        let s = lf_checker_rt::callee_cdecl!(
            4, u32, dummy.as_mut_ptr() as u32, bmat, inw.as_ptr() as u32
        );
        bB0[0] = rd32(s);
        bB0[1] = rd32(s.wrapping_add(4));
        bB0[2] = rd32(s.wrapping_add(8));
        bB0[3] = rd32(s.wrapping_add(0x0c));
        let r = v1(b);
        let q = v0(b);
        let c3 = add(sub(rdf(r.wrapping_add(4)), rdf(q.wrapping_add(4))), glob(C_HALF));
        b90[0] = add(add(mul(rdf(bmat + 0x10), c3), mul(rdf(bmat), z)), mul(rdf(bmat + 0x20), z))
            .to_bits();
        b90[1] = add(add(mul(rdf(bmat + 0x14), c3), mul(rdf(bmat + 4), z)), mul(rdf(bmat + 0x24), z))
            .to_bits();
        b90[2] = add(add(mul(rdf(bmat + 0x18), c3), mul(rdf(bmat + 8), z)), mul(rdf(bmat + 0x28), z))
            .to_bits();
        let q1 = add(mul(rdf(amat + 4), dy), mul(rdf(amat), dx));
        let q2 = add(mul(rdf(amat + 0x14), dy), mul(rdf(amat + 0x10), dx));
        let (u0, u1);
        if 0.0 > q2 {
            let r0 = v0(a);
            u0 = sub(rdf(r0), glob(C_Q));
            let r1 = v1(a);
            u1 = add(rdf(r1.wrapping_add(4)), glob(C_Q));
        } else {
            let r0 = v0(a);
            u0 = sub(rdf(r0), glob(C_Q));
            let r1 = v0(a);
            u1 = sub(rdf(r1.wrapping_add(4)), glob(C_Q));
        }
        let inw = [u0.to_bits(), u1.to_bits(), 0u32];
        let s = lf_checker_rt::callee_cdecl!(
            5, u32, dummy.as_mut_ptr() as u32, amat, inw.as_ptr() as u32
        );
        bA0[0] = rd32(s);
        bA0[1] = rd32(s.wrapping_add(4));
        bA0[2] = rd32(s.wrapping_add(8));
        bA0[3] = rd32(s.wrapping_add(0x0c));
        let c3 = add(sub(rdf(v1(a)), rdf(v0(a))), glob(C_HALF));
        b70[0] = add(add(mul(rdf(amat + 0x10), z), mul(c3, rdf(amat))), mul(rdf(amat + 0x20), z))
            .to_bits();
        b70[1] = add(add(mul(rdf(amat + 4), c3), mul(rdf(amat + 0x14), z)), mul(rdf(amat + 0x24), z))
            .to_bits();
        b70[2] = add(add(mul(rdf(amat + 8), c3), mul(rdf(amat + 0x18), z)), mul(rdf(amat + 0x28), z))
            .to_bits();
        let v10;
        if 0.0 > q1 {
            v10 = add(rdf(v1(a)), glob(C_Q));
        } else {
            v10 = sub(rdf(v0(a)), glob(C_Q));
        }
        let v12 = sub(rdf(v0(a).wrapping_add(4)), glob(C_Q));
        let inw = [v10.to_bits(), v12.to_bits(), 0u32];
        let s = lf_checker_rt::callee_cdecl!(
            6, u32, dummy.as_mut_ptr() as u32, amat, inw.as_ptr() as u32
        );
        bC0[0] = rd32(s);
        bC0[1] = rd32(s.wrapping_add(4));
        bC0[2] = rd32(s.wrapping_add(8));
        bC0[3] = rd32(s.wrapping_add(0x0c));
        let r = v1(a);
        let q = v0(a);
        let c3 = add(sub(rdf(r.wrapping_add(4)), rdf(q.wrapping_add(4))), glob(C_HALF));
        b60[0] = add(add(mul(rdf(amat + 0x10), c3), mul(rdf(amat), z)), mul(rdf(amat + 0x20), z))
            .to_bits();
        b60[1] = add(add(mul(rdf(amat + 0x14), c3), mul(rdf(amat + 4), z)), mul(rdf(amat + 0x24), z))
            .to_bits();
        b60[2] = add(add(mul(rdf(amat + 0x18), c3), mul(rdf(amat + 8), z)), mul(rdf(amat + 0x28), z))
            .to_bits();

        let rb0 = rdf(v1(b));
        let ra0 = rdf(v1(a));
        let flagword = (v10.to_bits() & 0xffff_ff00) | ((ra0 > rb0) as u32);
        let e1 = v2(b, dummy.as_mut_ptr() as u32);
        let e2 = v2(b, dummy.as_mut_ptr() as u32);
        let p = mul(rdf(e2.wrapping_add(4)), rdf(e1.wrapping_add(4)));
        let e3 = v2(b, dummy.as_mut_ptr() as u32);
        let e4 = v2(b, dummy.as_mut_ptr() as u32);
        let len = add(mul(rdf(e4), rdf(e3)), p).sqrt();

        let n = if mode == 1 { if (f18 as u8) == 0 { 2 } else { 8 } } else { 8 };
        let mut progress = false;
        let mut c1 = 0u32;
        if n > 0 {
            loop {
                let r5 = v2(a, dummy.as_mut_ptr() as u32);
                let r6 = v2(a, dummy.as_mut_ptr() as u32);
                let al: u32 = lf_checker_rt::callee_cdecl!(
                    10,
                    u32,
                    rdf(p0).to_bits(),
                    b50.as_mut_ptr() as u32,
                    b40.as_mut_ptr() as u32,
                    bB0.as_mut_ptr() as u32,
                    b90.as_mut_ptr() as u32,
                    bA0.as_mut_ptr() as u32,
                    b70.as_mut_ptr() as u32,
                    bC0.as_mut_ptr() as u32,
                    b60.as_mut_ptr() as u32,
                    rd32(r5),
                    rd32(r6.wrapping_add(4)),
                    len.to_bits(),
                    flagword
                );
                if (al as u8) != 1 {
                    break;
                }
                let mut x = sub(rdf(p0), glob(C_STEP));
                c1 += 1;
                wrf(p0, x);
                if 0.0 > x {
                    x = add(x, glob(C_TWO_PI));
                    wrf(p0, x);
                }
                progress = true;
                if !(c1 < n as u32) {
                    break;
                }
            }
        }
        let mut c2 = 0u32;
        if n > 0 {
            loop {
                let r5 = v2(a, dummy.as_mut_ptr() as u32);
                let r6 = v2(a, dummy.as_mut_ptr() as u32);
                let al: u32 = lf_checker_rt::callee_cdecl!(
                    11,
                    u32,
                    rdf(p1).to_bits(),
                    b50.as_mut_ptr() as u32,
                    b40.as_mut_ptr() as u32,
                    bB0.as_mut_ptr() as u32,
                    b90.as_mut_ptr() as u32,
                    bA0.as_mut_ptr() as u32,
                    b70.as_mut_ptr() as u32,
                    bC0.as_mut_ptr() as u32,
                    b60.as_mut_ptr() as u32,
                    rd32(r5),
                    rd32(r6.wrapping_add(4)),
                    len.to_bits(),
                    flagword
                );
                if (al as u8) != 1 {
                    if !progress {
                        return 0;
                    }
                    break;
                }
                let mut x = add(rdf(p1), glob(C_STEP));
                c1 += 1;
                c2 += 1;
                wrf(p1, x);
                if x > glob(C_TWO_PI) {
                    x = sub(x, glob(C_TWO_PI));
                    wrf(p1, x);
                }
                progress = true;
                if !(c2 < n as u32) {
                    break;
                }
            }
        } else if !progress {
            return 0;
        }
        let mut w10 = [flagword];
        let mut w18 = [len.to_bits()];
        let t: u32 =
            lf_checker_rt::callee_cdecl!(12, u32, a, w18.as_mut_ptr() as u32, w10.as_mut_ptr() as u32);
        if (t as u8) != 0 {
            let q = add(
                mul(rdf(bmat + 4), f32::from_bits(w10[0])),
                mul(rdf(bmat), f32::from_bits(w18[0])),
            );
            let tgt = if q > glob(C_ZERO) { pf } else { pg };
            wr8(tgt, 1);
            if c1 >= 3 {
                wr8(ph, 1);
            }
        }
        let u: u32 = lf_checker_rt::callee_cdecl!(13, u32, a);
        if (u as u8) != 0 {
            wr8(b + FLAG, rd8(b + FLAG) | FLAG_BIT);
        }
        lf_checker_rt::callee_cdecl!(14, u32, b, a);
        0
    }
});
