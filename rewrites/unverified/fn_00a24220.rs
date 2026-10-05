// original: 0x00a24220 ped_task_update_tracking_state (proposed)

/// Refresh this task's tracked position from its target, blend it towards the
/// goal, and update the heading/pitch followers.
///
/// `this` is the task object; the six stack arguments are a context pointer
/// (`a0`, flag word read at `+0x28` and a mode byte at `+0x26c`), three float
/// parameters (`a1..a3`), and two writable float slots (`a4`, `a5`).
///
/// The update runs in order: fetch the target triple through the locator hook
/// (callee 1, no stack arguments; the triple sits at `+0x3a0` of the returned
/// object) into `+0x150/0x154/0x158`, with `+0x15c` taking a word the original
/// reads from its own uninitialised stack scratch (zero under the checker's
/// defined stack fill). Unless the mirror flag (bit 0 of `+0x216`) is set,
/// each component is mirrored about the matching `+0x160/0x164/0x168` value.
/// When the stage at `+0x130` is above 3 (unsigned) and the blend flag (bit 2
/// of `+0x1a8`) is set, the planar distance to `+0x140/0x144` picks one of two
/// smoothing paths for `+0x150/0x154/0x158/0x2c0`; otherwise `+0x2c0` is reset
/// to 1. `+0x2c0` is then clamped into `[0, 1]` and the current position is
/// snapshotted to `+0x2b0..0x2bc` and `+0x160..0x16c`.
///
/// A normalised distance over tables indexed by `aux[0x2b0]` (where `aux` is
/// the object at `+0x204`) is clamped into `[0, 1]` and handed, with the
/// `+0x60` weight and the table index, to the range hook (callee 4). Two
/// stack slots are offered to the locator hook again (callee 2, both pointer
/// arguments skipped in the comparison since they address each side's own
/// frame); its answer goes to the adjust hook (callee 3), and when that
/// answer's low byte is non-zero the three words the locator wrote are added
/// into `+0x150/0x154/0x158`. The score hook (callee 5) takes `a4`/`a5` and
/// returns its score in ST0.
///
/// The tail scales `a4` by the score unless the context selects mode `0xc0`
/// with bit 2 of `+0x26c`, folds the score into `a5` together with `a1..a3`,
/// wraps both slots into `[-pi, pi]`, advances the heading follower at
/// `+0x218` and the pitch follower at `+0x21c` towards them with a dead zone
/// (width from the game-filled rate when the mirror flag is clear, else a
/// wide constant), and zeroes `+0x220/0x224` while resetting `+0x228` to 1.
/// All float operations keep the original's operand order.
///
/// The return value is the `a5` pointer. Original: 0x00a24220 (thiscall, six
/// stack words).
lf_checker_rt::export!(thiscall, rw_00a24220(
    this: u32,
    a0: u32,
    a1b: u32,
    a2b: u32,
    a3b: u32,
    a4: u32,
    a5: u32,
) -> u32 {
    unsafe {
        const C_ONE: u32 = 0x00FE88E8;
        const C_TENTH: u32 = 0x00FE879C;
        const C_SMALL: u32 = 0x00FE870C;
        const C_BLEND: u32 = 0x00E96440;
        const C_THIRD: u32 = 0x00FE8800;
        const C_FIVE: u32 = 0x00FE8AD8;
        const C_FIFTH: u32 = 0x00FE87D0;
        const C_NEG_PI: u32 = 0x00FE8DC4;
        const C_TAU: u32 = 0x00FE8AEC;
        const C_PI: u32 = 0x00FE8AA0;
        const C_WIDE: u32 = 0x00FE8C58;
        const C_ABS_MASK: u32 = 0x00FE8F80;
        const C_QUARTER: u32 = 0x00FE87E4;
        const G_RATE: u32 = 0x011735BC;
        const T_LO: u32 = 0x00E9B840;
        const T_HI: u32 = 0x00E9B86C;
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
        unsafe fn absf(a: f32) -> f32 {
            unsafe { f32::from_bits(a.to_bits() & rd32(lf_checker_rt::relocated(C_ABS_MASK))) }
        }

        // Fetch the target triple through the locator hook.
        let tgt: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
        let mut x3 = rdf(tgt.wrapping_add(0x3a0));
        let mut x4 = rdf(tgt.wrapping_add(0x3a4));
        let mut x5 = rdf(tgt.wrapping_add(0x3a8));
        let fill = f32::from_bits(STACK_FILL_WORD);
        wrf(this.wrapping_add(0x150), x3);
        wrf(this.wrapping_add(0x154), x4);
        wrf(this.wrapping_add(0x158), x5);
        wrf(this.wrapping_add(0x15c), fill);
        if rd8(this.wrapping_add(0x216)) & 1 == 0 {
            let m0 = rdf(this.wrapping_add(0x168));
            let m2 = rdf(this.wrapping_add(0x160));
            let m1 = rdf(this.wrapping_add(0x164));
            x5 = sub(x5, m0);
            x3 = sub(x3, m2);
            x4 = sub(x4, m1);
            let n0 = add(m0, x5);
            let n2 = add(m2, x3);
            let n1 = add(m1, x4);
            wrf(this.wrapping_add(0x158), n0);
            wrf(this.wrapping_add(0x150), n2);
            wrf(this.wrapping_add(0x154), n1);
            wrf(this.wrapping_add(0x15c), fill);
        }

        // Blend stage.
        let mut x5b: f32;
        if rd32(this.wrapping_add(0x130)) > 3 && rd8(this.wrapping_add(0x1a8)) & 4 != 0 {
            let mut x6 = rdf(this.wrapping_add(0x150));
            let mut x7 = rdf(this.wrapping_add(0x154));
            let mut xa = sub(x7, rdf(this.wrapping_add(0x144)));
            let mut xb = sub(x6, rdf(this.wrapping_add(0x140)));
            x5b = cf(C_ONE);
            xa = mul(xa, xa);
            xb = mul(xb, xb);
            xa = add(xa, xb);
            xa = core::hint::black_box(xa).sqrt();
            if x5b > xa {
                // Close path: ease towards the goal triple (fall-through).
                let g1 = rdf(this.wrapping_add(0x2b4));
                let g0 = rdf(this.wrapping_add(0x2b8));
                let g2 = rdf(this.wrapping_add(0x2b0));
                x3 = rdf(this.wrapping_add(0x158));
                let rate = cf(C_TENTH);
                x3 = sub(x3, g0);
                x7 = sub(x7, g1);
                x6 = sub(x6, g2);
                x3 = mul(x3, rate);
                x6 = mul(x6, rate);
                x7 = mul(x7, rate);
                x3 = add(x3, g0);
                x7 = add(x7, g1);
                x6 = add(x6, g2);
                wrf(this.wrapping_add(0x15c), fill);
                let mut xc = cf(C_SMALL);
                wrf(this.wrapping_add(0x158), x3);
                wrf(this.wrapping_add(0x154), x7);
                wrf(this.wrapping_add(0x150), x6);
                let s = rdf(this.wrapping_add(0x2c0));
                xc = sub(xc, s);
                xc = mul(xc, rate);
                xc = add(xc, s);
                wrf(this.wrapping_add(0x2c0), xc);
            } else {
                // Far path: keep position, only ease the weight.
                let s = rdf(this.wrapping_add(0x2c0));
                let mut xc = sub(x5b, s);
                xc = mul(xc, cf(C_BLEND));
                xc = add(xc, s);
                wrf(this.wrapping_add(0x2c0), xc);
            }
            let g0 = rdf(this.wrapping_add(0x2b8));
            let mut xd = sub(rdf(this.wrapping_add(0x158)), g0);
            xd = mul(xd, cf(C_THIRD));
            xd = add(xd, g0);
            wrf(this.wrapping_add(0x158), xd);
        } else {
            x5b = cf(C_ONE);
            wr32(this.wrapping_add(0x2c0), 0x3f800000);
        }
        // Clamp the weight into [0, 1].
        let mut xw = rdf(this.wrapping_add(0x2c0));
        if !(x5b > xw) {
            xw = x5b;
        } else if !(xw > 0.0) {
            xw = 0.0;
        }
        wrf(this.wrapping_add(0x2c0), xw);
        // Snapshot position to the goal triple and the mirror triple.
        let p0 = rd32(this.wrapping_add(0x150));
        let p1 = rdf(this.wrapping_add(0x154));
        let p2 = rdf(this.wrapping_add(0x158));
        let p3 = rd32(this.wrapping_add(0x15c));
        wrf(this.wrapping_add(0x2b4), p1);
        wr32(this.wrapping_add(0x2b0), p0);
        wrf(this.wrapping_add(0x2b8), p2);
        wr32(this.wrapping_add(0x2bc), p3);
        wrf(this.wrapping_add(0x164), p1);
        wr32(this.wrapping_add(0x160), p0);
        wrf(this.wrapping_add(0x168), p2);
        wr32(this.wrapping_add(0x16c), p3);

        // Normalised table distance, clamped into [0, 1].
        let sv150 = rdf(this.wrapping_add(0x150));
        let mut xd = sub(rdf(this.wrapping_add(0x40)), sv150);
        let sv154 = rdf(this.wrapping_add(0x154));
        let mut xe = sub(rdf(this.wrapping_add(0x44)), sv154);
        let mut xf = sub(rdf(this.wrapping_add(0x48)), rdf(this.wrapping_add(0x158)));
        xd = mul(xd, xd);
        xe = mul(xe, xe);
        xf = mul(xf, xf);
        xe = add(xe, xd);
        let aux = rd32(this.wrapping_add(0x204));
        let tidx = rd32(aux.wrapping_add(0x2b0));
        xe = add(xe, xf);
        let tbase_lo = lf_checker_rt::relocated(T_LO);
        let tbase_hi = lf_checker_rt::relocated(T_HI);
        let t1 = rdf(tbase_lo.wrapping_add(tidx.wrapping_mul(4)));
        let mut xg = sub(rdf(tbase_hi.wrapping_add(tidx.wrapping_mul(4))), t1);
        xe = core::hint::black_box(xe).sqrt();
        xe = sub(xe, t1);
        xe = div(xe, xg);
        let mut xcl: f32;
        if !(x5b > xe) {
            xcl = x5b;
        } else if xe > 0.0 {
            xcl = xe;
        } else {
            xcl = 0.0;
        }
        let w60 = rdf(this.wrapping_add(0x60));

        // Locator out-call through two frame slots, then the adjust hook.
        // Slot 0 stands in for the original's first slot (never read back);
        // slots 1..3 receive the three words the locator writes.
        let mut frame = [0u32; 4];
        let p1slot = frame.as_mut_ptr() as u32;
        let p2slot = frame.as_mut_ptr().wrapping_add(1) as u32;
        // NOTE: the original pushes P1 first, so arg0 is the second slot.
        let loc_ans: u32 = lf_checker_rt::callee_cdecl!(2, u32, p2slot, p1slot);
        let adj: u32 = lf_checker_rt::callee_cdecl!(3, u32, loc_ans);
        if (adj & 0xff) != 0 {
            let w0 = f32::from_bits(frame[1]);
            let w1 = f32::from_bits(frame[2]);
            let w2 = f32::from_bits(frame[3]);
            wrf(this.wrapping_add(0x150), add(w0, rdf(this.wrapping_add(0x150))));
            wrf(this.wrapping_add(0x154), add(w1, rdf(this.wrapping_add(0x154))));
            wrf(this.wrapping_add(0x158), add(w2, rdf(this.wrapping_add(0x158))));
        }

        // Range hook, then the score hook.
        let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, this, tidx, xcl.to_bits(), w60.to_bits());
        let score: f32 = lf_checker_rt::callee_thiscall!(5, f32, this, a4, a5);

        // Scale a4 unless the context selects the exempt mode.
        let ctx = a0;
        let mode = rd32(ctx.wrapping_add(0x28)) & 0x3c0;
        let mut x5c = cf(C_FIVE);
        if !(mode == 0xc0 && rd8(ctx.wrapping_add(0x26c)) & 4 != 0) {
            if x5c > score {
                let mut s = mul(score, cf(C_FIFTH));
                s = mul(s, rdf(a4));
                wrf(a4, s);
            }
        }
        // Fold the score into a5 with a1..a3.
        let mut ya = sub(rdf(this.wrapping_add(0x144)), rdf(this.wrapping_add(0x44)));
        let mut yb = sub(rdf(this.wrapping_add(0x140)), rdf(this.wrapping_add(0x40)));
        let mut yc = sub(rdf(this.wrapping_add(0x148)), rdf(this.wrapping_add(0x48)));
        ya = mul(ya, ya);
        yb = mul(yb, yb);
        yc = mul(yc, yc);
        ya = add(ya, yb);
        ya = add(ya, yc);
        let a1f = f32::from_bits(a1b);
        let mut yd = core::hint::black_box(ya).sqrt();
        if !(a1f > yd) {
            yd = a1f;
        }
        yd = add(yd, score);
        yd = div(yd, score);
        let mut za = mul(yd, f32::from_bits(a3b));
        let mut zb = mul(yd, f32::from_bits(a2b));
        za = add(za, rdf(a5));
        wrf(a5, za);
        let mut zc = sub(rdf(a4), zb);
        let x7c = cf(C_NEG_PI);
        let x4c = cf(C_TAU);
        let x6c = cf(C_PI);
        wrf(a4, zc);
        if x7c > zc {
            zc = add(zc, x4c);
            wrf(a4, zc);
        } else if !(zc > x6c) {
            // within range: keep the stored value
        } else {
            zc = sub(zc, x4c);
            wrf(a4, zc);
        }

        // Heading follower at +0x218 with a dead zone.
        let rate = lf_checker_rt::global::<f32>(G_RATE).read();
        let mut dead = mul(rate, x5c);
        if rd8(this.wrapping_add(0x216)) & 1 != 0 {
            dead = cf(C_WIDE);
        }
        let htarget = rdf(a4);
        let mut hcur = rdf(this.wrapping_add(0x218));
        let hdiff = sub(htarget, hcur);
        // Both arms zero the work register first; the zero below doubles as
        // the clamp floor for the anchor weight and the final reach test.
        let tail_zero = 0.0f32;
        if dead > absf(hdiff) {
            wrf(this.wrapping_add(0x218), htarget);
        } else if !(tail_zero > hdiff) {
            hcur = add(hcur, dead);
            wrf(this.wrapping_add(0x218), hcur);
        } else {
            hcur = sub(hcur, dead);
            wrf(this.wrapping_add(0x218), hcur);
        }
        // Wrap a5 into [-pi, pi].
        let mut pa = rdf(a5);
        let pdiff = sub(pa, rdf(this.wrapping_add(0x21c)));
        if pdiff > x6c {
            pa = sub(pa, x4c);
            wrf(a5, pa);
        } else if x7c > pdiff {
            pa = add(pa, x4c);
            wrf(a5, pa);
        }
        // Pitch follower at +0x21c against the near/far anchor.
        let ecx0 = rd32(this.wrapping_add(0x204));
        let edx0 = rd32(ecx0.wrapping_add(0x20));
        let anch = if edx0 == 0 {
            ecx0.wrapping_add(0x10)
        } else {
            edx0.wrapping_add(0x30)
        };
        let mut qa = sub(sv150, rdf(anch));
        let mut qb = sub(sv154, rdf(anch.wrapping_add(4)));
        // NOTE: sv150/sv154 are the pre-call saves, matching the original.
        let mut qx0 = mul(qb, qb);
        let mut qx1 = mul(qa, qa);
        qx0 = add(qx0, qx1);
        let mut x4d = cf(C_ONE);
        let qd = core::hint::black_box(qx0).sqrt();
        let mut qcur = rdf(this.wrapping_add(0x2a0));
        if x4d > qd {
            let mut qn = sub(cf(C_QUARTER), qcur);
            qn = mul(qn, cf(C_TENTH));
            qn = add(qn, qcur);
            wrf(this.wrapping_add(0x2a0), qn);
            if !(rd32(this.wrapping_add(0x130)) > 3 && rd8(this.wrapping_add(0x1a8)) & 4 != 0) {
                wr32(this.wrapping_add(0x2a0), 0x3e800000);
            }
        } else {
            let mut qn = sub(x4d, qcur);
            qn = mul(qn, cf(C_TENTH));
            qn = add(qn, qcur);
            wrf(this.wrapping_add(0x2a0), qn);
        }
        // Clamp the anchor weight into [tail_zero, 1].
        let mut qw = rdf(this.wrapping_add(0x2a0));
        if !(x4d > qw) {
            qw = x4d;
        } else if !(qw > tail_zero) {
            qw = tail_zero;
        }
        wrf(this.wrapping_add(0x2a0), qw);
        let mut pcur = rdf(this.wrapping_add(0x21c));
        let anchor_w = rdf(this.wrapping_add(0x2c0));
        let ptgt = rdf(a5);
        let pdiff2 = sub(ptgt, pcur);
        let reach = mul(anchor_w, dead);
        if reach > absf(pdiff2) {
            wrf(this.wrapping_add(0x21c), ptgt);
        } else if !(tail_zero > pdiff2) {
            pcur = add(pcur, reach);
            wrf(this.wrapping_add(0x21c), pcur);
        } else {
            pcur = sub(pcur, reach);
            wrf(this.wrapping_add(0x21c), pcur);
        }
        wr32(this.wrapping_add(0x220), 0);
        wr32(this.wrapping_add(0x224), 0);
        wr32(this.wrapping_add(0x228), 0x3f800000);
        a5
    }
});
