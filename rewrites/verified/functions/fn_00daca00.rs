// original: 0x00daca00 slew_param_to_curve (proposed)
//
// thiscall/1 (the stack argument is popped but never read): shape the input
// at [this+0x70] through a piecewise target curve into [this+0x54], then
// slew [this+0x50] toward it by at most one step. Returns the flag word.
//
// Five curve constants live in game .data with a flag word (bit N set once
// constant N is cached): on first use each is loaded from its .rdata
// default (0.04, 1.0, 25.0, 0.25, 1.25) and stored to its cache slot. The
// flag is read once; every bit test sees the entry value while the stores
// accumulate, and the final flag|0x1f is the return value.
//
// Target selection on input x, with c1..c5 the cached constants: x below
// c1 gives 0.0; x above c3 gives 1.0; x below c2 gives
// ((sqrt(x)-0.2)*c5)*0.5; otherwise ((sqrt(x)-1.0)*c4)*0.5+0.5. Every
// comparison is an ordered-greater test (an unordered input falls through
// to the last branch, matching comiss+jbe). The target is then square-root
// scaled by 3.0 and, unless the gate byte at [this+0x30] is set, clamped
// to 2.0 from above. The slew moves [this+0x50] to the target when it is
// within one step (0.2) above, else advances it by one step.
lf_checker_rt::export!(thiscall, rw_00daca00(this: u32, _a0: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x017a6544;
        const CACHE: [u32; 5] = [0x017a6540, 0x017a6548, 0x017a654c, 0x017a6550, 0x017a6554];
        const DEFLT: [u32; 5] = [0x00e9cad0, 0x00fe88e8, 0x00fe8b40, 0x00fe87e4, 0x00fe8920];
        const STEP: u32 = 0x00fe87d0; // 0.2
        const HALF: u32 = 0x00fe8830; // 0.5
        const TAIL_MUL: u32 = 0x00fe8a94; // 3.0
        const CLAMP_REF: u32 = 0x00fe8a24; // 2.0
        const TWO: f32 = 2.0;
        const ONE: f32 = 1.0;

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
        unsafe fn grdf(va: u32) -> f32 {
            unsafe { rdf(lf_checker_rt::relocated(va)) }
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

        let flag_addr = lf_checker_rt::relocated(FLAG);
        let flag0 = rd32(flag_addr);
        let mut flag = flag0;
        let mut c = [0.0f32; 5];
        // The second default is also kept live in a register; the other
        // loads happen only on their own init path.
        let c2reg = grdf(DEFLT[1]);
        for n in 0..5 {
            if flag0 & (1 << n) != 0 {
                c[n] = rdf(lf_checker_rt::relocated(CACHE[n]));
            } else {
                let v = if n == 1 { c2reg } else { grdf(DEFLT[n]) };
                c[n] = v;
                flag |= 1 << n;
                wr32(flag_addr, flag);
                wrf(lf_checker_rt::relocated(CACHE[n]), v);
            }
        }
        let step = grdf(STEP);
        let x = rdf(this + 0x70);
        if c[0] > x {
            wrf(this + 0x54, 0.0);
        } else if x > c[2] {
            wrf(this + 0x54, ONE);
        } else if c[1] > x {
            let t = mul(sub(x.sqrt(), step), c[4]);
            wrf(this + 0x54, mul(t, grdf(HALF)));
        } else {
            let t = mul(sub(x.sqrt(), c2reg), c[3]);
            wrf(this + 0x54, add(mul(t, grdf(HALF)), grdf(HALF)));
        }
        let mut t = rdf(this + 0x54);
        t = mul(t.sqrt(), grdf(TAIL_MUL));
        wrf(this + 0x54, t);
        if (this as *const u8).add(0x30).read() == 0 && t > grdf(CLAMP_REF) {
            wrf(this + 0x54, TWO);
        }
        let tgt = rdf(this + 0x54);
        let cur = rdf(this + 0x50);
        if sub(tgt, cur) > step {
            wrf(this + 0x50, add(cur, step));
        } else {
            wrf(this + 0x50, tgt);
        }
        flag
    }
});
