// original: 0x00a23be0 ped_task_solve_aim_and_pose (proposed)

/// Solve this task's aim direction from sensor fetches, then drive the pose
/// accumulators and the heading/pitch followers.
///
/// `this` is the task object; the six stack arguments are a sensor-block
/// pointer (`a0`, null returns immediately), a solver-context pointer (`a1`),
/// a solver mode word (`a2`), a rate float (`a3`) and two flag bytes (`a4`,
/// `a5`). A null sensor block returns the caller's entry `eax` (pinned by
/// contract); otherwise the weight at `+0x2a0` is eased a tenth of the way
/// towards 1 first.
///
/// Two fetch pairs run through the sensor block (callees 1+3 and 4+6: a
/// locator hook whose answer feeds a score hook returning in ST0), each score
/// negated; a second pair (callees 2+7 and 5+8) runs when the extended flag at
/// sensor `+0x3289` is set. When the object at `+0x118` is present, its
/// virtual slot `+0x28` is polled (callee 9): an answer of 7 with bit 1 of
/// that object's `+0x1c5` set negates the second score again. The score
/// magnitude (square-rooted unless it is not above 0) picks, past a 0.9
/// dead zone, between easing the energy at `+0x228` from indexed tables
/// (entry 0 or 1 of the game mode: forced to 1 when sensor `+0x328d` is set,
/// else the game-filled selector) scaled by the game-filled rate, or
/// resetting it to 1; the energy is then clamped to at most 1 (exactly 1 when
/// the extended flag is set) and multiplied by a third table entry.
///
/// The game-filled divisor (converted from integer) scales a quotient that is
/// remapped to a slope pair, defaulting to constants when the extended flag
/// is clear. The slopes, the scores and the rate word form two drive values;
/// the first either adds to the pitch follower at `+0x21c` (extended flag) or
/// eases it by thirty times the rate. Both drives and the object at `+0x204`
/// go to the projector hook (callee 11).
///
/// When `a4` is set, a ready byte at `+0x2c4` is set and the drives accumulate
/// into `+0x260/+0x264` (directly with the extended flag, else eased); the
/// solver hook (callee 12) then selects one of three constant bound sets for
/// its answer (0/1, 3, anything else), both accumulators are clamped into
/// their bounds, and when the stage at `+0x130` is zero each is eased halfway
/// towards its far bound unless clamping left it unchanged. When `a4` is
/// clear the drives go to the followers at `+0x21c/+0x218` instead (direct or
/// eased by the extended flag).
///
/// The combiner hook (callee 13) takes the follower pair and fills three words
/// that, scaled by 100, offset the `+0x160..0x16c` triple (whose last word is
/// a word the original reads from its own uninitialised stack scratch: zero
/// under the checker's defined fill). A flagged table lookup eases the dial
/// at `+0x60` halfway from 45 (skipped to a direct store when `a5` is clear
/// or the flag is clear, with a second lookup form when both are set) while
/// the shadow at `+0x2cc` eases likewise, and a final combine scaled by 75.05
/// writes the `+0x150..0x15c` triple (last word likewise uninitialised).
/// All float operations keep the original's operand order.
///
/// The return value is the last table index when the `+0x200` flag is set,
/// else the combiner hook's answer. Original: 0x00a23be0
/// (thiscall, six stack words).
lf_checker_rt::export!(thiscall, rw_00a23be0(
    this: u32,
    a0: u32,
    a1: u32,
    a2: u32,
    a3b: u32,
    a4b: u32,
    a5b: u32,
) -> u32 {
    unsafe {
        const C_ONE: u32 = 0x00FE88E8;
        const C_TENTH: u32 = 0x00FE879C;
        const C_NEG_MASK: u32 = 0x00FE8FA0;
        const C_ZERO: u32 = 0x00FE8628;
        const C_DEAD: u32 = 0x00FE88BC;
        const C_HALF: u32 = 0x00FE8830;
        const C_SLOPE_D0: u32 = 0x00FE8734;
        const C_SLOPE_D1: u32 = 0x00FE871C;
        const C_RATE: u32 = 0x00FE873C;
        const C_THIRTY: u32 = 0x00FE8B48;
        const C_QM: u32 = 0x00E9B9C8;
        const C_QA: u32 = 0x00E9B9C4;
        const C_HUNDRED: u32 = 0x00FE8BB0;
        const C_DIAL: u32 = 0x00FE8B64;
        const C_FINAL: u32 = 0x00E9B9EC;
        const G_MODE: u32 = 0x01160C6C;
        const G_DIVISOR: u32 = 0x01160E70;
        const G_RATE: u32 = 0x011735BC;
        const T_E0: u32 = 0x00EC9AC8;
        const T_E1: u32 = 0x00E9B9B8;
        const T_E2: u32 = 0x00EC9AD4;
        const T_DIAL: u32 = 0x00E9B814;
        const PINNED_EAX: u32 = 0x12345678;
        const STACK_FILL_WORD: u32 = 0;

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
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn cf(va: u32) -> f32 {
            unsafe { lf_checker_rt::global::<f32>(va).read() }
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
        unsafe fn neg(a: f32) -> f32 {
            unsafe {
                f32::from_bits(
                    a.to_bits() ^ rd32(lf_checker_rt::relocated(C_NEG_MASK)),
                )
            }
        }

        if a0 == 0 {
            return PINNED_EAX;
        }
        let edi = a0;
        // Ease the weight towards 1.
        let w0 = rdf(this.wrapping_add(0x2a0));
        let mut xe = sub(cf(C_ONE), w0);
        xe = mul(xe, cf(C_TENTH));
        xe = add(xe, w0);
        wrf(this.wrapping_add(0x2a0), xe);
        // Fetch pairs.
        let r1: u32 = lf_checker_rt::callee_thiscall!(1, u32, edi);
        let f1: f32 = lf_checker_rt::callee_cdecl!(3, f32, r1);
        let mut v14 = neg(f1);
        let r4: u32 = lf_checker_rt::callee_thiscall!(4, u32, edi);
        let f2: f32 = lf_checker_rt::callee_cdecl!(6, f32, r4);
        let mut vc = neg(f2);
        let ext = rd8(edi.wrapping_add(0x3289));
        if ext != 0 {
            let r1b: u32 = lf_checker_rt::callee_thiscall!(2, u32, edi, 0);
            let f3: f32 = lf_checker_rt::callee_cdecl!(7, f32, r1b);
            v14 = neg(f3);
            let r4b: u32 = lf_checker_rt::callee_thiscall!(5, u32, edi, 1);
            let f4: f32 = lf_checker_rt::callee_cdecl!(8, f32, r4b);
            vc = neg(f4);
        }
        // Virtual slot poll.
        let cx = rd32(this.wrapping_add(0x118));
        let mut x0 = vc;
        if cx != 0 {
            let vt = rd32(cx);
            let poll: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(vt.wrapping_add(0x28)) as usize);
            let ans: u32 = poll(cx);
            if ans == 7 && rd8(cx.wrapping_add(0x1c5)) & 2 != 0 {
                x0 = neg(x0);
                vc = x0;
            }
        }
        // Score magnitude.
        let x2 = v14;
        let mut x1 = mul(x0, x0);
        let x0b = mul(x2, x2);
        x1 = add(x1, x0b);
        if x1 > cf(C_ZERO) {
            x1 = core::hint::black_box(x1).sqrt();
        }
        let mut eax: u32 = rd32(lf_checker_rt::relocated(G_MODE));
        if rd8(edi.wrapping_add(0x328d)) != 0 {
            eax = 1;
        }
        if x1 > cf(C_DEAD) {
            let t0 = rdf(lf_checker_rt::relocated(T_E0).wrapping_add(eax.wrapping_mul(4)));
            let mut xw = mul(t0, cf(G_RATE));
            xw = add(xw, rdf(this.wrapping_add(0x228)));
            wrf(this.wrapping_add(0x228), xw);
            let x1b = rdf(lf_checker_rt::relocated(T_E1).wrapping_add(eax.wrapping_mul(4)));
            if xw > x1b {
                wrf(this.wrapping_add(0x228), x1b);
            }
        } else {
            wr32(this.wrapping_add(0x228), 0x3f800000);
        }
        // Clamp the energy.
        let mut xe2 = rdf(this.wrapping_add(0x228));
        let one = cf(C_ONE);
        if !(one > xe2) {
            xe2 = one;
        }
        if ext != 0 {
            xe2 = one;
        }
        let mut x1d = rdf(lf_checker_rt::relocated(T_E2).wrapping_add(eax.wrapping_mul(4)));
        x1d = mul(x1d, xe2);
        let f0 = rd32(lf_checker_rt::relocated(G_DIVISOR)) as i32 as f32;
        let mut v10a = x1d;
        let r8: u32 = lf_checker_rt::callee_cdecl!(10, u32, 0x8a);
        let mut x3 = f0;
        let x0e = r8 as i32 as f32;
        x3 = div(x3, x0e);
        x3 = mul(x3, cf(C_QM));
        x3 = add(x3, cf(C_QA));
        let mut x4: f32;
        if ext != 0 {
            x4 = x3;
        } else {
            x4 = cf(C_SLOPE_D0);
        }
        if ext == 0 {
            x3 = cf(C_SLOPE_D1);
        }
        // Drive values.
        let mut x1f = v10a;
        let mut x0f = mul(f32::from_bits(a3b), cf(C_RATE));
        let mut x2f = x1f;
        x2f = mul(x2f, v14);
        x1f = mul(x1f, vc);
        x2f = mul(x2f, x4);
        x1f = mul(x1f, x3);
        x2f = mul(x2f, x0f);
        x1f = mul(x1f, x0f);
        let mut t1 = x2f;
        let mut v1c = x2f;
        v10a = x1f;
        let x0g: f32;
        if ext != 0 {
            x0g = add(rdf(this.wrapping_add(0x21c)), x2f);
        } else {
            let mut t = mul(x2f, cf(C_THIRTY));
            t = mul(t, cf(G_RATE));
            x0g = add(t, rdf(this.wrapping_add(0x21c)));
        }
        v14 = x0g;
        let v14p = &v14 as *const f32 as u32;
        let v1cp = &v1c as *const f32 as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(11, u32, this, rd32(this.wrapping_add(0x204)), v14p, v1cp);
        let a4 = a4b as u8;
        if a4 != 0 {
            let mut x0h = t1;
            wr8(this.wrapping_add(0x2c4), 1);
            if ext != 0 {
                let mut x1h = v10a;
                x0h = add(x0h, rdf(this.wrapping_add(0x264)));
                x1h = add(x1h, rdf(this.wrapping_add(0x260)));
                wrf(this.wrapping_add(0x264), x0h);
                wrf(this.wrapping_add(0x260), x1h);
            } else {
                x0h = mul(x0h, cf(C_THIRTY));
                x0h = mul(x0h, cf(G_RATE));
                x0h = add(x0h, rdf(this.wrapping_add(0x264)));
                wrf(this.wrapping_add(0x264), x0h);
                let mut x0i = v10a;
                x0i = mul(x0i, cf(C_THIRTY));
                x0i = mul(x0i, cf(G_RATE));
                x0i = add(x0i, rdf(this.wrapping_add(0x260)));
                wrf(this.wrapping_add(0x260), x0i);
            }
            let r10: u32 = lf_checker_rt::callee_thiscall!(12, u32, a1, a2);
            let x7 = rdf(this.wrapping_add(0x260));
            let x6 = rdf(this.wrapping_add(0x264));
            let (mut x0j, mut x2j, mut x4j, mut x1j): (f32, f32, f32, f32);
            if r10 == 0 || r10 == 1 {
                x0j = cf(0x00E9B4EC);
                x2j = cf(0x00E9BA0C);
                x4j = cf(0x00E9BA04);
                x1j = cf(0x00E9B9D4);
            } else if r10 == 3 {
                x0j = cf(0x00E9B9E8);
                x2j = cf(0x00E9B510);
                x4j = cf(0x00E9BA04);
                x1j = cf(0x00E9B9D4);
            } else {
                x0j = cf(0x00E9B9E8);
                x1j = cf(0x00E9B9D8);
                x4j = cf(0x00E9BA08);
                x2j = cf(0x00E9BA00);
            }
            let mut x5 = if x7 > x4j { x7 } else { x4j };
            if !(x1j > x5) {
                x5 = x1j;
            }
            wrf(this.wrapping_add(0x260), x5);
            let mut x3k = if x6 > x2j { x6 } else { x2j };
            if !(x0j > x3k) {
                x3k = x0j;
            }
            if rd32(this.wrapping_add(0x130)) == 0 {
                wrf(this.wrapping_add(0x264), x3k);
                if x5 != x7 {
                    let mut x1m = sub(x1j, x4j);
                    x1m = mul(x1m, cf(C_HALF));
                    x1m = add(x1m, x4j);
                    wrf(this.wrapping_add(0x260), x1m);
                }
                if x3k != x6 {
                    let mut x0m = sub(x0j, x2j);
                    x0m = mul(x0m, cf(C_HALF));
                    x0m = add(x0m, x2j);
                    wrf(this.wrapping_add(0x264), x0m);
                }
            } else {
                wrf(this.wrapping_add(0x264), x3k);
            }
        } else {
            let x0n = v14;
            wrf(this.wrapping_add(0x21c), x0n);
            let x0o: f32;
            if ext != 0 {
                x0o = add(rdf(this.wrapping_add(0x218)), v10a);
            } else {
                let mut t = mul(v10a, cf(C_THIRTY));
                t = mul(t, cf(G_RATE));
                x0o = add(t, rdf(this.wrapping_add(0x218)));
            }
            wrf(this.wrapping_add(0x218), x0o);
        }
        // Follower pair through the alsor hook.
        let x0p = rdf(this.wrapping_add(0x21c));
        let mut wslot = [0u32; 3];
        let wptr = wslot.as_mut_ptr() as u32;
        // The hook's answer is the return value unless the flag below is set.
        let r13: u32 = lf_checker_rt::callee_cdecl!(
            13,
            u32,
            wptr,
            rdf(this.wrapping_add(0x218)).to_bits(),
            x0p.to_bits()
        );
        let mut x3q = f32::from_bits(wslot[2]);
        let mut x4q = f32::from_bits(wslot[0]);
        let mut x2q = f32::from_bits(wslot[1]);
        let hundred = cf(C_HUNDRED);
        x3q = mul(x3q, hundred);
        x4q = mul(x4q, hundred);
        x2q = mul(x2q, hundred);
        let mut x0q = rdf(this.wrapping_add(0x48));
        x4q = add(x4q, rdf(this.wrapping_add(0x40)));
        x0q = add(x0q, x3q);
        let mut x1q = add(rdf(this.wrapping_add(0x44)), x2q);
        wrf(this.wrapping_add(0x160), x4q);
        wrf(this.wrapping_add(0x168), x0q);
        wrf(this.wrapping_add(0x164), x1q);
        wrf(this.wrapping_add(0x16c), f32::from_bits(STACK_FILL_WORD));
        // Dial ease.
        let cl = rd8(this.wrapping_add(0x200)) & 1;
        let mut x1r = cf(C_DIAL);
        let mut idx: u32 = 0;
        if cl != 0 {
            let aux = rd32(this.wrapping_add(0x204));
            idx = rd32(aux.wrapping_add(0x2b0));
            x1r = sub(x1r, rdf(lf_checker_rt::relocated(T_DIAL).wrapping_add(idx.wrapping_mul(4))));
        }
        x1r = sub(x1r, rdf(this.wrapping_add(0x60)));
        x1r = mul(x1r, cf(C_HALF));
        x1r = add(x1r, rdf(this.wrapping_add(0x60)));
        let a5 = a5b as u8;
        let x0s: f32;
        if a5 != 0 && cl != 0 {
            let x1s = rdf(this.wrapping_add(0x2cc));
            let aux = rd32(this.wrapping_add(0x204));
            idx = rd32(aux.wrapping_add(0x2b0));
            let mut t = rdf(lf_checker_rt::relocated(T_DIAL).wrapping_add(idx.wrapping_mul(4)));
            t = sub(t, x1s);
            t = mul(t, cf(C_HALF));
            x0s = add(t, x1s);
        } else {
            x0s = mul(rdf(this.wrapping_add(0x2cc)), cf(C_HALF));
            wrf(this.wrapping_add(0x60), x1r);
        }
        wrf(this.wrapping_add(0x2cc), x0s);
        // Final combine.
        let mut x2t = rdf(this.wrapping_add(0x28));
        let cfin = cf(C_FINAL);
        let mut x4t = rdf(this.wrapping_add(0x20));
        let mut x1t = rdf(this.wrapping_add(0x24));
        let mut x3t = rdf(this.wrapping_add(0x44));
        x2t = mul(x2t, cfin);
        x4t = mul(x4t, cfin);
        x1t = mul(x1t, cfin);
        let mut x0u = rdf(this.wrapping_add(0x48));
        x4t = add(x4t, rdf(this.wrapping_add(0x40)));
        x0u = add(x0u, x2t);
        x3t = add(x3t, x1t);
        wrf(this.wrapping_add(0x150), x4t);
        wrf(this.wrapping_add(0x158), x0u);
        wrf(this.wrapping_add(0x154), x3t);
        wrf(this.wrapping_add(0x15c), f32::from_bits(STACK_FILL_WORD));
        if cl != 0 {
            idx
        } else {
            r13
        }
    }
});
