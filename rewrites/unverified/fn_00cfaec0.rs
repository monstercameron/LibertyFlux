// original: 0x00cfaec0 CTaskSimpleClimbLadder::vf13 (symbols)

/// Climb-ladder task update: blend a target point and steer toward the ladder.
///
/// `arg0` is the task owner (object at `+0x38` unless it is the excluded one
/// at `+0x7b4`, position block at `+0x20`); `arg1` is a float divisor. The
/// object's indirect slot 1 is polled first: answer 15 aborts with 0. An
/// inactive task (`this+0xca` clear) or the excluded object also returns 0.
///
/// Otherwise a 3-vector is formed from `this` fields (`+0x70..+0x88`) against
/// the position block, either blended through the weight at `outer+0x4c`
/// (when `outer = this+0x60` is live and mode `this+0x18` is 4, or mode is 5)
/// or plain. When the global enable byte is set, two shaping calls run on
/// `this+0x54` and the third component; equal answers continue through a
/// window check on the third component, unequal answers jump straight to the
/// fast path. The fast path scales the vector by `GDIV / arg1` and steers
/// through callee C. The slow path scales by `GSCALE * GDIV / arg1`,
/// normalises the vector when its length exceeds `GNORM`, scales by `GPOST`
/// and by an indirect slot 9 value, then steers through callee F. Both
/// steering paths return 1. Float operation order is the original's.
///
/// Original: 0x00cfaec0 (thiscall, two stack arguments, returns al).
lf_checker_rt::export!(thiscall, rw_00cfaec0(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const ACTIVE_OFF: u32 = 0xca;
        const OBJ_OFF: u32 = 0x38;
        const EXCL_OFF: u32 = 0x7b4;
        const OUTER_OFF: u32 = 0x60;
        const MODE_OFF: u32 = 0x18;
        const MODE_BLEND: u32 = 4;
        const MODE_BLEND2: u32 = 5;
        const BASE_OFF: u32 = 0x20;
        const WEIGHT_OFF: u32 = 0x4c;
        const SHAPE_ARG_OFF: u32 = 0x54;
        const PX: u32 = 0x70;
        const PY: u32 = 0x74;
        const PZ: u32 = 0x78;
        const TX: u32 = 0x80;
        const TY: u32 = 0x84;
        const TZ: u32 = 0x88;
        const ABORT_ANSWER: u32 = 15;
        const POLL_SLOT: u32 = 4;
        const READ_SLOT: u32 = 0x24;
        const G_ENABLE: u32 = 0x0105_39fc;
        const G_LO: u32 = 0x00fe_876c;
        const G_HI: u32 = 0x00fe_8d5c;
        const G_DIV: u32 = 0x00fe_88e8;
        const G_SCALE: u32 = 0x00fe_8ab8;
        const G_NORM: u32 = 0x00fe_8b68;
        const G_POST: u32 = 0x00fe_88bc;
        const POLL: u32 = 1;
        const SHAPE: u32 = 2;
        const STEER_FAST: u32 = 3;
        const GETVAL: u32 = 4;
        const READV: u32 = 5;
        const STEER_SLOW: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn rdglobal(file_va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(file_va)) }
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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn indirect0(target: u32, obj: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(target as usize);
                f(obj)
            }
        }
        #[inline(always)]
        unsafe fn indirect0f(target: u32, obj: u32) -> f32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> f32 =
                    core::mem::transmute(target as usize);
                f(obj)
            }
        }

        if rd8(this + ACTIVE_OFF) == 0 {
            return 0;
        }
        let obj = rd32(arg0 + OBJ_OFF);
        if obj != 0 && obj == rd32(arg0 + EXCL_OFF) {
            return 0;
        }
        let poll_target = rd32(rd32(obj) + POLL_SLOT);
        if indirect0(poll_target, obj) == ABORT_ANSWER {
            return 0;
        }
        let outer = rd32(this + OUTER_OFF);
        let mode = rd32(this + MODE_OFF);
        let base = rd32(arg0 + BASE_OFF);
        let (v20, v1c, v10);
        if (outer != 0 && mode == MODE_BLEND) || mode == MODE_BLEND2 {
            let x70 = rdf(this + PX);
            let x74 = rdf(this + PY);
            let mut x2 = sub(rdf(this + TX), x70);
            let mut x1 = sub(rdf(this + TY), x74);
            let mut x0 = sub(rdf(this + TZ), rdf(this + PZ));
            let w = rdf(outer + WEIGHT_OFF);
            x2 = mul(x2, w);
            x1 = mul(x1, w);
            let mut x6 = add(x70, x2);
            x0 = mul(x0, w);
            let mut x5 = add(x74, x1);
            x6 = sub(x6, rdf(base + 0x30));
            let mut x4 = add(rdf(this + PZ), x0);
            x5 = sub(x5, rdf(base + 0x34));
            x4 = sub(x4, rdf(base + 0x38));
            v20 = x6;
            v1c = x5;
            v10 = x4;
        } else {
            v20 = sub(rdf(this + TX), rdf(base + 0x30));
            v1c = sub(rdf(this + TY), rdf(base + 0x34));
            v10 = sub(rdf(this + TZ), rdf(base + 0x38));
        }
        let divisor = f32::from_bits(arg1);
        if rd8(lf_checker_rt::relocated(G_ENABLE)) != 0 {
            let b1: f32 = lf_checker_rt::callee_cdecl!(SHAPE, f32, rd32(this + SHAPE_ARG_OFF));
            let b2: f32 = lf_checker_rt::callee_cdecl!(SHAPE, f32, v10.to_bits());
            if b1 != b2 {
                let k = div(f32::from_bits(rdglobal(G_DIV)), divisor);
                let o0 = mul(v20, k);
                let o1 = mul(v1c, k);
                let o2 = mul(k, v10);
                let mut buf = [o0.to_bits(), o1.to_bits(), o2.to_bits()];
                lf_checker_rt::callee_thiscall!(STEER_FAST, u32, arg0, buf.as_mut_ptr() as u32);
                return 1;
            }
            let g1 = f32::from_bits(rdglobal(G_LO));
            if !(g1 > v10) {
                // fall through to slow path
            } else {
                let g2 = f32::from_bits(rdglobal(G_HI));
                if v10 > g2 {
                    let k = div(f32::from_bits(rdglobal(G_DIV)), divisor);
                    let o0 = mul(v20, k);
                    let o1 = mul(v1c, k);
                    let o2 = mul(k, v10);
                    let mut buf = [o0.to_bits(), o1.to_bits(), o2.to_bits()];
                    lf_checker_rt::callee_thiscall!(STEER_FAST, u32, arg0, buf.as_mut_ptr() as u32);
                    return 1;
                }
            }
        }
        // Slow path.
        let s = f32::from_bits(rdglobal(G_SCALE));
        let mut x2 = mul(v20, s);
        let mut x1 = mul(v1c, s);
        let mut x4 = mul(v10, s);
        let k = div(f32::from_bits(rdglobal(G_DIV)), divisor);
        x2 = mul(x2, k);
        x1 = mul(x1, k);
        x4 = mul(x4, k);
        let d: u32 = lf_checker_rt::callee_thiscall!(GETVAL, u32, arg0);
        let e: f32 = indirect0f(rd32(rd32(d) + READ_SLOT), d);
        let mut x3 = x1;
        let mut xa = x2;
        let mut xb = x4;
        let mut t0 = mul(xa, xa);
        let mut t1 = mul(x3, x3);
        t1 = add(t1, t0);
        t0 = mul(xb, xb);
        t1 = add(t1, t0);
        let len = core::hint::black_box(t1).sqrt();
        let mut n = f32::from_bits(rdglobal(G_NORM));
        if len > n {
            n = div(n, len);
            x3 = mul(x3, n);
            xb = mul(xb, n);
            t0 = mul(n, xa);
            xa = t0;
        }
        let p = f32::from_bits(rdglobal(G_POST));
        xa = mul(xa, p);
        x3 = mul(x3, p);
        xb = mul(xb, p);
        xa = mul(xa, e);
        x3 = mul(x3, e);
        xb = mul(xb, e);
        let mut buf = [xa.to_bits(), x3.to_bits(), xb.to_bits()];
        lf_checker_rt::callee_thiscall!(STEER_SLOW, u32, arg0, buf.as_mut_ptr() as u32);
        1
    }
});
