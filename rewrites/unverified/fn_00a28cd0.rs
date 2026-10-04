// original: 0x00a28cd0 task_timer_blend (proposed)

/// Update a ped task's blend weight and retry timer from scripted tuning
/// values, a virtual timer query, and two global counters.
///
/// `this` is the task (thiscall, no stack arguments). The weight at
/// `+WEIGHT_OFF` is refreshed twice, each time as the id 1 callee's float
/// answer (cdecl, five stack words: one computed word plus four tuning
/// globals) added to the stored weight and clamped into `[0, 1]` against a
/// tuning ceiling: first with the absolute masked difference of the pair at
/// `+SPAN_A`/`+SPAN_B`, then with the timer at `+TIMER_OFF`. Between the two
/// refreshes the virtual slot `VT_SLOT` of the object at `+SUB_OFF` is
/// queried (thiscall, float answer); when the stored stamp at `+STAMP_OFF`
/// exceeds both zero and the timer by that answer, the global tick at
/// `TICK_GLOBAL` and the new stamp are latched into `+LATCH_OFF`/`+TIMER_OFF`.
/// When the unsigned tick delta fails the tuning limit at `LIMIT_GLOBAL`, or
/// the timer is not positive, the short exit clears the timer, flags the
/// status byte `FLAG_GLOBAL` off and stores the query answer into the stamp.
/// Otherwise the id 3 ratio callee runs (cdecl, three stack words: the exact
/// unsigned tick ratio plus two tuning globals), its answer lands in the
/// ratio global `RATIO_GLOBAL`, the status byte is flagged on, the ratio of
/// the second id 1 answer over a tuning span is stored after it, and the
/// query answer is kept in the stamp slot (both exits store it: the first
/// id 1 answer's scratch slot is reused by the query answer first).
///
/// All float comparisons repeat the original's ordered-compare shape
/// (`ja`/`jbe` over `comiss`), including which side wins on NaN; the two
/// unsigned-to-float conversions are exact through 64-bit floats. The
/// function has no designed return value (EAX ends as callee residue on the
/// long path), so the contract compares everything except it.
///
/// Original: 0x00a28cd0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00a28cd0(this: u32) -> u32 {
    unsafe {
        const WEIGHT_OFF: u32 = 0x0074;
        const SPAN_A: u32 = 0x0208;
        const SPAN_B: u32 = 0x020c;
        const SUB_OFF: u32 = 0x0204;
        const VT_SLOT: u32 = 0x00fc;
        const STAMP_OFF: u32 = 0x024c;
        const TIMER_OFF: u32 = 0x0250;
        const LATCH_OFF: u32 = 0x0254;
        const ABS_MASK: u32 = 0x7fff_ffff;
        const TUNE_A: u32 = 0x0103_c0dc;
        const TUNE_B: u32 = 0x0103_c0e0;
        const TUNE_C: u32 = 0x0103_c0e4;
        const TUNE_D: u32 = 0x0103_c0e8;
        const TUNE_E: u32 = 0x0103_c0f4;
        const TUNE_F: u32 = 0x0103_c0f8;
        const TUNE_G: u32 = 0x0103_c0fc;
        const TUNE_H: u32 = 0x0103_c100;
        const TUNE_I: u32 = 0x0103_c104;
        const TUNE_J: u32 = 0x0103_c108;
        const TUNE_CEIL: u32 = 0x0103_c10c;
        const LIMIT_GLOBAL: u32 = 0x0103_c0f0;
        const TICK_GLOBAL: u32 = 0x0117_35c4;
        const FLAG_GLOBAL: u32 = 0x012d_d5e2;
        const RATIO_GLOBAL: u32 = 0x012d_d5e4;
        const RATIO2_GLOBAL: u32 = 0x012d_d5e8;

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
        unsafe fn tune(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn tunef(va: u32) -> f32 {
            unsafe { f32::from_bits(tune(va)) }
        }
        /// The original's ceiling-then-clamp: `v = min(v, ceil)` with the
        /// compare ordered so NaN takes the ceiling, then clamp into
        /// `[0, 1]` with NaN passing through.
        #[inline(always)]
        fn clamp01(mut v: f32, ceil: f32) -> f32 {
            if !(ceil > v) {
                v = ceil;
            }
            if 0.0 > v {
                v = 0.0;
            } else if v > 1.0 {
                v = 1.0;
            }
            v
        }

        let sub = rd32(this.wrapping_add(SUB_OFF));
        let span = rdf(this.wrapping_add(SPAN_A)) - rdf(this.wrapping_add(SPAN_B));
        let masked = span.to_bits() & ABS_MASK;
        let ans1: f32 = lf_checker_rt::callee_cdecl!(
            1,
            f32,
            masked,
            tune(TUNE_A),
            tune(TUNE_B),
            tune(TUNE_C),
            tune(TUNE_D)
        );
        let weight = rdf(this.wrapping_add(WEIGHT_OFF));
        let ceil = tunef(TUNE_CEIL);
        wrf(this.wrapping_add(WEIGHT_OFF), clamp01(ans1 + weight, ceil));

        // Virtual query through the sub-object, exactly like the original:
        // both sides land on the same planted stub.
        let vt = rd32(sub);
        let slot = rd32(vt.wrapping_add(VT_SLOT));
        let query: extern "thiscall" fn(u32) -> f32 =
            unsafe { core::mem::transmute(slot as usize) };
        let q = query(sub);

        let stamp = rdf(this.wrapping_add(STAMP_OFF));
        let tick = tune(TICK_GLOBAL);
        let d = stamp - q;
        let timer = rdf(this.wrapping_add(TIMER_OFF));
        if d > 0.0 && d > timer {
            wr32(this.wrapping_add(LATCH_OFF), tick);
            wrf(this.wrapping_add(TIMER_OFF), d);
        }
        let delta = tick.wrapping_sub(rd32(this.wrapping_add(LATCH_OFF)));
        let limit = tune(LIMIT_GLOBAL);
        let timer2 = rdf(this.wrapping_add(TIMER_OFF));
        if delta >= limit || !(timer2 > 0.0) {
            wr32(this.wrapping_add(TIMER_OFF), 0);
            unsafe { (lf_checker_rt::relocated(FLAG_GLOBAL) as *mut u8).write(0) };
            wrf(this.wrapping_add(STAMP_OFF), q);
            return 0;
        }
        let ratio = (delta as f32) / (limit as f32);
        let r3: f32 =
            lf_checker_rt::callee_cdecl!(3, f32, ratio.to_bits(), tune(TUNE_I), tune(TUNE_J));
        wrf(lf_checker_rt::relocated(RATIO2_GLOBAL), r3);
        let timer3 = rdf(this.wrapping_add(TIMER_OFF));
        let ans2: f32 = lf_checker_rt::callee_cdecl!(
            1,
            f32,
            timer3.to_bits(),
            tune(TUNE_E),
            tune(TUNE_F),
            tune(TUNE_G),
            tune(TUNE_H)
        );
        let weight2 = rdf(this.wrapping_add(WEIGHT_OFF));
        wrf(this.wrapping_add(WEIGHT_OFF), clamp01(ans2 + weight2, tunef(TUNE_CEIL)));
        let span2 = tunef(TUNE_H) - tunef(TUNE_G);
        unsafe { (lf_checker_rt::relocated(FLAG_GLOBAL) as *mut u8).write(1) };
        wrf(lf_checker_rt::relocated(RATIO_GLOBAL), ans2 / span2);
        // The stamp re-reads the same scratch slot the first id 1 answer
        // used, but the id 2 answer overwrote it in between, so both exits
        // store the query answer.
        wrf(this.wrapping_add(STAMP_OFF), q);
        0
    }
});
