// original: 0x00b300a0 ped_task_state_switch (proposed)

/// Per-state readiness check for a task record (thiscall, no stack args).
///
/// `this` points at a record headed by a state dword. The prologue bails out
/// with 1 when the record is detached (`FLAG_OFF` set but no sub-object) or
/// when its armed timer (`TIMER_BASE + TIMER_SPAN`, unsigned) has not passed
/// the global tick. Otherwise the state (minus 2, unsigned, above `0x1B`
/// rejected with 0) dispatches to one case:
/// state 2 compares a mode field and rejects two values; states 3-11 run a
/// fetch/identify callee pair and reject when the answer equals the state;
/// state 14 checks enable flags and a deadline against the global tick;
/// state 16 compares a float against a constant threshold; state 17 keeps
/// the record when the squared distance between two resolved points exceeds
/// a constant; states 18/24/25 require a null link; state 20 probes the
/// sub-object twice and rejects when either probe answers; state 29 probes
/// once when a global stage is at least 2. Every other state returns 0.
/// Returns 1 when the record is ready (or exempt), else 0. Only the low byte
/// of the result is significant.
///
/// Original: thiscall, result in AL with stale upper bytes, float
/// comparisons keep the original's unordered (NaN) behaviour.
lf_checker_rt::export!(thiscall, rw_00b300a0(this: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0x26;
        const SUB_OFF: u32 = 0x28;
        const LINK_OFF: u32 = 0x2C;
        const TIMER_BASE: u32 = 0x30;
        const TIMER_SPAN: u32 = 0x34;
        const MODE_OFF: u32 = 0x28;
        const MODE_MASK: u32 = 0x3C0;
        const MODE_WANT: u32 = 0xC0;
        const MODE_WANT_F: u32 = 0x80;
        const PT_SLOT: u32 = 0x20;
        const LOCAL_PT: u32 = 0x10;
        const FAR_PT: u32 = 0x30;
        const FETCH_THIS_DELTA: u32 = 0x2B0;
        const MODE_VALUE_OFF: u32 = 0xB80;
        const PROBE_BASE_OFF: u32 = 0x224;
        const PROBE_THIS_DELTA: u32 = 0x2E0;
        const PROBE_ARG_A: u32 = 0x38E;
        const PROBE_ARG_B: u32 = 0x38F;
        const ENABLE_BYTE: u32 = 0x211;
        const ENABLE_BITS: u32 = 0x264;
        const ENABLE_MASK: u32 = 0x20_0000;
        const DEADLINE_OFF: u32 = 0xD3C;
        const DEADLINE_BIAS: u32 = 0x7530;
        const KIND_OFF: u32 = 0x1304;
        const KIND_WANT: u32 = 4;
        const LEVEL_OFF: u32 = 0x1ED4;
        const STAGE_NEED: i32 = 2;
        const TICK_GLOBAL: u32 = 0x0117_35B4;
        const STAGE_GLOBAL: u32 = 0x011D_6FD4;
        const DIST_LIMIT_GLOBAL: u32 = 0x00FE_8B28;
        const LEVEL_LIMIT_GLOBAL: u32 = 0x00FE_879C;
        const CALLEE_FETCH: u32 = 1;
        const CALLEE_IDENTIFY: u32 = 2;
        const CALLEE_PROBE: u32 = 3;
        const CALLEE_GATED: u32 = 4;

        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
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
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { (a as *const f32).read_unaligned() }
        }

        if (this.wrapping_add(FLAG_OFF) as *const u8).read() != 0
            && rd32(this.wrapping_add(SUB_OFF)) == 0
        {
            return 1;
        }
        let span = rd32(this.wrapping_add(TIMER_SPAN));
        let tick = rd32(lf_checker_rt::relocated(TICK_GLOBAL));
        if span != 0
            && rd32(this.wrapping_add(TIMER_BASE)).wrapping_add(span) <= tick
        {
            return 1;
        }
        let state = rd32(this);
        if state.wrapping_sub(2) > 0x1B {
            return 0;
        }
        match state {
            2 => {
                let sub = rd32(this.wrapping_add(SUB_OFF));
                if sub == 0 {
                    return 1;
                }
                if rd32(sub.wrapping_add(MODE_OFF)) & MODE_MASK != MODE_WANT {
                    return 1;
                }
                let v = rd32(sub.wrapping_add(MODE_VALUE_OFF));
                if v == 3 || v == 4 { 0 } else { 1 }
            }
            3..=11 => {
                let sub = rd32(this.wrapping_add(SUB_OFF));
                if sub == 0 {
                    return 1;
                }
                if rd32(sub.wrapping_add(MODE_OFF)) & MODE_MASK != MODE_WANT {
                    return 1;
                }
                let fetched: u32 = lf_checker_rt::callee_thiscall!(
                    CALLEE_FETCH,
                    u32,
                    sub.wrapping_add(FETCH_THIS_DELTA)
                );
                let id: u32 =
                    lf_checker_rt::callee_cdecl!(CALLEE_IDENTIFY, u32, fetched);
                if id == state { 0 } else { 1 }
            }
            14 => {
                let sub = rd32(this.wrapping_add(SUB_OFF));
                if sub == 0 {
                    return 1;
                }
                if rd32(sub.wrapping_add(MODE_OFF)) & MODE_MASK != MODE_WANT {
                    return 1;
                }
                if (sub.wrapping_add(ENABLE_BYTE) as *const u8).read() == 0
                    && rd32(sub.wrapping_add(ENABLE_BITS)) & ENABLE_MASK == 0
                {
                    return 1;
                }
                let tick = rd32(lf_checker_rt::relocated(TICK_GLOBAL));
                if rd32(sub.wrapping_add(DEADLINE_OFF)).wrapping_add(DEADLINE_BIAS)
                    >= tick
                {
                    0
                } else {
                    1
                }
            }
            16 => {
                let sub = rd32(this.wrapping_add(SUB_OFF));
                if sub == 0 {
                    return 1;
                }
                if rd32(sub.wrapping_add(MODE_OFF)) & MODE_MASK != MODE_WANT_F {
                    return 1;
                }
                if rd32(sub.wrapping_add(KIND_OFF)) != KIND_WANT {
                    return 1;
                }
                let limit = rdf(lf_checker_rt::relocated(LEVEL_LIMIT_GLOBAL));
                let level = rdf(sub.wrapping_add(LEVEL_OFF));
                if !(limit >= level) { 0 } else { 1 }
            }
            17 => {
                let subobj = rd32(this.wrapping_add(SUB_OFF));
                if subobj == 0 {
                    return 1;
                }
                let link = rd32(this.wrapping_add(LINK_OFF));
                if link == 0 {
                    return 1;
                }
                let qb = rd32(link.wrapping_add(PT_SLOT));
                let pb = if qb != 0 {
                    qb.wrapping_add(FAR_PT)
                } else {
                    link.wrapping_add(LOCAL_PT)
                };
                let qa = rd32(subobj.wrapping_add(PT_SLOT));
                let pa = if qa != 0 {
                    qa.wrapping_add(FAR_PT)
                } else {
                    subobj.wrapping_add(LOCAL_PT)
                };
                let dx = sub(rdf(pa), rdf(pb));
                let dy = sub(rdf(pa.wrapping_add(4)), rdf(pb.wrapping_add(4)));
                let dz = sub(rdf(pa.wrapping_add(8)), rdf(pb.wrapping_add(8)));
                let mut dist = add(mul(dy, dy), mul(dx, dx));
                dist = add(dist, mul(dz, dz));
                let limit = rdf(lf_checker_rt::relocated(DIST_LIMIT_GLOBAL));
                if !(dist > limit) { 0 } else { 1 }
            }
            18 | 24 | 25 => {
                if rd32(this.wrapping_add(LINK_OFF)) != 0 {
                    0
                } else {
                    1
                }
            }
            20 => {
                let sub = rd32(this.wrapping_add(SUB_OFF));
                if sub == 0 {
                    return 1;
                }
                if rd32(sub.wrapping_add(MODE_OFF)) & MODE_MASK != MODE_WANT {
                    return 1;
                }
                let probe = rd32(sub.wrapping_add(PROBE_BASE_OFF))
                    .wrapping_add(PROBE_THIS_DELTA);
                let r1: u32 = lf_checker_rt::callee_thiscall!(
                    CALLEE_PROBE,
                    u32,
                    probe,
                    PROBE_ARG_A,
                    0u32
                );
                if (r1 as u8) != 0 {
                    return 0;
                }
                let r2: u32 = lf_checker_rt::callee_thiscall!(
                    CALLEE_PROBE,
                    u32,
                    probe,
                    PROBE_ARG_B,
                    0u32
                );
                if (r2 as u8) != 0 { 0 } else { 1 }
            }
            29 => {
                let stage = rd32(lf_checker_rt::relocated(STAGE_GLOBAL)) as i32;
                if stage < STAGE_NEED {
                    return 0;
                }
                let sub = rd32(this.wrapping_add(SUB_OFF));
                if sub == 0 {
                    return 0;
                }
                if rd32(sub.wrapping_add(MODE_OFF)) & MODE_MASK != MODE_WANT {
                    return 0;
                }
                let r: u32 =
                    lf_checker_rt::callee_thiscall!(CALLEE_GATED, u32, sub);
                if (r as u8) != 0 { 1 } else { 0 }
            }
            _ => 0,
        }
    }
});
