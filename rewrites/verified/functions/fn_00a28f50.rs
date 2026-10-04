// original: 0x00a28f50 task_aim_update (proposed)

/// Update a ped task's aim blend and weight from a target triple, tuning
/// tables and a mode flag.
///
/// `this` is the task (thiscall, two stack words: `a1` a state block
/// pointer, `a2` a float scale as bits). A null `a1` returns at once; a set
/// flag bit at `+FLAG_OFF` skips to the finish. Otherwise the mode word at
/// `a1+MODE_OFF` decides: 0, 1 or 2 run the main path, anything else skips
/// to the late path. The main path queries virtual slot `VT_SLOT` of
/// `a1[0]` (thiscall on `a1`, one stack word pointing at scratch; the
/// scratch address is skipped in the comparison and its zeroed contents
/// snapshotted), takes the returned triple dotted with the triple at
/// `a1[SUB_OFF]+0x10` in the order `(r1*s1 + r0*s0) + r2*s2`, and, when that
/// exceeds the table value selected by whether the mode is 2, scales the
/// overshoot into the aim at `+AIM_OFF`, else relaxes the aim toward a
/// constant. It then runs the id 2 blend callee (cdecl, five stack words:
/// the dot product, a mode-dependent factor, three tuning globals), adds
/// the float answer to the weight at `+WEIGHT_OFF` and clamps into `[0, 1]`
/// against a tuning ceiling, records whether the answer was ordered-above
/// zero and, when the unclamped sum is ordered-above a reference, raises the
/// flag byte `FLAG_GLOBAL` to 1, and stores the answer-over-span ratio in
/// `RATIO_GLOBAL`. The late path keeps the lesser of the aim and its table
/// value (table wins ties and NaN). The finish always stores
/// aim-minus-base (`+BASE_OFF`) into `+OUT_OFF`.
///
/// Ordered-compare shapes repeat the original exactly, including which side
/// wins on NaN. There is no designed return value (the null-arg exit leaks
/// entry EAX), so the contract compares everything except it.
///
/// Original: 0x00a28f50 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00a28f50(this: u32, a1: u32, a2bits: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0x0216;
        const MODE_OFF: u32 = 0x1304;
        const SUB_OFF: u32 = 0x0020;
        const VT_SLOT: u32 = 0x00ec;
        const AIM_OFF: u32 = 0x0180;
        const WEIGHT_OFF: u32 = 0x0074;
        const BASE_OFF: u32 = 0x02cc;
        const OUT_OFF: u32 = 0x0060;
        const INDEX_GLOBAL: u32 = 0x012b_d198;
        const TABLE_A: u32 = 0x0103_b76c;
        const TABLE_B: u32 = 0x0103_b774;
        const TABLE_C: u32 = 0x0103_b77c;
        const TABLE_D: u32 = 0x0103_b784;
        const TABLE_E: u32 = 0x0103_b79c;
        const TUNE_F: u32 = 0x0103_b8f8;
        const TUNE_G: u32 = 0x0103_b8fc;
        const TUNE_H: u32 = 0x0103_b900;
        const TUNE_I: u32 = 0x0103_b904;
        const TUNE_CEIL: u32 = 0x0103_b920;
        const SMOOTH_GLOBAL: u32 = 0x0103_c760;
        const FLAG_GLOBAL: u32 = 0x0103_b76b;
        const ABOVE_GLOBAL: u32 = 0x012b_d192;
        const RATIO_GLOBAL: u32 = 0x012b_d1b4;
        const SCALE_C: u32 = 0x00fe_8734;
        const RELAX_C: u32 = 0x00fe_8b64;
        const REF_C: u32 = 0x00e9_ad88;

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
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn tune(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn tunef(va: u32) -> f32 {
            unsafe { f32::from_bits(tune(va)) }
        }
        #[inline(always)]
        unsafe fn tabf(va: u32, idx: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(va).wrapping_add(idx))) }
        }
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
        #[inline(always)]
        unsafe fn finish(this: u32) {
            unsafe {
                let aim = rdf(this.wrapping_add(AIM_OFF));
                let base = rdf(this.wrapping_add(BASE_OFF));
                wrf(this.wrapping_add(OUT_OFF), aim - base);
            }
        }

        if a1 == 0 {
            return 0;
        }
        if rd8(this.wrapping_add(FLAG_OFF)) & 1 != 0 {
            finish(this);
            return 0;
        }
        let mode = rd32(a1.wrapping_add(MODE_OFF));
        let main = mode == 0 || mode == 2 || mode == 1;
        if main {
            let is_two = mode == 2;
            let vt = rd32(a1);
            let slot = rd32(vt.wrapping_add(VT_SLOT));
            let esi = rd32(a1.wrapping_add(SUB_OFF));
            let mut scratch = [0u32; 3];
            let query: extern "thiscall" fn(u32, u32) -> u32 =
                unsafe { core::mem::transmute(slot as usize) };
            let ret = query(a1, (&mut scratch).as_mut_ptr() as u32);
            let r0 = rdf(ret);
            let r1 = rdf(ret.wrapping_add(4));
            let p0 = r0 * rdf(esi.wrapping_add(0x10));
            let p1 = r1 * rdf(esi.wrapping_add(0x14));
            let mut dot = p1 + p0;
            let r2 = rdf(ret.wrapping_add(8));
            let p2 = r2 * rdf(esi.wrapping_add(0x18));
            dot = dot + p2;
            let idx = tune(INDEX_GLOBAL).wrapping_mul(4);
            let tab = if is_two { TABLE_B } else { TABLE_A };
            let x1 = tabf(tab, idx);
            let diff = dot - x1;
            if diff > 0.0 {
                let e = tabf(TABLE_E, idx);
                let t1 = e * diff;
                let a2 = f32::from_bits(a2bits);
                let t2 = a2 * tunef(SCALE_C);
                let t3 = t1 * t2;
                let aim = rdf(this.wrapping_add(AIM_OFF));
                wrf(this.wrapping_add(AIM_OFF), t3 + aim);
            } else {
                let aim = rdf(this.wrapping_add(AIM_OFF));
                let t0 = tunef(RELAX_C) - aim;
                let t1 = t0 * tunef(SMOOTH_GLOBAL);
                wrf(this.wrapping_add(AIM_OFF), t1 + aim);
            }
            let arg1 = if is_two {
                let c = tabf(TABLE_C, idx);
                c * tunef(TUNE_F)
            } else {
                tunef(TUNE_F)
            };
            let ans: f32 = lf_checker_rt::callee_cdecl!(
                2,
                f32,
                dot.to_bits(),
                arg1.to_bits(),
                tune(TUNE_G),
                tune(TUNE_H),
                tune(TUNE_I)
            );
            let w = rdf(this.wrapping_add(WEIGHT_OFF));
            let sum = ans + w;
            wrf(this.wrapping_add(WEIGHT_OFF), clamp01(sum, tunef(TUNE_CEIL)));
            let span = tunef(TUNE_I) - tunef(TUNE_H);
            let above = u8::from(ans > 0.0);
            let mut flag = rd8(lf_checker_rt::relocated(FLAG_GLOBAL));
            if sum > tunef(REF_C) {
                flag = 1;
            }
            wrf(lf_checker_rt::relocated(RATIO_GLOBAL), ans / span);
            unsafe {
                (lf_checker_rt::relocated(ABOVE_GLOBAL) as *mut u8).write(above);
                (lf_checker_rt::relocated(FLAG_GLOBAL) as *mut u8).write(flag);
            }
        }
        let idx2 = tune(INDEX_GLOBAL).wrapping_mul(4);
        let t = tabf(TABLE_D, idx2);
        let mut aim = rdf(this.wrapping_add(AIM_OFF));
        if !(t > aim) {
            aim = t;
        }
        wrf(this.wrapping_add(AIM_OFF), aim);
        finish(this);
        0
    }
});
